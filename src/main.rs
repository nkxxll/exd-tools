mod excalidraw;
use excalidraw::{
    structures::Element,
    utils::{self, simple_drawing},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let elems = vec![Element::Rectangle(utils::big_rectangle(100.0, 100.0))];
    let file = simple_drawing(elems);
    let res = serde_json::to_string(&file);
    println!("{}", res.unwrap());

    Ok(())
}
