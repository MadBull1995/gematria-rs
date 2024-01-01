//! Gematria-rs Library API
//!
//! This module provides an implementation of the Gematria, a traditional Hebrew numerology system.
//! It supports various methods such as Mispar Hechrechi, Mispar Gadol, Mispar Katan, and Otiyot BeMilui.
//!
//! The core functionality is encapsulated in the `GematriaContext` struct, which offers methods to
//! calculate gematria values for Hebrew characters and words. It supports optional caching for improved
//! performance and can be configured to preserve or remove Hebrew vowels in the calculations.
//!
//! [`GematriaBuilder`] facilitates a flexible construction of `GematriaContext` with various configuration options.
//!
//! Example usage with [`GematriaBuilder`]:
//! ```
//! use gematria_rs::{GematriaBuilder, GematriaMethod};
//!
//! let gmctx = GematriaBuilder::new()
//!     .with_method(GematriaMethod::MisparHechrechi)
//!     .with_cache(true)
//!     .with_vowels(true)
//!     .init_gematria();
//!
//! let hello = "שָׁלוֹם";
//! let res_1 = gmctx.calculate_value(hello);
//! println!("Gematria value: {}", res_1.value());
//! // The word original vowels preserved on the result
//! let hello_without_vowels = "שלום";
//! let res_2 = gmctx.calculate_value(hello_without_vowels);
//! assert_eq!(res_1.word(), hello);
//! assert_ne!(res_1.word(), hello_without_vowels);
//! assert_eq!(res_1.value(), res_2.value());
//! ```
//! Example usage with [`IntoGematriaVal`] trait:
//! ```
//! use gematria_rs::*;
//!
//! let c = 'א';
//! assert_eq!(c.gematria_val(&GematriaMethod::MisparKatan), 1);
//! let s = "בעזרת השם";
//! assert_eq!(
//!     s.gematria_val(&GematriaMethod::MisparHechrechi),
//!     1024
//! );
//! assert_eq!(
//!     s.to_string().gematria_val(&GematriaMethod::MisparGadol),
//!     1584
//! );
//! ```
//! Author: Amit Shmulevitch

mod methods;
use methods::OtyiotBeMilui;
pub use methods::{
    std_gematria_value, GematriaCalculation, GematriaMethod, MisparGadol, MisparHechrechi,
    MisparKatan,
};

use std::{cell::RefCell, collections::HashMap, io};

type GematriaCtxCache = RefCell<HashMap<(GematriaMethod, String), u32>>;

/// `GematriaContext` holds the core logic for gematria calculations.
/// It encapsulates the mapping of Hebrew characters to their numeric values and the chosen calculation strategy.
/// Optionally, it can cache calculated values for improved performance and handle vowel preservation in input words.
///
/// # Fields
/// - `character_map`: A mapping of Hebrew characters to their numeric values.
/// - `calculation_strategy`: The current gematria calculation strategy, implemented as a trait object.
/// - `cache`: An optional cache to store previously calculated gematria values for quick retrieval.
/// - `preserve_vowels`: A flag to determine whether to preserve Hebrew vowels in calculation results.
///
/// # Examples
///
/// Creating a default `GematriaContext`:
///
/// ```
/// use gematria_rs::GematriaContext;
///
/// // Initialize the builder and choose the calculation method
/// let gematria_context = GematriaContext::default();
///
/// // Use the context to calculate the gematria value of a word
/// let value = gematria_context.calculate_value("שלום");
/// println!("Gematria value: {}", value.value());
/// ```
///
/// Creating a new `GematriaContext` with a specific calculation method and default settings:
///
/// ```
/// use gematria_rs::{GematriaBuilder, GematriaMethod, GematriaContext};
///
/// // Initialize the builder and choose the calculation method
/// let gematria_context = GematriaBuilder::new()
///     .with_method(GematriaMethod::MisparGadol)
///     .init_gematria();
///
/// // Use the context to calculate the gematria value of a word
/// let value = gematria_context.calculate_value("שלום");
/// println!("Gematria value: {}", value.value());
/// ```
///
/// Creating a `GematriaContext` with caching enabled and vowel preservation:
///
/// ```
/// use gematria_rs::{GematriaBuilder, GematriaMethod, GematriaContext};
///
/// let gematria_context = GematriaBuilder::new()
///     .with_method(GematriaMethod::MisparGadol)
///     .with_cache(true)  // Enable caching
///     .with_vowels(true) // Preserve vowels
///     .init_gematria();
///
/// let value = gematria_context.calculate_value("גֵּם");
/// println!("Gematria value with vowels: {}", value.value());
/// ```
pub struct GematriaContext {
    // Hebrew character to numeric value mapping.
    character_map: HebrewCharacterMap,

