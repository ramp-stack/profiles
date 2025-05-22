use rust_on_rails::prelude::*;
use pelican_ui::prelude::*;

pub trait ListItemProfiles {
    fn credential(ctx: &mut Context, title: &str, subtitle: &str, color: Color) -> Self;
}

impl ListItemProfiles for ListItem {
    fn credential(ctx: &mut Context, title: &str, subtitle: &str, color: Color) -> Self {
        let white = ctx.get::<PelicanUI>().theme.colors.shades.white;
        let icon = AvatarContent::Icon("credential", AvatarIconStyle::Custom(color, white));
        ListItem::new(ctx, false, title, None, Some(subtitle), None, None, None, None, Some(icon), None, move |_ctx: &mut Context| {})
    }
}
