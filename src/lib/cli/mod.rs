mod cli;
pub use cli::*;

// Use cli::* to make sure imports not look like this:
// lib::cli::cli::Cli;
// Now it's more like this:
// lib::cli::Cli