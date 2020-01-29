use ears::{
    init,
    listener::{
        get_orientation, get_position, get_volume, set_orientation, set_position, set_volume,
    },
    AudioController, Music as ALMusic, ReverbEffect, Sound as ALSound, SoundData, State,
};

use presentation::prelude::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

macro_rules! sound_ctor {
    ( $(($struct_name:ident, $type_name:ident, $member:ident)),+ ) => {
        $(
            pub struct $struct_name {
                $member: RefCell<$type_name>,
                path: String,
                sound_type: String,
                duration: Duration,
                reverb: RefCell<Option<ReverbEffect>>,
            }

            impl $struct_name {
                fn new(path: &str, sound_type: &str) -> VerboseResult<Arc<Self>> {
                    let $member = $type_name::new(path)?;

                    Ok(Arc::new($struct_name {
                        duration: $member.get_duration(),
                        $member: RefCell::new($member),
                        path: path.to_string(),
                        sound_type: sound_type.to_string(),
                        reverb: RefCell::new(None),
                    }))
                }

                pub fn play(&self, enable_looping: bool) -> VerboseResult<()> {
                    let mut $member = self.$member.try_borrow_mut()?;

                    if !$member.is_playing() {
                        $member.play();
                        $member.set_looping(enable_looping);
                    }

                    Ok(())
                }

                pub fn stop_looping(&self) -> VerboseResult<()> {
                    self.$member.try_borrow_mut()?.set_looping(false);

                    Ok(())
                }

                pub fn stop(&self) -> VerboseResult<()> {
                    self.$member.try_borrow_mut()?.stop();

                    Ok(())
                }

                pub fn set_position(&self, position: impl Into<[f32; 3]>) -> VerboseResult<()> {
                    self.$member.try_borrow_mut()?.set_position(position.into());

                    Ok(())
                }

                pub fn set_direction(&self, direction: impl Into<[f32; 3]>) -> VerboseResult<()> {
                    self.$member.try_borrow_mut()?.set_direction(direction.into());

                    Ok(())
                }

                pub fn set_attenuation(&self, attenuation: f32) -> VerboseResult<()> {
                    self.$member.try_borrow_mut()?.set_attenuation(attenuation);

                    Ok(())
                }

                pub fn set_max_volume(&self, max_volume: f32) -> VerboseResult<()> {
                    self.$member.try_borrow_mut()?.set_max_volume(max_volume);

                    Ok(())
                }

                pub fn set_min_volume(&self, min_volume: f32) -> VerboseResult<()> {
                    self.$member.try_borrow_mut()?.set_min_volume(min_volume);

                    Ok(())
                }

                pub fn set_max_distance(&self, max_distance: f32) -> VerboseResult<()> {
                    self.$member.try_borrow_mut()?.set_max_distance(max_distance);

                    Ok(())
                }

                pub fn set_min_distance(&self, min_distance: f32) -> VerboseResult<()> {
                    self.$member.try_borrow_mut()?.set_reference_distance(min_distance);

                    Ok(())
                }

                pub fn set_relative(&self, s: bool) -> VerboseResult<()> {
                    self.$member.try_borrow_mut()?.set_relative(s);

                    Ok(())
                }

                pub fn set_pitch(&self, pitch: f32) -> VerboseResult<()> {
                    self.$member.try_borrow_mut()?.set_pitch(pitch);

                    Ok(())
                }

                pub fn file_path(&self) -> &str {
                    &self.path
                }

                pub fn sound_type(&self) -> &str {
                    &self.sound_type
                }

                pub fn duration(&self) -> Duration {
                    self.duration
                }

                pub fn set_reverb(&self, reverb_effect: Option<ReverbEffect>) -> VerboseResult<()> {
                    let mut reverb = self.reverb.try_borrow_mut()?;
                    *reverb = reverb_effect;

                    self.$member.try_borrow_mut()?.connect(&reverb);

                    Ok(())
                }
            }

            // safe since, OpenAL is thread safe
            unsafe impl Send for $struct_name {}
            unsafe impl Sync for $struct_name {}
        )*
    }
}

sound_ctor!((Sound, ALSound, sound), (Music, ALMusic, music));

impl Sound {
    fn from_data(
        path: &str,
        sound_type: &str,
        data: Rc<RefCell<SoundData>>,
    ) -> VerboseResult<Arc<Self>> {
        let sound = ALSound::new_with_data(data)?;

        Ok(Arc::new(Sound {
            duration: sound.get_duration(),
            sound: RefCell::new(sound),
            path: path.to_string(),
            sound_type: sound_type.to_string(),
            reverb: RefCell::new(None),
        }))
    }

    pub fn block(&self) -> VerboseResult<()> {
        let sound = self.sound.try_borrow()?;
        while sound.is_playing() {}

        Ok(())
    }

