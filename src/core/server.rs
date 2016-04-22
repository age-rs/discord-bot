// STD Dependencies -----------------------------------------------------------
use std::fmt;
use std::collections::HashMap;


// Discord Dependencies -------------------------------------------------------
use discord::model::{ChannelId, ServerId, VoiceState};


// Internal Dependencies ------------------------------------------------------
use super::{Handle, User};
use super::voice::Greeting;


// Server Abstraction ---------------------------------------------------------
pub struct Server {

    // General
    id: ServerId,
    name: String,
    channel_count: usize,
    member_count: usize,

    // Voice
    last_voice_channel: Option<ChannelId>,
    voice_receiver_handle: Option<()>,
    voice_states: Vec<(ChannelId, User)>,
    voice_greetings: HashMap<String, Greeting>

}

impl Server {

    pub fn new(id: ServerId) -> Server {
        Server {

            // General
            id: id,
            name: "".to_string(),
            channel_count: 0,
            member_count: 0,

            // Voice
            last_voice_channel: None,
            voice_receiver_handle: None,
            voice_states: Vec::new(),
            voice_greetings: HashMap::new()

        }
    }

}

// Voice Handling --------------------------------------------------------------
impl Server {

    pub fn initialize_voices(&mut self, handle: &mut Handle) {

        info!("[Server] [{}] [Voice] Initializing.", self);

        self.join_voice_channel(handle, None);

    }

    pub fn update_voice(&mut self, handle: &mut Handle, voice: VoiceState, user: User) {

        if user.id == handle.user_id() {
            if let Some(_) = voice.channel_id {
                info!("[Server] [{}] [Voice] Joined channel.", self);

            } else {
                info!("[Server] [{}] [Voice] Left channel.", self);
                self.voice_receiver_handle = None;
            }

        } else if let Some(channel_id) = voice.channel_id {
            // TODO IW: Make this look nicer, borrowck really isn't too smart
            // about this case
            if self.voice_states.iter().any(|&(_, ref u)| u.id == user.id) {
                {
                    // Update voice state channel
                    if let Some(mut voice_state) = self.voice_states.iter_mut().find(|&&mut (_, ref u)| u.id == user.id) {
                        voice_state.0 = channel_id;
                    }
                }
                info!("[Server] [{}] [{}] [Voice] User switched channel.", self, user);

            } else {
                info!("[Server] [{}] [{}] [Voice] User joined channel.", self, user);
                self.add_voice_state(channel_id, user);
            }

        } else {
            info!("[Server] [{}] [{}] [Voice] User left channel.", self, user);
            self.voice_states.retain(|&(_, ref u)| u.id != user.id);
        }

        if let Some(channel_id) = handle.get_server_voice(self.id).current_channel() {
            if self.voice_states.iter().filter(|&&(id, _)| id == channel_id).count() == 0 {
                info!("[Server] [{}] [Voice] Leaving empty channel.", self);
                handle.disconnect_server_voice(self.id)
            }
        }

    }

    pub fn join_voice_channel(&mut self, handle: &mut Handle, channel_id: Option<ChannelId>) -> bool {

        if let Some(target_id) = channel_id.or(self.last_voice_channel) {

            if self.last_voice_channel.is_none() || self.voice_receiver_handle.is_none() {
                info!("[Server] [{}] [Voice] Joining channel.", self);
                self.init_voice_connection(handle, target_id);
                true

            } else if channel_id.is_none() {
                info!("[Server] [{}] [Voice] Re-joining channel.", self);
                self.init_voice_connection(handle, target_id);
                true

            } else if channel_id != self.last_voice_channel {
                info!("[Server] [{}] [Voice] Switching channel.", self);
                self.init_voice_connection(handle, target_id);
                true

            } else {
                info!("[Server] [{}] [Voice] Already in channel.", self);
                false
            }

        } else {
            false
        }

    }

    fn init_voice_connection(&mut self, handle: &mut Handle, channel_id: ChannelId) {

        let voice_connection = handle.get_server_voice(self.id);
        voice_connection.connect(channel_id);

        match self.voice_receiver_handle {

            Some(ref handle) => {
                info!("[Server] [{}] [Voice] Resetting existing handle.", self);
            }

            None => {
                info!("[Server] [{}] [Voice] Creating new handle.", self);
                //let mut receiver = listener::AudioListener::new(
                //    self.audio_effect_queue.clone(),
                //    silent_effects,
                //    self.config.silence_threshold
                //);

                self.voice_receiver_handle = Some(());

                //self.audio_receiver_handle = receiver.take_handle();
                //voice_connection.set_receiver(Box::new(receiver));

                //voice_connection.play(
                //    Box::new(mixer::AudioMixer::new(self.audio_effect_queue.clone()))
                //);
            }

        }

        self.last_voice_channel = Some(channel_id);

    }

}

// Setters --------------------------------------------------------------------
impl Server {

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn clear_channels(&mut self) {
        self.channel_count = 0;
    }

    pub fn inc_channels(&mut self) {
        self.channel_count += 1;
    }

    pub fn dec_channels(&mut self) {
        self.channel_count -= 1;
    }

    pub fn clear_members(&mut self) {
        self.member_count = 0;
    }

    pub fn inc_members(&mut self) {
        self.member_count += 1;
    }

    pub fn dec_members(&mut self) {
        self.member_count -= 1;
    }

    pub fn clear_voice_states(&mut self) {
        self.voice_states.clear();
    }

    pub fn add_voice_state(&mut self, channel_id: ChannelId, user: User) {
        if user.is_bot {
            info!("[Server] [{}] [{}] [Voice] Ignored state for bot.", self, user);

        } else {
            info!("[Server] [{}] [{}] [Voice] State added.", self, user);
            self.voice_states.push((channel_id, user));
        }
    }

}


// Getters --------------------------------------------------------------------
impl Server {

    pub fn id(&self) -> &ServerId {
        &self.id
    }

}


// Traits  --------------------------------------------------------------------
impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

