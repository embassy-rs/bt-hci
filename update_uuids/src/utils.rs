/// Convert from "PascalCase" or "snake_case" or "IE mixed case" to "SCREAMING_SNAKE_CASE".
pub fn screaming_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut s = s.to_string();
    if s.contains(' ') {
        s = s.to_lowercase().replace(' ', "_");
    }
    // if whole string is uppercase, return it as is
    if s.chars().all(|c| c.is_ascii_uppercase()) {
        return s;
    }
    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        if first.is_numeric() {
            result.push('_'); // insert underscore before number
            result.push(first);
        } else {
            // otherwise deal with first letter by itself
            result.push(first.to_ascii_uppercase());
        }
    }
    let mut prev: Option<char> = None;
    for (index, c) in chars.enumerate() {
        if c.is_ascii_uppercase() {
            // handle existing uppercase characters that might be part of an acronym
            // as well as PascalCase
            // if previous character was not uppercase, insert underscore
            if let Some(prev) = prev {
                if !prev.is_ascii_uppercase() {
                    result.push('_');
                } else if let Some(next) = s.chars().nth(index + 2) {
                    // if next character is lowercase, insert underscore
                    if next.is_ascii_lowercase() {
                        result.push('_');
                    }
                }
            }
        } else if c.is_whitespace() {
            result.push('_'); // replace whitespace with underscore
        } else if c == '-' {
            result.push('_'); // replace hyphen with underscore
        } else if c == '_' {
            result.push('_'); // keep existing underscore
        }
        if c.is_alphanumeric() {
            result.push(c.to_ascii_uppercase());
        } // ignore other characters
        prev = Some(c);
    }
    result = result // special cases
        .replace("TEXTSUBSCRIPT", "")
        .replace("___", "_")
        .replace("__", "_")
        .replace('å', "A")
        .replace('ö', "O");
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_screaming_snake_case() {
        assert_eq!(screaming_snake_case("PascalCase"), "PASCAL_CASE");
        assert_eq!(screaming_snake_case("snake_case"), "SNAKE_CASE");
        assert_eq!(screaming_snake_case("DST Offset"), "DST_OFFSET");
        assert_eq!(screaming_snake_case("Headset - HS"), "HEADSET_HS");
        assert_eq!(screaming_snake_case("GAP"), "GAP");
        assert_eq!(screaming_snake_case("length (ångström)"), "LENGTH_ANGSTROM");
        assert_eq!(screaming_snake_case("LANAccessUsingPPP"), "LAN_ACCESS_USING_PPP");
        assert_eq!(screaming_snake_case("pressure (pound-force)"), "PRESSURE_POUND_FORCE");
        assert_eq!(screaming_snake_case("CIE 13.3-1995 Color"), "CIE_133_1995_COLOR");
        assert_eq!(screaming_snake_case("CO\\\\textsubscript{2} Conc"), "CO2_CONC");
        assert_eq!(screaming_snake_case("3D Display"), "_3D_DISPLAY");
    }
}
