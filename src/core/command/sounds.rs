// Internal Dependencies ------------------------------------------------------
use super::super::super::util;
use super::super::{Handle, Server, User};
use super::{Command, CommandResult};


// Sound Listing --------------------------------------------------------------
pub struct Sounds;


// Command Implementation -----------------------------------------------------
impl Command for Sounds {

    fn execute(&mut self, _: &mut Handle, server: &mut Server, user: &User) -> CommandResult {
        info!("[{}] [{}] [Command] [Sounds] Sound listing requested.", server, user);
        let mut effects = server.list_effects();
        effects.sort();
        Some(util::list_words("Sound Effects", effects, 100, 4))
    }

    fn requires_unique_server(&self) -> bool {
        true
    }

    fn auto_remove_message(&self) -> bool {
        true
    }

    fn private_response(&self)-> bool {
        true
    }

}

