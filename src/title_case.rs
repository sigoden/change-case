use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
    static ref RE_TOKENS: Regex = Regex::new(r"[^\s:–—-]+|.").unwrap();
    static ref RE_MANUAL_CASE: fancy_regex::Regex = fancy_regex::Regex::new(r".(?=[A-Z]|\..)").unwrap();
    static ref RE_SMALL_WORDS: fancy_regex::Regex = fancy_regex::Regex::new(r"\b(?:an?d?|a[st]|because|but|by|en|for|i[fn]|neither|nor|o[fnr]|only|over|per|so|some|tha[tn]|the|to|up|upon|vs?\.?|versus|via|when|with|without|yet)\b").unwrap();
    static ref RE_WHITESPACE: Regex = Regex::new(r"\s").unwrap();
    static ref RE_ALPHANUMERIC: Regex = Regex::new(r"[A-Za-z0-9\u00C0-\u00FF]").unwrap();
}

/// Change to title case
/// ```rust
/// use change_case::title_case;
/// assert_eq!(title_case("test"), "Test");
/// assert_eq!(title_case("two words"), "Two Words");
/// assert_eq!(title_case("we keep NASA capitalized"), "We Keep NASA Capitalized");
/// ```
pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    for ma in RE_TOKENS.find_iter(input) {
        let token = ma.as_str();
        let index = ma.start();
        let index2 = index + token.len();
        if
        // Ignore already capitalized words.
        !RE_MANUAL_CASE.is_match(token).unwrap() &&
            // Ignore small words except at beginning or end.
            (!RE_SMALL_WORDS.is_match(token).unwrap() || index == 0 || index2 == input.len()) &&
            // Ignore URLs
            (input.chars().nth(index2).map_or(true, |v| v != ':') ||
                input.chars().nth(index2 + 1).map_or(false, |v| RE_WHITESPACE.is_match(v.to_string().as_str())))
        {
            let new_token = RE_ALPHANUMERIC.replace(token, |v: &Captures| format!("{}", &v[0].to_uppercase()));
            result.push_str(new_token.as_ref())
        } else {
            result.push_str(token)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_case() {
        assert_eq!(title_case(""), "");
        assert_eq!(title_case("2019"), "2019");
        assert_eq!(title_case("test"), "Test");
        assert_eq!(title_case("two words"), "Two Words");
        assert_eq!(title_case("one. two."), "One. Two.");
        assert_eq!(title_case("a small word starts"), "A Small Word Starts");
        assert_eq!(title_case("small word ends on"), "Small Word Ends On");
        assert_eq!(title_case("we keep NASA capitalized"), "We Keep NASA Capitalized");
        assert_eq!(title_case("pass camelCase through"), "Pass camelCase Through");
        assert_eq!(title_case("follow step-by-step instructions"), "Follow Step-by-Step Instructions");
        assert_eq!(title_case("your hair[cut] looks (nice)"), "Your Hair[cut] Looks (Nice)");
        assert_eq!(title_case("leave Q&A unscathed"), "Leave Q&A Unscathed");
        assert_eq!(title_case("piña colada while you listen to ænima"), "Piña Colada While You Listen to Ænima");
        assert_eq!(title_case("start title – end title"), "Start Title – End Title");
        assert_eq!(title_case("start title–end title"), "Start Title–End Title");
        assert_eq!(title_case("start title — end title"), "Start Title — End Title");
        assert_eq!(title_case("start title—end title"), "Start Title—End Title");
        assert_eq!(title_case("start title - end title"), "Start Title - End Title");
        assert_eq!(title_case("don't break"), "Don't Break");
        assert_eq!(title_case(r#""double quotes""#), r#""Double Quotes""#);
        assert_eq!(title_case(r#"double quotes "inner" word"#), r#"Double Quotes "Inner" Word"#);
        assert_eq!(title_case("fancy double quotes “inner” word"), "Fancy Double Quotes “Inner” Word");
        assert_eq!(title_case("have you read “The Lottery”?"), "Have You Read “The Lottery”?");
        assert_eq!(title_case("one: two"), "One: Two");
        assert_eq!(title_case("one two: three four"), "One Two: Three Four");
        assert_eq!(title_case(r#"one two: "Three Four""#), r#"One Two: "Three Four""#);
        assert_eq!(title_case("email email@example.com address"), "Email email@example.com Address");
        assert_eq!(title_case("you have an https://example.com/ title"), "You Have an https://example.com/ Title");
        assert_eq!(title_case("_underscores around words_"), "_Underscores Around Words_");
        assert_eq!(title_case("*asterisks around words*"), "*Asterisks Around Words*");
        assert_eq!(title_case("this vs. that"), "This vs. That");
        assert_eq!(title_case("this vs that"), "This vs That");
        assert_eq!(title_case("this v. that"), "This v. That");
        assert_eq!(title_case("this v that"), "This v That");
        assert_eq!(title_case("Scott Moritz and TheStreet.com’s million iPhone la-la land"), "Scott Moritz and TheStreet.com’s Million iPhone La-La Land");
        assert_eq!(title_case("Notes and observations regarding Apple’s announcements from ‘The Beat Goes On’ special event"), "Notes and Observations Regarding Apple’s Announcements From ‘The Beat Goes On’ Special Event");
        assert_eq!(title_case("the quick brown fox jumps over the lazy dog"), "The Quick Brown Fox Jumps over the Lazy Dog");
        assert_eq!(title_case("newcastle upon tyne"), "Newcastle upon Tyne");
        assert_eq!(title_case("newcastle *upon* tyne"), "Newcastle *upon* Tyne");
    }
}
