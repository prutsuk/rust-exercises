mod config;
mod errors;
mod numbers;
mod users;

use std::path::Path;
use std::process;

use errors::AppError;

/// Run all demo exercises, propagating every error to the caller.
fn run() -> Result<(), AppError> {
    // --- Config loading ---
    let cfg_path = Path::new("example.cfg");
    match config::load_config(cfg_path) {
        Ok(cfg) => println!("config: {cfg:?}"),
        Err(AppError::Io(_)) => {
            eprintln!("note: {cfg_path:?} not found, skipping config demo");
        }
        Err(e) => return Err(e),
    }

    // --- Number parsing & validation ---
    let nums = numbers::parse_number_list("10 20 30")?;
    let avg = numbers::safe_average(&nums)?;
    println!("average of {nums:?} = {avg}");

    let val = numbers::checked_index(&nums, 1)?;
    println!("nums[1] = {val}");

    // --- User validation & lookup ---
    let alice = users::create_user("Alice", 30)?;
    let bob = users::create_user("Bob", 25)?;
    let roster = vec![alice, bob];

    let found = users::find_user(&roster, "alice")?;
    println!("found user: {found:?}");

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
