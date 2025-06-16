use pelican_ui::Context;

use crate::OrangeName;
use crate::service::Profile;
// use crate::pages::{BlockUser, UnblockUser};

use std::sync::{Arc, Mutex};

use pelican_ui_std::{IconButton, AppPage, IconButtonRow, NavigateEvent};

pub struct IconButtonProfiles;
impl IconButtonProfiles {
    pub fn block(
        ctx: &mut Context,
        orange_name: OrangeName,
        is_blocked: bool,
        on_exit: impl FnMut(&mut Context) + 'static,
    ) -> (&'static str, Box<dyn FnMut(&mut Context)>) {
        let label = if is_blocked { "unblock" } else { "block" };
        let closure = Box::new(move |ctx: &mut Context| {
            // let application_page = match is_blocked { 
            //     true => UnblockUser::new(ctx, &orange_name, account_return.lock().unwrap().take().unwrap()),
            //     false => BlockUser::new(ctx, &orange_name, account_return.lock().unwrap().take().unwrap())
            // };

            // ctx.trigger_event(NavigateEvent::new(application_page));
        });

        (label, closure)
    }

    pub fn messages(ctx: &mut Context, orange_name: OrangeName, on_exit: impl FnMut(&mut Context) + 'static,) -> (&'static str, Box<dyn FnMut(&mut Context)>) {
        let closure = Box::new(move |ctx: &mut Context| {
            // let mut rooms = ctx.state().get::<Rooms>();
            // for (id, room) in rooms.0.iter() {
            //     if room.authors.len() == 1 && room.authors[0] == orange_name {
            //         let (on_return, with_nav) =
            //             UserAccount::new(ctx, &orange_name, account_return.lock().unwrap().take().unwrap());
            //         let page = DirectMessage::new(ctx, id, (on_return.into_boxed(), with_nav));
            //         ctx.trigger_event(NavigateEvent::new(page));
            //         return;
            //     }
            // }

            // // Create new DM if none found
            // let id = uuid::Uuid::new_v4();
            // let (on_return, with_nav) = MessagesHome::new(ctx);
            // let (page, with_nav) = DirectMessage::new(ctx, &id, (on_return.into_boxed(), with_nav));
            // rooms.add(Room::new(vec![orange_name.clone()]), id);
            // ctx.state().set(&rooms);
            // ctx.trigger_event(NavigateEvent(Some(page.into_boxed()), with_nav));
        });

        ("messages", closure)
    }

    pub fn bitcoin(ctx: &mut Context, on_exit: impl FnMut(&mut Context) + 'static,) -> (&'static str, Box<dyn FnMut(&mut Context)>) {
        let closure = Box::new(move |ctx: &mut Context| {
            // let page = Amount::new(ctx);
            // ctx.trigger_event(NavigateEvent::new(page));
        });

        ("bitcoin", closure)
    }
}