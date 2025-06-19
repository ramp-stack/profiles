use std::collections::{BTreeSet, BTreeMap};
use std::sync::LazyLock;
use std::time::Duration;

use pelican_ui::runtime::{Services, Service, ThreadContext, async_trait, self};
use pelican_ui::hardware;
use pelican_ui::State;
use pelican_ui::air::{Request, air, Service as AirService};

use air::orange_name::OrangeName;
use air::storage::{PublicItem, Filter};
use air::Id;
use serde::{Serialize, Deserialize};

static PROFILE: LazyLock<Id> = LazyLock::new(|| Id::hash(&"ProfileV1".to_string()));

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Profiles(pub BTreeMap<OrangeName, Profile>);

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Name(pub Option<OrangeName>);

#[derive(Serialize, Deserialize, Debug)]
pub enum ProfileRequest {
    Remove(OrangeName),
    Add(OrangeName),
    InsertField(String, String),
    RemoveField(String),
}

pub type Profile = BTreeMap<String, String>;

pub struct ProfileService{
    listening: BTreeSet<OrangeName>,
    profile: Option<Profile>,
    id: Option<Id>
}

impl ProfileService {
    pub fn handle(&mut self, request: ProfileRequest) {
        match request {
            ProfileRequest::Remove(name) => {self.listening.remove(&name);},
            ProfileRequest::Add(name) => {self.listening.insert(name);},
            ProfileRequest::InsertField(key, value) => {self.profile.as_mut().map(|p| p.insert(key, value));},
            ProfileRequest::RemoveField(key) => {self.profile.as_mut().map(|p| p.remove(&key));},
        }
    }
}

#[derive(Debug)]
struct MissingOrangeName;
impl std::error::Error for MissingOrangeName {}
impl std::fmt::Display for MissingOrangeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {write!(f, "{:?}", self)}
}

impl Services for ProfileService {}

#[async_trait]
impl Service for ProfileService {
    type Send = (OrangeName, Profile, bool);
    type Receive = ProfileRequest;

    async fn new(_hardware: &mut hardware::Context) -> Self {
        ProfileService{
            listening: BTreeSet::new(),
            profile: None,
            id: None
        }
    }

    async fn run(&mut self, ctx: &mut ThreadContext<Self::Send, Self::Receive>) -> Result<Option<Duration>, runtime::Error> {
        let mut mutated = false;
        println!("Errored");
        if let Some(name) = ctx.hardware.cache.get::<Option<OrangeName>>("OrangeName").await {
            if self.profile.is_none() {
                let item = ctx.blocking_request::<AirService>(Request::ReadPublic(Filter::new(None, Some(name.clone()), Some(*PROFILE), None))).await?.read_public().pop();
                let profile = item.as_ref().and_then(|i| serde_json::from_slice(&i.2.payload).ok());
                mutated = profile.is_none();
                self.profile = Some(profile.unwrap_or(BTreeMap::new()));
                self.id = item.as_ref().map(|i| i.0);
            }

            while let Some((_, request)) = ctx.get_request() {
                mutated = mutated || (matches!(request, ProfileRequest::InsertField(_,_)) || matches!(request, ProfileRequest::RemoveField(_)));
                self.handle(request);
            }

            let profile = self.profile.as_ref().unwrap();

            if mutated {
                let item = PublicItem {
                    protocol: *PROFILE,
                    header: vec![],
                    payload: serde_json::to_vec(&profile).unwrap(),
                };
                match self.id {
                    Some(id) => {ctx.blocking_request::<AirService>(Request::UpdatePublic(id, item)).await?;},
                    None => {self.id = Some(ctx.blocking_request::<AirService>(Request::CreatePublic(item)).await?.create_public());}
                }
            }
            ctx.callback((name, profile.clone(), true));

            let mut requests: Vec<_> = self.listening.iter().map(|name| ctx.request::<AirService>(Request::ReadPublic(Filter::new(None, Some(name.clone()), Some(*PROFILE), None)))).collect();
            loop {
                let mut error = None;
                requests.retain(|request| {
                    match ctx.check_request(request) {
                        Some(Ok(processed)) => {
                            if let Some((name, profile)) = processed.read_public().pop().and_then(|i|
                                Some((i.1, serde_json::from_slice::<Profile>(&i.2.payload).ok()?))
                            ) { ctx.callback((name, profile, false)); }
                            false
                        },
                        Some(Err(e)) => {error = Some(e); false},
                        None => true
                    }
                });
                if let Some(error) = error {return Err(error.into());}
                if requests.is_empty() {break}
            }
        }

        Ok(Some(Duration::from_secs(5)))
    }

    fn callback(state: &mut State, response: Self::Send) {
        let mut profiles = state.get::<Profiles>().0;
        if response.2 {state.set(&Name(Some(response.0.clone())));}
        profiles.insert(response.0, response.1);
        state.set(&Profiles(profiles));
    }
}
