#![allow(dead_code)]

use pelican_ui::events::{Event, OnEvent, Key, NamedKey, KeyboardState, KeyboardEvent, TickEvent};
use pelican_ui::drawable::{Drawable, Component, Align, Span, Image};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component, ImageOrientation};
use crate::events::UpdateProfileEvent;
use crate::OrangeName;
use crate::service::{Name, Profiles, Profile};
use crate::plugin::{ProfilePlugin, ProfileRequest, ProfileHelper};
use crate::components::{AvatarProfiles, AvatarContentProfiles, TextInputProfiles, DataItemProfiles, IconButtonProfiles};
// use messages::{Rooms, Room};

use pelican_ui_std::{
    AppPage, Stack, Page,
    Header, IconButton,
    Avatar, AvatarContent,
    AvatarIconStyle,
    ExpandableText,
    TextStyle, Content,
    Offset, TextInput,
    Button, DataItem,
    Bumper, IconButtonRow,
    NavigateEvent,
    ButtonState,
    InputState,
};

use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use base64::{engine::general_purpose, Engine as _};
use serde_json::{json, Value};

use image::ImageFormat;

#[derive(Debug, Component, AppPage)]
pub struct Account(Stack, Page, #[skip] Receiver<(Vec<u8>, ImageOrientation)>, #[skip] ButtonState);

impl Account {
    pub fn new(ctx: &mut Context) -> (Self, bool) {
        let orange_name = ctx.state().get::<Name>().0.unwrap();
        let my_username = ProfileHelper::get_username(ctx);
        let my_biography = ProfileHelper::get_biography(ctx);

        let name_input = TextInputProfiles::username(ctx, my_username);
        let bio_input = TextInputProfiles::biography(ctx, my_biography);

        let address_item = DataItemProfiles::address_item(ctx, &String::new());
        let orange_name_item = DataItemProfiles::orange_name_item(ctx, &orange_name);
        
        let (sender, receiver) = mpsc::channel();
        let avatar_content = AvatarContentProfiles::from_orange_name(ctx, &orange_name);
        let avatar = AvatarProfiles::new_with_edit(ctx, avatar_content, sender);

        let save = Button::disabled(ctx, "Save", move |ctx: &mut Context| ctx.trigger_event(UpdateProfileEvent));

        let bumper = Bumper::single_button(ctx, save);
        let content = Content::new(Offset::Start, vec![Box::new(avatar), Box::new(name_input), Box::new(bio_input), Box::new(orange_name_item), Box::new(address_item)]);
        let header = Header::home(ctx, "Account");

        (Account(Stack::center(), Page::new(header, content, Some(bumper)), receiver, ButtonState::Default), true)
    }
}

impl OnEvent for Account {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let (orange_name, my_profile) = ProfileHelper::get_my_profile(ctx);
            let my_username = my_profile.get("username").unwrap();
            let my_biography = my_profile.get("biography").unwrap();

            let avatar = self.1.content().find::<Avatar>().unwrap();
            AvatarProfiles::try_update(ctx, avatar, self.2.try_recv());

            let username_input = &mut self.1.content().find_at::<TextInput>(1).unwrap();
            let name_changed = username_input.sync_input_value(&my_username);

            let biography_input = &mut self.1.content().find_at::<TextInput>(2).unwrap();
            let bio_changed = biography_input.sync_input_value(&my_biography);

            let button = self.1.bumper().as_mut().unwrap().find::<Button>().unwrap();
            button.update_state(ctx, (!name_changed && !bio_changed), name_changed || bio_changed, &mut self.3);
        } else if let Some(UpdateProfileEvent) = event.downcast_ref::<UpdateProfileEvent>() {
            let (orange_name, my_profile) = ProfileHelper::get_my_profile(ctx);

            let my_username = my_profile.get("username").unwrap();
            let my_biography = my_profile.get("biography").unwrap();

            let name_value = self.1.content().find_at::<TextInput>(1).unwrap().value().to_string();
            let bio_value = self.1.content().find_at::<TextInput>(2).unwrap().value().to_string();

            if name_value != *my_username {ProfileHelper::update(ctx, "username".to_string(), name_value.clone());}
            if name_value != *my_username {ProfileHelper::update(ctx, "biography".to_string(), bio_value.clone());}
        }

        true
    }
}

#[derive(Debug, Component, AppPage)]
pub struct UserAccount(Stack, Page);
impl OnEvent for UserAccount {}

