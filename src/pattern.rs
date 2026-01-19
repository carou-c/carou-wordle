use crate::colors::Color;
use crate::wordlist::N;
use crate::wordlist::N_SOL;
use crate::wordlist::N_TEST;
use crate::wordlist::WL_SOL;
use crate::wordlist::WL_TEST;

fn test_against(t: [u8; N], s: [u8; N]) -> u32 {
    let mut skip = [false; N];
    let mut pat = 0u32;

    for (i, &tl) in t.iter().enumerate() {
        if s[i] == tl {
            skip[i] = true;
            pat += Color::Green.encode() << (2 * i);
        };
    }

    let mut skip_s = skip;

    for (i, &tl) in t.iter().enumerate() {
        if skip[i] {
            continue;
        };

        if let Some((j, _)) = s
            .iter()
            .enumerate()
            .filter(|&(j, _)| !skip_s[j])
            .find(|&(_, &sl)| sl == tl)
        {
            pat += Color::Yellow.encode() << (2 * i);
            skip_s[j] = true;
        } else {
            pat += Color::Gray.encode() << (2 * i);
        }
    }

    pat
}

pub struct PatternTable {
    n_test: usize,
    n_sol: usize,
    data: Vec<u32>,
}

impl PatternTable {
    fn new(n_test: usize, n_sol: usize) -> Self {
        PatternTable {
            n_test,
            n_sol,
            data: vec![0u32; n_sol * n_test],
        }
    }

    pub fn build() -> Self {
        let mut pats = PatternTable::new(N_TEST, N_SOL);

        eprintln!("Computing PatternTable:");

        for (i, &t) in WL_TEST.iter().enumerate() {
            eprint!("\r{} / {}", i + 1, N_TEST);
            for (j, &s) in WL_SOL.iter().enumerate() {
                pats.set(i, j, test_against(t, s));
            }
        }

        eprintln!();

        pats
    }

    #[inline]
    pub fn get(&self, i: usize, j: usize) -> u32 {
        self.data[i * self.n_sol + j]
    }

    #[inline]
    pub fn set(&mut self, i: usize, j: usize, val: u32) {
        self.data[i * self.n_sol + j] = val;
    }

    #[inline]
    fn get_tslice(&self, i: usize) -> &[u32] {
        &self.data[(i * self.n_sol)..((i + 1) * self.n_sol)]
    }

    pub fn fill_buckets(&self, i: usize, buckets: &mut [u16]) {
        buckets.fill(0);

        for &pat in self.get_tslice(i) {
            buckets[pat as usize] += 1;
        }
    }
}

