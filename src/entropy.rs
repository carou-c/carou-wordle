use crate::wordlist::N;
use crate::wordlist::N_TEST;
use crate::wordlist::N_SOL;
use crate::pattern::PatternTable;

#[inline]
fn entropy(n_bucket: u16, n_sol: usize) -> f64 {
    let p = (n_bucket as f64) / (n_sol as f64);
    -p * p.log2()
}

pub fn entropy_solver(pats: &PatternTable, _depth: u8, n_sol: usize) -> usize {
    let mut max_s = -f64::INFINITY;
    let mut best = 0usize;
    let mut buckets = [0u16; 1 << (2 * N)];
    let mut state = [0usize; N_SOL];

    for (i, j) in state.iter_mut().enumerate() {
        *j = i;
    }

    eprintln!();

    for i in 0..N_TEST {
        eprint!("\r{} / {}", i + 1, N_TEST);

        pats.fill_buckets(i, &mut buckets, &state);

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
