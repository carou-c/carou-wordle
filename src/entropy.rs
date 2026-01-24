use crate::wordlist::N;
use crate::wordlist::N_TEST;
// use crate::wordlist::N_SOL;
use crate::pattern::PatternTable;

#[inline]
fn entropy(n_bucket: u16, n_sol: usize) -> f64 {
    let p = (n_bucket as f64) / (n_sol as f64);
    -p * p.log2()
}

fn entropy_test(buckets: &[u16], state: &[usize]) -> f64 {
    let mut h = 0f64;
    for &n in buckets {
        if n > 0 {
            h += entropy(n, state.len());
        }
    }

    h
}

pub fn best_entropy(pats: &PatternTable, state: &[usize]) -> (usize, f64) {
    let mut max_h = -f64::INFINITY;
    let mut best = 0usize;
    let mut buckets = [0u16; 1 << (2 * N)];

    // eprintln!();

    for i in 0..N_TEST {
        // eprint!("\r{} / {}", i + 1, N_TEST);

        pats.fill_buckets(i, &mut buckets, state);
        let h = entropy_test(&buckets, state);

        if h > max_h {
            best = i;
            max_h = h;
        }
    }

    (best, max_h)
}

pub fn entropy_tests(pats: &PatternTable, state: &[usize]) -> [(usize, f64); N_TEST] {
    let mut hs = [(0usize, 0f64); N_TEST];
    let mut buckets = [0u16; 1 << (2 * N)];

    // eprintln!();

    for (i, h) in hs.iter_mut().enumerate() {
        // eprint!("\r{} / {}", i + 1, N_TEST);

        pats.fill_buckets(i, &mut buckets, state);
        *h = (i, entropy_test(&buckets, state));
    }

    hs
}
