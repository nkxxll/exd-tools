use rand::Rng;

use super::structures::{BaseElement, Element, ExcalidrawFile, RectangleElement};

pub fn simple_drawing(elements: Vec<Element>) -> ExcalidrawFile {
    ExcalidrawFile {
        elements,
        ..Default::default()
    }
}

pub fn small_rectangle(x: f64, y: f64) -> RectangleElement {
    RectangleElement {
        base: BaseElement {
            x,
            y,
            width: 50.0,
            height: 50.0,
            ..BaseElement::default()
        },
    }
}

pub fn big_rectangle(x: f64, y: f64) -> RectangleElement {
    RectangleElement {
        base: BaseElement {
            x,
            y,
            width: 100.0,
            height: 100.0,
            ..BaseElement::default()
        },
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
