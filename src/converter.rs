use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

use crate::latin_manchu_unicode_mapper::get_latin_manchu_map;

pub trait ManchuConverter {
    /// Convert transcripted texts to Manchu Script and return a String
    ///
    /// ## Example
    ///
    /// ```rust
    /// use manchu_converter::ManchuConverter;
    ///
    /// fn main() {
    ///     let text = "manju";
    ///     let result = text.convert_to_manchu().unwrap();
    ///     assert_eq!(result, "ᠮᠠᠨᠵᡠ")
    /// }
    fn convert_to_manchu(&self) -> Result<String, String>;
}

impl ManchuConverter for str {
    #[inline]
    fn convert_to_manchu(&self) -> Result<String, String> {
        let latin_manchu_map = get_latin_manchu_map();
        let words = self.split_whitespace();
        let mut convert_result = String::new();
        let mut has_error = false;
        let mut error_words = Vec::new();
        for word in words {
            match convert_latin_to_manchu_unicode(word, &latin_manchu_map) {
                Ok(unicode_list) => {
                    let text = String::from_utf16(unicode_list.as_slice()).unwrap();
                    convert_result.push_str(&text);
                }
                Err(_) => {
                    has_error = true;
                    error_words.push(word);
                    convert_result.push_str(word);
                }
            }
            convert_result.push_str(" ");
        }
        if has_error {
            let error_message = format!("Error: Valid syllable not found in {:?}", error_words);
            return Err(error_message);
        }
        convert_result.pop();
        Ok(convert_result)
    }
}

fn convert_latin_to_manchu_unicode(
    word: &str,
    latin_manchu_map: &HashMap<&str, u16>,
) -> Result<Vec<u16>, String> {
    let graphemes = UnicodeSegmentation::graphemes(word, true).collect::<Vec<&str>>();
    let mut unicode_list = Vec::new();
    let mut i = 0;
    let mut has_error = false;
    loop {
        if i == graphemes.len() {
            break;
        }
        if graphemes[i] == "n" && i != graphemes.len() - 1 {
            if graphemes[i + 1] == "g" {
                match latin_manchu_map.get("ng") {
                    Some(unicode) => {
                        unicode_list.push(unicode.clone());
                        i += 2;
                        continue;
                    }
                    None => {
                        has_error = true;
                        break;
                    }
                }
            }
        }
        if graphemes[i] == "t" && i != graphemes.len() - 1 {
            if graphemes[i + 1] == "s" && graphemes[i + 2] == "'" {
                match latin_manchu_map.get("ts'") {
                    Some(unicode) => {
                        unicode_list.push(unicode.clone());
                        i += 3;
                        continue;
                    }
                    None => {
                        has_error = true;
                        break;
                    }
                }
            }
        }
        if graphemes[i] == "d" && i != graphemes.len() - 1 {
            if graphemes[i + 1] == "z" {
                match latin_manchu_map.get("dz") {
                    Some(unicode) => {
                        unicode_list.push(unicode.clone());
                        i += 2;
                        continue;
                    }
                    None => {
                        has_error = true;
                        break;
                    }
                }
            }
        }
        if graphemes[i] == "k" && i != graphemes.len() - 1 {
            if graphemes[i + 1] == "'" {
                match latin_manchu_map.get("k'") {
                    Some(unicode) => {
                        unicode_list.push(unicode.clone());
                        i += 2;
                        continue;
                    }
                    None => {
                        has_error = true;
                        break;
                    }
                }
            }
        }
        if graphemes[i] == "g" && i != graphemes.len() - 1 {
            if graphemes[i + 1] == "'" {
                match latin_manchu_map.get("g'") {
                    Some(unicode) => {
                        unicode_list.push(unicode.clone());
                        i += 2;
                        continue;
                    }
                    None => {
                        has_error = true;
                        break;
                    }
                }
            }
        }
        if graphemes[i] == "h" && i != graphemes.len() - 1 {
            if graphemes[i + 1] == "'" {
                match latin_manchu_map.get("h'") {
                    Some(unicode) => {
                        unicode_list.push(unicode.clone());
                        i += 2;
                        continue;
                    }
                    None => {
                        has_error = true;
                        break;
                    }
                }
            }
        }
        if graphemes[i] == "c" && i != graphemes.len() - 1 {
            if graphemes[i + 1] == "'" && i != graphemes.len() - 2 {
                if graphemes[i + 2] == "y" {
                    match latin_manchu_map.get("c'y") {
                        Some(unicode) => {
                            unicode_list.push(unicode.clone());
                            i += 3;
                            continue;
                        }
                        None => {
                            has_error = true;
                            break;
                        }
                    }
                }
            }
        }
        match latin_manchu_map.get(graphemes[i]) {
            Some(unicode) => {
                unicode_list.push(unicode.clone());
                i += 1;
                continue;
            }
            None => {
                has_error = true;
                break;
            }
        }
    }
    if has_error {
        let error_message = format!("Error: Valid syllable not found in {:?}", word);
        return Err(error_message);
    }
    Ok(unicode_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let latin_manchu_map = get_latin_manchu_map();
        let result = convert_latin_to_manchu_unicode("takūrafi", &latin_manchu_map).unwrap();
        assert_eq!(
            result,
            vec![0x1868, 0x1820, 0x1874, 0x1861, 0x1875, 0x1820, 0x1876, 0x1873]
        );

        let text = "cooha be acaha";
        let r = text.convert_to_manchu().unwrap();
        assert_eq!(r, "ᠴᠣᠣᡥᠠ ᠪᡝ ᠠᠴᠠᡥᠠ");

        let text_ng = "wesimburengge";
        let r_ng = text_ng.convert_to_manchu().unwrap();
        assert_eq!(r_ng, "ᠸᡝᠰᡳᠮᠪᡠᡵᡝᠩᡤᡝ");
    }
}
