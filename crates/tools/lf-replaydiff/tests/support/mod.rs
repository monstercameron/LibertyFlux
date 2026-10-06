//! Shared differential-test support: RNG, bar planter, image checks, lock.
//!
//! One lock guards each whole test: the runtime's global slots are plain
//! words behind stable addresses, so tests sharing a binary must not run
//! rewrite calls concurrently.

use std::sync::Mutex;
use std::sync::MutexGuard;

use lf_files_memory::replay_bar::ReplayBar;

static TEST_LOCK: Mutex<()> = Mutex::new(());

/// Holds the binary's test lock for the whole test.
pub fn lock() -> MutexGuard<'static, ()> {
    TEST_LOCK.lock().unwrap()
}

/// Minimal seeded RNG (xorshift32): deterministic across targets.
#[derive(Clone)]
pub struct Rng(pub u32);

impl Rng {
    /// Next word.
    pub fn u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    /// A word below `n`.
    pub fn below(&mut self, n: u32) -> u32 {
        if n == 0 {
            0
        } else {
            self.u32() % n
        }
    }

    /// A float from random bits.
    pub fn f32(&mut self) -> f32 {
        f32::from_bits(self.u32())
    }

    /// Fills `out` with random bytes.
    pub fn bytes(&mut self, out: &mut [u8]) {
        for b in out.iter_mut() {
            *b = self.u32() as u8;
        }
    }
}

/// Address of a test allocation as the rewrites see it (32-bit only).
#[must_use]
pub fn addr<T>(r: &T) -> u32 {
    (r as *const T as usize) as u32
}

/// Writes a little-endian word into a test image.
pub fn put_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

/// Reads a little-endian word from a test allocation.
#[must_use]
pub fn get_word(base: u32, off: u32) -> u32 {
    unsafe { (base.wrapping_add(off) as *const u32).read_unaligned() }
}

/// The time-base globals: narrow/wide numerators, narrow/wide denominators.
pub const NUM_NARROW_VA: u32 = 0x0105_c880;
/// The wide numerator word.
pub const NUM_WIDE_VA: u32 = 0x0105_c87c;
/// The narrow denominator word.
pub const DEN_NARROW_VA: u32 = 0x0105_c884;
/// The wide denominator word.
pub const DEN_WIDE_VA: u32 = 0x0105_c888;
/// The stamp-publish flag word.
pub const PUB_FLAGS_VA: u32 = 0x011f_70e0;
/// The stamp-publish stamp word.
pub const PUB_STAMP_VA: u32 = 0x011f_70e4;
/// The game-state word.
pub const STATE_VA: u32 = 0x0103_7720;
/// The notifier hub (relocated).
pub const HUB_VA: u32 = 0x0103_e498;

/// Size of the bar object image: words through `+0x100`.
pub const BAR_LEN: usize = 0x104;
/// Size of one planted slot entry: the tag byte and stamp word live here.
pub const ELEM_LEN: usize = 0x18;

/// A planted 32-bit bar: the object, its table, pointer array and entries.
///
/// Boxes are kept alive as long as the planted bar; every address the
/// rewrite touches points into them.
pub struct PlantedBar {
    /// The object bytes.
    pub obj: Box<[u8]>,
    /// The table object: array pointer, 16-bit count, padding.
    pub table: Box<[u8]>,
    /// The entry pointer array (one spare word when empty: never read).
    pub arr: Box<[u32]>,
    /// The entries: tag byte at `+1`, stamp word at `+0x14`.
    pub elems: Vec<Box<[u8]>>,
}

impl PlantedBar {
    /// Object base address.
    #[must_use]
    pub fn base(&self) -> u32 {
        addr(&self.obj[0])
    }

    /// Table object address (the empty-selection answer).
    #[must_use]
    pub fn table_addr(&self) -> u32 {
        addr(&self.table[0])
    }
}

