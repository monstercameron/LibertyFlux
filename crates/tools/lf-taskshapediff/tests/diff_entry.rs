//! Differential cases of the ped task entry initialiser against its verified
//! 32-bit rewrite.
//!
//! The lifted initialiser and the rewrite run on the same generated entry,
//! arguments, registry, scale factor, token, fetched word and mapped value.
//! The effects compared are:
//!
//! - the collaborator calls, in order, with their arguments as the rewrite's
//!   stubs read them (the entry handle, the registry handle and key, the
//!   token, whether the fetch's scratch descriptor is present, the constant
//!   zero of the mapping call and the index it maps);
//! - the entry words the initialiser owns, decoded from the planted entry
//!   after the rewrite and compared with the values the lift writes;
//! - every other byte of the entry, which the rewrite must leave as it found
//!   it.
//!
//! Each routine also has deliberately wrong versions. Each must differ from
//! its rewrite on at least one generated case.
//!
//! Runs on the 32-bit target only: the rewrite takes raw addresses, reads
//! planted globals and calls numbered stubs.

#![cfg(target_arch = "x86")]
#![allow(unsafe_code)]

use std::sync::Mutex;

use lf_peds_tasks::ped_task::{EntryArgs, EntryCallees, TaskEntry, truncate_indefinite};
use lf_taskshapediff::rewrites::fn_00b76660;
use lf_taskshapediff::{Xorshift, rt};

/// Bytes planted for the entry: the largest offset written is 0xbcb.
const ENTRY_BYTES: usize = 0xBD0;
/// Bytes planted for a source object: the key sits at offset 4.
const SOURCE_BYTES: usize = 0x10;
/// Bytes planted for the registry global's stand-in.
const REGISTRY_BYTES: usize = 0x10;
/// File address of the registry global.
const REGISTRY_VA: u32 = 0x0115_DC18;
/// File address of the scale global.
const SCALE_VA: u32 = 0x00FE_8898;
/// Generated cases per routine.
const CASES: u32 = 3000;
/// Seed of the case generator for every test in this program.
const SEED: u32 = 0x5EED_00B7;

/// Call tags in the log.
const TAG_SETUP: u32 = 1;
const TAG_RESOLVE: u32 = 2;
const TAG_FETCH: u32 = 3;
const TAG_MAP: u32 = 4;

/// The call log of the rewrite's stubs, in order.
static LOG: Mutex<Vec<Vec<u32>>> = Mutex::new(Vec::new());
/// The token the resolver stub answers.
static TOKEN: Mutex<u32> = Mutex::new(0);
/// The address of the word the fetch stub answers with.
static FETCHED_PTR: Mutex<u32> = Mutex::new(0);
/// The value the mapping stub answers.
static MAPPED: Mutex<u32> = Mutex::new(0);
/// Serialises the cases: the stubs, the callee table and the globals are
/// process-wide.
static LOCK: Mutex<()> = Mutex::new(());

fn log(entry: Vec<u32>) {
    LOG.lock().unwrap().push(entry);
}

extern "thiscall" fn stub_setup(entry: u32) -> u32 {
    log(vec![TAG_SETUP, entry]);
    0
}

extern "thiscall" fn stub_resolve(registry: u32, key: u32) -> u32 {
    log(vec![TAG_RESOLVE, registry, key]);
    *TOKEN.lock().unwrap()
}

extern "cdecl" fn stub_fetch(token: u32, descriptor: u32) -> u32 {
    log(vec![TAG_FETCH, token, u32::from(descriptor != 0)]);
    *FETCHED_PTR.lock().unwrap()
}

extern "cdecl" fn stub_map(zero: u32, index: u32) -> u32 {
    log(vec![TAG_MAP, zero, index]);
    *MAPPED.lock().unwrap()
}

/// The collaborator the lifted initialiser calls. It logs the same entries
/// as the rewrite's stubs, with the scratch descriptor and the constant zero.
struct Recorder {
    token: u32,
    fetched: u32,
    mapped: u32,
    log: Vec<Vec<u32>>,
}

impl EntryCallees for Recorder {
    fn setup(&mut self, entry: u32) {
        self.log.push(vec![TAG_SETUP, entry]);
    }
    fn resolve(&mut self, registry: u32, key: u32) -> u32 {
        self.log.push(vec![TAG_RESOLVE, registry, key]);
        self.token
    }
    fn fetch(&mut self, token: u32) -> u32 {
        self.log.push(vec![TAG_FETCH, token, 1]);
        self.fetched
    }
    fn map(&mut self, index: u32) -> u32 {
        self.log.push(vec![TAG_MAP, 0, index]);
        self.mapped
    }
}

/// One generated case and what the rewrite did with it.
struct Case {
    entry_before: Vec<u8>,
    entry_after: Vec<u8>,
    args: EntryArgs,
    entry_address: u32,
    registry: u32,
    scale_bits: u32,
    token: u32,
    fetched: u32,
    mapped: u32,
    returned: u32,
    calls: Vec<Vec<u32>>,
}

