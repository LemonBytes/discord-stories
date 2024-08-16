use crate::write_duration::Story;
use serde_json::{json, Value};
use std::{error::Error, fs, io::Write};

pub fn format_json(input: &str) -> Result<(), Box<dyn Error>> {
    // Format the JSON
    match generate_json(input) {
        Ok(formatted) => {
            let mut file_write = fs::File::options()
                .write(true)
                .truncate(true)
                .create(true)
                .open(r"C:\Users\miche\Desktop\Projekte\discord-stories\apps\generateVideo\public\temp_assets\story_fragments.json")
                .expect("failed at writing process");

            let json_str = remove_escaped_characters(formatted.as_str());
            let transformed = clean_json_string(&json_str);

            let json: Value = serde_json::from_str(&transformed).expect("Serialization failed");
            serde_json::to_writer(&mut file_write, &json).unwrap();

            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn generate_json(input: &str) -> Result<String, Box<dyn Error>> {
    // Parse the input string into a serde_json::Value
    let value: Value = serde_json::from_str(input)?;

    // Convert the Value back into a pretty-printed JSON string
    let formatted_json = serde_json::to_string_pretty(&value)?;

    Ok(formatted_json)
}

pub fn remove_escaped_characters(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            // Look at the next character to determine what to do
            if let Some(next_char) = chars.peek() {
                match next_char {
                    'n' | 't' | 'r' | '\\' => {
                        // Skip this backslash and the next character (e.g., \n, \\, etc.)
                        chars.next();
                    }
                    '\"' => {
                        // Replace \" with " by skipping the backslash and keeping the quote
                        chars.next();
                        output.push('"');
                    }
                    _ => {}
                }
            } else {
                // If the last character is a backslash, just keep it
                output.push(c);
            }
        } else {
            // Regular character, just add it to the output
            output.push(c);
        }
    }

    output
}

fn clean_json_string(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    // Skip the leading quotation mark
    if chars.next() == Some('\"') {
        while let Some(c) = chars.next() {
            // Skip the trailing quotation mark
            if c == '\"' && chars.peek().is_none() {
                break;
            }

            if c == '\\' {
                // Skip the backslash and the next character
                chars.next(); // Skip the character after the backslash
                continue;
            }

            result.push(c);
        }
    }

    result
}
