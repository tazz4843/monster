use crate::enums::*;
use raylib::core::color::Color;
use raylib::core::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::core::math::{Rectangle, Vector2};
use raylib::core::text::measure_text;
use raylib::core::texture::Texture2D;
use raylib::ffi::{MouseButton, MouseCursor};
use raylib::open_url;
use std::collections::HashMap;

pub fn draw_wall(
    d: &mut RaylibDrawHandle,
    wall_texture: &Texture2D,
    wall_type: WallType,
    x: i32,
    y: i32,
) {
    let source_rec = match wall_type {
        WallType::TallLeftRightWall => Rectangle::new(16.0, 48.0, 16.0, 16.0),
        WallType::TallVerticalWall => Rectangle::new(0.0, 16.0, 16.0, 16.0),
        WallType::TallUpperLeftCornerWall => Rectangle::new(0.0, 0.0, 16.0, 16.0),
        WallType::TallUpperRightCornerWall => Rectangle::new(32.0, 0.0, 16.0, 16.0),
        WallType::TallLowerLeftCornerWall => Rectangle::new(0.0, 32.0, 16.0, 16.0),
        WallType::TallLowerRightCornerWall => Rectangle::new(32.0, 32.0, 16.0, 16.0),
        WallType::TallTopBottomWall => Rectangle::new(16.0, 0.0, 16.0, 16.0),
        WallType::ShortLeftWall => Rectangle::new(64.0, 48.0, 16.0, 16.0),
        WallType::ShortLeftRightWall => Rectangle::new(0.0, 48.0, 16.0, 16.0),
        WallType::ShortRightWall => Rectangle::new(32.0, 48.0, 16.0, 16.0),
        WallType::ShortWall => Rectangle::new(48.0, 48.0, 16.0, 16.0),
        WallType::TallLeftWall => Rectangle::new(64.0, 0.0, 16.0, 16.0),
        WallType::TallRightWall => Rectangle::new(96.0, 0.0, 16.0, 16.0),
        WallType::TallWall => Rectangle::new(80.0, 0.0, 16.0, 16.0),
        WallType::Floor => Rectangle::new(16.0, 16.0, 16.0, 16.0),
    };

    d.draw_texture_pro(
        &wall_texture,
        source_rec,
        Rectangle::new((x * 48) as f32, (y * 48) as f32, 48.0, 48.0),
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );
}

pub fn draw_carpet(
    d: &mut RaylibDrawHandle,
    carpet_texture: &Texture2D,
    carpet_type: CarpetType,
    x: i32,
    y: i32,
) {
    let source_rec = match carpet_type {
        CarpetType::NoWalls => Rectangle::new(16.0, 16.0, 16.0, 16.0),
        CarpetType::LeftWall => Rectangle::new(0.0, 16.0, 16.0, 16.0),
        CarpetType::RightWall => Rectangle::new(32.0, 16.0, 16.0, 16.0),
        CarpetType::TopWall => Rectangle::new(16.0, 0.0, 16.0, 16.0),
        CarpetType::BottomWall => Rectangle::new(16.0, 32.0, 16.0, 16.0),
        CarpetType::LeftTopWall => Rectangle::new(0.0, 0.0, 16.0, 16.0),
        CarpetType::LeftBottomWall => Rectangle::new(0.0, 32.0, 16.0, 16.0),
        CarpetType::RightTopWall => Rectangle::new(32.0, 0.0, 16.0, 16.0),
        CarpetType::RightBottomWall => Rectangle::new(32.0, 32.0, 16.0, 16.0),
        CarpetType::LeftRightWall => Rectangle::new(48.0, 64.0, 16.0, 16.0),
        CarpetType::TopBottomWall => Rectangle::new(80.0, 80.0, 16.0, 16.0),
        CarpetType::UpperRightCornerWall => Rectangle::new(32.0, 80.0, 16.0, 16.0),
        CarpetType::UpperLeftCornerWall => Rectangle::new(0.0, 80.0, 16.0, 16.0),
        CarpetType::BottomRightCornerWall => Rectangle::new(0.0, 48.0, 16.0, 16.0),
        CarpetType::BottomLeftCornerWall => Rectangle::new(32.0, 48.0, 16.0, 16.0),
    };

    d.draw_texture_pro(
        &carpet_texture,
        source_rec,
        Rectangle::new((x * 48) as f32, (y * 48) as f32, 48.0, 48.0),
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );
}

