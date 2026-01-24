use std::io;
use std::io::Write;

use crate::colors::Color;
use crate::colors::encode;
use crate::entropy::best_entropy;
use crate::entropy::entropy_tests;
use crate::pattern::PatternTable;
use crate::wordlist::N;
use crate::wordlist::N_SOL;
use crate::wordlist::WL_SOL;
use crate::wordlist::WL_TEST;

pub mod colors;
pub mod entropy;
pub mod pattern;
pub mod wordlist;

fn main() {
    println!();

    let pats = PatternTable::load_or_build("pattern_table.wdpt");
    let mut state = vec![0usize; N_SOL];
    for (i, j) in state.iter_mut().enumerate() {
        *j = i;
    }

    let mut s: String;

    'outer: loop {
        println!("Solving via entropy_solver:");
        let (best, h) = best_entropy(&pats, &state);

        println!();
        println!(
            "Max entropy word: {} ({})",
            &str::from_utf8(&WL_TEST[best])
                .expect("Failed to transform word in WL_TEST to UTF-8 string."),
            h
        );

        print!("Show top k entropy words? (y [k = 10]/N) ");
        s = String::new();
        io::stdout().flush().expect("Failed to flush stdout.");
        io::stdin()
            .read_line(&mut s)
            .expect("Failed to read line from stdin.");

        if s.trim().len() < 3 {
            s = String::from(s.trim()) + " 10";
        }

        if let ("y ", k) = s.trim().split_at(2) {
            let k: usize = k.trim().parse().expect("k is not a valid usize");
            let mut hs = entropy_tests(&pats, &state);
            hs.sort_unstable_by(|&(_, h1), &(_, h2)| h2.total_cmp(&h1));
            for (i, &(w, hw)) in hs[..k].iter().enumerate() {
                println!(
                    "{}. {} ({})",
                    i + 1,
                    str::from_utf8(&WL_TEST[w])
                        .expect("Failed to transform word in WL_TEST to UTF-8 string."),
                    hw
                )
            }
        }

        let t: [u8; N];

        loop {
            print!("Your guess (or 'q' to quit): ");
            s = String::new();
            io::stdout().flush().expect("Failed to flush stdout.");
            io::stdin()
                .read_line(&mut s)
                .expect("Failed to read line from stdin.");
            let s = s.trim().as_bytes();

            if (s[0] == b'q') || (s[0] == b'Q') {
                break 'outer;
            }

            let (&[s], _) = s.as_chunks::<N>() else {
                println!(
                    "Invalid guess!! (Your guess doesn't have {} ASCII characters)",
                    N
                );
                println!();
                continue;
            };

            if !WL_TEST.contains(&s) {
                println!("Invalid guess!! (Your guess is not a valid test word)");
                println!();
                continue;
            }

            println!();
            t = s;
            break;
        }

        let mut res = [const { Color::Gray }; N];
        println!("Guess results (G for Green; Y for Yellow; - for Gray):");
        for (i, &l) in t.iter().enumerate() {
            loop {
                print!("({}) {}: ", i + 1, l as char);

                s = String::new();
                io::stdout().flush().expect("Failed to flush stdout.");
                io::stdin()
                    .read_line(&mut s)
                    .expect("Failed to read line from stdin.");

                match s.trim() {
                    "G" | "g" => res[i] = Color::Green,
                    "Y" | "y" => res[i] = Color::Yellow,
                    "-" => res[i] = Color::Gray,
                    _ => continue,
                }
                println!();
                break;
            }
        }

        let pat = encode(&res);

        let mut state_buckets = [const { Vec::new() }; 1 << (2 * N)];
        let i = WL_TEST.binary_search(&t).expect("Test word not found.");

        pats.fill_state_buckets(i, &mut state_buckets, &state);

        state = state_buckets[pat as usize].clone();
        println!();

        print!(
            "{} word remaining. Show remaining words? (Y/n)",
            state.len()
        );
        s = String::new();
        io::stdout().flush().expect("Failed to flush stdout.");
        io::stdin()
            .read_line(&mut s)
            .expect("Failed to read line from stdin.");

        match s.trim() {
            "n" => continue,
            _ => {
                let hs = entropy_tests(&pats, &state);
                let mut state_sorted: Vec<([u8; N], f64)> = state
                    .iter()
                    .map(|&s| {
                        let w = WL_SOL[s];
                        let test_idx = WL_TEST
                            .binary_search(&w)
                            .expect("Failed to find word in WL_SOL in WL_TEST");
                        (w, hs[test_idx].1)
                    })
                    .collect();
                state_sorted.sort_unstable_by(|&(_, h1), &(_, h2)| h2.total_cmp(&h1));
                for (w, h) in state_sorted {
                    println!(
                        "{} ({})",
                        str::from_utf8(&w)
                            .expect("Failed to transform word in WL_SOL to UTF-8 string."),
                        h
                    );
                }
            }
        }
    }
}
