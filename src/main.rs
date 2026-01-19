use crate::wordlist::N;
use crate::wordlist::N_TEST;
use crate::wordlist::N_SOL;
use crate::wordlist::WL_TEST;
use crate::pattern::PatternTable;


pub mod wordlist;
pub mod pattern;
pub mod colors;

#[inline]
fn entropy(n_bucket: u16, n_sol: usize) -> f64 {
    let p = (n_bucket as f64) / (n_sol as f64);
    -p * p.log2()
}

fn entropy_solver(pats: &PatternTable, _depth: u8, n_sol: usize) -> usize {
    let mut max_s = -f64::INFINITY;
    let mut best = 0usize;
    let mut buckets = [0u16; 1 << (2 * N)];

    for i in 0..N_TEST {
        print!("\r{} / {}", i + 1, N_TEST);

        pats.fill_buckets(i, &mut buckets);

        let mut s = 0f64;
        for &n in &buckets {
            if n > 0 {
                s += entropy(n, n_sol);
            }
        }

        if s > max_s {
            best = i;
            max_s = s;
        }
    }

    best
}

fn main() {

    println!();

    let pats = PatternTable::build();

    println!("Solving via entropy_solver:");
    let best = entropy_solver(&pats, 0, N_SOL);

    println!();
    println!(
        "Best word: {}",
        &str::from_utf8(&WL_TEST[best]).expect("Blah")
    );
}
