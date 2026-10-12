//! Shared harness for the differential tests: fixed-address test memory,
//! a deterministic generator, the normalised call logs and the comparisons.
//!
//! Every case runs the verified rewrite and the lifted method on the same
//! generated inputs. Both sides record their numbered callee calls (or the
//! lifted collaborator calls) in the same normalised form, so the logs
//! compare by meaning: an address that points into the object is named by
//! the part it points at, and every other word is compared as a word.

#![allow(dead_code)]

use std::slice;
use std::sync::{Mutex, MutexGuard};

use lf_peds_seekdiff::rt;

/// Trials per case.
pub const TRIALS: u32 = 1500;

/// A fixed-address block of test memory. The vector never reallocates
/// after construction, so the address stays valid for the arena's life.
pub struct Arena {
    words: Vec<u32>,
    len: usize,
}

impl Arena {
    pub fn zeroed(len: usize) -> Self {
        Self { words: vec![0; len.div_ceil(4).max(1)], len }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut a = Self::zeroed(bytes.len());
        a.bytes_mut().copy_from_slice(bytes);
        a
    }

    pub fn from_words(words: &[u32]) -> Self {
        let mut bytes = Vec::with_capacity(words.len() * 4);
        for w in words {
            bytes.extend_from_slice(&w.to_le_bytes());
        }
        Self::from_bytes(&bytes)
    }

    pub fn addr(&self) -> u32 {
        self.words.as_ptr() as usize as u32
    }

    pub fn bytes(&self) -> &[u8] {
        // SAFETY: the vector owns `words.len() * 4 >= len` initialised bytes.
        unsafe { slice::from_raw_parts(self.words.as_ptr() as *const u8, self.len) }
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: as in `bytes`, with exclusive access through `&mut self`.
        unsafe { slice::from_raw_parts_mut(self.words.as_mut_ptr() as *mut u8, self.len) }
    }
}

/// Small deterministic generator (xorshift64*), so every run sees the same
/// inputs.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    pub fn next_u32(&mut self) -> u32 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32
    }

    pub fn below(&mut self, n: u32) -> u32 {
        self.next_u32() % n
    }

    pub fn chance(&mut self, one_in: u32) -> bool {
        self.below(one_in) == 0
    }

    pub fn bytes(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| self.next_u32() as u8).collect()
    }

    /// A word that is often a special value: zero, one, the signed and
    /// unsigned limits, the top bit alone, the float infinities and
    /// not-a-number with and without its sign bit, and small counts.
    pub fn word(&mut self) -> u32 {
        const SPECIAL: [u32; 14] = [
            0,
            1,
            0xFFFF_FFFF,
            0x7FFF_FFFF,
            0x8000_0000,
            0x7FC0_0000,
            0xFFC0_0000,
            0x7F80_0000,
            0xFF80_0000,
            0x3F80_0000,
            0x0000_0007,
            0x0000_0008,
            0x0000_00FF,
            0x0000_0100,
        ];
        if self.below(3) == 0 {
            SPECIAL[self.below(SPECIAL.len() as u32) as usize]
        } else {
            self.next_u32()
        }
    }

    pub fn words<const N: usize>(&mut self) -> [u32; N] {
        std::array::from_fn(|_| self.word())
    }
}

/// A normalised argument: a word, or the name of the thing it points into.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Norm {
    W(u32),
    S(&'static str),
}

/// A normalised callee call, or a lifted collaborator call, with the object
/// image at the time of the call when the case watches one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ev {
    pub name: &'static str,
    pub args: Vec<Norm>,
    pub snap: Option<Vec<u8>>,
}

