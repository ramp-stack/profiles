use pelican_ui::Context;
use pelican_ui::air::OrangeName;
use crate::plugin::ProfilePlugin;

use pelican_ui_std::{ DataItem, Button, NavigateEvent };

pub struct DataItemProfiles;
impl DataItemProfiles {
    pub fn orange_name_item(ctx: &mut Context, orange_name: &OrangeName) -> DataItem {
        let orange = orange_name.to_string();
        let orange_name = orange.strip_prefix("orange_name:").unwrap_or(orange.as_str()).to_string();
        let copy = orange_name.clone();
        let copy_button = Button::secondary(ctx, Some("copy"), "Copy", None, move |ctx: &mut Context| ctx.hardware.copy(copy.clone()), None);
        DataItem::new(ctx, None, "Orange Name", Some(&orange_name), None, None, Some(vec![copy_button]))
    }

    pub fn biography_item(ctx: &mut Context, orange_name: &OrangeName) -> DataItem {
        let mut bio = ProfilePlugin::biography(ctx, orange_name);
        if bio.is_empty() {bio = "No bio yet.".to_string();}
        DataItem::new(ctx, None, "About me", Some(&bio), None, None, None)
    }

    pub fn address_item(ctx: &mut Context, address: &str) -> DataItem {
        let copy = address.to_string();
        let copy_button = Button::secondary(ctx, Some("copy"), "Copy", None, move |ctx: &mut Context| ctx.hardware.copy(copy.clone()), None);
        DataItem::new(ctx, None, "Bitcoin address", Some(address), None, None, Some(vec![copy_button]))
    }

    pub fn connect_computer(ctx: &mut Context) -> DataItem {
        let link_button = Button::secondary(ctx, Some("link"), "Connect Computer", None, move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(2)), None);
        DataItem::new(ctx, None, "Connect to a Computer", Some("Connect this device to a laptop or desktop computer to back up accounts or create a savings wallet."), None, None, Some(vec![link_button]))
    }
}