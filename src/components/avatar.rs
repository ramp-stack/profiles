use std::sync::mpsc::Sender;
use pelican_ui::Context;
use pelican_ui::hardware::ImageOrientation;

use crate::plugin::{ProfilePlugin, ProfileRequest};

use pelican_ui::air::OrangeName;

use pelican_ui_std::{
    Avatar,
    AvatarContent,
    AvatarIconStyle,
    EncodedImage
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
            if let Some(base64_png) = EncodedImage::encode(bytes, orientation) {
                let mut guard = ctx.get::<ProfilePlugin>();
                let (plugin, ctx) = guard.get();
                this.set_content(AvatarContent::Image(EncodedImage::decode(ctx, &base64_png)));
                plugin.request(ProfileRequest::InsertField("avatar".to_string(), base64_png));
                // let me = ProfilePlugin::me(ctx).0;
                // let img = image::load_from_memory(&result_buf).unwrap().to_rgba8();
                // let asset_image = ctx.assets.add_image(img);

                // this.set_content(AvatarContent::Image(asset_image));
            }
        }
    }
}

pub struct AvatarContentProfiles;
impl AvatarContentProfiles {
    pub fn from_orange_name(ctx: &mut Context, orange_name: &OrangeName) -> AvatarContent {
        match ProfilePlugin::avatar(ctx, orange_name).as_str() {
            "" => AvatarContentProfiles::default(),
            bytes => AvatarContent::Image(EncodedImage::decode(ctx, &bytes.to_string()))
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> AvatarContent {
        AvatarContent::Icon("profile", AvatarIconStyle::Secondary)
    }
}
