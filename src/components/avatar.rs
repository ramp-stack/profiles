use std::sync::mpsc::Sender;
use pelican_ui::Context;
use pelican_ui::hardware::ImageOrientation;
use base64::{engine::general_purpose, Engine};

use crate::plugin::{ProfilePlugin, ProfileRequest};

use pelican_ui::air::OrangeName;
use crate::service::Profiles;

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

    pub fn try_update(ctx: &mut Context, this: &mut Avatar, result: Result<(Vec<u8>, ImageOrientation), std::sync::mpsc::TryRecvError>) {
        if let Ok((bytes, orientation)) = result {
            if let Ok(dynamic) = image::load_from_memory(&bytes) {
                let image = orientation.apply_to(image::DynamicImage::ImageRgba8(dynamic.to_rgba8()));
                let image = image.resize_exact(256, 256, image::imageops::FilterType::Lanczos3);
                let mut png_bytes = Vec::new();
                image.write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png).unwrap();
                let base64_png = general_purpose::STANDARD.encode(&png_bytes);

                let mut guard = ctx.get::<ProfilePlugin>();
                let (plugin, ctx) = guard.get();
                plugin.request(ProfileRequest::InsertField("avatar".to_string(), base64_png));
                let asset_image = ctx.assets.add_image(image.into());

                this.set_content(AvatarContent::Image(asset_image));
            }
        }
    }
}

pub struct AvatarContentProfiles;
impl AvatarContentProfiles {
    pub fn from_orange_name(ctx: &mut Context, orange_name: &OrangeName) -> AvatarContent {
        let profiles = ctx.state().get_or_default::<Profiles>();
        let profile = profiles.0.get(orange_name).unwrap();
        match profile.get("avatar") {
            None => AvatarContentProfiles::default(),
            Some(bytes) => {
                let png_bytes = general_purpose::STANDARD.decode(bytes).unwrap();
                let image = image::load_from_memory(&png_bytes).unwrap();
                let image = ctx.assets.add_image(image.into());
                AvatarContent::Image(image)
            },
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> AvatarContent {
        AvatarContent::Icon("profile", AvatarIconStyle::Secondary)
    }
}
