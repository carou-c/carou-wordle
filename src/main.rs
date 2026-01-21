use std::io;
use std::io::Write;

use crate::colors::Color;
use crate::colors::encode;
use crate::entropy::best_entropy;
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

    'outer: loop {
        println!("Solving via entropy_solver:");
        let (best, _) = best_entropy(&pats, &state);
        println!();
        println!(
            "Max entropy word: {}",
            &str::from_utf8(&WL_TEST[best])
                .expect("Failed to transform word in WL_TEST to UTF-8 string.")
        );

        let mut s = String::new();
        let t: [u8; N];

        loop {
            print!("Your guess (or 'q' to quit): ");
            io::stdout().flush().expect("Failed to flush stdout.");
            io::stdin()
                .read_line(&mut s)
                .expect("Failed to read line from stdin.");
            let s = s.trim().as_bytes();

            if s[0] == b'q' {
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

                println!("{:?}", s.trim());
                match s.trim() {
                    "G" => res[i] = Color::Green,
                    "Y" => res[i] = Color::Yellow,
                    "-" => res[i] = Color::Gray,
                    _ => continue,
                }
                println!();
                break;
            }
        }

        let pat = encode(&res);

        let mut bucket_states = [const { Vec::new() }; 1 << (2 * N)];
        let i = WL_TEST.binary_search(&t).expect("Test word not found.");

        pats.fill_bucket_states(i, &mut bucket_states, &state);

        state = bucket_states[pat as usize].clone();
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
                let ws: Vec<&str> = state
                    .iter()
                    .map(|&w| {
                        str::from_utf8(&WL_SOL[w])
                            .expect("Failed to transform word in WL_TEST to UTF-8 string.")
                    })
                    .collect();
                println!("{:#?}", ws);
            }
        }
    }
}
