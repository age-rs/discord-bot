// Internal Dependencies ------------------------------------------------------
use super::super::{Handle, Server, User};
use super::{Command, CommandResult};


// Command NotFound Message ---------------------------------------------------
pub struct NotFound {
    name: String
}


// Interface ------------------------------------------------------------------
impl NotFound {
    pub fn new(name: &str) -> NotFound {
        NotFound {
            name: name.to_string()
        }
    }
}


// Command Implementation -----------------------------------------------------
impl Command for NotFound {

    fn execute(&mut self, _: &mut Handle, server: &mut Server, user: &User) -> CommandResult {

        info!("[{}] [{}] [Command] [NotFound] The command \"{}\" does not exist.", server, user, self.name);

        Some(vec![format!(
            "The command `{}` does not exist, please type `!help` for a list of all available commands.",
            self.name
        )])

    }

    fn requires_unique_server(&self) -> bool {
        false
    }

    fn auto_remove_message(&self) -> bool {
        false
    }

    fn private_response(&self)-> bool {
        true
    }

}

