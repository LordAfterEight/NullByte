use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use rotilities;

mod main_menu;
mod save_menu;
mod settings_menu;
mod game;
mod structs;
mod input;
mod ui;

fn setup_window() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "NullByte".to_owned(),
        window_width: (1920.0) as i32,
        window_height: (1080.0) as i32,
        high_dpi: false,
        window_resizable: false,
        fullscreen: true,
        icon: Some(Icon {
            small: include_bytes!("../assets/sprites/Icon_16x16.bin").clone(),
            medium: include_bytes!("../assets/sprites/Icon_32x32.bin").clone(),
            big: include_bytes!("../assets/sprites/Icon_64x64.bin").clone(),
        }),
        ..Default::default()
    }
}

#[macroquad::main(setup_window())]
async fn main() {
    macroquad::input::show_mouse(false);

    let mut game = crate::structs::Game::init().await;

    game.audio.music_sinks.push(rotilities::new_sink(&game.audio.stream_handle));
    game.audio.sfx_sinks.push(rotilities::new_sink(&game.audio.stream_handle));

    rotilities::set_audio_volume(&game.audio.music_sinks[0], game.settings.mus_vol);
    rotilities::set_audio_volume(&game.audio.sfx_sinks[0], game.settings.sfx_vol);

    let domain_name = game.data.player.domain.name();
    game.drp.connect(&game.audio.sfx_sinks[0]);

    loop {
        match game.current_screen {
            crate::structs::Screens::MainMenu => {
                game.drp.update(Some(&domain_name), "Main Menu");
                if game.previous_screen == Some(crate::structs::Screens::InGame) {
                    let mut new_volume = game.settings.mus_vol;
                    while game.audio.music_sinks[0].volume() > 0.001 {
                        rotilities::set_audio_volume(&game.audio.music_sinks[0], new_volume);
                            new_volume *= 0.95;
                        crate::main_menu::render_main_menu(&mut game);
                        game.cursor.update();
                        macroquad::window::next_frame().await;
                    }
                    rotilities::stop_audio(&game.audio.music_sinks[0]);
                    rotilities::set_audio_volume(&game.audio.music_sinks[0], game.settings.mus_vol);
                    game.previous_screen = None;
                } else if game.audio.music_sinks[0].empty() {
                    rotilities::play_audio(&game.audio.music_sinks[0], "./assets/sound/music/NullByte (Main Menu theme).mp3");
                }
                crate::main_menu::render_main_menu(&mut game);
            },
            crate::structs::Screens::SaveMenu => {
                crate::save_menu::render_save_menu(&mut game).await;
                if game.audio.music_sinks[0].empty() {
                    rotilities::play_audio(&game.audio.music_sinks[0], "./assets/sound/music/NullByte (Main Menu theme).mp3");
                }
            },
            crate::structs::Screens::SettingsMenu => {
                crate::settings_menu::render_settings_screen(&mut game).await;
                if game.audio.music_sinks[0].empty() {
                    match game.previous_screen {
                        Some(crate::structs::Screens::InGame) => rotilities::play_audio(&game.audio.music_sinks[0], "./assets/sound/music/Signal Lost.mp3"),
                        Some(crate::structs::Screens::MainMenu) => rotilities::play_audio(&game.audio.music_sinks[0], "./assets/sound/music/NullByte (Main Menu theme).mp3"),
                        _ => {}
                    }
                }
            },
            crate::structs::Screens::InGame => {
                crate::game::render_game_screen(&mut game).await;
            },

            _ => {}
        }

        game.cursor.update();
        macroquad::window::next_frame().await;
    }
}