impl UserAccount {
    pub fn new(
        ctx: &mut Context,
        orange_name: &OrangeName,
        account_return: (Box<dyn AppPage>, bool),
    ) -> (Self, bool) {
        let profiles = ctx.state().get::<Profiles>();
        let user = profiles.0.get(orange_name).unwrap();
        let username = user.get("username").unwrap();

        let my_orange_name = ProfileHelper::get_my_profile(ctx).0;
        let is_blocked = ProfileHelper::has_blocked(ctx, &orange_name, &my_orange_name);

        let account_return = Arc::new(Mutex::new(Some(account_return)));

        let messages = IconButtonProfiles::messages(ctx, orange_name.clone(), account_return.clone());
        let bitcoin = IconButtonProfiles::bitcoin(ctx);
        let block = IconButtonProfiles::block(ctx, orange_name.clone(), account_return.clone(), is_blocked);

        let buttons = IconButtonRow::new(ctx, vec![messages, bitcoin, block]);


        let back = IconButton::navigation(ctx, "left", move |ctx| {
            let page = account_return.clone().lock().unwrap().take().unwrap();
            ctx.trigger_event(NavigateEvent(Some(page.0), page.1));
        });

        let address = DataItemProfiles::address_item(ctx, "");
        let orange_name_item = DataItemProfiles::orange_name_item(ctx, &orange_name);
        let about_me = DataItemProfiles::biography_item(ctx, user);
        let avatar = AvatarProfiles::user(ctx, &orange_name);

        let header = Header::stack(ctx, Some(back), &username, None);
        let content = Content::new(Offset::Start, vec![Box::new(avatar), Box::new(buttons), Box::new(about_me), Box::new(orange_name_item), Box::new(address)]);

        (UserAccount(Stack::center(), Page::new(header, content, None)), false)
    }

}

#[derive(Debug, Component, AppPage)]
pub struct BlockUser(Stack, Page);
impl OnEvent for BlockUser {}

