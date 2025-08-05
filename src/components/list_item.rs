use pelican_ui::drawable::Color;
use pelican_ui::Context;

use pelican_ui_std::{
    ListItem,
    AvatarContent,
    AvatarIconStyle,
};

pub struct ListItemProfiles;
impl ListItemProfiles {
    pub fn credential(ctx: &mut Context, title: &str, subtitle: &str, color: Color) -> ListItem {
        let white = ctx.theme.colors.shades.white;
        let icon = AvatarContent::Icon("credential", AvatarIconStyle::Custom(color, white));
        ListItem::new(ctx, false, title, None, Some(subtitle), None, None, None, None, Some(icon), None, true, move |_ctx: &mut Context| {})
    }
}
