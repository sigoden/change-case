use lazy_static::lazy_static;
use regex::{Regex, Replacer};

mod title_case;
pub use title_case::title_case;

lazy_static! {
    static ref RE_SPLIT_1: Regex = Regex::new(r"([a-z0-9])([A-Z])").unwrap();
    static ref RE_SPLIT_2: Regex = Regex::new(r"([A-Z])([A-Z][a-z])").unwrap();
    static ref RE_STRIP: Regex = Regex::new(r"(?i)[^A-Z0-9]+").unwrap();
}

type Fransform = dyn Fn(&str, usize) -> String;

/// Control the behavier of change case
pub struct Config {
    split_regex: Vec<Regex>,
    strip_regex: Vec<Regex>,
    delimiter: String,
    transform: Box<Fransform>,
}

impl Config {
    /// Change regex used to split into word segments
    pub fn set_split_regex(mut self, value: Vec<Regex>) -> Self {
        self.split_regex = value;
        self
    }
    /// Change regex used to remove extraneous characters
    pub fn set_strip_regex(mut self, value: Vec<Regex>) -> Self {
        self.strip_regex = value;
        self
    }
    /// Change value used between words (e.g. " ")
    pub fn set_delimiter(mut self, value: &str) -> Self {
        self.delimiter = value.into();
        self
    }
    /// Change the transform function used to transform each word segment (e.g. [transform_lower_case](fn.transform_lower_case.html))
    pub fn set_transform(mut self, value: Box<Fransform>) -> Self {
        self.transform = value;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            split_regex: vec![RE_SPLIT_1.clone(), RE_SPLIT_2.clone()],
            strip_regex: vec![RE_STRIP.clone()],
            delimiter: " ".into(),
            transform: Box::new(|part: &str, _index: usize| part.to_lowercase()),
        }
    }
}

/// Core function to change case
/// ```rust
/// use regex::Regex;
/// use change_case::{change_case, Config};
/// let config = Config::default()
///     .set_split_regex(vec![Regex::new("([a-z])([A-Z0-9])").unwrap()]);
/// assert_eq!(change_case("camel2019", config), "camel 2019");
/// assert_eq!(change_case("camel2019", Config::default()), "camel2019");
/// ```

pub fn change_case(input: &str, config: Config) -> String {
    let result = replace(
        input,
        config.split_regex.iter().map(|v| (v, "$1\0$2")).collect(),
    );
    let result = replace(
        result.as_str(),
        config.strip_regex.iter().map(|v| (v, "\0")).collect(),
    );
    let result = result.trim_start_matches("\0").trim_end_matches("\0");
    let transform = config.transform;

    let parts: Vec<String> = result
        .split("\0")
        .enumerate()
        .map(|(index, part)| (transform)(part, index))
        .collect();
    parts.join(config.delimiter.as_str())
}

fn replace<R: Replacer>(input: &str, reps: Vec<(&Regex, R)>) -> String {
    reps.into_iter().fold(input.to_string(), |acc, re| {
        re.0.replace_all(acc.as_str(), re.1).to_string()
    })
}

/// Change to upper case
/// ```rust
/// use change_case::upper_case;
/// assert_eq!(upper_case(""), "");
/// assert_eq!(upper_case("test"), "TEST");
/// assert_eq!(upper_case("test string"), "TEST STRING");
/// assert_eq!(upper_case("Test String"), "TEST STRING");
/// assert_eq!(upper_case("\u{0131}"), "I");
/// ```
pub fn upper_case(input: &str) -> String {
    input.to_uppercase()
}

/// Only change the first charactor to upper case
/// ```rust
/// use change_case::upper_case_first;
/// assert_eq!(upper_case_first(""), "");
/// assert_eq!(upper_case_first("test"), "Test");
/// assert_eq!(upper_case_first("TEST"), "TEST");
/// ```
pub fn upper_case_first(input: &str) -> String {
    if input.len() == 0 {
        return String::new();
    }
    let (first, last) = input.split_at(1);
    format!("{}{}", upper_case(first), last)
}

/// Change to lower case
/// ```rust
/// use change_case::lower_case;
/// assert_eq!(lower_case(""), "");
/// assert_eq!(lower_case("test"), "test");
/// assert_eq!(lower_case("TEST"), "test");
/// assert_eq!(lower_case("test string"), "test string");
/// assert_eq!(lower_case("TEST STRING"), "test string");
/// ```
pub fn lower_case(input: &str) -> String {
    input.to_lowercase()
}

/// Only change the first charactor to lower case
/// ```rust
/// use change_case::lower_case_first;
/// assert_eq!(lower_case_first(""), "");
/// assert_eq!(lower_case_first("Test"), "test");
/// assert_eq!(lower_case_first("TEST"), "tEST");
/// ```
pub fn lower_case_first(input: &str) -> String {
    if input.len() == 0 {
        return String::new();
    }
    let (first, last) = input.split_at(1);
    format!("{}{}", lower_case(first), last)
}

