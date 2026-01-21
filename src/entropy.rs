use crate::wordlist::N;
use crate::wordlist::N_TEST;
// use crate::wordlist::N_SOL;
use crate::pattern::PatternTable;

#[inline]
fn entropy(n_bucket: u16, n_sol: usize) -> f64 {
    let p = (n_bucket as f64) / (n_sol as f64);
    -p * p.log2()
}

fn entropy_test(pats: &PatternTable, i: usize, buckets: &mut [u16], state: &[usize]) -> f64 {
    pats.fill_buckets(i, buckets, state);

    let mut s = 0f64;
    for &mut n in buckets {
        if n > 0 {
            s += entropy(n, state.len());
        }
    }

    s
}

pub fn best_entropy(pats: &PatternTable, state: &[usize]) -> (usize, f64) {
    let mut max_s = -f64::INFINITY;
    let mut best = 0usize;
    let mut buckets = [0u16; 1 << (2 * N)];

    // eprintln!();

    for i in 0..N_TEST {
        // eprint!("\r{} / {}", i + 1, N_TEST);

        let s = entropy_test(pats, i, &mut buckets, state);

        if s > max_s {
            best = i;
            max_s = s;
        }
    }

    (best, max_s)
}

// pub fn entropy_solver(pats: &PatternTable, depth: u8, state: &[usize]) -> (usize, f64) {
//     if depth == 1 {
//         return best_entropy(pats, state);
//     };
//
//     let mut max_s = -f64::INFINITY;
//     let mut best = 0usize;
//     let mut buckets = [0u16; 1 << (2 * N)];
//     let mut bucket_states = [const { Vec::new() }; 1 << (2 * N)];
//     let mut new_entropies = [-f64::INFINITY; 1 << (2 * N)];
//
//     for new_entropy in &
//
//     eprintln!();
//
//     for i in 0..N_TEST {
//         eprint!("\r{} / {}", i + 1, N_TEST);
//
//         let si = entropy_test(pats, i, &mut buckets, state);
//
//         pats.fill_bucket_states(i, &mut bucket_states, state);
//
//         for new_state in &bucket_states {
//             if new_state.is_empty() {
//                 continue;
//             }
//
//             let (_, sd) = entropy_solver(pats, depth - 1, new_state);
//
//             if (si + sd) > max_s {
//                 best = i;
//                 max_s = si + sd;
//             }
//         }
//
//     }
//
//     (best, max_s)
// }
