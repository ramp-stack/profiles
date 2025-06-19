use pelican_ui::{Context, Plugin};
use pelican_ui::runtime;
use crate::OrangeName;
pub use crate::service::{Profile, Profiles, ProfileRequest, ProfileService, Name};
use serde_json::{Value, json};
use std::hash::{DefaultHasher, Hasher, Hash};

use std::fs;

pub struct ProfilePlugin(runtime::Context);
impl Plugin for ProfilePlugin {
    fn new(ctx: &mut Context) -> Self {ProfilePlugin(ctx.runtime.clone())}
}
impl ProfilePlugin {
    pub fn request(&mut self, request: ProfileRequest) {
        self.0.send::<ProfileService>(&request)
    }
}

pub struct ProfileHelper;

impl ProfileHelper {
    pub fn has_blocked(ctx: &mut Context, user_a: &OrangeName, user_b: &OrangeName) -> bool {
        let profiles = ctx.state().get::<Profiles>();
        let user = profiles.0.get(&user_a).unwrap();
        user.get("blocked_orange_names")
            .and_then(|s| serde_json::from_str::<Vec<String>>(s).ok())
            .map_or(false, |list| list.contains(&user_b.to_string()))
    }

    pub fn get_my_profile(ctx: &mut Context) -> (OrangeName, Profile) {
        let orange_name = ctx.state().get::<Name>().0.unwrap();
        let profiles = ctx.state().get::<Profiles>();
        let my_profile = profiles.0.get(&orange_name).unwrap();
        (orange_name, my_profile.clone())
    }

    pub fn block(ctx: &mut Context, orange_name: &OrangeName) {
        Self::get_my_profile(ctx).1
            .get("blocked_orange_names")
            .and_then(|s| serde_json::from_str::<Value>(s).ok())
            .and_then(|val| val.as_array().cloned())
            .map(|mut list| {
                list.push(json!(orange_name));
                Value::Array(list)
            }).or_else(|| Some(json!([orange_name])))
            .map(|v| {
                ctx.get::<ProfilePlugin>().request(ProfileRequest::InsertField("blocked_orange_names".into(), v.to_string()))
            });
    }

    pub fn unblock(ctx: &mut Context, orange_name: &OrangeName) {
        Self::get_my_profile(ctx).1
            .get("blocked_orange_names")
            .and_then(|s| serde_json::from_str::<Vec<Value>>(s).ok())
            .map(|names| {
                let filtered: Vec<_> = names.into_iter().filter(|v| *v != json!(orange_name)).collect();
                ctx.get::<ProfilePlugin>().request(ProfileRequest::InsertField(
                    "blocked_orange_names".into(),
                    json!(filtered).to_string(),
                ));
            });
    }


    pub fn get_username(ctx: &mut Context) -> String {
        let (orange_name, my_profile) = Self::get_my_profile(ctx);
        my_profile.get("username").map(ToString::to_string).unwrap_or_else(|| {
            let name = NameGenerator::new(&orange_name.to_string().as_str());
            ctx.get::<ProfilePlugin>().request(ProfileRequest::InsertField("username".into(), name.clone()));
            name
        })
    }

    pub fn get_biography(ctx: &mut Context) -> String {
        let (_, my_profile) = Self::get_my_profile(ctx);
        my_profile.get("biography").map(ToString::to_string).unwrap_or_else(|| {
            ctx.get::<ProfilePlugin>().request(ProfileRequest::InsertField("biography".into(), String::new()));
            String::new()
        })
    }

    pub fn update(ctx: &mut Context, key: String, value: String) {
        ctx.get::<ProfilePlugin>().request(ProfileRequest::InsertField(key, value));
    }
}

pub struct NameGenerator;

impl NameGenerator {
    pub fn new(input: &str) -> String {
        let (adjectives, nouns) = Self::load_words();
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        let hash = hasher.finish();
        let adj_index = ((hash % u16::MAX as u64) as usize) % adjectives.len();
        let noun_index = ((hash / u16::MAX as u64) as usize) % nouns.len();
        let adj = Self::capitalize(&adjectives[adj_index]);
        let noun = Self::capitalize(&nouns[noun_index]);
        format!("{}{}", adj, noun)
    }

    fn capitalize(word: &str) -> String {
        let mut chars = word.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    }

    fn load_words() -> (Vec<String>, Vec<String>) {
        let adjectives = include_str!("../assets/adjectives.txt")
            .lines()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect();
        let nouns = include_str!("../assets/animals.txt")
            .lines()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect();
        (adjectives, nouns)
    }

}
