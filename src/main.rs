mod excalidraw;
mod dsl;
use excalidraw::structures::{Element, Side};
use excalidraw::utils::{arrow_from_to, simple_drawing, Generator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut g = Generator::default();
    let mut first = g.big_rectangle(100.0, 100.0);
    let mut second = g.big_rectangle(-100.0, 100.0);
    let mut third = g.big_rectangle(0.0, 0.0);
    let arrow = Element::Arrow(arrow_from_to(&mut third, &mut second, Side::BOTTOM, Side::TOP));
    let arrow2 = Element::Arrow(arrow_from_to(&mut third, &mut first, Side::BOTTOM, Side::TOP));
    let elems: Vec<Element> = vec![Element::Rectangle(first), Element::Rectangle(second), Element::Rectangle(third), arrow, arrow2];
    let file = simple_drawing(elems);
    let res = serde_json::to_string(&file);
    println!("{}", res.unwrap());

    Ok(())
}
