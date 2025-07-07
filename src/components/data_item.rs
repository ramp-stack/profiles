use pelican_ui::Context;
use pelican_ui::air::OrangeName;
use crate::plugin::ProfilePlugin;

use pelican_ui_std::{ DataItem, Button };

pub struct DataItemProfiles;
impl DataItemProfiles {
    pub fn orange_name_item(ctx: &mut Context, orange_name: &OrangeName) -> DataItem {
        let orange = orange_name.to_string();
        let orange_name = orange.strip_prefix("orange_name:").unwrap_or(orange.as_str()).to_string();
        let copy = orange_name.clone();
        let copy_button = Button::secondary(ctx, Some("copy"), "Copy", None, move |ctx: &mut Context| ctx.hardware.copy(copy.clone()), Some("Copied".to_string()));
        DataItem::new(ctx, None, "Orange Name", Some(&orange_name), None, None, Some(vec![copy_button]))
    }

    pub fn biography_item(ctx: &mut Context, orange_name: &OrangeName) -> DataItem {
        let mut bio = ProfilePlugin::biography(ctx, orange_name);
        if bio.is_empty() {bio = "No bio yet.".to_string();}
        DataItem::new(ctx, None, "About me", Some(&bio), None, None, None)
    }

    pub fn address_item(ctx: &mut Context, address: &str) -> DataItem {
        let copy = address.to_string();
        let copy_button = Button::secondary(ctx, Some("copy"), "Copy", None, move |ctx: &mut Context| ctx.hardware.copy(copy.clone()), Some("Copied".to_string()));
        DataItem::new(ctx, None, "Bitcoin address", Some(address), None, None, Some(vec![copy_button]))
    }
}