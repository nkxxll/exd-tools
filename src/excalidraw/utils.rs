use rand::Rng;

use super::structures::{ExcalidrawFile, Rectangle};

pub fn simple_drawing(elements: Vec<Rectangle>) -> ExcalidrawFile {
    ExcalidrawFile {
        elements,
        ..Default::default()
    }
}

pub fn small_rectangle(x: f64, y: f64) -> Rectangle {
    Rectangle {
        x,
        y,
        ..Default::default()
    }
}

pub fn big_rectangle(x: f64, y: f64) -> Rectangle {
    Rectangle {
        x,
        y,
        width: 100.0,
        height: 100.0,
        ..Default::default()
    }
}

pub(crate) fn rand_element_id() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    const ID_LEN: usize = 22;

    let mut rng = rand::rng();
    (0..ID_LEN)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub(crate) fn generate_index(index: usize) -> String {
    let chars = "0123456789abcdefghijklmnopqrstuvw";
    if index > chars.len() * chars.len() {
        panic!("index too high")
    }
    let last = chars.chars().nth(index % chars.len()).unwrap();
    let first = chars.chars().nth(index / chars.len()).unwrap();
    // rest has to be build like alphanumeric field
    format!("b{}{}", first, last)
}
