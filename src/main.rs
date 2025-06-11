mod excalidraw;
use excalidraw::utils::{simple_drawing, Generator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let g = Generator::default();
    let elems = vec![g.big_rectangle(100.0, 100.0)];
    let file = simple_drawing(elems);
    let res = serde_json::to_string(&file);
    println!("{}", res.unwrap());

    Ok(())
}
