mod excalidraw;
use excalidraw::structures::Element;
use excalidraw::utils::{arrow_from_to, simple_drawing, Generator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut g = Generator::default();
    let first = g.big_rectangle(100.0, 100.0);
    let second = g.big_rectangle(100.0, 50.0);
    let arrow = Element::Arrow(arrow_from_to(&first, &second));
    let elems: Vec<Element> = vec![Element::Rectangle(first), Element::Rectangle(second), arrow];
    let file = simple_drawing(elems);
    let res = serde_json::to_string(&file);
    println!("{}", res.unwrap());

    Ok(())
}
