use manchu_converter::ManchuConverter;

fn main() {
    let text = "manju";
    let result = text.convert_to_manchu(&None).unwrap();
    println!("{}", result) // ᠮᠠᠨᠵᡠ
}
