use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    // Associated function to create a Flag
    pub fn opt_flag(name: &str, desc: &str) -> Self {
        Flag {
            short_hand: format!("-{}", &name[0..1]),  // First character of the name for shorthand
            long_hand: format!("--{}", name),         // Full name for longhand
            desc: desc.to_string(),
        }
    }
}

// Callback type, represents functions like div and rem
pub type Callback = fn(&str, &str) -> Result<String, String>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    // Add a flag and its callback function to the map
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert((flag.short_hand.clone(), flag.long_hand.clone()), func);
    }

    // Executes the associated function for a flag
    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        // Look for the input flag in the map
        for ((short, long), func) in &self.flags {
            if *short == input || *long == input {
                return func(argv[0], argv[1]);
            }
        }
        Err("Flag not found".to_string())
    }
}

// Function to divide two strings, converting them to f64s and returning the result or error
pub fn div(a: &str, b: &str) -> Result<String, String> {
    let a: f64 = a.parse().map_err(|_| "Invalid float literal".to_string())?;
    let b: f64 = b.parse().map_err(|_| "Invalid float literal".to_string())?;
    if b == 0.0 {
        return Err("Division by zero".to_string());
    }
    Ok((a / b).to_string())
}

// Function to find the remainder of two strings, converting them to f64s
pub fn rem(a: &str, b: &str) -> Result<String, String> {
    let a: f64 = a.parse().map_err(|_| "Invalid float literal".to_string())?;
    let b: f64 = b.parse().map_err(|_| "Invalid float literal".to_string())?;
    if b == 0.0 {
        return Err("Division by zero".to_string());
    }
    Ok((a % b).to_string())
}