    pub fn set_air_absorption_factor(&self, factor: f32) -> VerboseResult<()> {
        self.sound
            .try_borrow_mut()?
            .set_air_absorption_factor(factor);

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct VolumeInfo {
    pub master_volume: f32,
    pub music_volume: f32,
    pub other_volume: f32,
}

pub struct SoundHandler {
    volume_info: VolumeInfo,

    // sound handling
    sound_volumes: HashMap<String, f32>,
    sounds: HashMap<String, Vec<Arc<Sound>>>,

    // music handling
    music: Vec<Arc<Music>>,

    // 'clever' data handling
    data: HashMap<String, Rc<RefCell<SoundData>>>,
}

impl SoundHandler {
    pub(crate) fn new(volume_info: VolumeInfo) -> VerboseResult<SoundHandler> {
        init()?;
        set_volume(volume_info.master_volume);

        Ok(SoundHandler {
            volume_info,

            sound_volumes: HashMap::new(),
            sounds: HashMap::new(),

            music: Vec::new(),

            data: HashMap::new(),
        })
    }

    pub fn set_position(&self, position: impl Into<[f32; 3]>) {
        let pos = position.into();
        let current_position = get_position();

        if pos != current_position {
            set_position(pos);
        }
    }

    pub fn set_direction(&self, direction: impl Into<[f32; 3]>, up: impl Into<[f32; 3]>) {
        let dir = direction.into();
        let up = up.into();

        let (current_direction, current_up) = get_orientation();

        if dir != current_direction || up != current_up {
            set_orientation(dir, up);
        }
    }

    pub fn set_global_volume(&self, volume: f32) {
        let current_volume = get_volume();

        if volume != current_volume {
            set_volume(current_volume);
        }
    }

    pub fn load_sound(&mut self, path: &str, sound_type: &str) -> VerboseResult<Arc<Sound>> {
        // create sound
        let sound = match self.data.get(path) {
            Some(data) => Sound::from_data(path, sound_type, data.clone())?,
            None => {
                let sound = Sound::new(path, sound_type)?;
                self.data
                    .insert(path.to_string(), sound.sound.try_borrow()?.get_datas());

                sound
            }
        };

        // query volume for given sound type
        let volume = match self.sound_volumes.get(sound_type) {
            Some(volume) => *volume,
            None => self.volume_info.other_volume,
        };

        // set volume
        if let Ok(mut internal_sound) = sound.sound.try_borrow_mut() {
            internal_sound.set_volume(volume);
        }

        // add sound to internal map
        match self.sounds.get_mut(sound_type) {
            Some(sounds) => sounds.push(sound.clone()),
            None => {
                self.sounds
                    .insert(sound_type.to_string(), vec![sound.clone()]);
            }
        }

        Ok(sound)
    }

    pub fn load_music(&mut self, path: &str) -> VerboseResult<Arc<Music>> {
        // create music
        let music = Music::new(path, "music")?;

        // set volume
        if let Ok(mut internal_music) = music.music.try_borrow_mut() {
            internal_music.set_volume(self.volume_info.music_volume);
        }

        // add music to internal vector
        self.music.push(music.clone());

        Ok(music)
    }

    pub fn set_volume(&mut self, sound_type: &str, volume: f32) {
        // check for master volume
        if sound_type == "master" {
            if get_volume() != volume {
                self.volume_info.master_volume = volume;

                set_volume(volume);
            }
        }
        // check if sound type is music, since we separate it
        else if sound_type == "music" {
            if self.volume_info.master_volume != volume {
                // set volume
                self.volume_info.music_volume = volume;

                // iterate every music to set its volume
                for music in self.music.iter() {
                    if let Ok(mut internal_music) = music.music.try_borrow_mut() {
                        internal_music.set_volume(volume);
                    }
                }
            }
        }
        // check all other sound types
        else {
            // check map if sound type is already present
            match self.sound_volumes.get_mut(sound_type) {
                Some(stype) => {
                    // set volume
                    *stype = volume;

                    // check if sounds map has requested sound type
                    if let Some(sounds) = self.sounds.get(sound_type) {
                        // iterate every sound and set its volume
                        for sound in sounds.iter() {
                            if let Ok(mut internal_sound) = sound.sound.try_borrow_mut() {
                                internal_sound.set_volume(volume);
                            }
                        }
                    }
                }
                None => {
                    // insert volume for requested sound type
                    self.sound_volumes.insert(sound_type.to_string(), volume);
                }
            }
        }
    }

    pub fn pause(&mut self) {
        // check if sounds are playing
        for sounds in self.sounds.values() {
            for sound in sounds {
                if let Ok(mut internal_sound) = sound.sound.try_borrow_mut() {
                    // if sound is playing, pause it
                    if let State::Playing = internal_sound.get_state() {
                        internal_sound.pause();
                    }
                }
            }
        }

        for music in self.music.iter() {
            if let Ok(mut internal_music) = music.music.try_borrow_mut() {
                // if music is playing, pause it
                if let State::Playing = internal_music.get_state() {
                    internal_music.pause();
                }
            }
        }
    }

    pub fn resume(&self) {
        // check if sounds are paused
        for sounds in self.sounds.values() {
            for sound in sounds {
                if let Ok(mut internal_sound) = sound.sound.try_borrow_mut() {
                    // if sound is paused, resume it
                    if let State::Paused = internal_sound.get_state() {
                        internal_sound.play();
                    }
                }
            }
        }

        for music in self.music.iter() {
            if let Ok(mut internal_music) = music.music.try_borrow_mut() {
                // if music is paused, resume it
                if let State::Paused = internal_music.get_state() {
                    internal_music.play();
                }
            }
        }
    }

    pub fn remove_sound(&mut self, sound: &Arc<Sound>) -> VerboseResult<()> {
        if let Some(sounds) = self.sounds.get_mut(sound.sound_type()) {
            erase_arc(sounds, sound);
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.sounds.clear();
        self.music.clear();
        self.data.clear();
    }
}

// should be safe
unsafe impl Send for SoundHandler {}
unsafe impl Sync for SoundHandler {}
