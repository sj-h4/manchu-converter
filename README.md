# manchu-converter

This project is in work in progress.

A simple converter for Manchu script. It converts transliteration to Manchu script.

## Example

```rust
use manchu_converter::convert::ManchuConverter;

fn main() {
    let text = "wesimburengge";
    let r = text.convert_to_manchu().unwrap();
    assert_eq!(r, "ᠸᡝᠰᡳᠮᠪᡠᡵᡝᠩᡤᡝ")
}
```
