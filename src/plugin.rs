use pelican_ui::{Context, Plugin};
use pelican_ui::runtime;
use pelican_ui::air::OrangeName;
pub use crate::service::{Profile, Profiles, ProfileRequest, ProfileService, Name};
use serde_json::{Value, json};
use std::hash::{DefaultHasher, Hasher, Hash};

pub struct ProfilePlugin(runtime::Context);
impl Plugin for ProfilePlugin {
    fn new(ctx: &mut Context) -> Self {ProfilePlugin(ctx.runtime.clone())}
}
impl ProfilePlugin {
    pub fn request(&mut self, request: ProfileRequest) {
        self.0.send::<ProfileService>(&request)
    }

    pub fn has_blocked(ctx: &mut Context, user_a: &OrangeName, user_b: &OrangeName) -> bool {
        let profiles = ctx.state().get_or_default::<Profiles>().clone();
        let user = profiles.0.get(user_a).unwrap();
        user.get("blocked_orange_names")
            .and_then(|s| serde_json::from_str::<Vec<String>>(s).ok())
            .is_some_and(|list| list.contains(&user_b.to_string()))
    }

    pub fn me(ctx: &mut Context) -> (OrangeName, Profile) {
        let orange_name = ctx.state().get::<Name>().unwrap().0.clone();
        let my_profile = ctx.state().get_or_default::<Profiles>().0.get(&orange_name).unwrap();
        (orange_name.clone(), my_profile.clone())
    }

    pub fn block(ctx: &mut Context, orange_name: &OrangeName) {
        if let Some(v) = Self::me(ctx).1
            .get("blocked_orange_names")
            .and_then(|s| serde_json::from_str::<Value>(s).ok())
            .and_then(|val| val.as_array().cloned())
            .map(|mut list| {
                list.push(json!(orange_name));
                Value::Array(list)
            }).or_else(|| Some(json!([orange_name]))) { 
                let mut guard = ctx.get::<ProfilePlugin>();
                let plugin = guard.get().0;
                plugin.request(ProfileRequest::InsertField("blocked_orange_names".into(), v.to_string())) 
            }
    }

    pub fn unblock(ctx: &mut Context, orange_name: &OrangeName) {
        if let Some(names) = Self::me(ctx).1
            .get("blocked_orange_names")
            .and_then(|s| serde_json::from_str::<Vec<Value>>(s).ok()) { 
                let filtered: Vec<_> = names.into_iter().filter(|v| *v != json!(orange_name)).collect();
                let mut guard = ctx.get::<ProfilePlugin>();
                let plugin = guard.get().0;
                plugin.request(ProfileRequest::InsertField(
                    "blocked_orange_names".into(),
                    json!(filtered).to_string(),
                ));
            }
    }

    pub fn get_username(ctx: &mut Context) -> String {
        let (orange_name, my_profile) = Self::me(ctx);
        my_profile.get("username").map(ToString::to_string).unwrap_or_else(|| {
            let name = NameGenerator::new(orange_name.to_string().as_str());
            let mut guard = ctx.get::<ProfilePlugin>();
            let plugin = guard.get().0;
            plugin.request(ProfileRequest::InsertField("username".into(), name.clone()));
            name
        })
    }

    pub fn get_biography(ctx: &mut Context) -> String {
        Self::me(ctx).1.get("biography").map(ToString::to_string).unwrap_or_else(|| {
            let mut guard = ctx.get::<ProfilePlugin>();
            let plugin = guard.get().0;
            plugin.request(ProfileRequest::InsertField("biography".into(), String::new()));
            String::new()
        })
    }

    pub fn update(ctx: &mut Context, key: String, value: String) {
        let mut guard = ctx.get::<ProfilePlugin>();
        let plugin = guard.get().0;
        plugin.request(ProfileRequest::InsertField(key, value));
    }
}

pub struct NameGenerator;

impl NameGenerator {
    #[allow(clippy::new_ret_no_self)]
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

    pub fn display_name(name: String) -> String {
        if name.len() <= 10 {
            if name.len() < 5 { return "My Profile".to_string();}
            return name.to_string();
        }
        let first = name.split_whitespace().next().unwrap_or("");
        let mut last = 0;
        let mut chunks = vec![];
        for (i, c) in first.char_indices().skip(1) {
            if c.is_uppercase() {
                chunks.push(&first[last..i]);
                last = i;
            }
        }
        chunks.push(&first[last..]);
        let mut combined = String::new();
        for c in chunks {
            match combined.len() + c.len() <= 10 {
                true => combined.push_str(c),
                false => break,
            }
        }
        match !combined.is_empty() {
            true => combined,
            false if first.len() <= 10 => first.to_string(),
            false => first.chars().take(7).collect::<String>() + "..."
        }
    }
}