/// Plants the 32-bit bar for `bar`, with `clock_link` at `+0x94`.
///
/// Unmodelled bytes (the word at `+0x00`, gaps, table padding, entry
/// slack) get random fill so a stray write shows up in the diff.
pub fn plant_bar(bar: &ReplayBar, clock_link: u32, rng: &mut Rng) -> PlantedBar {
    let n = bar.slots.len();
    assert!(n <= 0xffff, "the count word holds 16 bits");
    let mut elems: Vec<Box<[u8]>> = Vec::with_capacity(n);
    for slot in &bar.slots {
        let mut e = vec![0u8; ELEM_LEN];
        rng.bytes(&mut e);
        e[1] = slot.tag;
        put_u32(&mut e, 0x14, slot.stamp);
        elems.push(e.into_boxed_slice());
    }
    let mut arr = vec![0u32; n.max(1)].into_boxed_slice();
    for (cell, elem) in arr.iter_mut().zip(elems.iter()) {
        *cell = addr(&elem[0]);
    }
    let arr_addr = addr(&arr[0]);
    let mut table = vec![0u8; 8].into_boxed_slice();
    {
        let t = table.as_mut();
        rng.bytes(t);
        put_u32(t, 0, arr_addr);
        t[4..6].copy_from_slice(&(n as u16).to_le_bytes());
    }
    let table_addr = addr(&table[0]);
    let mut obj = vec![0u8; BAR_LEN].into_boxed_slice();
    {
        let o = obj.as_mut();
        rng.bytes(o);
        put_u32(o, 0x04, bar.seconds.to_bits());
        put_u32(o, 0x08, bar.origin.to_bits());
        put_u32(o, 0x0c, bar.weight.to_bits());
        put_u32(o, 0x10, bar.width.to_bits());
        put_u32(o, 0x20, bar.cursor);
        put_u32(o, 0x94, clock_link);
        put_u32(o, 0x9c, table_addr);
        put_u32(o, 0xa0, bar.selected);
        put_u32(o, 0xa8, bar.rect0.0.to_bits());
        put_u32(o, 0xac, bar.rect0.1.to_bits());
        put_u32(o, 0xb0, bar.rect0.2.to_bits());
        put_u32(o, 0xb4, bar.rect0.3.to_bits());
        put_u32(o, 0xb8, bar.bound_lo);
        put_u32(o, 0xbc, bar.rect1.0.to_bits());
        put_u32(o, 0xc0, bar.rect1.1.to_bits());
        put_u32(o, 0xc4, bar.rect1.2.to_bits());
        put_u32(o, 0xc8, bar.rect1.3.to_bits());
        put_u32(o, 0xcc, bar.bound_hi);
        put_u32(o, 0xd0, bar.hi.to_bits());
        put_u32(o, 0xec, bar.lo.to_bits());
        put_u32(o, 0xf8, bar.selected);
        put_u32(o, 0xfc, bar.sel_mark);
        put_u32(o, 0x100, bar.total);
    }
    PlantedBar {
        obj,
        table,
        arr,
        elems,
    }
}

/// Word offsets where the live image differs from its snapshot.
#[must_use]
pub fn diff_words(before: &[u8], base: u32) -> Vec<usize> {
    let mut out = Vec::new();
    for (i, chunk) in before.chunks_exact(4).enumerate() {
        let now = get_word(base, (i * 4) as u32);
        if now != u32::from_le_bytes(chunk.try_into().unwrap()) {
            out.push(i * 4);
        }
    }
    out
}

/// Copies the live image words back into a snapshot.
pub fn snap(base: u32, len: usize) -> Vec<u8> {
    let mut out = vec![0u8; len];
    for (i, chunk) in out.chunks_exact_mut(4).enumerate() {
        chunk.copy_from_slice(&get_word(base, (i * 4) as u32).to_le_bytes());
    }
    out
}

/// Float edge corpus: zeros, ones, infinities, NaNs with payloads,
/// subnormals, conversion boundaries, plus seeded random bits.
pub fn float_corpus(rng: &mut Rng, random: usize) -> Vec<f32> {
    let mut out = vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        0.5,
        -0.5,
        2.0,
        17.0,
        100.0,
        1000.0,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::from_bits(0x7fc0_0001),
        f32::from_bits(0xffc0_0002),
        f32::from_bits(0x7f80_0001),
        f32::from_bits(0x0000_0001),
        f32::from_bits(0x8000_0001),
        f32::from_bits(0x007f_ffff),
        2147483648.0,
        -2147483648.0,
        2147483647.0,
        9223372036854775808.0,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
    ];
    for _ in 0..random {
        out.push(rng.f32());
    }
    out
}

/// Unsigned edge corpus: zeros, ones, sign boundary, top values, markers.
pub fn int_corpus(rng: &mut Rng, random: usize) -> Vec<u32> {
    let mut out = vec![
        0,
        1,
        2,
        5,
        6,
        0x0e,
        100,
        0xff,
        0x100,
        0x7fff_ffff,
        0x8000_0000,
        0xffff_ff00,
        0xffff_ffff,
    ];
    for _ in 0..random {
        out.push(rng.u32());
    }
    out
}
