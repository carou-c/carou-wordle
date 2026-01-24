use crate::colors::Color;
use crate::wordlist::N;
use crate::wordlist::N_SOL;
use crate::wordlist::N_TEST;
use crate::wordlist::WL_SOL;
use crate::wordlist::WL_TEST;

use std::fs::File;
use std::io::{Write, BufWriter};
use std::io::{Read, BufReader};

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
            data: vec![0u32; n_test * n_sol],
        }
    }

    fn build() -> Self {
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

    pub fn fill_buckets(&self, i: usize, buckets: &mut [u16], state: &[usize]) {
        buckets.fill(0);

        for &j in state {
            let pat = self.get(i, j);
            buckets[pat as usize] += 1;
        }
    }

    pub fn fill_state_buckets(&self, i: usize, state_buckets: &mut [Vec<usize>], state: &[usize]) {
        state_buckets.fill(Vec::new());

        for &j in state {
            let pat = self.get(i, j);
            state_buckets[pat as usize].push(j);
        }
    }

    fn save(&self, path: &str) -> std::io::Result<()> {
        eprintln!("Saving PatternTable to disk at {}", path);

        let file = File::create(path)?;
        let mut w = BufWriter::new(file);

        // HEADER
        w.write_all(b"WDPT")?;              // magic
        w.write_all(&[1u8])?;               // version
        w.write_all(&[N as u8])?;           // word_len
        w.write_all(&[(2 * N) as u8])?;     // pattern_bits
        w.write_all(&[0u8])?;               // reserved

        w.write_all(&(self.n_test as u32).to_le_bytes())?;  // N_TEST
        w.write_all(&(self.n_sol as u32).to_le_bytes())?;   // N_SOL

        // DATA
        for &pat in &self.data {
            w.write_all(&pat.to_le_bytes())?;
        }

        w.flush()?;

        Ok(())
    }
 
    fn load(path: &str) -> std::io::Result<Self> {
        eprintln!("Loading PatternTable from disk at {}", path);

        let file = File::open(path)?;
        let mut r = BufReader::new(file);

        // HEADER

        // magic
        let mut magic = [0u8; 4];
        r.read_exact(&mut magic)?;
        if &magic != b"WDPT" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid magic number",
            ));
        }

        let mut buf = [0u8; 1];

        // version
        r.read_exact(&mut buf)?;
        if buf[0] != 1 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unsupported version",
            ));
        }

        // word_len
        r.read_exact(&mut buf)?;
        if (buf[0] as usize) != N {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Word length mismatch",
            ));
        }

        // pattern_bits
        r.read_exact(&mut buf)?;
        if (buf[0] as usize) != 2 * N {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Pattern encoding mismatch",
            ));
        }

        // reserved, ignore
        r.read_exact(&mut buf)?;

        // n_test and n_sol
        let mut buf4 = [0u8; 4];

        r.read_exact(&mut buf4)?;
        let n_test = u32::from_le_bytes(buf4) as usize;

        r.read_exact(&mut buf4)?;
        let n_sol = u32::from_le_bytes(buf4) as usize;

        // ---- DATA ----
        let mut data = Vec::with_capacity(n_test * n_sol);

        for _ in 0..(n_test * n_sol) {
            r.read_exact(&mut buf4)?;
            data.push(u32::from_le_bytes(buf4));
        }

        Ok(PatternTable { n_test, n_sol, data })
    }

    pub fn load_or_build(path: &str) -> Self {
        match Self::load(path) {
            Ok(pats) => pats,
            Err(_) => {
                let pats = Self::build();
                pats.save(path).expect("Error saving PatternTable");
                pats
            }
        }
    }
}
