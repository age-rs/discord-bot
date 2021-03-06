// STD Dependencies -----------------------------------------------------------
use std::fmt;


// Discord Dependencies -------------------------------------------------------
use discord::model::ChannelId;


// Internal Dependencies ------------------------------------------------------
use ::bot::{Bot, BotConfig};
use ::text_util::list_lines;
use ::core::{EventQueue, Message};
use ::action::{ActionHandler, ActionGroup, MessageActions};


// Action Implementation ------------------------------------------------------
pub struct Action {
    message: Message
}

impl Action {
    pub fn new(message: Message) -> Box<Action> {
        Box::new(Action {
            message: message
        })
    }
}

impl ActionHandler for Action {
    fn run(&mut self, bot: &mut Bot, _: &BotConfig, _: &mut EventQueue) -> ActionGroup {

        if let Some(server) = bot.get_server(&self.message.server_id) {

            let streamers: Vec<String> = server.list_streamers().into_iter().map(|streamer| {

                let channel_id: u64 = streamer.channel_id.parse().expect("Invalid channel id!");
                let channel_id = ChannelId(channel_id);

                format!(
                    "`{}` (https://twitch.tv/{}) in **#{}**",
                    streamer.twitch_nick,
                    streamer.twitch_nick,
                    server.channel_name(&channel_id).unwrap_or_else(|| "".to_string())
                )

            }).collect();

            if streamers.is_empty() {
                MessageActions::Send::private(
                    &self.message,
                    format!("No streamers are being watched on {}.", server.name)
                )

            } else {
                list_lines("Watched Twitch Streamers", &streamers, 25).into_iter().map(|text| {
                    MessageActions::Send::single_private(&self.message, text) as Box<ActionHandler>

                }).collect()
            }

        } else {
            vec![]
        }

    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Action] [ListStreamers]")
    }
}

