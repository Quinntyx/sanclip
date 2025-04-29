use clipboard_master::{ClipboardHandler, CallbackResult};
use copypasta::{ClipboardContext, ClipboardProvider};
use url::Url;

use crate::model::consts::TRACKING_PARAMETERS;

use std::io;

pub struct Sanitizer {
    pub clip: ClipboardContext
}

impl ClipboardHandler for Sanitizer {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let contents = match self.clip.get_contents() {
            Ok(c) => c,
            Err(_) => return CallbackResult::Stop
        };


        let san = match sanitize_url_string(&contents) {
            Ok(s) => s,
            Err(_) => {
                println!("Can't sanitize {}", &contents);
                return CallbackResult::Next
            }
        };

        if san != contents {
            println!("Sanitizing {}", contents);
            println!("    Result: {}", san.clone());
            match self.clip.set_contents(san) {
                Ok(_) => (),
                Err(_) => return CallbackResult::Stop
            }
        }

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

/// Attempts to sanitize a URL string by removing known tracking parameters.
///
/// # Arguments
/// * `url_str` - The URL string to sanitize.
///
/// # Returns
/// * `Ok(String)` containing the potentially sanitized URL. If no tracking parameters
///   were found or the input was not a valid URL with a query string, the original
///   (or equivalent) URL string is returned.
/// * `Err(url::ParseError)` if the initial input string cannot be parsed as a URL.
fn sanitize_url_string(url_str: &str) -> Result<String, url::ParseError> {
    // 1. Parse the URL. If invalid, return the error.
    let mut url = Url::parse(url_str)?;

    // 2. Check if there's actually a query string to sanitize.
    if url.query().is_none() {
        return Ok(url_str.to_string()); // Nothing to do, return original string equivalent.
    }

    let mut needs_sanitizing = false;

    // 3. Filter the query parameters.
    let filtered_pairs: Vec<(String, String)> = url
        .query_pairs()
        // Keep only pairs where the key is NOT in our tracking set.
        .filter_map(|(key, value)| {
            if TRACKING_PARAMETERS.contains(key.as_ref()) {
                needs_sanitizing = true; // Mark that we found something to remove
                None // Filter this pair out
            } else {
                Some((key.into_owned(), value.into_owned())) // Keep this pair
            }
        })
        .collect();

    // 4. If no tracking parameters were found, return the original URL string.
    if !needs_sanitizing {
        return Ok(url_str.to_string());
    }

    // 5. Rebuild the URL with the filtered parameters.
    if filtered_pairs.is_empty() {
        // If filtering removed all parameters, clear the query string entirely.
        url.set_query(None);
    } else {
        // Otherwise, serialize the remaining pairs back into a query string.
        // This handles URL encoding correctly.
        match serde_urlencoded::to_string(&filtered_pairs) {
            Ok(new_query) => url.set_query(Some(&new_query)),
            Err(_) => {
                // Should ideally not happen with valid String pairs, but handle defensively.
                // Return the original string in case of unexpected serialization error.
                eprintln!("Warning: Failed to serialize filtered query parameters for URL: {}", url_str);
                return Ok(url_str.to_string());
            }
        }
    }

    // 6. Return the modified URL as a string.
    Ok(url.to_string())
}