    // The current gematria calculation strategy.
    calculation_strategy: Box<dyn GematriaCalculation>,

    // Optional cache for storing previously calculated values.
    cache: Option<GematriaCtxCache>,

    // Flag to determine whether to preserve vowels in calculations results.
    preserve_vowels: bool,
}

impl Default for GematriaContext {
    fn default() -> Self {
        GematriaBuilder::new().init_gematria()
    }
}

/// Used to alias the standard hebrew alphabet mapping.
pub type CharMap = HashMap<char, u32>;
/// Used to alias the "filled letters" hebrew alphabet mapping.
pub type FullCharMap = HashMap<char, Vec<char>>;

/// `HebrewCharacterMap` maps Hebrew characters to their corresponding numeric indices.
#[derive(Debug, Clone)]
pub struct HebrewCharacterMap {
    char_to_index: CharMap,
    // filled_letters: FullCharMap,
}

/// `GematriaResult` represents the result of a gematria calculation,
/// including the calculated value, the method used, and the original word.
#[derive(Debug, Clone)]
pub struct GematriaResult {
    // The calculated gematria value.
    value: u32,

    // The gematria calculation method used.
    method: GematriaMethod,

    // The original word for which the gematria value was calculated.
    word: String,
}

/// `GematriaBuilder` provides a builder pattern for constructing [`GematriaContext`].
/// It allows specifying the gematria calculation method, whether to enable caching, and vowel preservation.
/// Example usage:
/// ```
/// use gematria_rs::{GematriaBuilder, GematriaMethod};
///
/// let gmctx = GematriaBuilder::new()
///     .with_method(GematriaMethod::MisparHechrechi)
///     .with_cache(true)
///     .with_vowels(true)
///     .init_gematria();
///
/// let hello = "שָׁלוֹם";
/// let res_1 = gmctx.calculate_value(hello);
/// println!("Gematria value: {}", res_1.value());
/// // The word original vowels preserved on the result
/// let hello_without_vowels = "שלום";
/// let res_2 = gmctx.calculate_value(hello_without_vowels);
/// assert_eq!(res_1.word(), hello);
/// assert_ne!(res_1.word(), hello_without_vowels);
/// assert_eq!(res_1.value(), res_2.value());
/// ```
#[derive(Debug, Clone, Default)]
pub struct GematriaBuilder {
    // Optional calculation method.
    method: Option<GematriaMethod>,

    // Flag to enable or disable caching, defaulted to false.
    enable_cache: bool,

    // Flag to preserve or remove vowels in the input, defaulted to false.
    presevre_vowels: bool,
}

/// Used to create a hebrew letter filled map, used for [`methods::GematriaMethod::OtiyotBeMilui`] calculations.
fn create_hebrew_filled_letters_map() -> FullCharMap {
    let full_names = vec![
        ('א', vec!['א', 'ל', 'ף']),
        ('ב', vec!['ב', 'י', 'ת']),
        ('ג', vec!['ג', 'י', 'מ', 'ל']),
        ('ד', vec!['ד', 'ל', 'ת']),
        ('ה', vec!['ה', 'א']),
        ('ו', vec!['ו', 'י', 'ו']),
        ('ז', vec!['ז', 'י', 'ן']),
        ('ח', vec!['ח', 'י', 'ת']),
        ('ט', vec!['ט', 'י', 'ת']),
        ('י', vec!['י', 'ו', 'ד']),
        ('כ', vec!['כ', 'ף']),
        ('ל', vec!['ל', 'מ', 'ד']),
        ('מ', vec!['מ', 'ם']),
        ('נ', vec!['נ', 'ו', 'ן']),
        ('ס', vec!['ס', 'מ', 'ך']),
        ('ע', vec!['ע', 'י', 'ן']),
        ('פ', vec!['פ', 'א']),
        ('צ', vec!['צ', 'ד', 'י']),
        ('ק', vec!['ק', 'ו', 'ף']),
        ('ר', vec!['ר', 'י', 'ש']),
        ('ש', vec!['ש', 'י', 'ן']),
        ('ת', vec!['ת', 'י', 'ו']),
    ];

    let mut full_name_map = HashMap::new();
    for (letter, name) in full_names.into_iter() {
        full_name_map.insert(letter, name);
    }

    full_name_map
}

