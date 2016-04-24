// STD Dependencies -----------------------------------------------------------
use std::fmt;


// Discord Dependencies -------------------------------------------------------
use discord::model::UserId;
use discord::model::User as DiscordUser;


// User Abstraction -----------------------------------------------------------
#[derive(Debug, PartialEq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub nickname: String,
    pub is_bot: bool,
    pub mute: bool,
    pub deaf: bool
}


// User Implementation --------------------------------------------------------
impl User {

    pub fn new(user: &DiscordUser) -> User {
        User {
            id: user.id,
            name: user.name.to_string(),
            nickname: format!("{}#{}", user.name, user.discriminator),
            is_bot: user.bot,
            mute: false,
            deaf: false
        }
    }

}


// Traits  --------------------------------------------------------------------
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.nickname)
    }
}

