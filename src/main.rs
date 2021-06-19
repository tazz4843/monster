use monster::drawing::*;
use monster::enums::*;
use monster::load_image::load_image;
use rand::Rng;
use raylib::prelude::*;
use std::collections::HashMap;
use std::time::SystemTime;
#[cfg(not(debug_assertions))]
use std::hint::unreachable_unchecked;

fn main() {
    // TODO: open linux mint homepage then output ud2 on macOS systems

    set_trace_log(TraceLogType::LOG_ALL);

    let (mut rl, thread) = raylib::init()
        .size(624, 624)
        .title("Monster: A Interactive Book")
        .msaa_4x()
        .build();

    // almost everything relies on the framerate we get: if we don't get 60 it'll look weird as hell
    // and if you can't get 60fps here there's something wrong with you
    rl.set_target_fps(60);

    trace_log(TraceLogType::LOG_DEBUG, "loading game...");
    let st = SystemTime::now();

    {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        draw_text(&mut d, "Loading...", 36, 300, 300, None);
    }

    let mut rng = rand::thread_rng();

    let pepe_heart = {
        let pepe_heart = Image::load_image("assets/5be49ea7-71de-437d-96ed-10579401923f.png")
            .expect("failed to load image: make sure it exists");
        rl.load_texture_from_image(&thread, &pepe_heart)
            .expect("img GPU upload failed")
    };
    let walls = {
        let walls = Image::load_image("assets/d4acf548-34a5-4d8d-9bf6-6dc27d11727c.png")
            .expect("failed to load image: make sure it exists");
        rl.load_texture_from_image(&thread, &walls)
            .expect("img GPU upload failed")
    };
    let carpet = {
        let carpet = Image::load_image("assets/08eaff5d-a5f3-4db3-b8d4-b36bbed254a5.png")
            .expect("failed to load image: make sure it exists");
        rl.load_texture_from_image(&thread, &carpet)
            .expect("img GPU upload failed")
    };
    let props = {
        let props = Image::load_image("assets/60431044-4171-444d-9e7f-69c4462d1309.png")
            .expect("failed to load image: make sure it exists");
        rl.load_texture_from_image(&thread, &props)
            .expect("img GPU upload failed")
    };

    let humans = {
        let mut humans: HashMap<PersonType, Texture2D> = HashMap::with_capacity(19);
        load_image(
            &mut rl,
            "assets/6f655738-b18c-497d-9ce2-bfd1bf962c1b.png",
            PersonType::Gnome(GnomeType::NoType),
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/fd9750da-2369-4fa5-96b0-c479b5e001bb.png",
            PersonType::CatKid(CatKidType::NoType),
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/3a050313-167a-4734-8843-ed6d45125ad1.png",
            PersonType::Chick(ChickType::NoType),
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/369b91f3-fa2d-4860-a560-5c207bd3a0e9.png",
            PersonType::DuckBoy(DuckType::NoType),
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/2acab477-8e13-4f02-b909-61636a4e4d74.png",
            PersonType::Fox,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person001.png",
            PersonType::Person1,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person002.png",
            PersonType::Person2,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person003.png",
            PersonType::Person3,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person004.png",
            PersonType::Person4,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person005.png",
            PersonType::Person5,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person006.png",
            PersonType::Person6,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person007.png",
            PersonType::Person7,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person008.png",
            PersonType::Person8,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person009.png",
            PersonType::Person9,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person010.png",
            PersonType::Person10,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person011.png",
            PersonType::Person11,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person012.png",
            PersonType::Person12,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person013.png",
            PersonType::Person13,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person014.png",
            PersonType::Person14,
            &mut humans,
            &thread,
        );
        load_image(
            &mut rl,
            "assets/people/person015.png",
            PersonType::Person15,
            &mut humans,
            &thread,
        );
        humans
    };

    #[allow(unused_variables)]
    #[cfg(debug_assertions)]
    let mut game_state: u16 = 2;

    #[cfg(not(debug_assertions))]
    let mut game_state: u16 = 0;

    let mut loop_counter: i32 = 0;
    let mut sat_counter: u8 = 255;
    let mut x = 0;

    let mut jury_pos: Vec<(f32, f32, PersonType)> = Vec::new();
    for _ in 1..=12 {
        jury_pos.push((
            rng.gen_range(48..=156) as f32,
            rng.gen_range(48..=96) as f32,
            rand_person(&mut rng, false),
        ));
    }

    let mut audience_pos: Vec<(f32, f32, PersonType)> = Vec::new();
    for _ in 1..=100 {
        audience_pos.push((
            rng.gen_range(48..=550) as f32,
            rng.gen_range(260..=540) as f32,
            rand_person(&mut rng, true),
        ));
    }

    let tt = st.elapsed().expect("system clock rolled back").as_millis();
    trace_log(
        TraceLogType::LOG_DEBUG,
        format!("loaded game in {}ms", tt).as_str(),
    );

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if d.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
            trace_log(
                TraceLogType::LOG_TRACE,
                format!("mouse pos: x {} y {}", d.get_mouse_x(), d.get_mouse_y()).as_str(),
            )
        }

        d.clear_background(Color::WHITE);

        match game_state {
            // the number defines the stage we're currently at
            0 => {
                draw_text(&mut d, "Monster", 36, 312, 200, None);
                draw_text(&mut d, "An Interactive Book", 24, 312, 250, None);
                draw_text(&mut d, "Press ENTER to continue...", 24, 312, 600, None);
                if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    game_state += 1;
                }
            }

            1 => {
                draw_text(&mut d, "Note:", 30, 312, 200, None);
                draw_text(&mut d, "Some changes were made to", 20, 312, 250, None);
                draw_text(
                    &mut d,
                    "characters due to lack of free pixel art.",
                    20,
                    312,
                    270,
                    None,
                );
                draw_text(&mut d, "Press ENTER to continue...", 24, 312, 600, None);
                if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    game_state += 1;
                }
            }

            2 => {
                // draw walls
                {
                    // Draw all the corner walls
                    draw_wall(&mut d, &walls, WallType::TallUpperLeftCornerWall, 0, 0);
                    draw_wall(&mut d, &walls, WallType::TallUpperRightCornerWall, 12, 0);
                    draw_wall(&mut d, &walls, WallType::TallLowerLeftCornerWall, 0, 12);
                    draw_wall(&mut d, &walls, WallType::TallLowerRightCornerWall, 12, 12);

                    // Draw the vertical walls
                    for i in 1..12 {
                        draw_wall(&mut d, &walls, WallType::TallTopBottomWall, i, 0);
                        if i < 5 || i > 7 {
                            draw_wall(&mut d, &walls, WallType::TallTopBottomWall, i, 12);
                        }
                    }
                    for i in 1..12 {
                        draw_wall(&mut d, &walls, WallType::TallVerticalWall, 0, i);
                        draw_wall(&mut d, &walls, WallType::TallVerticalWall, 12, i);
                    }
                }
                // draw carpets
                {
                    for x in 1..=11 {
                        for y in 1..=11 {
                            let carpet_type = match (x, y) {
                                (1, 1) => CarpetType::LeftTopWall,
                                (1, 11) => CarpetType::LeftBottomWall,
                                (11, 1) => CarpetType::RightTopWall,
                                (11, 11) => CarpetType::RightBottomWall,
                                (5, 11) => CarpetType::BottomLeftCornerWall,
                                (6, 11) => CarpetType::NoWalls,
                                (7, 11) => CarpetType::BottomRightCornerWall,
                                (x, _) if x == 1 => CarpetType::LeftWall,
                                (x, _) if x == 11 => CarpetType::RightWall,
                                (_, y) if y == 1 => CarpetType::TopWall,
                                (_, y) if y == 11 => CarpetType::BottomWall,
                                _ => CarpetType::NoWalls,
                            };
                            draw_carpet(&mut d, &carpet, carpet_type, x, y);
                        }
                    }
                    draw_carpet(&mut d, &carpet, CarpetType::LeftWall, 5, 12);
                    draw_carpet(&mut d, &carpet, CarpetType::NoWalls, 6, 12);
                    draw_carpet(&mut d, &carpet, CarpetType::RightWall, 7, 12);
                }
                // draw stand
                {
                    draw_prop(&mut d, &props, PropTypes::LargeBlueChair, 6, 1);
                    draw_prop(&mut d, &props, PropTypes::LargeBlueChair, 9, 1);
                    draw_wall(&mut d, &walls, WallType::ShortRightWall, 4, 2);
                    for i in 5..10 {
                        draw_wall(
                            &mut d,
                            &walls,
                            match i {
                                5 => WallType::TallLeftWall,
                                6 => WallType::TallWall,
                                7 => WallType::TallRightWall,
                                _ => WallType::ShortWall,
                            },
                            i,
                            2,
                        );
                    }
                    draw_wall(&mut d, &walls, WallType::TallLeftRightWall, 9, 2);
                    draw_wall(&mut d, &walls, WallType::ShortLeftWall, 10, 2);

                    for i in 1..=11 {
                        if i < 4 || i > 8 {
                            draw_wall(&mut d, &walls, WallType::ShortWall, i, 5)
                        }
                    }
                    draw_wall(&mut d, &walls, WallType::ShortLeftWall, 4, 5);
                    draw_wall(&mut d, &walls, WallType::ShortRightWall, 8, 5);
                }
                // draw judge
                draw_person(
                    &mut d,
                    &humans,
                    PersonType::Gnome(GnomeType::WalkingForward1),
                    291.0,
                    48.0,
                );

                // draw jury area
                for x in 1..=3 {
                    for y in 1..=2 {
                        draw_wall(&mut d, &walls, WallType::Floor, x, y)
                    }
                }

                // draw jury
                for (x, y, person_type) in jury_pos.iter() {
                    draw_person(&mut d, &humans, *person_type, *x, *y)
                }

                for (x, y, person_type) in audience_pos.iter() {
                    draw_person(&mut d, &humans, *person_type, *x, *y)
                }

                // draw defense/prosecution tables
                for i in 1..=11 {
                    if i < 5 || i > 7 {
                        draw_prop(&mut d, &props, PropTypes::LargeTable, i, 4)
                    }
                }

                {
                    // defense lawyer
                    draw_person(&mut d, &humans, PersonType::Person1, 448.0, 210.0);
                    // steve
                    draw_person(&mut d, &humans, PersonType::Person4, 496.0, 210.0);
                    // king
                    draw_person(&mut d, &humans, PersonType::Person5, 400.0, 210.0);
                    // petrocelli
                    draw_person(&mut d, &humans, PersonType::Person12, 128.0, 210.0);
                }

                if loop_counter < 100 {
                    d.draw_rectangle(0, 0, 624, 624, Color::new(0, 0, 0, sat_counter));
                    sat_counter = sat_counter.saturating_sub(4);
                } else if loop_counter > 4300 {
                    d.draw_rectangle(0, 0, 624, 624, Color::new(0, 0, 0, sat_counter));
                    sat_counter = sat_counter.saturating_add(4);
                }

                if loop_counter > 150 && loop_counter < 1550 {
                    d.draw_rectangle_rounded(
                        Rectangle::new(10.0, 305.0, 604.0, 290.0),
                        0.1,
                        10,
                        Color::WHITE,
                    );
                    if loop_counter % 3 == 0 {
                        x += 1;
                    }
                    let text = "Steve looks at the jury and sees one juror staring at him.\n\
                    He looks back at them and makes eye contact.\n\
                    He notices the juror has a somewhat disgusted facial expression.\n\
                    As he continues looking over, he wonders why.\n\
                    He flashes back to the start of the trial and what O'Brien said:\n\
                    \"You're young, you're black, and you're on trial: what else do they need to know?\"";
                    d.draw_text_rec(
                        d.get_font_default(),
                        match text.get(0..x) {
                            Some(t) => t,
                            None => text,
                        },
                        Rectangle::new(15.0, 310.0, 594.0, 280.0),
                        20.0,
                        1.0,
                        true,
                        Color::BLACK,
                    )
                } else if loop_counter == 1599 || loop_counter == 2801 {
                    x = 0;
                } else if loop_counter > 1600 && loop_counter < 2800 {
                    d.draw_rectangle_rounded(
                        Rectangle::new(10.0, 305.0, 604.0, 290.0),
                        0.1,
                        10,
                        Color::WHITE,
                    );
                    if loop_counter % 3 == 0 {
                        x += 1;
                    }
                    let text = "To try and break the ice, he waves.\n\
                    The juror snarls and looks away.\n\
                    With a heavy heart, he looks up at the American flag over the judge and places \
                    his trust in the 12 members of the jury who are the ones to decide his fate.\n\
                    He knows he is innocent and hopes the justice system sees him the same way.";
                    d.draw_text_rec(
                        d.get_font_default(),
                        match text.get(0..x) {
                            Some(t) => t,
                            None => text,
                        },
                        Rectangle::new(15.0, 310.0, 594.0, 280.0),
                        20.0,
                        1.0,
                        true,
                        Color::BLACK,
                    )
                } else if loop_counter > 2900 && loop_counter < 4200 {
                    d.draw_rectangle_rounded(
                        Rectangle::new(10.0, 305.0, 604.0, 290.0),
                        0.1,
                        10,
                        Color::WHITE,
                    );
                    if loop_counter % 3 == 0 {
                        x += 1;
                    }
                    let text = "But as he thinks of all this, he worries more and more about \
                    the mainly white jury. He flashes back to O'Brien's statement again, and just can't \
                    get it out of his mind. It's hard not to be worried or stressed under this situation.\n\
                    Steve is handling it remarkably well.";
                    d.draw_text_rec(
                        d.get_font_default(),
                        match text.get(0..x) {
                            Some(t) => t,
                            None => text,
                        },
                        Rectangle::new(15.0, 310.0, 594.0, 280.0),
                        20.0,
                        1.0,
                        true,
                        Color::BLACK,
                    )
                }

                loop_counter += 1;
                if loop_counter > 4400 {
                    loop_counter = 0;
                    game_state += 1;
                }
            }

            #[cfg(not(debug_assertions))]
            3 => {
                game_state = u16::MAX;
            }

            u16::MAX => {
                d.draw_texture(&pepe_heart, 312 - (117 / 2), 200 - (86 / 2), Color::WHITE);
                draw_text(&mut d, "Thanks for checking this out!", 32, 312, 300, None);
                draw_text(&mut d, "Source code is public:", 20, 312, 350, None);
                draw_text(
                    &mut d,
                    "https://github.com/tazz4843/monster-storybook",
                    20,
                    300,
                    375,
                    Some("https://github.com/tazz4843/monster-storybook"),
                );
                draw_text(&mut d, "Press ESC to exit", 20, 312, 600, None);
            }

            #[cfg(debug_assertions)]
            _ => {
                unreachable!()
            }

            #[cfg(not(debug_assertions))]
            _ => unsafe { unreachable_unchecked() },
        }
        d.draw_text(d.get_fps().to_string().as_str(), 0, 0, 8, Color::WHITE);
    }
}