fn create_hebrew_index_map() -> CharMap {
    let letters = vec![
        'א', 'ב', 'ג', 'ד', 'ה', 'ו', 'ז', 'ח', 'ט', 'י', 'כ', 'ל', 'מ', 'נ', 'ס', 'ע', 'פ', 'צ',
        'ק', 'ר', 'ש', 'ת', // Final forms
        'ך', 'ם', 'ן', 'ף', 'ץ',
    ];

    let mut std_index_map = HashMap::new();
    for (index, letter) in letters.into_iter().enumerate() {
        std_index_map.insert(letter, (index + 1) as u32);
    }

    std_index_map
}

impl GematriaBuilder {
    /// Creates new `GematriaBuilder`.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Method to enable caching of values.
    pub fn with_cache(mut self, enable: bool) -> Self {
        self.enable_cache = enable;
        self
    }

    /// Sets a specific method to init the [`GematriaContext`], it is defaulted to [`methods::GematriaMethod::MisparHechrechi`].
    pub fn with_method(mut self, method: GematriaMethod) -> Self {
        self.method = Some(method);
        self
    }

    /// Will preserve the original vowels on outputs.
    pub fn with_vowels(mut self, presevre_vowels: bool) -> Self {
        self.presevre_vowels = presevre_vowels;
        self
    }

    /// Initializes the gematria library and returns necessary data structures.
    pub fn init_gematria(self) -> GematriaContext {
        let char_to_index = create_hebrew_index_map();
        let map = HebrewCharacterMap { char_to_index };
        let method = self.method.unwrap_or(GematriaMethod::MisparHechrechi);
        GematriaContext::new(map, method, self.enable_cache, self.presevre_vowels)
    }
}

// Utils function to parse the method of gematria.
fn process_method_dyn(
    method: GematriaMethod,
    char_map: HebrewCharacterMap,
) -> Box<dyn GematriaCalculation> {
    let strategy: Box<dyn GematriaCalculation> = match method {
        GematriaMethod::MisparHechrechi => Box::new(MisparHechrechi),
        GematriaMethod::MisparGadol => Box::new(MisparGadol),
        GematriaMethod::MisparKatan => Box::new(MisparKatan),
        GematriaMethod::OtiyotBeMilui => Box::new(OtyiotBeMilui::new(
            create_hebrew_filled_letters_map(),
            char_map.char_to_index,
        )),
        _ => unimplemented!(
            "{:?} is not yet implemented to calculate gematria values.",
            method
        ),
    };

    strategy
}

impl GematriaContext {
    pub fn new(
        char_map: HebrewCharacterMap,
        method: GematriaMethod,
        enable_cache: bool,
        preserve_vowels: bool,
    ) -> Self {
        let strategy = process_method_dyn(method, char_map.clone());

        let cache = if enable_cache {
            Some(RefCell::new(HashMap::new()))
        } else {
            None
        };

        Self {
            character_map: char_map,
            calculation_strategy: strategy,
            cache,
            preserve_vowels,
        }
    }

    /// Processing different hebrew vowels, will check against the flags passed to `GematriaContext`.
    fn handle_vowels(&self, word: &str) -> String {
        if self.preserve_vowels {
            word.to_string()
        } else {
            self.remove_hebrew_vowels(word)
        }
    }

    /// Removes all hebrew vowels from text.
    fn remove_hebrew_vowels(&self, text: &str) -> String {
        text.chars().filter(|&c| !self.is_hebrew_vowel(c)).collect()
    }

    /// Wheter this [`char`] vowel is included with vowel.
    fn is_hebrew_vowel(&self, c: char) -> bool {
        matches!(c, '\u{0591}'..='\u{05C7}')
    }

