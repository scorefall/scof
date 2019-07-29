use muon_rs as muon;
use scof;

fn main() {
    let input = include_str!("../scof/Style.muon");

    let style: scof::Style = muon::from_str(input).unwrap();

    println!("{:?}", style);

    let output = muon::to_string(&style).unwrap();

    println!("{}", output);

    let style_clone: scof::Style = muon::from_str(&output).unwrap();

    assert_eq!(style, style_clone);
}
