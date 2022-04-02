use bevy::prelude::*;

pub struct UpdateTickTimer(pub Timer);

impl UpdateTickTimer {
    pub fn new(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, true))
    }
}