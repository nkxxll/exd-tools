use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;

use super::structures::{Binding, Element, ExcalidrawArrow, ExcalidrawFile, Rectangle};

pub struct Generator {
    pub index: usize,
}

impl Default for Generator {
    fn default() -> Self {
        Self { index: 0 }
    }
}

impl Generator {
    pub fn small_rectangle(&mut self, x: f64, y: f64) -> Rectangle {
        self.index += 1;
        Rectangle {
            x,
            y,
            index: generate_index(self.index),
            ..Default::default()
        }
    }

    pub fn big_rectangle(&mut self, x: f64, y: f64) -> Rectangle {
        self.index += 1;
        Rectangle {
            x,
            y,
            width: 100.0,
            height: 100.0,
            index: generate_index(self.index),
            ..Default::default()
        }
    }
}

pub fn simple_drawing(elements: Vec<Element>) -> ExcalidrawFile {
    ExcalidrawFile {
        elements,
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
    let chars = "0123456789abcdefghijklmnopqrstuvwxyz";
    if index > chars.len() * chars.len() {
        panic!("index too high")
    }
    let last = chars.chars().nth(index % chars.len()).unwrap();
    let first = chars.chars().nth(index / chars.len()).unwrap();
    // rest has to be build like alphanumeric field
    format!("b{}{}", first, last)
}

pub(crate) fn updated_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub(crate) fn arrow_from_to(this: &Rectangle, other: &Rectangle) -> ExcalidrawArrow {
    let y = 100.0;
    let x = 150.0;
    let height = 50.0;
    let points = vec![[0.0, 0.0], [0.0, -50.0]];
    let start_binding = Some(Binding{
        element_id: this.id.clone(),
        focus: 0.0,
        gap: 10.0,
    });
    let end_binding = Some(Binding{
        element_id: other.id.clone(),
        focus: 0.0,
        gap: 10.0,
    });
    ExcalidrawArrow {
        x,
        y,
        width: 1.5,
        height,
        points,
        start_binding,
        end_binding,
        ..Default::default()
    }
}

#[cfg(test)]
mod test {
    use super::generate_index;

    #[test]
    fn test_index_gen() {
        let exp1 = "b00";
        let exp2 = "b01";
        let exp3 = "b11";
        let exp4 = "bzz";
        let res1 = generate_index(0);
        let res2 = generate_index(1);
        let res3 = generate_index(37);
        let res4 = generate_index(36 * 36 - 1);

        assert_eq!(exp1, res1);
        assert_eq!(exp2, res2);
        assert_eq!(exp3, res3);
        assert_eq!(exp4, res4);
    }
}
