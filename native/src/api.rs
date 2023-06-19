// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

// A plain enum without any fields. This is similar to Dart- or C-style enums.
// flutter_rust_bridge is capable of generating code for enums with fields
// (@freezed classes in Dart and tagged unions in C).
pub enum Platform {
    Unknown,
    Android,
    Ios,
    Windows,
    Unix,
    MacIntel,
    MacApple,
    Wasm,
}

// A function definition in Rust. Similar to Dart, the return type must always be named
// and is never inferred.
pub fn platform() -> Platform {
    // This is a macro, a special expression that expands into code. In Rust, all macros
    // end with an exclamation mark and can be invoked with all kinds of brackets (parentheses,
    // brackets and curly braces). However, certain conventions exist, for example the
    // vector macro is almost always invoked as vec![..].
    //
    // The cfg!() macro returns a boolean value based on the current compiler configuration.
    // When attached to expressions (#[cfg(..)] form), they show or hide the expression at compile time.
    // Here, however, they evaluate to runtime values, which may or may not be optimized out
    // by the compiler. A variety of configurations are demonstrated here which cover most of
    // the modern oeprating systems. Try running the Flutter application on different machines
    // and see if it matches your expected OS.
    //
    // Furthermore, in Rust, the last expression in a function is the return value and does
    // not have the trailing semicolon. This entire if-else chain forms a single expression.
    if cfg!(windows) {
        Platform::Windows
    } else if cfg!(target_os = "android") {
        Platform::Android
    } else if cfg!(target_os = "ios") {
        Platform::Ios
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        Platform::MacApple
    } else if cfg!(target_os = "macos") {
        Platform::MacIntel
    } else if cfg!(target_family = "wasm") {
        Platform::Wasm
    } else if cfg!(unix) {
        Platform::Unix
    } else {
        Platform::Unknown
    }
}

use std::process::Command;

pub fn return_ls_output() -> Vec<String>{

    // First, we will try for ls -la /root/ without any extra permission, for in case,
    // the app is already running with root user privileges

    let output =
        Command::new("ls")
            .arg("-la")
            .arg("/root/")
            .output()
            .unwrap();
    if output.status.success() {
        let mut result = Vec::new();
        let mut temporary_string = String::new();

        for i in output.stdout {
            // If i is newline character
            // We simply push current string to result
            if i == 10 {
                result.push(temporary_string.clone());
                temporary_string = String::new();
            }else {
                temporary_string.push(i as char);
            }
        }
        return result;
    }


    // Now, let us try with polkit or pkexec

    let output =
        Command::new("pkexec")
            .arg("ls")
            .arg("-la")
            .arg("/root/")
            .output()
            .unwrap();
    if output.status.success() {
        let mut result = Vec::new();
        let mut temporary_string = String::new();

        for i in output.stdout {
            // If i is newline character
            // We simply push current string to result
            if i == 10 {
                result.push(temporary_string.clone());
                temporary_string = String::new();
            }else {
                temporary_string.push(i as char);
            }
        }
        return result;
    }
    // If neither of options worked, we display message
    return vec![String::from("Sorry, you must either open this app as root or enable polkit / pkexec")];
}

// The convention for Rust identifiers is the snake_case,
// and they are automatically converted to camelCase on the Dart side.
pub fn rust_release_mode() -> bool {
    cfg!(not(debug_assertions))
}