    /// Gets the hebrew char index within the alphabet order (1 based).
    fn get_indices_for_word(&self, word: &str) -> Vec<u32> {
        word.chars()
            .filter_map(|c| self.character_map.char_to_index.get(&c))
            .cloned()
            .collect()
    }

    /// Util function for calculate gematria value without using cache.
    fn calculate_value_no_cache(&self, word: &str) -> u32 {
        self.get_indices_for_word(word)
            .iter()
            .map(|&index| self.calculation_strategy.calculate_value(index))
            .sum()
    }

    /// Gets the current method used to calculate Gematria on the current [`GematriaContext`].
    pub fn get_current_method(&self) -> GematriaMethod {
        self.calculation_strategy.method_type()
    }

    /// Calculates the gematria value of a single Hebrew character.
    pub fn calculate_char_value(&self, character: char) -> u32 {
        let method = self.get_current_method();
        let cache_key = (method, character.to_string());

        // Check if value is in cache
        if let Some(ref cache) = self.cache {
            let cache = cache.borrow();
            if let Some(&value) = cache.get(&cache_key) {
                return value;
            }
        }

        // Calculate and cache the value if not found
        if let Some(index) = self.get_character_index(&character) {
            let value = self.calculation_strategy.calculate_value(*index);
            if let Some(ref cache) = self.cache {
                cache.borrow_mut().insert(cache_key, value);
            }
            value
        } else {
            0
        }
    }

    /// Calculates the gematria value of a Hebrew word or phrase.
    pub fn calculate_value(&self, text: &str) -> GematriaResult {
        let method = self.get_current_method();
        let processed_text = self.handle_vowels(text);
        // Check if caching is enabled and use it if available
        if let Some(ref cache) = self.cache {
            let mut cache = cache.borrow_mut();
            if let Some(&value) = cache.get(&(method, processed_text.to_string())) {
                return GematriaResult::new(value, method, processed_text.to_owned());
            }

            let val = self.calculate_value_no_cache(&processed_text);
            cache.insert((method, processed_text.to_string()), val);
            return GematriaResult::new(val, self.get_current_method(), processed_text.to_owned());
        }

        // Calculate without cache
        let val = self.calculate_value_no_cache(&processed_text);
        GematriaResult::new(val, self.get_current_method(), processed_text.to_owned())
    }

