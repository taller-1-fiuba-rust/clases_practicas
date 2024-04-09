use regex::regex::Regex;

fn main() {
    let regex = Regex::new("ab.*c.*f");

    println!("Hello!");
    println!("Your regex is {:?}", regex);

    let value = "abacd€f";
    println!("Your value is {:?}", value);

    match regex.unwrap().test(value) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use regex::regex::Regex;

    #[test]
    fn test_no_ascii() {
        let value = "abcd€f";

        let regex = Regex::new("ab.*c").unwrap();

        let matches = regex.test(value);
        // assert!(matches.is_err());
        // assert_eq!(matches.unwrap_err(), "el input no es ascii");

        assert!(matches.is_ok());
        assert_eq!(matches.unwrap(), true);
    }

    #[test]
    fn test_match() -> Result<(), &'static str> {
        let value = "abcdef";

        let regex = Regex::new("ab.*e")?;

        let matches = regex.test(value)?;

        assert_eq!(matches, true);

        Ok(())
    }

    #[test]
    fn test_no_match() -> Result<(), &'static str> {
        let value = "abcdef";

        let regex = Regex::new("ab.*h")?;

        let matches = regex.test(value)?;
        assert_eq!(matches, false);

        Ok(())
    }

    #[test]
    fn test_match_2() -> Result<(), &'static str> {
        let value = "ab1234cdefg";

        let regex = Regex::new("ab.*c.*f")?;

        let matches = regex.test(value)?;
        assert_eq!(matches, true);

        Ok(())
    }

    #[test]
    fn test_no_match_2() -> Result<(), &'static str> {
        let value = "ab1234cdegh";

        let regex = Regex::new("ab.*c.*f")?;

        let matches = regex.test(value)?;
        assert_eq!(matches, false);

        Ok(())
    }

    #[test]
    fn test_match_3() -> Result<(), &'static str> {
        let value = "ab1234ccefg";

        let regex = Regex::new("ab.*c{2}")?;

        let matches = regex.test(value)?;
        assert_eq!(matches, true);

        Ok(())
    }
}
