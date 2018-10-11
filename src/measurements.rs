use tokesies::*;

pub struct XFilter;

impl filters::Filter for XFilter {
    fn on_char(&self, c: &char) -> (bool, bool) {
        match *c {
            'x' => (true, false),
            'X' => (true, false),
            _ => (false, false),
        }
    }
}

enum Units {
    Inches,
    Feet,
}

pub fn determine_dimensions_from_spec(spec: &str) -> (f64, f64, f64) {
    let tokens = FilteredTokenizer::new(XFilter {}, spec).collect::<Vec<Token>>();
    let width = get_from_spec_token(tokens.get(0).unwrap().term(), Units::Inches);
    let height = get_from_spec_token(tokens.get(1).unwrap().term(), Units::Inches);
    let length = get_from_spec_token(tokens.get(2).unwrap().term(), Units::Feet);

    (width, height, length)
}

fn get_from_spec_token(spec_token: &str, desired_unit: Units) -> f64 {
    const CONVERSION_FACTOR_INCHES_FEET: f64 = 12.0;
    let mut parse_result: f64;
    let replaced = spec_token.trim().replace("'", "").replace("\"", "");
    let result = replaced.parse();

    parse_result = match result {
        Ok(v) => v,
        Err(e) => panic!("Unable to parse result {} due to {}", replaced, e),
    };

    match desired_unit {
        Units::Feet => if spec_token.ends_with("\"") {
            parse_result = parse_result / CONVERSION_FACTOR_INCHES_FEET;
        },
        Units::Inches => if spec_token.ends_with("'") {
            parse_result = parse_result * CONVERSION_FACTOR_INCHES_FEET;
        },
    }

    parse_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_retrieve_feet_from_spec_token_specifying_feet() {
        let spec_token = "8'";
        let result = get_from_spec_token(&spec_token, Units::Feet);

        assert_eq!(result, 8.0);
    }

    #[test]
    fn it_should_retrieve_feet_from_spec_token_specifying_nothing() {
        let spec_token = "8";
        let result = get_from_spec_token(&spec_token, Units::Feet);

        assert_eq!(result, 8.0);
    }

    #[test]
    fn it_should_retrieve_feet_from_spec_token_specifying_inches() {
        let spec_token = "12\"";
        let result = get_from_spec_token(&spec_token, Units::Feet);

        assert_eq!(result, 1.0);
    }

    #[test]
    fn it_should_retrieve_inches_from_spec_token_specifying_inches() {
        let spec_token = "10\"";
        let result = get_from_spec_token(&spec_token, Units::Inches);

        assert_eq!(result, 10.0);
    }

    #[test]
    fn it_should_retrieve_inches_from_spec_token_specifying_nothing() {
        let spec_token = "8";
        let result = get_from_spec_token(&spec_token, Units::Inches);

        assert_eq!(result, 8.0);
    }

    #[test]
    fn it_should_retrieve_inches_from_spec_token_specifying_feet() {
        let spec_token = "10\'";
        let result = get_from_spec_token(&spec_token, Units::Inches);

        assert_eq!(result, 120.0);
    }
}
