use macroquad::prelude::*;

pub async fn render_save_menu(game: &mut crate::structs::Game<'_>) {
    let exit_button =
        crate::ui::Button::new("Back", 5.0, 2.5, 100.0, 30.0, crate::ui::ButtonType::Push);

    let mut col1 = WHITE;
    let mut col2 = GRAY;

    let newgame_label_x_alignment = screen_width() / 2.0 + 300.0;

    let mut name_label = crate::ui::TextInputLabel::new(
        Some("Name:".to_string()),
        newgame_label_x_alignment,
        screen_height() / 3.0 - 20.0,
        300.0,
        30.0,
    );

    let mut age_label = crate::ui::TextInputLabel::new(
        Some("Age:".to_string()),
        newgame_label_x_alignment,
        screen_height() / 3.0 + 20.0,
        300.0,
        30.0,
    );

    let start_button = crate::ui::Button::new(
        "Start",
        newgame_label_x_alignment,
        screen_height() / 3.0 + 60.0,
        100.0,
        30.0,
        crate::ui::ButtonType::Push,
    );

    while game.current_screen == crate::structs::Screens::SaveMenu {
        let mut saves = Vec::<crate::ui::Button>::new();
        let mut del_btns = Vec::<crate::ui::Button>::new();
        let mut i = 0.0;
        for save in std::fs::read_dir("./data/saves/").unwrap() {
            let save = save.unwrap();
            let save_path = save.path();

            if save_path.is_file() {
                saves.push(crate::ui::Button::new(
                    &format!(
                        "{}",
                        save_path
                            .to_str()
                            .unwrap()
                            .trim_start_matches("./data/saves/")
                            .trim_end_matches(".save")
                    ),
                    400.0,
                    300.0 + i * 35.0,
                    250.0,
                    30.0,
                    crate::ui::ButtonType::Push,
                ));
                del_btns.push(crate::ui::Button::new(
                    "Delete",
                    250.0,
                    300.0 + i * 35.0,
                    100.0,
                    30.0,
                    crate::ui::ButtonType::Push,
                ));
            }

            i += 1.0;
        }

        draw_text_ex(
            "Saves",
            screen_width() / 2.0 - measure_text("Saves", Some(&game.fonts[1]), 30, 1.0).width / 2.0,
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

        draw_text_ex(
            "Load Game",
            screen_width() / 4.0
                - measure_text("Load Game", Some(&game.fonts[1]), 25, 1.0).width / 2.0,
            180.0,
            TextParams {
                font_size: 25,
                color: col1,
                font: Some(&game.fonts[1]),
                ..Default::default()
            },
        );

        draw_text_ex(
            "New Game",
            3.0 * screen_width() / 4.0
                - measure_text("New Game", Some(&game.fonts[1]), 25, 1.0).width / 2.0,
            180.0,
            TextParams {
                font_size: 25,
                color: col2,
                font: Some(&game.fonts[1]),
                ..Default::default()
            },
        );

        draw_line(0.0, 40.0, screen_width(), 40.0, 2.0, RED);

        draw_line(
            screen_width() / 2.0,
            240.0,
            screen_width() / 2.0,
            screen_height() - 200.0,
            2.0,
            RED,
        );

        name_label.update(&game.audio.sfx_sinks[0], &game.fonts[0]);
        age_label.update(&game.audio.sfx_sinks[0], &game.fonts[0]);
        exit_button.draw(Some(&game.fonts[1]));
        start_button.draw(Some(&game.fonts[1]));

        if saves.is_empty() {
            draw_text_ex(
                "No existing Saves",
                365.0,
                300.0,
                TextParams {
                    font: Some(&game.fonts[0]),
                    font_size: 20,
                    color: col1,
                    ..Default::default()
                },
            );
        } else {
            for i in 0..saves.len() {
                saves[i].draw(Some(&game.fonts[1]));
                if saves[0].is_clicked(&game.audio.sfx_sinks[0]) {
                    game.load_game(saves[0].label.trim_start_matches("./data/saves/"));
                    game.current_screen = crate::structs::Screens::InGame;
                    break;
                }
                del_btns[i].draw(Some(&game.fonts[1]));
                if del_btns[i].is_clicked(&game.audio.sfx_sinks[0]) {
                    match std::fs::remove_file(format!("./data/saves/{}.save", saves[i].label)) {
                        Ok(_) => {}
                        Err(_) => rotilities::play_audio(
                            &game.audio.sfx_sinks[0],
                            "./assets/sound/sfx/fail.mp3",
                        ),
                    }
                }
            }
        }
        if game.cursor.x > screen_width() / 2.0 {
            draw_line(
                3.0 * screen_width() / 4.0 - 60.0,
                190.0,
                3.0 * screen_width() / 4.0 + 60.0,
                190.0,
                4.0,
                WHITE,
            );
            draw_rectangle(
                50.0,
                200.0,
                800.0,
                800.0,
                Color::new(0.0, 0.0, 0.0, 0.5),
            );
            col1 = GRAY;
            col2 = WHITE;
        } else {
            draw_line(
                screen_width() / 4.0 - 60.0,
                190.0,
                screen_width() / 4.0 + 60.0,
                190.0,
                4.0,
                WHITE,
            );
            draw_rectangle(
                screen_width() / 2.0 + 50.0,
                200.0,
                800.0,
                800.0,
                Color::new(0.0, 0.0, 0.0, 0.5),
            );
            col1 = WHITE;
            col2 = GRAY;
        }

        if exit_button.is_hovered() {
            game.cursor.hovers_clickable = true;
        } else {
            game.cursor.hovers_clickable = false;
        }


        if exit_button.is_clicked(&game.audio.sfx_sinks[0]) {
            game.current_screen = game.previous_screen.take().unwrap();
            game.previous_screen = Some(crate::structs::Screens::SaveMenu);
            break;
        }

        if name_label.is_hovered() || age_label.is_hovered() {
            game.cursor.hovers_clickable = true;
        }

        let name_label_text = name_label.use_input(game).1;
        let age_label_text = age_label.use_input(game).1;

        if name_label.is_active {
            age_label.is_active = false;
            game.data.player.name = match name_label_text {
                Some(c) => c,
                None => "Player".to_string(),
            };
        }

        if age_label.is_active {
            name_label.is_active = false;
            game.data.player.age = match age_label_text {
                Some(c) => {
                    if c.len() > 0 {
                        c.parse().unwrap()
                    } else {
                        18
                    }
                }
                None => 18,
            };
        }

        if start_button.is_clicked(&game.audio.sfx_sinks[0]) && name_label.text.len() > 0 {
            game.current_screen = crate::structs::Screens::InGame;
            game.cursor.hovers_clickable = false;
            game.create_game_file();
            break;
        }
        game.cursor.update();
        macroquad::window::next_frame().await;
    }
}
