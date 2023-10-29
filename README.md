# manchu-converter

This project is in work in progress.

A simple converter for Manchu script. It converts transliteration to Manchu script.

## Example

```rust
use manchu_converter::ManchuConverter;

fn main() {
    let text = "bejing be baha";
    let result = text.convert().unwrap();
    assert_eq!(result, "ᠪᡝᠵᡳᠩ ᠪᡝ ᠪᠠᡥᠠ")
}
```

