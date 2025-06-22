use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;

use super::structures::{
    Binding, BoundElement, Element, ExcalidrawArrow, ExcalidrawFile, ExcalidrawRectangle, Side,
};

pub struct Generator {
    pub index: usize,
}

impl Default for Generator {
    fn default() -> Self {
        Self { index: 0 }
    }
}

impl Generator {
    pub fn small_rectangle(&mut self, x: f64, y: f64) -> ExcalidrawRectangle {
        self.index += 1;
        ExcalidrawRectangle {
            x,
            y,
            index: generate_index(self.index),
            ..Default::default()
        }
    }

    pub fn big_rectangle(&mut self, x: f64, y: f64) -> ExcalidrawRectangle {
        self.index += 1;
        ExcalidrawRectangle {
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

fn get_left_middle(rect: &ExcalidrawRectangle) -> (f64, f64) {
    (rect.x, rect.y + rect.height / 2.0)
}

fn get_bottom_middle(rect: &ExcalidrawRectangle) -> (f64, f64) {
    (rect.x + rect.width / 2.0, rect.y + rect.height)
}

fn get_top_middle(rect: &ExcalidrawRectangle) -> (f64, f64) {
    (rect.x + rect.width / 2.0, rect.y)
}

fn get_right_middle(rect: &ExcalidrawRectangle) -> (f64, f64) {
    (rect.x + rect.width, rect.y + rect.height / 2.0)
}

pub(crate) fn arrow_from_to(
    this: &mut ExcalidrawRectangle,
    other: &mut ExcalidrawRectangle,
    this_side: Side,
    other_side: Side,
) -> ExcalidrawArrow {
    let (start, end) = match (this_side, other_side) {
        (Side::RIGHT, Side::RIGHT) => (get_right_middle(this), get_right_middle(other)),
        (Side::RIGHT, Side::LEFT) => (get_right_middle(this), get_left_middle(other)),
        (Side::RIGHT, Side::TOP) => (get_right_middle(this), get_top_middle(other)),
        (Side::RIGHT, Side::BOTTOM) => (get_right_middle(this), get_bottom_middle(other)),
        (Side::LEFT, Side::LEFT) => (get_left_middle(this), get_left_middle(other)),
        (Side::LEFT, Side::RIGHT) => (get_left_middle(this), get_right_middle(other)),
        (Side::LEFT, Side::TOP) => (get_left_middle(this), get_top_middle(other)),
        (Side::LEFT, Side::BOTTOM) => (get_left_middle(this), get_bottom_middle(other)),
        (Side::TOP, Side::TOP) => (get_top_middle(this), get_top_middle(other)),
        (Side::TOP, Side::RIGHT) => (get_top_middle(this), get_right_middle(other)),
        (Side::TOP, Side::LEFT) => (get_top_middle(this), get_left_middle(other)),
        (Side::TOP, Side::BOTTOM) => (get_top_middle(this), get_bottom_middle(other)),
        (Side::BOTTOM, Side::BOTTOM) => (get_bottom_middle(this), get_bottom_middle(other)),
        (Side::BOTTOM, Side::RIGHT) => (get_bottom_middle(this), get_right_middle(other)),
        (Side::BOTTOM, Side::LEFT) => (get_bottom_middle(this), get_left_middle(other)),
        (Side::BOTTOM, Side::TOP) => (get_bottom_middle(this), get_top_middle(other)),
    };
    let vector = (end.0 - start.0, end.1 - start.1);
    let points = vec![[0.0, 0.0], [vector.0, vector.1]];
    let height = (vector.0 * vector.0 + vector.1 * vector.1).sqrt();
    let start_binding = Some(Binding {
        element_id: this.id.clone(),
        focus: 0.0,
        gap: 10.0,
    });
    let end_binding = Some(Binding {
        element_id: other.id.clone(),
        focus: 0.0,
        gap: 10.0,
    });
    let arrow = ExcalidrawArrow {
        x: start.0,
        y: start.1,
        width: 1.5,
        height,
        points,
        start_binding,
        end_binding,
        ..Default::default()
    };
    let bound_arrow = BoundElement {
        id: arrow.id.clone(),
        r#type: "arrow".to_string(),
    };
    match this.bound_elements.as_mut() {
        Some(b) => {
            b.push(bound_arrow.clone());
        }
        None => {
            this.bound_elements = Some(vec![bound_arrow.clone()]);
        }
    }
    match other.bound_elements.as_mut() {
        Some(b) => {
            b.push(bound_arrow);
        }
        None => {
            other.bound_elements = Some(vec![bound_arrow]);
        }
    }
    other.bound_elements = Some(vec![BoundElement {
        id: arrow.id.clone(),
        r#type: "arrow".to_string(),
    }]);
    arrow
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
