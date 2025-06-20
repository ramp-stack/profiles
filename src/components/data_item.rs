use pelican_ui::Context;

use crate::OrangeName;
use crate::service::Profile;

use pelican_ui_std::{ DataItem, Button };

pub struct DataItemProfiles;
impl DataItemProfiles {
    pub fn orange_name_item(ctx: &mut Context, orange_name: &OrangeName) -> DataItem {
        let copy = orange_name.to_string();
        let copy_button = Button::secondary(ctx, Some("copy"), "Copy", None, move |ctx: &mut Context| ctx.hardware.copy(copy.clone()));
        DataItem::new(ctx, None, "Orange Name", Some(orange_name.to_string().as_str()), None, None, Some(vec![copy_button]))
    }

    pub fn biography_item(ctx: &mut Context, user: &Profile) -> DataItem {
        let bio = match user.get("biography") {
            Some(b) if b.is_empty() => "No bio yet.",
            Some(b) => b,
            None => "No bio yet.",
        };
        DataItem::new(ctx, None, "About me", Some(bio), None, None, None)
    }

    pub fn address_item(ctx: &mut Context, address: &str) -> DataItem {
        let copy = address.to_string();
        let copy_button = Button::secondary(ctx, Some("copy"), "Copy", None, move |ctx: &mut Context| ctx.hardware.copy(copy.clone()));
        DataItem::new(ctx, None, "Bitcoin address", Some(address), None, None, Some(vec![copy_button]))
    }
}