    /// Searches for words in the provided text with a gematria value matching that of the target word.
    pub fn search_matching_words(&self, target_word: &str, text: &str) -> Vec<String> {
        let target_value = self.calculate_value(target_word).value();
        text.split_whitespace()
            .flat_map(|w| w.split('\u{05BE}'))
            .filter_map(|word| {
                let processed_text = self.handle_vowels(word);
                let word_value = self.calculate_value(&processed_text).value();
                if word_value == target_value {
                    Some(processed_text)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Searches for words in the provided text with a gematria value matching that of the target value.
    pub fn search_matching_values(&self, target_value: &u32, text: &str) -> Vec<String> {
        text.split_whitespace()
            .flat_map(|w| w.split('\u{05BE}'))
            .filter_map(|word| {
                let processed_text = self.handle_vowels(word);
                let word_value = self.calculate_value(&processed_text).value();
                if word_value == *target_value {
                    Some(processed_text)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Reads a text and groups words with matching gematria values, avoiding duplicates.
    ///
    /// # Examples:
    ///
    /// ```
    /// use std::io;
    /// use gematria_rs::GematriaContext;
    ///
    /// let gmctx = GematriaContext::default();
    /// let grouped_result = gmctx.group_words_by_gematria("נכנס יין יצא סוד")?;
    ///
    /// assert_eq!(grouped_result, vec![(70, vec!["יין".to_string(),"סוד".to_string()])]);
    /// # Ok::<(), io::Error>(())
    /// ```
    pub fn group_words_by_gematria(&self, text: &str) -> io::Result<Vec<(u32, Vec<String>)>> {
        let mut grouped_words = HashMap::new();
        for word in text.split_whitespace().flat_map(|w| w.split('\u{05BE}')) {
            let processed_text = self.handle_vowels(word);

            let value = self.calculate_value(&processed_text).value();

            grouped_words
                .entry(value)
                .or_insert_with(Vec::new)
                .push_if_not_exists(processed_text);
        }

        // Filter out entries with only one word
        grouped_words.retain(|_, v| v.len() > 1);

        // Convert HashMap to Vec and sort by the length of the vectors
        let mut grouped_vec: Vec<(u32, Vec<String>)> = grouped_words.into_iter().collect();

        // Sort by the length of the vectors (primary) and gematria value (secondary)
        grouped_vec.sort_by(|a, b| match b.1.len().cmp(&a.1.len()) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            other => other,
        });

        // Return the sorted Vec
        Ok(grouped_vec)
    }

    /// Gets the index of a Hebrew character.
    pub fn get_character_index(&self, character: &char) -> Option<&u32> {
        self.character_map.char_to_index.get(character)
    }

    /// Sets the current gematria method to desired one.
    pub fn set_method(&mut self, method: GematriaMethod) {
        self.calculation_strategy = process_method_dyn(method, self.character_map.clone());
    }
}

/// `GematriaResult` used for structured result of calculations.
impl GematriaResult {
    /// Creates a new result object.
    pub fn new(value: u32, method: GematriaMethod, word: String) -> Self {
        GematriaResult {
            method,
            value,
            word,
        }
    }

    /// Gets the gematria value.
    pub fn value(&self) -> u32 {
        self.value
    }

    /// Gets the calculation method used.
    pub fn method(&self) -> &GematriaMethod {
        &self.method
    }

    /// Gets the word for which the gematria value was calculated.
    pub fn word(&self) -> &str {
        &self.word
    }
}

// Helper function to add a word to the vector if it doesn't already exist
trait PushIfNotExists {
    fn push_if_not_exists(&mut self, item: String);
}

impl PushIfNotExists for Vec<String> {
    fn push_if_not_exists(&mut self, item: String) {
        if !self.contains(&item) {
            self.push(item);
        }
    }
}

/// Used to implement `Into` functionality for easy calculations.
pub trait IntoGematriaVal {
    /// Calculate the value for a given cipher
    fn gematria_val(&self, method: &GematriaMethod) -> u32;
}

impl IntoGematriaVal for char {
    /// # Examples
    /// ```
    /// use gematria_rs::*;
    ///
    /// let val = 'א'.gematria_val(&GematriaMethod::MisparHechrechi);
    /// assert_eq!(val, 1)
    /// ```
    fn gematria_val(&self, method: &GematriaMethod) -> u32 {
        let gmctx = GematriaBuilder::new()
            .with_method(*method)
            .with_vowels(true)
            .init_gematria();
        gmctx.calculate_char_value(*self)
    }
}

impl IntoGematriaVal for String {
    /// # Examples
    /// ```
    /// use gematria_rs::*;
    ///
    /// let val = "בעזרת השם".to_string().gematria_val(&GematriaMethod::MisparHechrechi);
    /// assert_eq!(val, 1024)
    /// ```
    fn gematria_val(&self, method: &GematriaMethod) -> u32 {
        let gmctx = GematriaBuilder::new()
            .with_method(*method)
            .with_vowels(true)
            .init_gematria();
        gmctx.calculate_value(self).value()
    }
}

impl IntoGematriaVal for str {
    /// # Examples
    /// ```
    /// use gematria_rs::*;
    ///
    /// let val = "בעזרת השם".gematria_val(&GematriaMethod::MisparHechrechi);
    /// assert_eq!(val, 1024)
    /// ```
    fn gematria_val(&self, method: &GematriaMethod) -> u32 {
        let gmctx = GematriaBuilder::new()
            .with_method(*method)
            .with_vowels(true)
            .init_gematria();
        gmctx.calculate_value(self).value()
    }
}

#[cfg(test)]
mod tests {
    // Tests to validate the functionality of the Gematria-rs library
    use super::*;

    #[test]
    fn test_default_map() {
        let gmctx = GematriaContext::default();

        assert_eq!(gmctx.get_current_method(), GematriaMethod::MisparHechrechi);
        assert_eq!(gmctx.calculate_char_value('א'), 1);
        assert_eq!(gmctx.calculate_char_value('ב'), 2);
        assert_eq!(gmctx.calculate_char_value('ג'), 3);
        assert_eq!(gmctx.calculate_char_value('ד'), 4);
        assert_eq!(gmctx.calculate_char_value('ה'), 5);
        assert_eq!(gmctx.calculate_char_value('ו'), 6);
        assert_eq!(gmctx.calculate_char_value('ז'), 7);
        assert_eq!(gmctx.calculate_char_value('ח'), 8);
        assert_eq!(gmctx.calculate_char_value('ט'), 9);
        assert_eq!(gmctx.calculate_char_value('י'), 10);
        assert_eq!(gmctx.calculate_char_value('כ'), 20);
        assert_eq!(gmctx.calculate_char_value('ל'), 30);
        assert_eq!(gmctx.calculate_char_value('מ'), 40);
        assert_eq!(gmctx.calculate_char_value('נ'), 50);
        assert_eq!(gmctx.calculate_char_value('ס'), 60);
        assert_eq!(gmctx.calculate_char_value('ע'), 70);
        assert_eq!(gmctx.calculate_char_value('פ'), 80);
        assert_eq!(gmctx.calculate_char_value('צ'), 90);
        assert_eq!(gmctx.calculate_char_value('ק'), 100);
        assert_eq!(gmctx.calculate_char_value('ר'), 200);
        assert_eq!(gmctx.calculate_char_value('ש'), 300);
        assert_eq!(gmctx.calculate_char_value('ת'), 400);
        assert_eq!(gmctx.calculate_char_value('ך'), 20);
        assert_eq!(gmctx.calculate_char_value('ם'), 40);
        assert_eq!(gmctx.calculate_char_value('ן'), 50);
        assert_eq!(gmctx.calculate_char_value('ף'), 80);
        assert_eq!(gmctx.calculate_char_value('ץ'), 90);
    }

    #[test]
    fn test_filled_letters_map() {
        let gmctx = GematriaBuilder::new()
            .with_method(GematriaMethod::OtiyotBeMilui)
            .init_gematria();

        assert_eq!(gmctx.calculate_char_value('א'), 111);
        assert_eq!(gmctx.calculate_char_value('ב'), 412);
        assert_eq!(gmctx.calculate_char_value('ג'), 83);
        assert_eq!(gmctx.calculate_char_value('ד'), 434);
        assert_eq!(gmctx.calculate_char_value('ה'), 6);
        assert_eq!(gmctx.calculate_char_value('ו'), 22);
        assert_eq!(gmctx.calculate_char_value('ז'), 67);
        assert_eq!(gmctx.calculate_char_value('ח'), 418);
        assert_eq!(gmctx.calculate_char_value('ט'), 419);
        assert_eq!(gmctx.calculate_char_value('י'), 20);
        assert_eq!(gmctx.calculate_char_value('כ'), 100);
        assert_eq!(gmctx.calculate_char_value('ל'), 74);
        assert_eq!(gmctx.calculate_char_value('מ'), 80);
        assert_eq!(gmctx.calculate_char_value('נ'), 106);
        assert_eq!(gmctx.calculate_char_value('ס'), 120);
        assert_eq!(gmctx.calculate_char_value('ע'), 130);
        assert_eq!(gmctx.calculate_char_value('פ'), 81);
        assert_eq!(gmctx.calculate_char_value('צ'), 104);
        assert_eq!(gmctx.calculate_char_value('ק'), 186);
        assert_eq!(gmctx.calculate_char_value('ר'), 510);
        assert_eq!(gmctx.calculate_char_value('ש'), 360);
        assert_eq!(gmctx.calculate_char_value('ת'), 416);
    }

    #[test]
    fn test_hechrechi_final_forms() {
        let gmctx = GematriaBuilder::new()
            .with_method(GematriaMethod::MisparHechrechi)
            .init_gematria();

        let character = 'צ';
        let character_final = 'ץ';
        let value = gmctx.calculate_char_value(character);
        let value_same = gmctx.calculate_char_value(character_final);

        assert_eq!(value, value_same);
    }

    #[test]
    fn test_gadol_final_forms() {
        let gmctx = GematriaBuilder::new()
            .with_method(GematriaMethod::MisparGadol)
            .init_gematria();

        let character = 'צ';
        let character_final = 'ץ';
        let value = gmctx.calculate_char_value(character);
        let value_not_same = gmctx.calculate_char_value(character_final);

        assert_ne!(value, value_not_same);
        assert_eq!(value_not_same, 900);
    }

    #[test]
    fn test_katan_final_forms() {
        let gmctx = GematriaBuilder::new()
            .with_method(GematriaMethod::MisparKatan)
            .init_gematria();

        let character = 'צ';
        let character_final = 'ץ';
        let value = gmctx.calculate_char_value(character);
        let value_same = gmctx.calculate_char_value(character_final);

        assert_eq!(value, value_same);
        assert_eq!(value_same, 9);
    }

    #[test]
    fn test_calculate_value() {
        let gmctx = GematriaBuilder::new()
            .with_method(GematriaMethod::MisparHechrechi)
            .with_cache(true)
            .init_gematria();

        let word = "שלום"; // Replace with an actual Hebrew word
        let result = gmctx.calculate_value(word);

        assert_eq!(result.value, 376);
    }

    #[test]
    fn test_vowles() {
        let gmctx = GematriaBuilder::new()
            .with_method(GematriaMethod::MisparHechrechi)
            .with_vowels(true)
            .init_gematria();

        let shalom_without_vowel = "שלום";
        let shalom_with_vowel = "שָׁלוֹם";
        let without_vowel = gmctx.calculate_value(shalom_without_vowel);
        let with_vowel = gmctx.calculate_value(shalom_with_vowel);

        assert_eq!(without_vowel.value, with_vowel.value);
        assert_eq!(shalom_with_vowel, with_vowel.word());
        assert_ne!(shalom_without_vowel, with_vowel.word());
    }

    #[test]
    fn test_phrase() {
        let gmctx = GematriaBuilder::new()
            .with_method(GematriaMethod::MisparHechrechi)
            .with_vowels(true)
            .init_gematria();

        let bh_phrase = "בעזרת השם";
        let bh_phrase_result = gmctx.calculate_value(bh_phrase);

        assert_eq!(bh_phrase_result.value(), 1024);
    }

    #[test]
    fn test_alter_method() {
        let mut gmctx = GematriaBuilder::new()
            .with_method(GematriaMethod::MisparHechrechi)
            .with_cache(true)
            .init_gematria();

        let aleph = 'א';
        let aleph_std_result = gmctx.calculate_char_value(aleph);

        assert_eq!(aleph_std_result, 1);

        gmctx.set_method(GematriaMethod::OtiyotBeMilui);
        let aleph_filled_result = gmctx.calculate_char_value(aleph);

        assert_eq!(aleph_filled_result, 111);
    }

    #[test]
    fn test_search_match() {
        let gmctx = GematriaBuilder::new()
            .with_method(GematriaMethod::MisparHechrechi)
            .with_cache(true)
            .with_vowels(true)
            .init_gematria();

        let target_word = "יין";
        let text = "נכנס יין יצא סוד";
        let matching_words = gmctx.search_matching_words(target_word, text);

        assert!(matching_words.contains(&"סוד".to_string()));
    }

    #[test]
    fn test_group_words_by_gematria() {
        let gmctx = GematriaBuilder::new()
            .with_method(GematriaMethod::MisparHechrechi)
            .with_cache(true)
            .with_vowels(true)
            .init_gematria();

        let text = "נכנס יין יצא סוד";
        let result = gmctx.group_words_by_gematria(text).unwrap();

        // Assert that each group has more than one word
        assert!(result.iter().all(|(_, v)| v.len() > 1));

        // Assert that the vector is sorted by the length of each group
        assert!(result.windows(2).all(|w| w[0].1.len() >= w[1].1.len()));
    }

    #[test]
    fn test_trait_char() {
        let method = &GematriaMethod::MisparHechrechi;
        let c = 'א';
        let val = c.gematria_val(method);

        assert_eq!(val, 1);
    }

    #[test]
    fn test_trait_string() {
        let method = &GematriaMethod::MisparHechrechi;
        let s = "סוד".to_string();
        let val = s.gematria_val(method);

        assert_eq!(val, 70);
    }

    #[test]
    fn test_trait_str() {
        let method = &GematriaMethod::MisparHechrechi;
        let s = "סוד";
        let val = s.gematria_val(method);

        assert_eq!(val, 70);
    }
}
