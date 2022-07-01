use std::collections::HashMap;

pub struct Solution {
    roman_number: HashMap<String, i32>
}

impl Solution {
    pub fn new() -> Self {
        Solution{
            roman_number:HashMap::from([(String::from("IV"), 4), (String::from("IX"), 9),
                                        (String::from("XL"), 40), (String::from("XC"), 90),
                                        (String::from("CD"), 400), (String::from("CM"),900),
                                        (String::from("I"), 1), (String::from("V"), 5),
                                        (String::from("X"), 10), (String::from("L"), 50),
                                        (String::from("C"), 100), (String::from("D"), 500),
                                        (String::from("M"), 1000)])
        }
    }

    pub fn roman_to_int(&self, s: String) -> i32 {
        let mut value: i32 = 0;
        let mut roman_string = s;

        while !roman_string.is_empty() {
            let c = &roman_string[..1];
            match c {
                "I" | "X" | "C" => {
                    match roman_string.char_indices().nth(1) {
                        Some(_n) => {
                            let twochars = &roman_string[..2];
                            match self.roman_number.get(twochars) {
                                Some(_n) => {
                                    value += self.roman_number.get(twochars).unwrap();
                                    roman_string.remove(1);
                                }
                                None => {
                                    value += self.roman_number.get(c).unwrap();
                                }
                            }
                        }
                        None => {
                            value += self.roman_number.get(c).unwrap();
                        }
                    }
                }
                _ => {
                    value += self.roman_number.get(c).unwrap();
                }
            }

            roman_string.remove(0);
        }
        value
    }
}