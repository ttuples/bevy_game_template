#![allow(clippy::type_complexity)]

#[macro_use]
mod utils;

mod audio;
mod asset;

use bevy::prelude::*;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>();
    
    app.add_plugins((
        asset::plugin,
        audio::plugin,
    ));

    #[cfg(debug_assertions)]
    {
        app.add_plugins((FrameTimeDiagnosticsPlugin::default(), LogDiagnosticsPlugin::default()));
    }
}
