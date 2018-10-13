extern crate tokesies;

pub mod numerical;
pub mod measurements;

use std::collections::HashMap;
use std::io;
use std::fmt;
use std::fmt::Write;
use std::hash::{Hash, Hasher};

use numerical::FractionalValue;
use measurements::determine_dimensions_from_spec;

pub fn get_conversion_chart_inches() -> HashMap<FractionalValue, f64> {
    let mut conversion_chart_inches: HashMap<FractionalValue, f64> = HashMap::new();

    conversion_chart_inches.insert(FractionalValue::new(1.0), 0.75);
    conversion_chart_inches.insert(FractionalValue::new(1.25), 1.0);
    conversion_chart_inches.insert(FractionalValue::new(1.5), 1.25);
    conversion_chart_inches.insert(FractionalValue::new(2.0), 1.5);
    conversion_chart_inches.insert(FractionalValue::new(3.0), 2.5);
    conversion_chart_inches.insert(FractionalValue::new(4.0), 3.5);
    conversion_chart_inches.insert(FractionalValue::new(5.0), 4.5);
    conversion_chart_inches.insert(FractionalValue::new(6.0), 5.5);
    conversion_chart_inches.insert(FractionalValue::new(7.0), 6.25);
    conversion_chart_inches.insert(FractionalValue::new(8.0), 7.25);
    conversion_chart_inches.insert(FractionalValue::new(10.0), 9.25);
    conversion_chart_inches.insert(FractionalValue::new(12.0), 11.25);

    conversion_chart_inches
}

pub fn to_actual_size_in_inches(input: f64) -> f64 {
    let conversion_chart_inches = get_conversion_chart_inches();
    let decomposed_float = FractionalValue::new(input);
    let lookup_result = conversion_chart_inches.get(&decomposed_float);

    match lookup_result {
        Some(x) => *x,
        None => panic!("{} was not found in the lookup table"),
    }
}

pub struct Settings {
    blade_width_inches: f64,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            blade_width_inches: 0.125,
        }
    }

    pub fn new_from_hashmap(hashmap: HashMap<String, String>) -> Settings {
        // Default value for cut width is 1/8".
        let mut blade_width_inches = 0.125;
        if hashmap.contains_key("blade_width_inches") {
            let bwi_string = hashmap.get("blade_width_inches").unwrap();
            blade_width_inches = bwi_string.parse::<f64>().unwrap();
        }

        Settings {
            blade_width_inches: blade_width_inches,
        }
    }

    pub fn get_blade_width_inches(self) -> f64 {
        self.blade_width_inches
    }
}

pub struct Lumber {
    width_inches: f64,
    height_inches: f64,
    length_feet: f64,
    is_nominal: bool,
}

impl PartialEq for Lumber {
    fn eq(&self, other: &Lumber) -> bool {
        self.get_identifier_string() == other.get_identifier_string()
    }
}

impl Eq for Lumber {}

impl Hash for Lumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_identifier_string().hash(state);
    }
}

impl fmt::Display for Lumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}x{}x{}",
            self.width_inches, self.height_inches, self.length_feet
        )
    }
}

impl Lumber {
    pub fn get_identifier_string(&self) -> String {
        let mut result = String::new();
        write!(&mut result, "{}", self)
            .expect("Unable to write output to retrieve lumber identifier");

        result
    }

    pub fn get_width_in_inches(&self) -> f64 {
        self.width_inches
    }

    pub fn get_height_in_inches(&self) -> f64 {
        self.height_inches
    }

    pub fn get_length_in_feet(&self) -> f64 {
        self.length_feet
    }

    pub fn get_length_in_inches(&self) -> f64 {
        12.0 * self.get_length_in_feet()
    }

    pub fn create_nominal(width_inches: f64, height_inches: f64, length_feet: f64) -> Lumber {
        Lumber {
            width_inches: width_inches,
            height_inches: height_inches,
            length_feet: length_feet,
            is_nominal: true,
        }
    }

    pub fn create_actual(width_inches: f64, height_inches: f64, length_feet: f64) -> Lumber {
        Lumber {
            width_inches: width_inches,
            height_inches: height_inches,
            length_feet: length_feet,
            is_nominal: false,
        }
    }

    pub fn create_from_spec(spec: &String) -> Lumber {
        let whl_tuple = determine_dimensions_from_spec(spec);

        Lumber::create_actual(whl_tuple.0, whl_tuple.1, whl_tuple.2)
    }

    pub fn as_actual_size(&self) -> Lumber {
        if self.is_nominal {
            Lumber {
                width_inches: to_actual_size_in_inches(self.get_width_in_inches()),
                height_inches: to_actual_size_in_inches(self.get_height_in_inches()),
                length_feet: self.get_length_in_feet(),
                is_nominal: false,
            }
        } else {
            Lumber {
                width_inches: self.width_inches,
                height_inches: self.height_inches,
                length_feet: self.length_feet,
                is_nominal: false,
            }
        }
    }

