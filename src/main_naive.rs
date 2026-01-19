use crate::wordlist::N;
use crate::wordlist::WL_SOL;
use rand::seq::SliceRandom;

pub mod wordlist;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ResColor {
    Green,
    Yellow,
    Gray,
}

const fn color_priority(c: ResColor) -> u8 {
    match c {
        ResColor::Green => 0,
        ResColor::Yellow => 1,
        ResColor::Gray => 2,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Res {
    colors: [ResColor; N],
    testword: [u8; N],
}

fn test(t: [u8; N], s: [u8; N]) -> Res {
    let mut skip = [false; N];
    let mut c = [ResColor::Gray; N];

    for (i, &tl) in t.iter().enumerate() {
        if s[i] == tl {
            skip[i] = true;
            c[i] = ResColor::Green;
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
            c[i] = ResColor::Yellow;
            skip_s[j] = true;
        }
    }
    Res {
        colors: c,
        testword: t,
    }
}

const BASE_ORDER: [usize; N] = {
    let mut a = [0usize; N];
    let mut i = 0;
    while i < N {
        a[i] =  i;
        i += 1;
    }
    a
};

fn is_compatible(
    res: Res,
    w: [u8; N],
) -> bool {
    let t = res.testword;
    let colors = res.colors;
    let mut order = BASE_ORDER;

    let mut freq = [0u8; 26];
    for &c in w.iter() {
        freq[(c - b'A') as usize] += 1;
    }

    order.sort_unstable_by_key(|&i| color_priority(colors[i]));

    // println!("{:?}", colors);
    // println!("{:?}", vec);

    for i in order {
        match colors[i] {
            ResColor::Green => {
                if t[i] == w[i] {
                    freq[(t[i] - b'A') as usize] -= 1;
                } else {
                    return false;
                }
            }
            ResColor::Yellow => {
                let idx = (t[i] - b'A') as usize;
                if freq[idx] > 0 {
                    freq[idx] -= 1;
                } else {
                    return false;
                }
            }
            ResColor::Gray => {
                if freq[(t[i] - b'A') as usize] > 0 {
                    return false;
                }
            }
        };
    }

    true
}

fn main() {
    // let v = b"HELLO".to_vec();
    // let mask = (1u32 << 17) | (1u32 << 3);

    // println!("encoded b'L' = {:?}", b32encode(b'L'));
    // println!("mask = {:?}", mask);
    // println!("{:?}", letter_is_compatible(mask, v[3]));

    let mut wl_sample = WL_SOL;
    let mut rng = rand::rng();
    let (wl_sample, _) = wl_sample[..].partial_shuffle(&mut rng, 200);

    for &t in wl_sample.iter() {
        let mut t_entropy: usize = 0;
        for &s in wl_sample.iter() {
            let res = test(t, s);
            for &w in wl_sample.iter() {
                if !is_compatible(res, w) { t_entropy+=1;};
            }
        }
        println!("{} -> {}", str::from_utf8(&t).expect(""), t_entropy);
    }

}