fn transform_pascal_case(input: &str, index: usize) -> String {
    if input.len() == 0 {
        return String::new();
    }
    let (first, last) = input.split_at(1);
    let mut first = upper_case(first);
    if index > 0 {
        let first_char = first.chars().nth(0).unwrap();
        if first_char >= '0' && first_char <= '9' {
            first = format!("_{}", first)
        }
    }
    format!("{}{}", first, lower_case(last))
}

/// Change to pascal case
/// ```rust
/// use change_case::pascal_case;
/// assert_eq!(pascal_case(""), "");
/// assert_eq!(pascal_case("test"), "Test");
/// assert_eq!(pascal_case("test string"), "TestString");
/// assert_eq!(pascal_case("Test String"), "TestString");
/// assert_eq!(pascal_case("TestV2"), "TestV2");
/// assert_eq!(pascal_case("version 1.2.10"), "Version_1_2_10");
/// assert_eq!(pascal_case("version 1.21.0"), "Version_1_21_0");
/// ```
pub fn pascal_case(input: &str) -> String {
    let config = Config::default()
        .set_delimiter("")
        .set_transform(Box::new(transform_pascal_case));
    change_case(input, config)
}

fn transform_camel_case(input: &str, index: usize) -> String {
    if index == 0 {
        return lower_case(input);
    }
    transform_pascal_case(input, index)
}

/// Change to camel case
/// ```rust
/// use change_case::camel_case;
/// assert_eq!(camel_case(""), "");
/// assert_eq!(camel_case("test"), "test");
/// assert_eq!(camel_case("test string"), "testString");
/// assert_eq!(camel_case("Test String"), "testString");
/// assert_eq!(camel_case("TestV2"), "testV2");
/// assert_eq!(camel_case("_foo_bar_"), "fooBar");
/// assert_eq!(camel_case("version 1.2.10"), "version_1_2_10");
/// assert_eq!(camel_case("version 1.21.0"), "version_1_21_0");
/// ```
pub fn camel_case(input: &str) -> String {
    let config = Config::default()
        .set_delimiter("")
        .set_transform(Box::new(transform_camel_case));
    change_case(input, config)
}

fn transform_capital_case(input: &str, _index: usize) -> String {
    upper_case_first(lower_case(input).as_str())
}

/// Change to capital case
/// ```rust
/// use change_case::captial_case;
/// assert_eq!(captial_case(""), "");
/// assert_eq!(captial_case("test"), "Test");
/// assert_eq!(captial_case("test string"), "Test String");
/// assert_eq!(captial_case("Test String"), "Test String");
/// assert_eq!(captial_case("TestV2"), "Test V2");
/// assert_eq!(captial_case("version 1.2.10"), "Version 1 2 10");
/// assert_eq!(captial_case("version 1.21.0"), "Version 1 21 0");
/// ```
pub fn captial_case(input: &str) -> String {
    let config = Config::default()
        .set_delimiter(" ")
        .set_transform(Box::new(transform_capital_case));
    change_case(input, config)
}

fn transform_upper_case(input: &str, _index: usize) -> String {
    upper_case(input)
}

/// Change to constant case
/// ```rust
/// use change_case::constant_case;
/// assert_eq!(constant_case(""), "");
/// assert_eq!(constant_case("test"), "TEST");
/// assert_eq!(constant_case("test string"), "TEST_STRING");
/// assert_eq!(constant_case("Test String"), "TEST_STRING");
/// assert_eq!(constant_case("dot.case"), "DOT_CASE");
/// assert_eq!(constant_case("path/case"), "PATH_CASE");
/// assert_eq!(constant_case("TestV2"), "TEST_V2");
/// assert_eq!(constant_case("version 1.2.10"), "VERSION_1_2_10");
/// assert_eq!(constant_case("version 1.21.0"), "VERSION_1_21_0");
/// ```
pub fn constant_case(input: &str) -> String {
    let config = Config::default()
        .set_delimiter("_")
        .set_transform(Box::new(transform_upper_case));
    change_case(input, config)
}

fn transform_lower_case(input: &str, _index: usize) -> String {
    lower_case(input)
}

/// Change to dot case
/// ```rust
/// use change_case::dot_case;
/// assert_eq!(dot_case(""), "");
/// assert_eq!(dot_case("test"), "test");
/// assert_eq!(dot_case("test string"), "test.string");
/// assert_eq!(dot_case("Test String"), "test.string");
/// assert_eq!(dot_case("dot.case"), "dot.case");
/// assert_eq!(dot_case("path/case"), "path.case");
/// assert_eq!(dot_case("TestV2"), "test.v2");
/// assert_eq!(dot_case("version 1.2.10"), "version.1.2.10");
/// assert_eq!(dot_case("version 1.21.0"), "version.1.21.0");
/// ```
pub fn dot_case(input: &str) -> String {
    let config = Config::default()
        .set_delimiter(".")
        .set_transform(Box::new(transform_lower_case));
    change_case(input, config)
}

