use regex::Regex;

use poem_openapi::Object;

/// takes async_nats::Message and parses it into Message struct
type NatsMessage = async_nats::Message;

#[derive(Debug, Clone, Object)]
pub struct Message {
    pub date: String,
    pub host: String,
    pub program: String,
    pub message: String,
}

impl Message {
    /// Creates new Message instance
    pub fn new(date: String, host: String, program: String, message: String) -> Self {
        Message {
            date,
            host,
            program,
            message,
        }
    }

    pub fn empty() -> Self {
        let date = String::new();
        let host = String::new();
        let program = String::new();
        let message = String::new();
        Message::new(date, host, program, message)
    }

    /// parses text data int Message struct using regex
    fn from_regex(text: String) -> Option<Self> {
        let pattern = match Regex::new(r"^<\d+>(\w+ \d+ \d+:\d+:\d+)\s+(\S+)\s+(\S+)\s+(.*)") {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{e}");
                return None;
            }
        };

        let (date, host, program, message) = match pattern.captures(&text) {
            Some(captures) => (
                captures[1].to_string(),
                captures[2].to_string(),
                captures[3].to_string(),
                captures[4].to_string(),
            ),
            None => {
                eprintln!("Faile to capture groups");
                return None;
            }
        };

        Some(Message::new(date, host, program, message))
    }

    pub fn from_nats(message: NatsMessage) -> Option<Self> {
        let text = String::from_utf8(message.payload.to_vec()).expect("Invalid UTF8");
        Message::from_regex(text)
    }
}

// From trait requires to return instance of <T>, I need to return Option<T>
// impl From<NatsMessage> for Message {
//     fn from(message: NatsMessage) -> Self {
//         let text = String::from_utf8(message.payload.to_vec()).expect("Invalid UTF8");
//         match Message::from_regex(text) {
//             Some(v) => v,
//             None => {
//                 return Message::empty();
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_constructor() {
        let date = String::from("Test 1");
        let host = String::from("Test 2");
        let program = String::from("Test 3");
        let message = String::from("Test 4");

        let message = Message::new(date, host, program, message);

        assert_eq!(message.date, String::from("Test 1"));
        assert_ne!(message.host, String::from("Test 1"));
        assert_eq!(message.host, String::from("Test 2"));
        assert_eq!(message.program, String::from("Test 3"));
        assert_eq!(message.message, String::from("Test 4"));
    }

    #[test]
    fn test_message_constructor_example() {
        let date = String::from("Jul 16 19:11:07");
        let host = String::from("host.local");
        let program = String::from("example_package.desktop[7101]");
        let message = String::from("Example log message: OK");

        let message = Message::new(date, host, program, message);

        assert_eq!(message.date, String::from("Jul 16 19:11:07"));
        assert_eq!(message.host, String::from("host.local"));
        assert_eq!(
            message.program,
            String::from("example_package.desktop[7101]")
        );
        assert_eq!(message.message, String::from("Example log message: OK"));
    }

    #[test]
    fn test_message_from_regex_example() {
        let text = String::from(
            "<1>Jul 16 19:11:07 host.local example_package.desktop[7101]: Example log message: OK",
        );

        let message = Message::from_regex(text).unwrap();

        assert_eq!(message.date, String::from("Jul 16 19:11:07"));
        assert_eq!(message.host, String::from("host.local"));
        assert_eq!(
            message.program,
            String::from("example_package.desktop[7101]:")
        );
        assert_eq!(message.message, String::from("Example log message: OK"));
    }

    #[test]
    fn test_message_from_regex_from_panic() {
        let text = String::from(
            "<14>Jul 17 20:36:17 fedora com.discordapp.Discord.desktop[1 20:36:17.591 › The resource https://discordapp.com/ass.woff2 was preloaded using link preload but not used within a few seconds from the window's load event. Please make sure it has an appropriate `as` value and it is preloaded intentionally.",
        );

        let message = Message::from_regex(text).unwrap();
        println!("{message:#?}");

        assert_eq!(message.date, String::from("Jul 17 20:36:17"));
        assert_eq!(message.host, String::from("fedora"));
        assert_eq!(
            message.program,
            String::from("com.discordapp.Discord.desktop[1")
        );
        assert_eq!(
            message.message,
            String::from(
                "20:36:17.591 › The resource https://discordapp.com/ass.woff2 was preloaded using link preload but not used within a few seconds from the window's load event. Please make sure it has an appropriate `as` value and it is preloaded intentionally."
            )
        );
    }
}