/// Normalises one recorded rewrite call. `query_id` names the callee whose
/// second argument is a stack word of the rewrite (its out pointer).
pub fn norm_call(
    raw: &rt::RawCall,
    id_names: &[(u32, &'static str)],
    addr_names: &[(u32, &'static str)],
    query_id: Option<u32>,
) -> Ev {
    let name = id_names
        .iter()
        .find(|(id, _)| *id == raw.id)
        .map_or("UNMAPPED", |(_, n)| *n);
    let args = raw
        .args
        .iter()
        .enumerate()
        .map(|(i, &w)| {
            if Some(raw.id) == query_id && i == 1 {
                return Norm::S("out");
            }
            addr_names
                .iter()
                .find(|(a, _)| *a == w)
                .map_or(Norm::W(w), |(_, n)| Norm::S(n))
        })
        .collect();
    Ev { name, args, snap: raw.snap.clone() }
}

/// Compares the rewrite's calls, normalised, with the lifted calls.
pub fn compare_calls(
    raw: &[rt::RawCall],
    id_names: &[(u32, &'static str)],
    addr_names: &[(u32, &'static str)],
    query_id: Option<u32>,
    lifted: &[Ev],
) -> Result<(), String> {
    let rewritten: Vec<Ev> = raw
        .iter()
        .map(|r| norm_call(r, id_names, addr_names, query_id))
        .collect();
    if rewritten != lifted {
        return Err(format!(
            "call logs differ:\n  rewrite {rewritten:?}\n  lifted  {lifted:?}"
        ));
    }
    Ok(())
}

/// The memory a rewrite leaves behind must equal the lifted side's image.
pub fn compare_bytes(what: &str, raw: &[u8], lifted: &[u8]) -> Result<(), String> {
    if raw != lifted {
        let at = raw.iter().zip(lifted).position(|(a, b)| a != b);
        return Err(format!("{what} images differ, first at {at:?}"));
    }
    Ok(())
}

pub fn rd32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

pub fn wr32(b: &mut [u8], off: usize, v: u32) {
    b[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

/// The test binary's runs share the runtime's scripted state, so each test
/// holds this lock for its whole body.
static SERIAL: Mutex<()> = Mutex::new(());

pub fn lock() -> MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}

/// Runs the trials of one case and panics on the first disagreement. Returns
/// the number of trials per branch label the case reported, and asserts that
/// every label is reached.
pub fn run_cases(
    name: &str,
    labels: &[&'static str],
    mut trial: impl FnMut(&mut Rng) -> Result<&'static str, String>,
) {
    let mut rng = Rng::new(0x5EEC_C0DE ^ name.len() as u64);
    let mut branches: Vec<(&'static str, u32)> = Vec::new();
    for n in 0..TRIALS {
        match trial(&mut rng) {
            Ok(branch) => match branches.iter_mut().find(|(b, _)| *b == branch) {
                Some((_, c)) => *c += 1,
                None => branches.push((branch, 1)),
            },
            Err(e) => panic!("{name}: trial {n} differs: {e}"),
        }
    }
    eprintln!("{name}: {TRIALS} trials agree; branches {branches:?}");
    for label in labels {
        assert!(
            branches.iter().any(|(b, c)| b == label && *c > 0),
            "{name}: branch {label} never ran"
        );
    }
}

/// Runs the trials of a deliberately wrong lift and returns how many
/// disagreed with the rewrite. The first disagreement is printed.
pub fn count_disagreements(
    name: &str,
    mut trial: impl FnMut(&mut Rng) -> Result<&'static str, String>,
) -> u32 {
    let mut rng = Rng::new(0x5EEC_C0DE ^ name.len() as u64);
    let mut bad = 0;
    for _ in 0..TRIALS {
        if let Err(e) = trial(&mut rng) {
            if bad == 0 {
                eprintln!("{name}: first disagreement: {e}");
            }
            bad += 1;
        }
    }
    eprintln!("{name}: {bad} of {TRIALS} trials disagree");
    bad
}

/// Points numbered callees at named stubs.
pub fn install(stubs: &[(u32, &str)]) {
    for (id, name) in stubs {
        rt::set_callee(*id, rt::stub(name));
    }
}
