use rust_on_rails::prelude::*;
use pelican_ui::prelude::*;
use pelican_ui::components::list_item::ListItem as PelicanListItem;

pub struct ListItem;
impl ListItem {
    pub fn credential(ctx: &mut Context, title: &str, subtitle: &str, color: Color) -> PelicanListItem {
        let white = ctx.get::<PelicanUI>().theme.colors.shades.white;
        let icon = AvatarContent::Icon("credential", AvatarIconStyle::Custom(color, white));
        PelicanListItem::new(ctx, false, title, None, Some(subtitle), None, None, None, None, Some(icon), None, move |_ctx: &mut Context| {})
    }
}
