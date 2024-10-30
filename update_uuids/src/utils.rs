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
    for c in chars {
        if c.is_ascii_uppercase() {
            result.push('_'); // insert underscore before uppercase letter, except the first letter
            result.push(c.to_ascii_uppercase());
        } else if c.is_whitespace() {
            result.push('_'); // replace whitespace with underscore
        } else if c == '-' {
            result.push('_'); // replace hyphen with underscore
        } else if c == '_' {
            result.push('_'); // keep existing underscore
        } else if c.is_alphanumeric() {
            result.push(c.to_ascii_uppercase());
        } // ignore other characters
    }
    result = result.replace("TEXTSUBSCRIPT", "").replace("___", "_"); // special case
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
        assert_eq!(
            screaming_snake_case("pressure (pound-force per square inch)"),
            "PRESSURE_POUND_FORCE_PER_SQUARE_INCH"
        );
        assert_eq!(
            screaming_snake_case("CIE 13.3-1995 Color Rendering Index"),
            "CIE_133_1995_COLOR_RENDERING_INDEX"
        );
        assert_eq!(
            screaming_snake_case("CO\\\\textsubscript{2} Concentration"),
            "CO2_CONCENTRATION"
        );
        assert_eq!(screaming_snake_case("3D Display"), "_3D_DISPLAY");
    }
}
