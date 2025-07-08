#![allow(dead_code)]

use pelican_ui::events::{Event, OnEvent, TickEvent};
use pelican_ui::drawable::{Drawable, Component, Align};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{include_assets, Context, Component};
use pelican_ui::hardware::ImageOrientation;
use crate::events::UpdateProfileEvent;
use pelican_ui::air::OrangeName;
use crate::plugin::ProfilePlugin;
use crate::components::{AvatarProfiles, AvatarContentProfiles, TextInputProfiles, DataItemProfiles};

use pelican_ui_std::{
    AppPage, Stack, Page,
    Header, IconButton,
    Avatar, ExpandableText,
    TextStyle, Content,
    Offset, TextInput,
    Button, Bumper, 
    IconButtonRow,
    NavigateEvent,
    ButtonState, Text,
    QRCodeScanner, 
    QRCodeScannedEvent,
    Brand, 
};

#[derive(Debug, Component)]
pub struct ScanQR(Stack, Page, #[skip] Option<String>);

impl AppPage for ScanQR {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        // Ok(Box::new(Address::new(ctx, self.2)))
        Ok(self)
    }
}

impl ScanQR {
    fn new(ctx: &mut Context, address: Option<String>) -> Self {
        let text_size = ctx.theme.fonts.size.md;
        let text = Text::new(ctx, "Scan the QR code displayed on your laptop or desktop computer", TextStyle::Secondary, text_size, Align::Center);
        let content = Content::new(Offset::Center, vec![Box::new(QRCodeScanner::new(ctx))]);
        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Scan QR Code", None);
        ScanQR(Stack::default(), Page::new(Some(header), content, None), address)
    }
}

impl OnEvent for ScanQR {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(QRCodeScannedEvent(data)) = event.downcast_ref::<QRCodeScannedEvent>() {
            self.2 = Some(data.to_string());
            ctx.trigger_event(NavigateEvent(0));
        }
        true
    }
}

#[derive(Debug, Component)]
pub struct DownloadDesktop(Stack, Page);
impl OnEvent for DownloadDesktop {}

impl AppPage for DownloadDesktop {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { Err(self) }
}

impl DownloadDesktop {
    pub fn new(ctx: &mut Context) -> Self {
        ctx.assets.include_assets(include_assets!("./assets"));
        let mut illustrations = ctx.theme.brand.illustrations.clone();
        illustrations.insert(ctx, "desktop_wallet", "desktop_wallet.png");
        ctx.theme.brand.illustrations = illustrations;

        let desktop = ctx.theme.brand.illustrations.get("desktop_wallet").clone();
        let text_size = ctx.theme.fonts.size.h4;
        let desktop = Brand::new(desktop, (300.0, 150.0));
        let instructions = ExpandableText::new(ctx, "Install the orange desktop app on your laptop or desktop computer.", TextStyle::Heading, text_size, Align::Center, None);
        let link = Text::new(ctx, "desktop.orange.me", TextStyle::Heading, text_size, Align::Center);
        let content = Content::new(Offset::Center, vec![Box::new(desktop), Box::new(instructions), Box::new(link)]);
        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Download desktop app", None);

        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        let bumper = Bumper::single_button(ctx, button);
        
        DownloadDesktop(Stack::default(), Page::new(Some(header), content, Some(bumper)))
    }
}