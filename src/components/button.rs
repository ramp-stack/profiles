use pelican_ui::Context;
use pelican_ui_std::AppPage;

use crate::pages::Account;

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