use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

const MAX_LENGTH: usize = 17;

fn shorter(long: &str) -> String {
    if long.chars().count() > MAX_LENGTH {
        let shorter: String = long.chars().take(MAX_LENGTH).collect();
        format!("|{}", shorter)
    } else {
        format!("|{}", long)
    }
}
/*
fn shorter(long: &str) -> String {
    if long.len() > MAX_LENGTH {
        format!("|{}", &long[..MAX_LENGTH])
    } else {
        format!("|{}", long)
    }
}
*/

fn main() {
    // Get the current time and calculate the modulo
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get current time")
        .as_secs()
        % 60;
    let seconds_modulo = current_time % 2;

    let output = if seconds_modulo == 0 {
        // Execute the command to get the song title
        Command::new("/usr/bin/playerctl")
            .args(&["--player=spotifyd", "metadata", "title"])
            .output()
            .expect("Failed to execute playerctl command")
    } else {
        // Execute the command to get the artist name
        Command::new("/usr/bin/playerctl")
            .args(&["--player=spotifyd", "metadata", "artist"])
            .output()
            .expect("Failed to execute playerctl command")
    };

    // Convert the command output to a UTF-8 string
    let output_string = String::from_utf8_lossy(&output.stdout);

    // Remove trailing newline character
    let trimmed_output = output_string.trim();

    // Print the shortened result
    let result = shorter(trimmed_output);
    println!("{}", result);
}
