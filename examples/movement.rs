use muon_rs as muon;
use scof::Movement;

fn main() {
    let input = include_str!("../scof/Movement/The Beginning.muon");

    let style: Movement = muon::from_str(input).unwrap();

    println!("{:?}", style);

    let output = muon::to_string(&style).unwrap();

    println!("{}", output);

    let style_clone: Movement = muon::from_str(&output).unwrap();

    assert_eq!(style, style_clone);
}
