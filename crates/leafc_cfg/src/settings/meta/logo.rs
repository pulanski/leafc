#![allow(non_snake_case)]

use owo_colors::colors::*;
use owo_colors::OwoColorize;

//                _
//    _       _ _(_)_     |  Documentation: https://docs.julialang.org
//   (_)     | (_) (_)    |
//    _ _   _| |_  __ _   |  Type "?" for help, "]?" for Pkg help.
//   | | | | | | |/ _` |  |
//   | | |_| | | | (_| |  |  Version 1.8.3 (2022-11-14)
//  _/ |\__'_|_|_|\__'_|  |  Built by Homebrew (v1.8.3_1)
// |__/                   |

use smartstring::alias::String;

use crate::defs::LEAFC_DOCS_URL;

use super::version::LEAFC_TARGET;
use super::version::LEAFC_VERSION;

/// The **logo** of the **leafc** compiler.
/// This is the **logo** that is **displayed** when the **compiler** is **run** in **REPL** mode
/// or when the **compiler** is **run** with the `--interactive` flag during **batch** mode.
pub fn LOGO() -> String {
    String::from(
        r#"
    _       _ _(_)_     |  Documentation: "#,
    ) + LEAFC_DOCS_URL
        + r#"
   (_)     | (_) (_)    |
    _ _   _| |_  __ _   |  Type "?" for help, "]?" for Pkg help.
   | | | | | | |/ _` |  |
   | | |_| | | | (_| |  |  Version "#
        + LEAFC_VERSION
        + r#"
    _/ |\__'_|_|_|\__'_|  |  Built by Homebrew (v1.8.3_1)
    |__/                   |

Version "#
        + LEAFC_VERSION
        + "\n\n"
        + tree_logo()
        + "\n\n"
}

pub fn tree_logo() -> String {
    String::from("              &&\n".green().to_string())
        + "            &&&&&\n".green().to_string()
        + "          &&&".green().to_string()
        + "\\/".bright_red().to_string()
        + "& &&&".green().to_string()
        + "                           888                    .d888\n".green().to_string()
        + "         &&".green().to_string()
        + "|,/  |/".bright_red().to_string()
        + "& &&".green().to_string()
        + "                         888                   d88P\"\n".green().to_string()
        + "          &&".green().to_string()
        + "/   /  /_".bright_red().to_string()
        + "&  &&".green().to_string()
        + "                     888                   888\n".green().to_string()
        + "            \\  {  |_____/_".bright_red().to_string()
        + "&".green().to_string()
        + "                    888  .d88b.   8888b.  888888 .d8888b\n".green().to_string()
        + "            {  / /          ".bright_red().to_string()
        + "&&&".green().to_string()
        + "                888 d8P  Y8b     \"88b 888   d88P\"\n".green().to_string()
        + "            `, \\{___________/_".bright_red().to_string()
        + "&&".green().to_string()
        + "               888 88888888 .d888888 888   888\n".green().to_string()
        + "             } }{       \\".bright_red().to_string()
        + "                      888 Y8b.     888  888 888   Y88b.\n".green().to_string()
        + "             }{{         \\____".bright_red().to_string()
        + "&&".green().to_string()
        + "               888  \"Y8888  \"Y888888 888    \"Y8888P\n".green().to_string()
        + "            {}{           ".bright_red().to_string()
        + "`&".green().to_string()
        + "\\".bright_red().to_string()
        + "&&\n".green().to_string()
        + "            {{}".bright_red().to_string()
        + "             &&\n".green().to_string()
        + "      , -=-~{ .-^- _".bright_red().to_string()
        + "                    Documentation".bright_blue().to_string()
        + ": ".black().to_string()
        + "https://docs.leaf-lang.org/book/\n".cyan().to_string()
        + "            `}\n".bright_red().to_string()
        + "             {".bright_red().to_string()
        + "                                Type".bright_yellow().italic().to_string()
        + " \"".bright_red().italic().to_string()
        + "?".cyan().italic().to_string()
        + "\"".bright_red().italic().to_string()
        + " for help".bright_yellow().italic().to_string()
        + ",".black().italic().to_string()
        + "\"".bright_red().italic().to_string()
        + "]?".cyan().italic().to_string()
        + "\"".bright_red().italic().to_string()
        + " for Pkg help".bright_yellow().italic().to_string()
        + ".\n".black().italic().to_string()
        + "\n\n                      Version".bright_blue().to_string()
        + ": ".black().to_string()
        + LEAFC_VERSION.italic().to_string()
        + "\n                             Target".bright_blue().to_string()
        + ": ".black().to_string()
        + LEAFC_TARGET.italic().to_string()
}

//              &&
//            &&&&&
//          &&&\/& &&&                           888                    .d888
//         &&|,/  |/& &&                         888                   d88P"
//          &&/   /  /_&  &&                     888                   888
//            \  {  |_____/_&                    888  .d88b.   8888b.  888888 .d8888b
//            {  / /          &&&                888 d8P  Y8b     "88b 888   d88P"
//            `, \{___________/_&&               888 88888888 .d888888 888   888
//             } }{       \                      888 Y8b.     888  888 888   Y88b.
//             }{{         \____&                888  "Y8888  "Y888888 888    "Y8888P
//            {}{           `&\&&
//            {{}             &&          Documentation: https://docs.leaf-lang.org/book/
//      , -=-~{ .-^- _                    Documentation: https://docs.leaf-lang.org/book/
// ejm        `}                                Type "?" for help, "]?" for Pkg help.
//             {
//
//                      Version leafc v0.1.0 (9ab85c8 2023-02-04)
//                             target: macos-unix-aarch64
//
// leaf [1]>

// 888                    .d888
// 888                   d88P"
// 888                   888
// 888  .d88b.   8888b.  888888 .d8888b
// 888 d8P  Y8b     "88b 888   d88P"
// 888 88888888 .d888888 888   888
// 888 Y8b.     888  888 888   Y88b.
// 888  "Y8888  "Y888888 888    "Y8888P
