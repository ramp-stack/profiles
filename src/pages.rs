#![allow(dead_code)]

use pelican_ui::events::{Event, OnEvent, TickEvent};
use pelican_ui::drawable::{Drawable, Component, Align};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};
use pelican_ui::hardware::ImageOrientation;
use crate::events::UpdateProfileEvent;
use crate::OrangeName;
use crate::service::{Name, Profiles};
use crate::plugin::ProfileHelper;
use crate::components::{AvatarProfiles, AvatarContentProfiles, TextInputProfiles, DataItemProfiles, IconButtonProfiles};
// use messages::{Rooms, Room};

use pelican_ui_std::{
    AppPage, Stack, Page,
    Header, IconButton,
    Avatar, ExpandableText,
    TextStyle, Content,
    Offset, TextInput,
    Button, Bumper, 
    IconButtonRow,
    NavigateEvent,
    ButtonState,
};

use std::sync::mpsc::{self, Receiver};

#[derive(Debug, Component)]
pub struct Account(Stack, Page, #[skip] Receiver<(Vec<u8>, ImageOrientation)>, #[skip] ButtonState);

impl AppPage for Account {
    fn has_nav(&self) -> bool { true }
    fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        Err(self)
    }
}

impl Account {
    pub fn new(ctx: &mut Context) -> Self {
        let orange_name = ctx.state().get::<Name>().0.unwrap();
        let my_username = ProfileHelper::get_username(ctx);
        let my_biography = ProfileHelper::get_biography(ctx);

        let name_input = TextInputProfiles::username(ctx, my_username);
        let bio_input = TextInputProfiles::biography(ctx, my_biography);

        let address_item = DataItemProfiles::address_item(ctx, "");
        let orange_name_item = DataItemProfiles::orange_name_item(ctx, &orange_name);
        
        let (sender, receiver) = mpsc::channel();
        let avatar_content = AvatarContentProfiles::from_orange_name(ctx, &orange_name);
        let avatar = AvatarProfiles::new_with_edit(ctx, avatar_content, sender);

        let save = Button::disabled(ctx, "Save", move |ctx: &mut Context| ctx.trigger_event(UpdateProfileEvent));

        let bumper = Bumper::single_button(ctx, save);
        let content = Content::new(Offset::Start, vec![Box::new(avatar), Box::new(name_input), Box::new(bio_input), Box::new(orange_name_item), Box::new(address_item)]);
        let header = Header::home(ctx, "Account");

        Account(Stack::center(), Page::new(header, content, Some(bumper)), receiver, ButtonState::Default)
    }
}

impl OnEvent for Account {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let my_profile = ProfileHelper::get_my_profile(ctx).1;
            let my_username = my_profile.get("username").unwrap();
            let my_biography = my_profile.get("biography").unwrap();

            let avatar = self.1.content().find::<Avatar>().unwrap();
            AvatarProfiles::try_update(ctx, avatar, self.2.try_recv());

            let username_input = &mut self.1.content().find_at::<TextInput>(1).unwrap();
            let name_changed = username_input.sync_input_value(my_username);

            let biography_input = &mut self.1.content().find_at::<TextInput>(2).unwrap();
            let bio_changed = biography_input.sync_input_value(my_biography);

            let button = self.1.bumper().as_mut().unwrap().find::<Button>().unwrap();
            button.update_state(ctx, !name_changed && !bio_changed, name_changed || bio_changed, &mut self.3);
        } else if let Some(UpdateProfileEvent) = event.downcast_ref::<UpdateProfileEvent>() {
            let my_profile = ProfileHelper::get_my_profile(ctx).1;

            let my_username = my_profile.get("username").unwrap();
            let my_biography = my_profile.get("biography").unwrap();

            let name_value = self.1.content().find_at::<TextInput>(1).unwrap().value().to_string();
            let bio_value = self.1.content().find_at::<TextInput>(2).unwrap().value().to_string();

            if name_value != *my_username {ProfileHelper::update(ctx, "username".to_string(), name_value.clone());}
            if bio_value != *my_biography {ProfileHelper::update(ctx, "biography".to_string(), bio_value.clone());}
        }

        true
    }
}