/// The address of a planted buffer, as the 32-bit rewrite sees it.
fn address_of(buffer: &[u8]) -> u32 {
    buffer.as_ptr() as usize as u32
}

/// Reads a little-endian word at a byte offset of a planted buffer.
fn word_at(buffer: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        buffer[offset],
        buffer[offset + 1],
        buffer[offset + 2],
        buffer[offset + 3],
    ])
}

/// Writes a little-endian word at a byte offset of a planted buffer.
fn set_word(buffer: &mut [u8], offset: usize, value: u32) {
    buffer[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

/// Random bytes of the given length.
fn random_bytes(rng: &mut Xorshift, len: usize) -> Vec<u8> {
    (0..len).map(|_| (rng.next_u32() & 0xFF) as u8).collect()
}

/// Generates a case, plants it and runs the rewrite on it.
fn run_rewrite(rng: &mut Xorshift) -> Case {
    let entry = random_bytes(rng, ENTRY_BYTES);
    let entry_before_seed = entry.clone();
    let entry_address = address_of(&entry);

    let mut source = random_bytes(rng, SOURCE_BYTES);
    let key = rng.word();
    set_word(&mut source, 4, key);
    let has_source = rng.next_u32() % 4 != 0;
    let source_pointer = if has_source { address_of(&source) } else { 0 };

    let registry_memory = random_bytes(rng, REGISTRY_BYTES);
    let registry = address_of(&registry_memory);
    rt::set_extra(REGISTRY_VA, registry);

    let scale_bits = match rng.next_u32() % 4 {
        0 => 0x3F00_0000,
        1 => 0x7FC0_0000,
        _ => rng.word(),
    };
    let scale_memory = scale_bits.to_le_bytes();
    rt::set_extra(SCALE_VA, address_of(&scale_memory));

    let token = match rng.next_u32() % 3 {
        0 => 0,
        _ => rng.word() | 1,
    };
    let fetched_block = vec![rng.word(), rng.word()];
    let fetched_ptr = fetched_block.as_ptr() as usize as u32;
    let fetched = fetched_block[0];
    let mapped = rng.word();
    *TOKEN.lock().unwrap() = token;
    *FETCHED_PTR.lock().unwrap() = fetched_ptr;
    *MAPPED.lock().unwrap() = mapped;

    let args = EntryArgs {
        kind: rng.word(),
        kind_flag: rng.word(),
        source: source_pointer,
        source_key: if has_source { Some(key) } else { None },
        convert: rng.word(),
    };
    let a3 = args.convert;
    LOG.lock().unwrap().clear();
    let returned =
        fn_00b76660::rw_00b76660(entry_address, args.kind, args.kind_flag, args.source, a3);
    let calls = std::mem::take(&mut *LOG.lock().unwrap());
    let entry_after = entry;
    Case {
        entry_before: entry_before_seed,
        entry_after,
        args,
        entry_address,
        registry,
        scale_bits,
        token,
        fetched,
        mapped,
        returned,
        calls,
    }
}

/// The owned words of an entry, decoded from a planted entry's bytes.
fn decode(entry: &[u8]) -> TaskEntry {
    TaskEntry {
        kind: entry[0xBC0],
        kind_flag: word_at(entry, 0xBC4),
        source: word_at(entry, 0x960),
        cleared_99c: word_at(entry, 0x99C),
        handle: word_at(entry, 0x994),
        mapped: word_at(entry, 0x990),
        cleared_aa0: word_at(entry, 0xAA0),
        sentinels: [
            word_at(entry, 0xAA4),
            word_at(entry, 0xAA8),
            word_at(entry, 0xAAC),
            word_at(entry, 0xAB0),
        ],
        enabled: u16::from_le_bytes([entry[0xBCA], entry[0xBCB]]),
        cleared_bb8: word_at(entry, 0xBB8),
    }
}

/// Whether a byte offset of the entry is one of the words the lift owns.
fn owned(offset: usize) -> bool {
    const RANGES: [(usize, usize); 10] = [
        (0xBC0, 0xBC1),
        (0xBC4, 0xBC8),
        (0x960, 0x964),
        (0x99C, 0x9A0),
        (0x994, 0x998),
        (0x990, 0x994),
        (0xAA0, 0xAA4),
        (0xAA4, 0xAB4),
        (0xBCA, 0xBCC),
        (0xBB8, 0xBBC),
    ];
    RANGES.iter().any(|&(lo, hi)| offset >= lo && offset < hi)
}

/// Checks the rewrite's own effects: every byte the lift does not own is
/// unchanged.
fn check_rewrite_effects(case: &Case) {
    for offset in 0..ENTRY_BYTES {
        if !owned(offset) {
            assert_eq!(
                case.entry_before[offset], case.entry_after[offset],
                "entry byte {offset:#x} is unchanged"
            );
        }
    }
}

/// Runs the lift on a case and returns its entry and its calls.
fn run_lift(case: &Case) -> (TaskEntry, Vec<Vec<u32>>) {
    let mut entry = decode(&case.entry_before);
    let mut recorder = Recorder {
        token: case.token,
        fetched: case.fetched,
        mapped: case.mapped,
        log: Vec::new(),
    };
    entry.initialise(
        case.entry_address,
        case.args,
        case.registry,
        f32::from_bits(case.scale_bits),
        &mut recorder,
    );
    (entry, recorder.log)
}

/// Asserts that the lift matches the rewrite on one case.
fn assert_case(case: &Case) {
    check_rewrite_effects(case);
    assert_eq!(case.returned, 0, "the rewrite returns no value of its own");
    let (entry, calls) = run_lift(case);
    assert_eq!(calls, case.calls, "the collaborator calls match in order");
    assert_eq!(entry, decode(&case.entry_after), "the owned words match");
}

/// The deviations of a wrong version.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The first kind slot takes the second argument.
    KindFromSecond,
    /// The fetched word is not stored as the handle.
    FetchNotStored,
    /// The mapping uses the saturating conversion of the verified
    /// block-constructor rewrite instead of the processor's.
    SaturatingConvert,
    /// The enable halfword is left at zero.
    NoEnable,
    /// The fetch is made even when the token is zero.
    FetchWithoutToken,
    /// The second sentinel is one off.
    SentinelOff,
}

/// The wrong version of the initialiser: the same calls and words, with one
/// deviation. Returns the entry and the calls.
fn run_wrong(kind: Wrong, case: &Case) -> (TaskEntry, Vec<Vec<u32>>) {
    let mut entry = decode(&case.entry_before);
    let mut recorder = Recorder {
        token: case.token,
        fetched: case.fetched,
        mapped: case.mapped,
        log: Vec::new(),
    };
    recorder.setup(case.entry_address);
    entry.kind = if matches!(kind, Wrong::KindFromSecond) {
        (case.args.kind_flag & 0xFF) as u8
    } else {
        (case.args.kind & 0xFF) as u8
    };
    entry.kind_flag = case.args.kind_flag & 0xFF;
    entry.source = case.args.source;
    entry.cleared_99c = 0;
    entry.enabled = if matches!(kind, Wrong::NoEnable) {
        0
    } else {
        1
    };
    entry.cleared_aa0 = 0;
    entry.sentinels = [0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF];
    if matches!(kind, Wrong::SentinelOff) {
        entry.sentinels[1] = 0xFFFF_FFFE;
    }
    if let Some(key) = case.args.source_key {
        let token = recorder.resolve(case.registry, key);
        if token != 0 || matches!(kind, Wrong::FetchWithoutToken) {
            let fetched = recorder.fetch(token);
            if !matches!(kind, Wrong::FetchNotStored) {
                entry.handle = fetched;
            }
        }
    }
    if (case.args.convert & 0xFF) as u8 != 0 {
        let widened = f64::from(entry.handle) as f32;
        let scaled =
            core::hint::black_box(widened) * core::hint::black_box(f32::from_bits(case.scale_bits));
        let index = if matches!(kind, Wrong::SaturatingConvert) {
            scaled as i32
        } else {
            truncate_indefinite(scaled)
        };
        entry.mapped = recorder.map(index as u32);
    }
    entry.cleared_bb8 = 0;
    (entry, recorder.log)
}

/// Counts the cases on which a wrong version differs from the rewrite.
fn wrong_mismatches(kind: Wrong) -> u32 {
    let mut rng = Xorshift::new(SEED);
    let mut differing = 0;
    for _ in 0..CASES {
        let case = run_rewrite(&mut rng);
        let (entry, calls) = run_wrong(kind, &case);
        if calls != case.calls || entry != decode(&case.entry_after) {
            differing += 1;
        }
    }
    eprintln!("wrong {kind:?}: differing {differing} of {CASES}");
    differing
}

/// Plants the four stubs in the rewrite's callee slots.
fn plant_stubs() {
    rt::set_callee(1, stub_setup as *const () as usize as u32);
    rt::set_callee(2, stub_resolve as *const () as usize as u32);
    rt::set_callee(3, stub_fetch as *const () as usize as u32);
    rt::set_callee(4, stub_map as *const () as usize as u32);
}

#[test]
fn entry_init_matches_its_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    plant_stubs();
    let mut rng = Xorshift::new(SEED);
    let mut mapped_cases = 0;
    let mut fetched_cases = 0;
    for _ in 0..CASES {
        let case = run_rewrite(&mut rng);
        assert_case(&case);
        if case.args.convert & 0xFF != 0 {
            mapped_cases += 1;
        }
        if case.args.source_key.is_some() && case.token != 0 {
            fetched_cases += 1;
        }
    }
    eprintln!("cases {CASES}: conversions {mapped_cases}, fetches {fetched_cases}");
}

#[test]
fn entry_init_wrong_versions_are_caught() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    plant_stubs();
    for kind in [
        Wrong::KindFromSecond,
        Wrong::FetchNotStored,
        Wrong::SaturatingConvert,
        Wrong::NoEnable,
        Wrong::FetchWithoutToken,
        Wrong::SentinelOff,
    ] {
        assert!(
            wrong_mismatches(kind) > 0,
            "wrong version {kind:?} is caught"
        );
    }
}
