use crate::enums::PersonType;
use raylib::core::texture::{Image, Texture2D};
use raylib::{RaylibHandle, RaylibThread};
use std::collections::HashMap;

pub fn load_image(
    rl: &mut RaylibHandle,
    path: &str,
    person_type: PersonType,
    humans: &mut HashMap<PersonType, Texture2D>,
    thread: &RaylibThread,
) {
    let person = Image::load_image(path).expect("failed to load image: make sure it exists");
    humans.insert(
        person_type,
        rl.load_texture_from_image(&thread, &person)
            .expect("img GPU upload failed"),
    );
}