#[derive(Debug, Component)]
pub struct UserAccount(Stack, Page, #[skip] Option<Box<dyn AppPage>>);
impl OnEvent for UserAccount {}

impl AppPage for UserAccount {
    fn has_nav(&self) -> bool { false }
    fn navigate(mut self: Box<Self>, _ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(self.2.take().unwrap()),
            _ => Err(self)
        }
    }
}

impl UserAccount {
    pub fn new(ctx: &mut Context, orange_name: OrangeName, on_exit: Box<dyn AppPage>) -> Self {
        let profiles = ctx.state().get::<Profiles>();
        let user = profiles.0.get(&orange_name).unwrap();

        let my_orange_name = ProfileHelper::get_my_profile(ctx).0;
        let is_blocked = ProfileHelper::has_blocked(ctx, &orange_name, &my_orange_name);

        let exit = move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0));

        let bitcoin = IconButtonProfiles::bitcoin(ctx, exit);
        let messages = IconButtonProfiles::messages(ctx, orange_name.clone(), exit);
        let block = IconButtonProfiles::block(ctx, orange_name.clone(), is_blocked, exit);
        let buttons = IconButtonRow::new(ctx, vec![messages, bitcoin, block]);

        let address = DataItemProfiles::address_item(ctx, "");
        let orange_name_item = DataItemProfiles::orange_name_item(ctx, &orange_name);
        let about_me = DataItemProfiles::biography_item(ctx, user);
        let avatar = AvatarProfiles::user(ctx, &orange_name);
        let content = Content::new(Offset::Start, vec![Box::new(avatar), Box::new(buttons), Box::new(about_me), Box::new(orange_name_item), Box::new(address)]);

        let back = IconButton::navigation(ctx, "left", exit);
        let header = Header::stack(ctx, Some(back), user.get("username").unwrap(), None);
        UserAccount(Stack::center(), Page::new(header, content, None), Some(on_exit))
    }
}

#[derive(Debug, Component)]
pub struct BlockUser(Stack, Page, #[skip] Option<Box<dyn AppPage>>, #[skip] OrangeName);
impl OnEvent for BlockUser {}

impl AppPage for BlockUser {
    fn has_nav(&self) -> bool { false }
    fn navigate(mut self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(self.2.take().unwrap()),
            1 => Ok(Box::new(UserBlocked::new(ctx, self.3, self.2.take().unwrap()))),
            _ => Err(self)
        }
    }
}

