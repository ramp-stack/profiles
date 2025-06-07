use std::sync::mpsc::Sender;
use pelican_ui::Context;
use pelican_ui::ImageOrientation;
use base64::{engine::general_purpose, Engine};

use crate::service::Profiles;
use maverick_os::air::air;
use air::orange_name::OrangeName;

use pelican_ui_std::{
    Avatar,
    AvatarContent,
    AvatarIconStyle,
};

pub struct AvatarProfiles;
impl AvatarProfiles {
    pub fn new_with_edit(ctx: &mut Context, avatar_content: AvatarContent, sender: Sender<(Vec<u8>, ImageOrientation)>) -> Avatar {
        Avatar::new(ctx, avatar_content,
            Some(("edit", AvatarIconStyle::Secondary)), false, 128.0,
            Some(Box::new(move |ctx: &mut Context| {
                ctx.hardware.open_photo_picker(sender.clone());
            })),
        )
    }

    pub fn new_with_block(ctx: &mut Context, orange_name: &OrangeName) -> Avatar {
        let content = AvatarContentProfiles::from_orange_name(ctx, orange_name);
        Avatar::new(ctx, content, Some(("block", AvatarIconStyle::Danger)), false, 96.0, None)
    }

    pub fn new_with_unblock(ctx: &mut Context, orange_name: &OrangeName) -> Avatar {
        let content = AvatarContentProfiles::from_orange_name(ctx, orange_name);
        Avatar::new(ctx, content, Some(("block", AvatarIconStyle::Danger)), false, 96.0, None)
    }

    pub fn user(ctx: &mut Context, orange_name: &OrangeName) -> Avatar {
        let content = AvatarContentProfiles::from_orange_name(ctx, orange_name);
        Avatar::new(ctx, content, None, false, 128.0, None)
    }
}

pub struct AvatarContentProfiles;
impl AvatarContentProfiles {
    pub fn from_orange_name(ctx: &mut Context, orange_name: &OrangeName) -> AvatarContent {
        let profiles = ctx.state().get::<Profiles>();
        let profile = profiles.0.get(&orange_name).unwrap();
        match profile.get("avatar") {
            None => AvatarContentProfiles::default(ctx),
            Some(bytes) => {
                let png_bytes = general_purpose::STANDARD.decode(bytes).unwrap();
                let image = image::load_from_memory(&png_bytes).unwrap();
                let image = ctx.assets.add_image(image.into());
                AvatarContent::Image(image)
            },
        }
    }

    pub fn default(ctx: &mut Context) -> AvatarContent {
        AvatarContent::Icon("profile", AvatarIconStyle::Secondary)
    }
}