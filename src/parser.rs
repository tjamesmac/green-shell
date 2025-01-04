use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, space1},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Greeting {
    pub greeting: String,
    pub name: String,
}

pub fn parse_greeting(input: &str) -> IResult<&str, Greeting> {
    let (input, greeting) = tag("hello")(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = alpha1(input)?;

    Ok((
        input,
        Greeting {
            greeting: greeting.to_string(),
            name: name.to_string(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use super::*;

    #[test]
    fn test_parse_greeting() {
        let input = "hello world";
        let (remaining, greeting) = parse_greeting(input).unwrap();

        assert_eq!(
            greeting,
            Greeting {
                name: "world".to_string(),
                greeting: "hello".to_string()
            }
        );
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_greeting_failure() {
        let input = "hi world";
        let result = parse_greeting(input);

        assert!(result.is_err());

        if let Err(nom::Err::Error(e)) = result {
            assert_eq!(e.code, ErrorKind::Tag); // Access the error code
        }
    }
}
