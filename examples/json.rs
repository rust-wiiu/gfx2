use gfx2::Gfx2;

fn main() {
    let bytes = std::fs::read("tests/program.gsh").unwrap();
    let from_binary = Gfx2::parse(&bytes).unwrap();

    let json = serde_json::to_string(&from_binary).unwrap();

    println!("{json}");

    let from_serde: Gfx2 = serde_json::from_str(&json).unwrap();

    println!("{from_serde:?}");
}
