use pelican_ui::{Context, Plugin};
use maverick_os::air::AirService;
use maverick_os::runtime;
pub use crate::service::ProfileRequest;

pub struct ProfilePlugin(runtime::Context);
impl Plugin for ProfilePlugin {
    fn new(ctx: &mut Context) -> Self {ProfilePlugin(ctx.runtime.clone())}
}
impl ProfilePlugin {
    pub fn request(&mut self, request: ProfileRequest) {
        self.0.send::<AirService>(serde_json::to_string(&request).unwrap())
    }
}