impl BlockUser {
    pub fn new(ctx: &mut Context, orange_name: &OrangeName, account_return: (Box<dyn AppPage>, bool)) -> (Self, bool) {
        let profiles = ctx.state().get::<Profiles>();
        let user = profiles.0.get(&orange_name).unwrap();

        let theme = &ctx.theme;
        let text_size = theme.fonts.size.h4;

        let account_return = Arc::new(Mutex::new(Some(account_return)));

        let confirm_orange_name = orange_name.clone();
        let confirm_account_return = account_return.clone();
        let confirm = Button::primary(ctx, "Block", move |ctx: &mut Context| {
            let page = UserBlocked::new(ctx, &confirm_orange_name, confirm_account_return.lock().unwrap().take().unwrap());
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let cancel_orange_name = orange_name.clone();
        let cancel_account_return = account_return.clone();
        let cancel = Button::close(ctx, "Cancel", move |ctx: &mut Context| {
            let page = UserAccount::new(ctx, &cancel_orange_name, cancel_account_return.lock().unwrap().take().unwrap());
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let back_orange_name = orange_name.clone();
        let back_account_return = account_return.clone();
        let back = IconButton::navigation(ctx, "left", move |ctx: &mut Context| {
            let page = UserAccount::new(ctx, &back_orange_name, back_account_return.lock().unwrap().take().unwrap());
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let bumper = Bumper::double_button(ctx, cancel, confirm);
        let avatar = AvatarProfiles::new_with_block(ctx, orange_name);

        let username = user.get("username").unwrap();
        let msg = format!("Are you sure you want to block {}?", username);
        let text = ExpandableText::new(ctx, &msg, TextStyle::Heading, text_size, Align::Center);
        let content = Content::new(Offset::Center, vec![Box::new(avatar), Box::new(text)]);
        let header = Header::stack(ctx, Some(back), "Block user", None);
        (BlockUser(Stack::default(), Page::new(header, content, Some(bumper))), false)
    }
}

#[derive(Debug, Component, AppPage)]
pub struct UserBlocked(Stack, Page);
impl OnEvent for UserBlocked {}

impl UserBlocked {
    pub fn new(ctx: &mut Context, orange_name: &OrangeName, account_return: (Box<dyn AppPage>, bool)) -> (Self, bool) {
        let profiles = ctx.state().get::<Profiles>();
        let user = profiles.0.get(&orange_name).unwrap();

        ProfileHelper::block(ctx, orange_name);

        let theme = &ctx.theme;
        let text_size = theme.fonts.size.h4;
        let account_return = Arc::new(Mutex::new(Some(account_return)));

        let button_orange_name = orange_name.clone();
        let button_account_return = account_return.clone();
        let button = Button::close(ctx, "Done",  move |ctx: &mut Context| {
            let page = UserAccount::new(ctx, &button_orange_name, button_account_return.lock().unwrap().take().unwrap());
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let bumper = Bumper::single_button(ctx, button);
        let avatar = AvatarProfiles::new_with_block(ctx, orange_name);

        let username = user.get("username").unwrap();
        let msg = format!("{} has been blocked", username);
        let text = ExpandableText::new(ctx, &msg, TextStyle::Heading, text_size, Align::Center);
        let content = Content::new(Offset::Center, vec![Box::new(avatar), Box::new(text)]);

        let close_orange_name = orange_name.clone();
        let close_account_return = account_return.clone();
        let close = IconButton::close(ctx,  move |ctx: &mut Context| {
            let page = UserAccount::new(ctx, &close_orange_name, close_account_return.lock().unwrap().take().unwrap());
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let header = Header::stack(ctx, Some(close), "User blocked", None);
        (UserBlocked(Stack::default(), Page::new(header, content, Some(bumper))), false)
    }
}

#[derive(Debug, Component, AppPage)]
pub struct UnblockUser(Stack, Page);
impl OnEvent for UnblockUser {}

impl UnblockUser {
    pub fn new(ctx: &mut Context, orange_name: &OrangeName, account_return: (Box<dyn AppPage>, bool)) -> (Self, bool) {
        let profiles = ctx.state().get::<Profiles>();
        let user = profiles.0.get(&orange_name).unwrap();

        let text_size = ctx.theme.fonts.size.h4;

        let account_return = Arc::new(Mutex::new(Some(account_return)));

        let unblock_orange_name = orange_name.clone();
        let unblock_account_return = account_return.clone();
        let confirm = Button::primary(ctx, "Unblock", move |ctx: &mut Context| {
            let page = UserUnblocked::new(ctx, &unblock_orange_name, unblock_account_return.lock().unwrap().take().unwrap());
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let cancel_orange_name = orange_name.clone();
        let cancel_account_return = account_return.clone();
        let cancel = Button::close(ctx, "Cancel", move |ctx: &mut Context| {
            let page = UserAccount::new(ctx, &cancel_orange_name, cancel_account_return.lock().unwrap().take().unwrap());
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let back_orange_name = orange_name.clone();
        let back_account_return = account_return.clone();
        let back = IconButton::navigation(ctx, "left", move |ctx: &mut Context| {
            let page = UserAccount::new(ctx, &back_orange_name, back_account_return.lock().unwrap().take().unwrap());
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let bumper = Bumper::double_button(ctx, cancel, confirm);
        let avatar = AvatarProfiles::new_with_unblock(ctx, orange_name); 
        
        let username = user.get("username").unwrap();
        let msg = format!("Are you sure you want to unblock {}?", username);
        let text = ExpandableText::new(ctx, &msg, TextStyle::Heading, text_size, Align::Center);
        let content = Content::new(Offset::Center, vec![Box::new(avatar), Box::new(text)]);
        let header = Header::stack(ctx, Some(back), "Unblock user", None);
        (UnblockUser(Stack::default(), Page::new(header, content, Some(bumper))), false)
    }
}

#[derive(Debug, Component, AppPage)]
pub struct UserUnblocked(Stack, Page);
impl OnEvent for UserUnblocked {}

impl UserUnblocked {
    pub fn new(ctx: &mut Context, orange_name: &OrangeName, account_return: (Box<dyn AppPage>, bool)) -> (Self, bool) {
        let profiles = ctx.state().get::<Profiles>();
        let user = profiles.0.get(&orange_name).unwrap();

        ProfileHelper::unblock(ctx, orange_name);

        let text_size = ctx.theme.fonts.size.h4;
        let account_return = Arc::new(Mutex::new(Some(account_return)));

        let button_orange_name = orange_name.clone();
        let button_account_return = account_return.clone();
        let button = Button::close(ctx, "Done", move |ctx: &mut Context| {
            let page = UserAccount::new(ctx, &button_orange_name, button_account_return.lock().unwrap().take().unwrap());
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let bumper = Bumper::single_button(ctx, button);
        let avatar = AvatarProfiles::new_with_unblock(ctx, orange_name);

        let username = user.get("username").unwrap();
        let msg = format!("{} has been unblocked", username);
        let text = ExpandableText::new(ctx, &msg, TextStyle::Heading, text_size, Align::Center);
        let content = Content::new(Offset::Center, vec![Box::new(avatar), Box::new(text)]);

        let close_orange_name = orange_name.clone();
        let close_account_return = account_return.clone();
        let close = IconButton::close(ctx, move |ctx: &mut Context| {
            let page = UserAccount::new(ctx, &close_orange_name, close_account_return.lock().unwrap().take().unwrap());
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let header = Header::stack(ctx, Some(close), "User unblocked", None);
        (UserUnblocked(Stack::default(), Page::new(header, content, Some(bumper))), false)
    }
}
