use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

// This plugin is responsible to control the game audio
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AudioPlugin);
    // app.add_systems(OnEnter(GameState::Playing), start_audio);
}

// #[derive(Resource)]
// struct AudioAsset(Handle<AudioInstance>);

// fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
//     audio.pause();
//     let handle = audio
//         .play(audio_assets.music.clone())
//         .looped()
//         .with_volume(0.3)
//         .handle();
//     commands.insert_resource(AudioAsset(handle));
// }