pub fn draw_prop(
    d: &mut RaylibDrawHandle,
    prop_texture: &Texture2D,
    prop_type: PropTypes,
    x: i32,
    y: i32,
) {
    let source_rec = match prop_type {
        PropTypes::SmallChair => Rectangle::new(80.0, 0.0, 16.0, 16.0),
        PropTypes::LargePinkChair => Rectangle::new(16.0, 32.0, 16.0, 16.0),
        PropTypes::LargeRedChair => Rectangle::new(32.0, 32.0, 16.0, 16.0),
        PropTypes::LargeBlueChair => Rectangle::new(48.0, 32.0, 16.0, 16.0),
        PropTypes::SmallTable => Rectangle::new(96.0, 0.0, 16.0, 16.0),
        PropTypes::LargeTable => Rectangle::new(112.0, 0.0, 16.0, 16.0),
    };

    d.draw_texture_pro(
        &prop_texture,
        source_rec,
        Rectangle::new((x * 48) as f32, (y * 48) as f32, 48.0, 48.0),
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );
}

/// Draws text centered upon a certain position.
#[inline(always)]
pub fn draw_text(
    d: &mut RaylibDrawHandle,
    text: &str,
    font_size: i32,
    x: i32,
    y: i32,
    extern_link: Option<&str>,
) {
    let diff_x = measure_text(text, font_size) / 2;
    let diff_y = font_size / 2;

    let x = x - diff_x;
    let y = y - diff_y;

    d.draw_text(text, x, y, font_size, Color::BLACK);

    if let Some(extern_link) = extern_link {
        let hitbox = Rectangle::new(x as f32, y as f32, (diff_x * 2) as f32, (diff_y * 2) as f32);
        if hitbox.check_collision_point_rec(d.get_mouse_position()) {
            d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_POINTING_HAND);
            if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                open_url(extern_link)
            }
        } else {
            d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
        }
    }
}

#[inline(always)]
pub fn draw_horizontal_line(d: &mut RaylibDrawHandle, y: i32) {
    d.draw_line(0, y, 600, y, Color::BLACK)
}