impl BlockUser {
    pub fn new(ctx: &mut Context, orange_name: OrangeName, on_exit: Box<dyn AppPage>) -> Self {
        let profiles = ctx.state().get::<Profiles>();
        let user = profiles.0.get(&orange_name).unwrap();

        let theme = &ctx.theme;
        let text_size = theme.fonts.size.h4;

        let confirm = Button::primary(ctx, "Block", move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        let cancel = Button::close(ctx, "Cancel", move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let back = IconButton::navigation(ctx, "left", move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));

        let bumper = Bumper::double_button(ctx, cancel, confirm);
        let avatar = AvatarProfiles::new_with_block(ctx, &orange_name);

        let username = user.get("username").unwrap();
        let msg = format!("Are you sure you want to block {}?", username);
        let text = ExpandableText::new(ctx, &msg, TextStyle::Heading, text_size, Align::Center);
        let content = Content::new(Offset::Center, vec![Box::new(avatar), Box::new(text)]);
        let header = Header::stack(ctx, Some(back), "Block user", None);

        BlockUser(Stack::default(), Page::new(header, content, Some(bumper)), Some(on_exit), orange_name)
    }
}

#[derive(Debug, Component)]
pub struct UserBlocked(Stack, Page, #[skip] Option<Box<dyn AppPage>>);
impl OnEvent for UserBlocked {}

impl AppPage for UserBlocked {
    fn has_nav(&self) -> bool { false }
    fn navigate(mut self: Box<Self>, _ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(self.2.take().unwrap()),
            _ => Err(self)
        }
    }
}

impl UserBlocked {
    pub fn new(ctx: &mut Context, orange_name: OrangeName, on_exit: Box<dyn AppPage>) -> Self {
        let profiles = ctx.state().get::<Profiles>();
        let user = profiles.0.get(&orange_name).unwrap();
        ProfileHelper::block(ctx, &orange_name);

        let text_size = ctx.theme.fonts.size.h4;
        let username = user.get("username").unwrap();
        let msg = format!("{} has been blocked", username);
        let text = ExpandableText::new(ctx, &msg, TextStyle::Heading, text_size, Align::Center);
        let avatar = AvatarProfiles::new_with_block(ctx, &orange_name);
        let content = Content::new(Offset::Center, vec![Box::new(avatar), Box::new(text)]);

        let close = IconButton::close(ctx,  move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let button = Button::close(ctx, "Done", move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));

        let bumper = Bumper::single_button(ctx, button);
        let header = Header::stack(ctx, Some(close), "User blocked", None);
        UserBlocked(Stack::default(), Page::new(header, content, Some(bumper)), Some(on_exit))
    }
}

#[derive(Debug, Component)]
pub struct UnblockUser(Stack, Page, #[skip] Option<Box<dyn AppPage>>, #[skip] OrangeName);
impl OnEvent for UnblockUser {}

impl AppPage for UnblockUser {
    fn has_nav(&self) -> bool { false }
    fn navigate(mut self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(self.2.take().unwrap()),
            1 => Ok(Box::new(UserUnblocked::new(ctx, self.3, self.2.take().unwrap()))),
            _ => Err(self)
        }
    }
}

impl UnblockUser {
    pub fn new(ctx: &mut Context, orange_name: OrangeName, on_exit: Box<dyn AppPage>) -> Self {
        let profiles = ctx.state().get::<Profiles>();
        let user = profiles.0.get(&orange_name).unwrap();

        let msg = format!("Are you sure you want to unblock {}?", user.get("username").unwrap());
        let text_size = ctx.theme.fonts.size.h4;
        let text = ExpandableText::new(ctx, &msg, TextStyle::Heading, text_size, Align::Center);
        let avatar = AvatarProfiles::new_with_unblock(ctx, &orange_name); 
        let content = Content::new(Offset::Center, vec![Box::new(avatar), Box::new(text)]);

        let confirm = Button::primary(ctx, "Unblock", move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        let cancel = Button::close(ctx, "Cancel", move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let back = IconButton::navigation(ctx, "left", move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));

        let bumper = Bumper::double_button(ctx, cancel, confirm);
        let header = Header::stack(ctx, Some(back), "Unblock user", None);
        UnblockUser(Stack::default(), Page::new(header, content, Some(bumper)), Some(on_exit), orange_name)
    }
}

#[derive(Debug, Component)]
pub struct UserUnblocked(Stack, Page, #[skip] Option<Box<dyn AppPage>>);
impl OnEvent for UserUnblocked {}

impl AppPage for UserUnblocked {
    fn has_nav(&self) -> bool { false }
    fn navigate(mut self: Box<Self>, _ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(self.2.take().unwrap()),
            _ => Err(self)
        }
    }
}

impl UserUnblocked {
    pub fn new(ctx: &mut Context, orange_name: OrangeName, on_exit: Box<dyn AppPage>) -> Self {
        let profiles = ctx.state().get::<Profiles>();
        let user = profiles.0.get(&orange_name).unwrap();
        ProfileHelper::unblock(ctx, &orange_name);

        let msg = format!("{} has been unblocked", user.get("username").unwrap());
        let text_size = ctx.theme.fonts.size.h4;
        let text = ExpandableText::new(ctx, &msg, TextStyle::Heading, text_size, Align::Center);
        let avatar = AvatarProfiles::new_with_unblock(ctx, &orange_name);
        let content = Content::new(Offset::Center, vec![Box::new(avatar), Box::new(text)]);

        let close = IconButton::close(ctx, move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let button = Button::close(ctx, "Done", move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));

        let bumper = Bumper::single_button(ctx, button);
        let header = Header::stack(ctx, Some(close), "User unblocked", None);
        UserUnblocked(Stack::default(), Page::new(header, content, Some(bumper)), Some(on_exit))
    }
}
