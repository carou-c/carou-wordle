
#[derive(Debug)]
struct ComplexRes(Vec<[u32; N]>);

const BASE_32_ALPHABET: &[u8; 32] = b"234567ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn b32encode_c(c: u8) -> usize {
    BASE_32_ALPHABET
        .binary_search(&c)
        .expect("Char cannot be encoded as Base32!!")
}

fn b32encode_w(w: [u8; N]) -> [usize; N] {
    w.map(b32encode_c)
}

fn cmask_is(c: usize) -> u32 {
    1u32 << c
}

fn is_compatible_c(cmask: u32, c: usize) -> bool {
    ((1u32 << c) & cmask) != 0
}

fn is_compatible_w(wmask: [u32; N], w: [usize; N]) -> bool {
    wmask
        .iter()
        .zip(w.iter())
        .any(|(&cmask, &c)| is_compatible_c(cmask, c))
}

fn complex_is_compatible(ComplexRes(res): ComplexRes, w: [usize; N]) -> bool {
    res.iter().any(|&wmask| is_compatible_w(wmask, w))
}

fn complex_test(t: [usize; 5], s: [usize; 5]) -> ComplexRes {
    let mut skip_t = [false; N];
    let mut mask = [!0u32; N];

    for (i, &tl) in t.iter().enumerate() {
        if s[i] == tl {
            skip_t[i] = true;
            mask[i] &= cmask_is(tl);
        } else if !s.contains(&tl) {
            skip_t[i] = true;
            for m in &mut mask {
                *m &= !cmask_is(tl);
            }
        }
    }

    let mut skip_s = skip_t;
    let mut masks = vec![mask];

    for (i, &tl) in t.iter().enumerate() {
        if skip_t[i] {
            continue;
        };

        if let Some((j, _)) = s
            .iter()
            .enumerate()
            .filter(|&(j, _)| !skip_s[j])
            .find(|&(_, &sl)| sl == tl)
        {
            let mut new_masks = vec![];
            for m in &mut masks {
                m[i] &= !cmask_is(tl);
                // let mut new_masks = vec![];
                for k in 0..N {
                    if k == i {
                        continue;
                    }
                    let mut new_mask = *m;
                    new_mask[k] &= cmask_is(tl);
                    new_masks.push(new_mask);
                }
            }
            masks = new_masks;
            skip_s[j] = true;
        }
    }
    ComplexRes(masks)
}
