// Display Section
use std::fmt;

// Define a structure which `fmt::Display` will be implemented for. This is simply
// a tuple struct containing an `i32` bound to the name `Structure`.
pub struct DisplayStruct(i32);

impl DisplayStruct {
    // Constructor
    pub fn new() -> DisplayStruct {
        DisplayStruct(4)
    }
    pub fn new_with_value(value: i32) -> DisplayStruct {
        DisplayStruct(value)
    }
}

// Extend the Structure with the Display option
impl fmt::Display for DisplayStruct {
    // In order to use the `{}` marker, the trait `fmt::Display` must be implemented
    // manually for the type.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element of the DisplayStructure tple into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed.
        write!(f, "{}", self.0)
    }
}
