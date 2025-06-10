use std::collections::{BTreeSet, BTreeMap};
use std::sync::LazyLock;
use std::time::Duration;

use maverick_os::runtime::{Channel, Service, ServiceContext, async_trait, Callback, self};
use maverick_os::hardware;
use maverick_os::air::AirService;
use maverick_os::State;
use maverick_os::air::air;

use air::orange_name::OrangeName;
use air::server::{Request, Error};
use air::storage::{PublicItem, Filter, Client};
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
    name: Option<OrangeName>,
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

#[async_trait]
impl Service for ProfileService {
    async fn new(_hardware: &mut hardware::Context) -> Self {
        ProfileService{
            listening: BTreeSet::new(),
            profile: None,
            name: None,
            id: None
        }
    }

    async fn run(&mut self, ctx: &mut ServiceContext, channel: &mut Channel) -> Result<Duration, runtime::Error> {
        match async {
            let mut mutated = false;

            let air = ctx.get::<AirService>();
            if self.profile.is_none() {
                let name = air.my_name();
                self.name = Some(name.clone());
                let item = air.read_public(Filter::new(None, Some(name.clone()), Some(*PROFILE), None)).await?.pop();
                self.id = item.as_ref().map(|i| i.0);
                let profile = item.and_then(|i| serde_json::from_slice(&i.2.payload).ok());
                mutated = profile.is_none();
                self.profile = Some(profile.unwrap_or(BTreeMap::new()));
            }

            while let Some(request) = channel.receive() {
                if let Ok(request) = serde_json::from_str(&request) {
                    mutated = (matches!(request, ProfileRequest::InsertField(_,_)) || matches!(request, ProfileRequest::RemoveField(_)) || mutated);
                    self.handle(request);
                }
            }

            let name = self.name.clone().unwrap();
            let profile = self.profile.as_ref().unwrap();
            let endpoint = air.resolver.endpoint(&name, None, None).await?;

            if mutated {
                let item = PublicItem {
                    protocol: *PROFILE,
                    header: vec![],
                    payload: serde_json::to_vec(&profile).unwrap(),
                };
                match self.id {
                    Some(id) => {air.update_public(id, item).await?;},
                    None => {self.id = Some(air.create_public(item).await?);}
                }
            }
            channel.send(serde_json::to_string(&(name, profile, true)).unwrap());

            let clients = self.listening.iter().map(|name| Client::read_public(Filter::new(None, Some(name.clone()), Some(*PROFILE), None))).collect::<Vec<_>>();
            let batch = Request::batch(clients.iter().map(|c| c.build_request()).collect());
            let responses = air.purser.send(&mut air.resolver, &endpoint, batch).await?.batch()?;
            for (client, response) in clients.iter().zip(responses) {
                let item = client.process_response(&mut air.resolver, response).await?.read_public().pop();
                if let Some((name, profile)) = item.and_then(|i| Some((i.1, serde_json::from_slice::<Profile>(&i.2.payload).ok()?))) {
                    channel.send(serde_json::to_string(&(name, profile, false)).unwrap());
                }
            }
            Ok::<(), Error>(())
        }.await {
            Ok(()) => {},
            Err(e) => log::error!("{:?}", e)
        }

        Ok(Duration::from_secs(5))
    }

    fn callback(&self) -> Box<Callback> {Box::new(|state: &mut State, response: String| {
        let mut profiles = state.get::<Profiles>().0;
        let (name, profile, me) = serde_json::from_str::<(OrangeName, Profile, bool)>(&response).unwrap();
        if me {state.set(&Name(Some(name.clone())));}
        profiles.insert(name, profile);
        state.set(&Profiles(profiles));
    })}
}
