use rust_on_rails::prelude::*;
use pelican_ui::prelude::*;
use crate::prelude::ListItemProfiles;

/// Represents various user credentials.
pub enum Credential {
    /// Credential proving the user is not a bot.
    NotABot,
    /// Credential confirming the user's real name.
    RealName,
    /// Credential confirming the user has access to a US bank account.
    USAccount,
    /// Credential proving the user is over 18 years of age.
    EighteenPlus,
}


impl Credential {
    /// Retrieves a [`ListItem`] representing the credential with description and associated color.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context used to generate the [`ListItem`].
    ///
    /// # Returns
    ///
    /// A `ListItem` representing the credential with its name, description, and associated color.
    pub fn get(&self, ctx: &mut Context) -> ListItem {
        let color = self.color();
        match self {
            Credential::NotABot => ListItem::credential(ctx, "Not-A-Bot", "The Not-A-Bot credential proves you're a real person.", color),
            Credential::RealName => ListItem::credential(ctx, "Real Name", "Users with the Real Name credential have a display name that matches their real name.", color),
            Credential::USAccount => ListItem::credential(ctx, "US Account", "The US Account credential proves you have access to a US bank account.", color),
            Credential::EighteenPlus => ListItem::credential(ctx, "18+", "The 18+ Credential proves you're over 18 years of age.", color),
        }
    }

    fn color(&self) -> Color {
        match self {
            Credential::NotABot => Color::from_hex("1191E6", 255),
            Credential::RealName => Color::from_hex("F5BD14", 255),
            Credential::USAccount => Color::from_hex("3CCB5A", 255),
            Credential::EighteenPlus => Color::from_hex("363737", 255)
        }
    }
}