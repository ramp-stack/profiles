     
use pelican_ui::Context;

use pelican_ui_std::{ TextInput, InputState };

const NO_ICON: Option<(&str, fn(&mut Context, &mut String))> = None::<(&'static str, fn(&mut Context, &mut String))>;

pub struct TextInputProfiles;
impl TextInputProfiles {  
    pub fn username(ctx: &mut Context, username: String) -> TextInput {
        TextInput::new(ctx, Some(&username), Some("Name"), "Account name...", None, NO_ICON)
    }

    pub fn biography(ctx: &mut Context, biography: String) -> TextInput {
        TextInput::new(ctx, Some(&biography), Some("About me"), "About me...", None, NO_ICON)
    }
}