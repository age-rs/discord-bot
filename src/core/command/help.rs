// Internal Dependencies ------------------------------------------------------
use super::super::{Handle, Server, User};
use super::{Command, CommandResult};


// Statics --------------------------------------------------------------------
static HELP_TEXT: &'static str = "
**The following commands are currently available:**

- `!s <sound>` - Plays the requested sound immediately. See below for details on what `<sound>` can be.
- `!q <sound>` - Queues up the requested sound to be played once all other currently playing / queued sounds have finished.
- `!sounds` - Sends a listing of all available sound effects in a private channel.
- `!silence` - Immediately stops all playing sounds and removes all other queued effects.
- `!reload` - Reloads the sound list from the on disk flac files.
- `!help` - Displays this help text.

**Sound Effects**

Sound effects can be played by requesting them via the `!s <sound>` command,
where `<sound>` can either be the *full name*, a *group prefix*, or a *wildcard*.

- `full name` - Simply check the `!sounds` listing.
- `group prefix` - This is the part of a sound name before the first `_`  character (e.g. `siw` is the *group prefix* for `siw_kaffee` etc.), requesting a *group prefix* will select a random sound from the group.
- `wildcards` - These are either `*` for any random sound or `*word*` for any sound effect which contains the specified word part.";


// Bot Usage Information ------------------------------------------------------
pub struct Help;


// Interface ------------------------------------------------------------------
impl Help {
    pub fn new() -> Help {
        Help
    }
}


// Command Implementation -----------------------------------------------------
impl Command for Help {

    fn execute(&self, _: &mut Handle, server: &mut Server, user: &User) -> CommandResult {
        info!("[{}] [{}] [Command] [Help] Usage information requested.", server, user);
        Some(vec![HELP_TEXT.to_string()])
    }

    fn is_unique(&self) -> bool {
        false
    }

    fn auto_remove(&self) -> bool {
        true
    }

}

