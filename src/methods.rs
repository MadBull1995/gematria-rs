// Defines the core gematria calculation methods and traits for the Gematria-rs library.

use crate::{CharMap, FullCharMap};

/// Enumerates various gematria calculation methods.
/// Includes traditional and specialized methods like Mispar Hechrechi and Otiyot BeMilui.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GematriaMethod {
    MisparHechrechi,
    MisparGadol,
    MisparKatan,
    MisparSiduri,
    MisparBoneh,
    MisparMeugal,
    MisparMusafi,
    OtiyotBeMilui,
}

/// A trait defining the common functionality for gematria calculations.
pub trait GematriaCalculation {
    /// Calculates the gematria value for a given letter index.
    fn calculate_value(&self, letter_index: u32) -> u32;

    /// Returns the type of gematria calculation method.
    fn method_type(&self) -> GematriaMethod;
}

/// Calculates the standard gematria value for a given Hebrew letter based on its index.
/// 
/// $$f(x) = 10^{\left\lfloor \frac{x - 1}{9} \right\rfloor} \times \left((x - 1) \mod 9 + 1\right)$$
/// 
/// Here's a breakdown of this formula:
///
/// - **\( x \)**: This represents the index of the Hebrew character in the alphabet. For example, א (Aleph) is 1, ב (Bet) is 2, and so on.
///
/// - **\( 10^(⌊(x - 1) / 9⌋) \)**: This part of the formula calculates the magnitude of the number. It divides the index minus one by 9, floors the result (which means it rounds down to the nearest whole number), and then raises 10 to the power of this floored value.
///
/// - **\( ((x - 1) \mod 9 + 1) \)**: This part of the formula calculates the unit value. It takes the index minus one, calculates its modulo 9 (the remainder when divided by 9), and adds 1 to this result.
///
/// - The final value of \( f(x) \) is the product of these two parts.
/// 
/// In the standard gematria system (Mispar Hechrechi), each Hebrew letter is assigned a numerical value.
/// The first ten letters (א to י) are numbered 1 to 10. The next eight letters (כ to צ) are assigned values
/// 20 to 90 in increments of 10. The final four letters (ק to ת) are assigned values 100 to 400 in increments
/// of 100. This function takes the index of a Hebrew letter and calculates its corresponding gematria value
/// following this system.
///
/// The calculation is based on the pattern where each letter's value is a power of 10 multiplied by its position
/// within its group (1-10, 11-18, or 19-22).
///
/// # Arguments
///
/// * `letter_index` - A reference to the index of the letter in the Hebrew alphabet (1-based).
///
/// # Returns
///
/// Returns the standard gematria value as a `u32`.
///
/// # Example
///
/// ```
/// use gematria_rs::std_gematria_value;
/// let aleph_index = 1;
/// let aleph_value = std_gematria_value(&aleph_index);
/// assert_eq!(aleph_value, 1);
///
/// let lamed_index = 12;
/// let lamed_value = std_gematria_value(&lamed_index);
/// assert_eq!(lamed_value, 30);
/// ```
/// 
/// # References
///
/// More information about the standard gematria system can be found on the Wikipedia page:
/// [Gematria - Wikipedia](https://en.wikipedia.org/wiki/Gematria#Standard_encoding_(Mispar_Hechrechi))
pub fn std_gematria_value(letter_index: &u32) -> u32 {
    10u32.pow((letter_index - 1) / 9) * (((letter_index - 1) % 9) + 1)
}

#[derive(Clone)]
pub struct MisparHechrechi;

impl GematriaCalculation for MisparHechrechi {
    fn calculate_value(&self, letter_index: u32) -> u32 {
        match letter_index {
            // Unique handling for final forms
            23 => 20, // ך
            24 => 40, // ם
            25 => 50, // ן
            26 => 80, // ף
            27 => 90, // ץ
            // Standard calculation for other letters
            index => std_gematria_value(&index),
        }
    }

    fn method_type(&self) -> GematriaMethod {
        GematriaMethod::MisparHechrechi
    }
}

#[derive(Clone)]
pub struct MisparGadol;

impl GematriaCalculation for MisparGadol {
    fn calculate_value(&self, letter_index: u32) -> u32 {
        match letter_index {
            // Unique handling for final forms
            23 => 500, // ך
            24 => 600, // ם
            25 => 700, // ן
            26 => 800, // ף
            27 => 900, // ץ
            // Standard calculation for other letters
            index => std_gematria_value(&index),
        }
    }

    fn method_type(&self) -> GematriaMethod {
        GematriaMethod::MisparGadol
    }
}

#[derive(Clone)]
pub struct MisparKatan;

impl GematriaCalculation for MisparKatan {
    fn calculate_value(&self, letter_index: u32) -> u32 {
        let value = match letter_index {
            // Unique handling for final forms
            23 => 500, // ך
            24 => 600, // ם
            25 => 700, // ן
            26 => 800, // ף
            27 => 900, // ץ
            // Standard calculation for other letters
            _ => std_gematria_value(&letter_index),
        };

        // Reduce the value to a single digit
        Self::reduce_to_single_digit(value)
    }

    fn method_type(&self) -> GematriaMethod {
        GematriaMethod::MisparKatan
    }
}

impl MisparKatan {
    fn reduce_to_single_digit(mut value: u32) -> u32 {
        while value >= 10 {
            value = value
                .to_string()
                .chars()
                .map(|d| d.to_digit(10).unwrap())
                .sum();
        }
        value
    }
}

/// Represents the Otiyot BeMilui method where each letter is represented by its full spelling.
pub struct OtyiotBeMilui {
    filled_letters: FullCharMap,
    char_to_index: CharMap,
}

impl GematriaCalculation for OtyiotBeMilui {
    fn calculate_value(&self, letter_index: u32) -> u32 {
        let mut val = 0;
        // Convert index to character first
        if let Some(letter) = self.index_to_char(letter_index) {
            val = self.explode_full_letters(letter)
        };

        val
    }

    fn method_type(&self) -> GematriaMethod {
        GematriaMethod::OtiyotBeMilui
    }
}

impl OtyiotBeMilui {
    pub fn new(full_map: FullCharMap, index_map: CharMap) -> Self {
        Self {
            filled_letters: full_map,
            char_to_index: index_map,
        }
    }

    fn explode_full_letters(&self, charcter: char) -> u32 {
        if let Some(filled_form) = self.filled_letters.get(&charcter) {
            filled_form
                .iter()
                .filter_map(|&c| Some(
                    match self.char_to_index.get(&c).unwrap() {
                        // Unique handling for final forms
                        23 => 20, // ך
                        24 => 40, // ם
                        25 => 50, // ן
                        26 => 80, // ף
                        27 => 90, // ץ
                        // Standard calculation for other letters
                        index => std_gematria_value(index),
                    }))
                .sum()
        } else {
            0
        }
    }

    fn index_to_char(&self, index: u32) -> Option<char> {
        self.char_to_index
            .iter()
            .find_map(|(&c, &i)| if i == index { Some(c) } else { None })
    }
}
