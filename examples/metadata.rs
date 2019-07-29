use muon_rs as muon;
use scof;

fn main() {
    let input = include_str!("../scof/Meta.muon");

    let meta: scof::Meta = muon::from_str(input).unwrap();

    println!("{:?}", meta);

    let output = muon::to_string(&meta).unwrap();

    println!("{}", output);

    assert_eq!(input, output);
}
