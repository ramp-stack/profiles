use pelican_ui::{Context, Component};
use pelican_ui::drawable::{Component, Drawable};
use pelican_ui::layout::{SizeRequest, Area, Layout};
use pelican_ui::events::OnEvent;
use pelican_ui_std::AppPage;

use crate::pages::Account;
use pelican_ui_std::{Row, IconButton, Callback};

type ProfileButton = (&'static str, Box<dyn FnMut(&mut Context) -> Box<dyn AppPage>>);

pub struct IconButtonProfiles;
impl IconButtonProfiles {
    pub fn block(_ctx: &mut Context) -> ProfileButton {
        let closure = Box::new(move |ctx: &mut Context| {
            // let application_page = match is_blocked { 
            //     true => UnblockUser::new(ctx, &orange_name, account_return.lock().unwrap().take().unwrap()),
            //     false => BlockUser::new(ctx, &orange_name, account_return.lock().unwrap().take().unwrap())
            // };

            // ctx.trigger_event(NavigateEvent::new(application_page));
            Box::new(Account::new(ctx)) as Box<dyn AppPage>
        });

        ("block", closure)
    }
}

#[derive(Debug, Component)]
pub struct IconButtonRow(Row, Vec<IconButton>);
impl OnEvent for IconButtonRow {}

impl IconButtonRow {
    pub fn new(ctx: &mut Context, buttons: Vec<(&'static str, Callback)>) -> Self {
        let buttons = buttons.into_iter().map(|(i, on_click)| IconButton::secondary(ctx, i, on_click)).collect();
        IconButtonRow(Row::center(24.0), buttons)
    }
}