/// Change to header case
/// ```rust
/// use change_case::header_case;
/// assert_eq!(header_case(""), "");
/// assert_eq!(header_case("test"), "Test");
/// assert_eq!(header_case("test string"), "Test-String");
/// assert_eq!(header_case("Test String"), "Test-String");
/// assert_eq!(header_case("TestV2"), "Test-V2");
/// assert_eq!(header_case("version 1.2.10"), "Version-1-2-10");
/// assert_eq!(header_case("version 1.21.0"), "Version-1-21-0");
/// ```
pub fn header_case(input: &str) -> String {
    let config = Config::default()
        .set_delimiter("-")
        .set_transform(Box::new(transform_capital_case));
    change_case(input, config)
}

/// Change to param case
/// ```rust
/// use change_case::param_case;
/// assert_eq!(param_case(""), "");
/// assert_eq!(param_case("test"), "test");
/// assert_eq!(param_case("test string"), "test-string");
/// assert_eq!(param_case("Test String"), "test-string");
/// assert_eq!(param_case("TestV2"), "test-v2");
/// assert_eq!(param_case("version 1.2.10"), "version-1-2-10");
/// assert_eq!(param_case("version 1.21.0"), "version-1-21-0");
/// ```
pub fn param_case(input: &str) -> String {
    let config = Config::default()
        .set_delimiter("-")
        .set_transform(Box::new(transform_lower_case));
    change_case(input, config)
}

/// Change to path case
/// ```rust
/// use change_case::path_case;
/// assert_eq!(path_case(""), "");
/// assert_eq!(path_case("test"), "test");
/// assert_eq!(path_case("test string"), "test/string");
/// assert_eq!(path_case("Test String"), "test/string");
/// assert_eq!(path_case("TestV2"), "test/v2");
/// assert_eq!(path_case("version 1.2.10"), "version/1/2/10");
/// assert_eq!(path_case("version 1.21.0"), "version/1/21/0");
/// ```
pub fn path_case(input: &str) -> String {
    let config = Config::default()
        .set_delimiter("/")
        .set_transform(Box::new(transform_lower_case));
    change_case(input, config)
}

fn transform_sentence_case(input: &str, index: usize) -> String {
    let input = lower_case(input);
    if index == 0 {
        upper_case_first(input.as_str())
    } else {
        input
    }
}

/// Change to sentence case
/// ```rust
/// use change_case::sentence_case;
/// assert_eq!(sentence_case(""), "");
/// assert_eq!(sentence_case("test"), "Test");
/// assert_eq!(sentence_case("test string"), "Test string");
/// assert_eq!(sentence_case("Test String"), "Test string");
/// assert_eq!(sentence_case("TestV2"), "Test v2");
/// assert_eq!(sentence_case("version 1.2.10"), "Version 1 2 10");
/// assert_eq!(sentence_case("version 1.21.0"), "Version 1 21 0");
/// ```
pub fn sentence_case(input: &str) -> String {
    let config = Config::default()
        .set_delimiter(" ")
        .set_transform(Box::new(transform_sentence_case));
    change_case(input, config)
}

/// Change to snake case
/// ```rust
/// use change_case::snake_case;
/// assert_eq!(snake_case(""), "");
/// assert_eq!(snake_case("_id"), "id");
/// assert_eq!(snake_case("test"), "test");
/// assert_eq!(snake_case("test string"), "test_string");
/// assert_eq!(snake_case("Test String"), "test_string");
/// assert_eq!(snake_case("TestV2"), "test_v2");
/// assert_eq!(snake_case("version 1.2.10"), "version_1_2_10");
/// assert_eq!(snake_case("version 1.21.0"), "version_1_21_0");
/// ```
pub fn snake_case(input: &str) -> String {
    let config = Config::default()
        .set_delimiter("_")
        .set_transform(Box::new(transform_lower_case));
    change_case(input, config)
}

/// Change to swap case
/// ```rust
/// use change_case::swap_case;
/// assert_eq!(swap_case(""), "");
/// assert_eq!(swap_case("test"), "TEST");
/// assert_eq!(swap_case("test string"), "TEST STRING");
/// assert_eq!(swap_case("Test String"), "tEST sTRING");
/// assert_eq!(swap_case("TestV2"), "tESTv2");
/// assert_eq!(swap_case("sWaP cAsE"), "SwAp CaSe");
/// ```
pub fn swap_case(input: &str) -> String {
    input
        .chars()
        .into_iter()
        .map(|v| {
            if v.is_lowercase() {
                v.to_uppercase().to_string()
            } else {
                v.to_lowercase().to_string()
            }
        })
        .collect()
}
