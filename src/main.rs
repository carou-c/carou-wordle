use crate::wordlist::N_SOL;
use crate::wordlist::WL_TEST;
use crate::pattern::PatternTable;
use crate::entropy::entropy_solver;

pub mod wordlist;
pub mod pattern;
pub mod colors;
pub mod entropy;

fn main() {

    println!();

    let pats = PatternTable::load_or_build("pattern_table.wdpt");

    println!("Solving via entropy_solver:");
    let best = entropy_solver(&pats, 0, N_SOL);

    println!();
    println!(
        "Best word: {}",
        &str::from_utf8(&WL_TEST[best]).expect("Blah")
    );
}
