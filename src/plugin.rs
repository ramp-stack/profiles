use pelican_ui::{Context, Plugin};
use maverick_os::air::AirService;
pub use crate::service::ProfileRequest;

pub struct ProfilePlugin;
impl Plugin for ProfilePlugin {
    fn new(_ctx: &mut Context) -> Self {ProfilePlugin}
}
impl ProfilePlugin {
    pub fn request(&self, ctx: &mut Context, request: ProfileRequest) {
        ctx.runtime.send::<AirService>(serde_json::to_string(&request).unwrap())
    }
}