pub fn draw_person(
    d: &mut RaylibDrawHandle,
    person_texture: &HashMap<PersonType, Texture2D>,
    person_type: PersonType,
    x: f32,
    y: f32,
) {
    if x < 0.0 || y < 0.0 || x > 624.0 || y > 624.0 {
        return;
    }

    let (source_rec, real_type, dest_rec) = match person_type {
        PersonType::Gnome(g) => (
            Rectangle::new(
                match g {
                    GnomeType::IdleRight => 80,
                    GnomeType::WalkingRight => 64,
                    GnomeType::WalkingForward1 => 0,
                    GnomeType::WalkingForward2 => 16,
                    GnomeType::WalkingBackward1 => 32,
                    GnomeType::WalkingBackward2 => 48,
                    GnomeType::NoType => {
                        panic!("GnomeType::NoType is only used for inserting into HashMaps: don't use it!")
                    }
                } as f32,
                0.0,
                16.0,
                16.0,
            ),
            PersonType::Gnome(GnomeType::NoType),
            Rectangle::new(x, y, 48.0, 48.0),
        ),
        PersonType::CatKid(c) => (
            Rectangle::new(
                match c {
                    CatKidType::NoType => {
                        panic!("CatKidType::NoType is only used for inserting into HashMaps: don't use it!")
                    }
                    CatKidType::WalkingForward1 => 0,
                    CatKidType::WalkingForward2 => 16,
                } as f32,
                0.0,
                15.0,
                17.0,
            ),
            PersonType::CatKid(CatKidType::NoType),
            Rectangle::new(x, y, 48.0, 48.0),
        ),
        PersonType::Chick(c) => (
            Rectangle::new(
                match c {
                    ChickType::NoType => {
                        panic!("ChickType::NoType is only used for inserting into HashMaps: don't use it!")
                    }
                    ChickType::WalkingForward1 => 0,
                    ChickType::WalkingForward2 => 13,
                } as f32,
                0.0,
                12.0,
                17.0,
            ),
            PersonType::Chick(ChickType::NoType),
            Rectangle::new(x, y, 24.0, 34.0),
        ),
        PersonType::DuckBoy(d) => (
            Rectangle::new(
                match d {
                    DuckType::NoType => {
                        panic!("ChickType::NoType is only used for inserting into HashMaps: don't use it!")
                    }
                    DuckType::WalkingForward1 => 0,
                    DuckType::WalkingForward2 => 13,
                } as f32,
                0.0,
                12.0,
                17.0,
            ),
            PersonType::DuckBoy(DuckType::NoType),
            Rectangle::new(x, y, 24.0, 34.0),
        ),
        PersonType::Fox => (
            Rectangle::new(0.0, 0.0, 16.0, 16.0),
            PersonType::Fox,
            Rectangle::new(x, y, 48.0, 48.0),
        ),
        PersonType::Person1 => (
            Rectangle::new(0.0, 0.0, 11.0, 18.0),
            PersonType::Person1,
            Rectangle::new(x, y, 22.0, 36.0),
        ),
        PersonType::Person2 => (
            Rectangle::new(0.0, 0.0, 14.0, 19.0),
            PersonType::Person2,
            Rectangle::new(x, y, 28.0, 38.0),
        ),
        PersonType::Person3 => (
            Rectangle::new(0.0, 0.0, 14.0, 19.0),
            PersonType::Person3,
            Rectangle::new(x, y, 28.0, 38.0),
        ),
        PersonType::Person4 => (
            Rectangle::new(0.0, 0.0, 13.0, 19.0),
            PersonType::Person4,
            Rectangle::new(x, y, 26.0, 38.0),
        ),
        PersonType::Person5 => (
            Rectangle::new(0.0, 0.0, 13.0, 19.0),
            PersonType::Person5,
            Rectangle::new(x, y, 26.0, 38.0),
        ),
        PersonType::Person6 => (
            Rectangle::new(0.0, 0.0, 14.0, 19.0),
            PersonType::Person6,
            Rectangle::new(x, y, 28.0, 38.0),
        ),
        PersonType::Person7 => (
            Rectangle::new(0.0, 0.0, 14.0, 19.0),
            PersonType::Person7,
            Rectangle::new(x, y, 28.0, 38.0),
        ),
        PersonType::Person8 => (
            Rectangle::new(0.0, 0.0, 14.0, 19.0),
            PersonType::Person8,
            Rectangle::new(x, y, 28.0, 38.0),
        ),
        PersonType::Person9 => (
            Rectangle::new(0.0, 0.0, 13.0, 19.0),
            PersonType::Person9,
            Rectangle::new(x, y, 26.0, 38.0),
        ),
        PersonType::Person10 => (
            Rectangle::new(0.0, 0.0, 13.0, 19.0),
            PersonType::Person10,
            Rectangle::new(x, y, 26.0, 38.0),
        ),
        PersonType::Person11 => (
            Rectangle::new(0.0, 0.0, 13.0, 19.0),
            PersonType::Person11,
            Rectangle::new(x, y, 26.0, 38.0),
        ),
        PersonType::Person12 => (
            Rectangle::new(0.0, 0.0, 14.0, 19.0),
            PersonType::Person12,
            Rectangle::new(x, y, 28.0, 38.0),
        ),
        PersonType::Person13 => (
            Rectangle::new(0.0, 0.0, 14.0, 19.0),
            PersonType::Person13,
            Rectangle::new(x, y, 28.0, 38.0),
        ),
        PersonType::Person14 => (
            Rectangle::new(0.0, 0.0, 14.0, 19.0),
            PersonType::Person14,
            Rectangle::new(x, y, 28.0, 38.0),
        ),
        PersonType::Person15 => (
            Rectangle::new(0.0, 0.0, 14.0, 19.0),
            PersonType::Person15,
            Rectangle::new(x, y, 28.0, 38.0),
        ),
    };

    let texture = person_texture
        .get(&real_type)
        .expect("texture for type not loaded?");

    d.draw_texture_pro(
        &texture,
        source_rec,
        dest_rec,
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );
}
