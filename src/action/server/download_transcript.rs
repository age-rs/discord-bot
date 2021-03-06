// STD Dependencies -----------------------------------------------------------
use std::fmt;


// Internal Dependencies ------------------------------------------------------
use ::bot::{Bot, BotConfig};
use ::core::{EventQueue, Message};
use ::action::{ActionHandler, ActionGroup, MessageActions};


// Action Implementation ------------------------------------------------------
pub struct Action {
    message: Message,
    effect_name: String,
    upload_url: String
}

impl Action {
    pub fn new(
        message: Message,
        effect_name: String,
        upload_url: String

    ) -> Box<Action> {
        Box::new(Action {
            message: message,
            effect_name: effect_name,
            upload_url: upload_url
        })
    }
}

impl ActionHandler for Action {
    fn run(&mut self, bot: &mut Bot, _: &BotConfig, _: &mut EventQueue) -> ActionGroup {

        if let Some(server) = bot.get_server(&self.message.server_id) {

            if server.has_effect(&self.effect_name) {

                info!("{} Downloading for effect {}...", self, self.effect_name);

                if let Err(err) = server.download_transcript(
                    &self.effect_name,
                    &self.upload_url
                ) {
                    warn!("{} Download failed: {}", self, err);
                    MessageActions::Send::public(
                        &self.message,
                        format!(
                            "Download of the transcript `{}` failed, please try again.",
                            self.effect_name
                        )
                    )

                } else {
                    info!("{} Download successful.", self);
                    MessageActions::Send::public(
                        &self.message,
                        format!(
                            "The transcript was successfully downloaded to the server and is now available for the sound effect `{}`!",
                            self.effect_name
                        )
                    )
                }

            } else {
                MessageActions::Send::public(
                    &self.message,
                    format!(
                        "No effect named \"{}\" found on current server, cannot upload transcript.",
                        self.effect_name
                    )
                )
            }

        } else {
            vec![]
        }

    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Action] [DownloadTranscript] \"{}\" from on Server#{}",
            self.upload_url, self.message.server_id
        )
    }
}

