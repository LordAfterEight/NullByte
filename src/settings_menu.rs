use macroquad::prelude::*;

pub async fn render_settings_screen(game: &mut crate::structs::Game<'_>) {
    let back_button =
        crate::ui::Button::new("Back", 5.0, 2.5, 100.0, 30.0, crate::ui::ButtonType::Push);
    let main_menu_button = crate::ui::Button::new(
        "Main Menu",
        5.0,
        screen_height() - 35.0,
        150.0,
        30.0,
        crate::ui::ButtonType::Push,
    );
    let mut sfx_vol_label = crate::ui::TextInputLabel::new(
        Some("SFX Volume".to_string()),
        screen_width() / 2.0 - 50.0,
        300.0,
        100.0,
        30.0,
    );
    let mut aud_vol_label = crate::ui::TextInputLabel::new(
        Some("Music Volume".to_string()),
        screen_width() / 2.0 - 50.0,
        335.0,
        100.0,
        30.0,
    );

    sfx_vol_label.text = ((game.settings.sfx_vol * 100.0) as u8).to_string();
    aud_vol_label.text = ((game.settings.mus_vol * 100.0) as u8).to_string();

    loop {
        draw_text_ex(
            "Settings",
            screen_width() / 2.0 - measure_text("Settings", None, 30, 1.0).width / 2.0,
            30.0,
            TextParams {
                font_size: 30,
                color: WHITE,
                font: Some(&game.fonts[1]),
                ..Default::default()
            },
        );

        draw_text_ex(
            format!("{}", chrono::Local::now().format("%H:%M:%S%.3f")).as_str(),
            screen_width() - 150.0,
            30.0,
            TextParams {
                font_size: 15,
                color: WHITE,
                font: Some(&game.fonts[0]),
                ..Default::default()
            },
        );

        draw_line(0.0, 40.0, screen_width(), 40.0, 2.0, RED);

        if back_button.is_hovered() {
            game.cursor.hovers_clickable = true;
        } else {
            game.cursor.hovers_clickable = false;
        }

        if back_button.is_clicked(&game.audio.sfx_sinks[0]) {
            game.current_screen = game.previous_screen.take().unwrap();
            game.previous_screen = Some(crate::structs::Screens::SaveMenu);
            break;
        }

        if game.previous_screen == Some(crate::structs::Screens::InGame) {
            main_menu_button.draw(Some(&game.fonts[1]));
            if main_menu_button.is_clicked(&game.audio.sfx_sinks[0]) {
                game.save_game();
                game.data = crate::structs::Data::init();
                game.current_screen = crate::structs::Screens::MainMenu;
                break;
            }
        }

        let sfx_vol = sfx_vol_label.use_input(game).1;
        let aud_vol = aud_vol_label.use_input(game).1;

        if sfx_vol.is_some() {
            let new_vol: f32 = (sfx_vol.unwrap().parse().unwrap_or(0) as f32) / 100.0;
            game.audio.sfx_sinks[0].set_volume(new_vol);
            game.settings.sfx_vol = new_vol;
        }
        if aud_vol.is_some() {
            let new_vol: f32 = (aud_vol.unwrap().parse().unwrap_or(0) as f32) / 100.0;
            game.audio.music_sinks[0].set_volume(new_vol);
            game.settings.mus_vol = new_vol;
        }

        sfx_vol_label.update(&game.audio.sfx_sinks[0], &game.fonts[0]);
        aud_vol_label.update(&game.audio.sfx_sinks[0], &game.fonts[0]);

        back_button.draw(Some(&game.fonts[1]));
        game.cursor.update();
        macroquad::window::next_frame().await;
    }
}