    fn get_nearest_nominal_size_inches(inches: f64) -> Result<f64, io::Error> {
        if inches <= 1.0 {
            return Ok(1.0);
        } else if inches < 1.25 {
            return Ok(1.25);
        } else if inches < 1.5 {
            return Ok(1.5);
        } else if inches <= 2.0 {
            return Ok(2.0);
        } else if inches <= 3.0 {
            return Ok(3.0);
        } else if inches <= 4.0 {
            return Ok(4.0);
        } else if inches <= 5.0 {
            return Ok(5.0);
        } else if inches <= 6.0 {
            return Ok(6.0);
        } else if inches <= 7.0 {
            return Ok(7.0);
        } else if inches <= 8.0 {
            return Ok(8.0);
        } else if inches <= 9.0 {
            return Ok(9.0);
        } else if inches <= 10.0 {
            return Ok(10.0);
        } else if inches <= 11.0 {
            return Ok(11.0);
        } else if inches < 12.0 {
            return Ok(12.0);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unable to determine nominal size (in inches) for value",
            ));
        }
    }

    fn get_nearest_nominal_size_feet(feet: f64) -> Result<f64, io::Error> {
        if feet > 16.0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Boards typically do not come in longer than 16' lengths",
            ));
        }

        Ok(feet.ceil())
    }

    pub fn as_nearest_nominal(&self) -> Lumber {
        if self.is_nominal {
            Lumber {
                width_inches: self.width_inches,
                height_inches: self.height_inches,
                length_feet: self.length_feet,
                is_nominal: true,
            }
        } else {
            // The first thing we should do is make sure that we correctly organize the
            // length and height, as there isn't any such thing as a 4x1.
            let mut width = self.width_inches;
            let mut height = self.height_inches;
            if width > height {
                width = self.height_inches;
                height = self.width_inches;
            }

            let width =
                Lumber::parse_nominal_size_result(Lumber::get_nearest_nominal_size_inches(width));

            let height =
                Lumber::parse_nominal_size_result(Lumber::get_nearest_nominal_size_inches(height));

            let length_feet = Lumber::parse_nominal_size_result(
                Lumber::get_nearest_nominal_size_feet(self.length_feet),
            );

            Lumber {
                width_inches: width,
                height_inches: height,
                length_feet: length_feet,
                is_nominal: false,
            }
        }
    }

    fn parse_nominal_size_result(result: Result<f64, io::Error>) -> f64 {
        let value = match result {
            Ok(v) => v,
            Err(e) => panic!("Unable to retrieve nominal size for input, due to: {}", e),
        };

        value
    }
}

pub struct CutList {
    settings: Settings,
    required_boards: HashMap<Lumber, i32>,
}

impl fmt::Display for CutList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Purchase List\n--------\n");
        for l in self.required_boards.iter() {
            write!(f, "{} ({})\n", l.0, l.1);
        }

        Ok(())
    }
}

impl CutList {
    pub fn new() -> CutList {
        let settings = Settings::new();
        let required_boards: HashMap<Lumber, i32> = HashMap::new();

        CutList {
            settings: settings,
            required_boards: required_boards,
        }
    }

    pub fn new_with_settings(settings: Settings) -> CutList {
        let required_boards: HashMap<Lumber, i32> = HashMap::new();

        CutList {
            settings: settings,
            required_boards: required_boards,
        }
    }

    pub fn add(&mut self, lumber: Lumber) {
        let current_count: i32 = if self.required_boards.contains_key(&lumber) {
            *self.required_boards.get(&lumber).unwrap()
        } else {
            0
        };

        self.required_boards.insert(lumber, current_count + 1);
    }

    pub fn get_num_boards(&self) -> i32 {
        let mut len = 0;
        for board_tuple in &self.required_boards {
            len = len + board_tuple.1;
        }

        len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_1_by_2_should_be_3_4_by_1_5_inches() {
        let one_by_two_nominal = Lumber::create_nominal(1.0, 2.0, 8.0);
        let one_by_two_actual = one_by_two_nominal.as_actual_size();

        assert_eq!(one_by_two_actual.get_width_in_inches(), 0.75);
        assert_eq!(one_by_two_actual.get_height_in_inches(), 1.5);
        assert_eq!(one_by_two_actual.get_length_in_feet(), 8.0);
    }

    #[test]
    fn get_nearest_nominal_size_inches_should_return_1_25_for_1_22_inches() {
        let actual_result = Lumber::get_nearest_nominal_size_inches(1.22);
        let actual = match actual_result {
            Ok(v) => v,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, 1.25);
    }

    #[test]
    fn a_0_25_inch_by_5_32_inch_actual_size_should_convert_to_a_1_x6_nominal_size() {
        let actual_lumber_dim = Lumber::create_actual(0.25, 5.32, 6.22);
        let nominal_board = actual_lumber_dim.as_nearest_nominal();

        assert_eq!(nominal_board.get_width_in_inches(), 1.0);
        assert_eq!(nominal_board.get_height_in_inches(), 6.0);
        assert_eq!(nominal_board.get_length_in_feet(), 7.0);
    }

    #[test]
    #[should_panic]
    fn it_should_throw_an_exception_for_a_board_over_16_feet() {
        let actual_lumber_dim = Lumber::create_actual(2.0, 4.0, 18.0);
        let _nominal_board = actual_lumber_dim.as_nearest_nominal();
    }

    #[test]
    fn the_identifier_string_for_a_2x4_should_be_2x4x8() {
        let two_by_four = Lumber::create_nominal(2.0, 4.0, 8.0);
        let id_string = two_by_four.get_identifier_string();

        assert_eq!(id_string, "2x4x8");
    }

    #[test]
    fn settings_should_return_a_default_blade_width_of_0125in() {
        let settings = Settings::new();

        assert_eq!(settings.get_blade_width_inches(), 0.125);
    }

    #[test]
    fn settings_specified_blade_width_should_be_preserved() {
        let mut hm: HashMap<String, String> = HashMap::new();
        hm.insert(String::from("blade_width_inches"), String::from(".334"));
        let settings = Settings::new_from_hashmap(hm);

        assert_eq!(settings.get_blade_width_inches(), 0.334);
    }
}
