//! Differential cases: every lifted function against its verified rewrite.
//!
//! One case per record in `lf_lift::registry::LIFTED` (the last test checks
//! that). Each case generates inputs, runs the rewrite and the lift, and
//! compares results, recorded calls and, for the slot table, state; then it
//! runs a deliberately wrong lift that must be caught (see
//! `lf_lift_diff::check`).
//!
//! Runs on the 32-bit target against the real checker runtime, and on any
//! other host against the stand-in, except the one case marked 32-bit only.

// Inputs and expected values are built from words of every width on
// purpose; the casts are the boundary narrowings the cases document.
// Rewrites are wrapped in closures so one case reads the same on both
// targets (on the 32-bit target they are `extern` functions).
#![allow(
    clippy::redundant_closure,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::too_many_lines
)]

use core::cell::RefCell;
use core::fmt;
use core::ops::Range;

use lf_core::boundary::{AddressMap, element_addr};
use lf_core::{Arena, Handle};
use lf_lift::forward::{
    self, Effect, EffectBase, Gates, InsertionSort, IntroSort, Lifecycle, Probe, RankLookup,
};
use lf_lift::pure;
use lf_lift::registry::LIFTED;
use lf_lift::slot_table::{
    self, ENTRY_COUNT, Entry, KEY_COUNT, KeyEntry, SlotTableOps, SlotTableState, layout,
};
use lf_lift_diff::rewrites as rw;
use lf_lift_diff::rt::{self, Call, Sig};
use lf_lift_diff::{F32, F64, Rng, VaImage, check, inputs};

/// Inputs per case.
const N: usize = 4000;

/// Every word below 0x140, then `N` generated words.
fn words(seed: u64) -> Vec<u32> {
    let mut out: Vec<u32> = (0..0x140).collect();
    out.extend(inputs(seed, N, Rng::edge_u32));
    out
}

/// Every byte value, with and without junk in the upper bits.
fn bytes_with_junk(seed: u64) -> Vec<u32> {
    let mut rng = Rng::new(seed);
    let mut out: Vec<u32> = (0..0x100).collect();
    out.extend((0..0x100).map(|b| b | (rng.u32() & 0xFFFF_FF00)));
    out
}

fn pairs(seed: u64) -> Vec<(u32, u32)> {
    inputs(seed, N, |r| {
        let a = r.edge_u32();
        let b = if r.one_in(4) { a } else { r.edge_u32() };
        (a, b)
    })
}

/// A float that is small and ordinary half the time, an edge the rest.
fn small_f32(r: &mut Rng) -> f32 {
    if r.one_in(2) {
        r.below(9) as f32 * 0.5 - 2.0
    } else {
        r.edge_f32()
    }
}

// ---------------------------------------------------------------------
// Pure functions.
// ---------------------------------------------------------------------

#[test]
fn rw_0088bcd0_scaled_product_floor() {
    check(
        "rw_0088bcd0",
        &pairs(1),
        |&(a, b)| rw::rw_0088bcd0(a, b),
        |&(a, b)| pure::scaled_product_floor(a, b),
        // Plausible mistake: rounding to nearest instead of flooring.
        |&(a, b)| ((a as f32) * (b as f32) * pure::MILLI).round() as i64 as u32,
    );
}

#[test]
fn rw_008a6a00_round_half_away() {
    check(
        "rw_008a6a00",
        &inputs(2, N, Rng::edge_f32),
        |&x| rw::rw_008a6a00(x),
        |&x| pure::round_half_away(x),
        // Plausible mistake: Rust's saturating conversion.
        |&x| {
            if x < 0.0 {
                (x - 0.5) as i32
            } else {
                (x + 0.5) as i32
            }
        },
    );
}

#[test]
fn rw_008aadb0_lerp_clamped() {
    let ins = inputs(3, N, |r| {
        (
            small_f32(r),
            small_f32(r),
            small_f32(r),
            small_f32(r),
            small_f32(r),
        )
    });
    check(
        "rw_008aadb0",
        &ins,
        |&(a, b, c, d, e)| F32(rw::rw_008aadb0(a, b, c, d, e)),
        |&(a, b, c, d, e)| F32(pure::lerp_clamped(a, b, c, d, e)),
        // Plausible mistake: the textbook blend, which rounds differently.
        |&(a, b, c, d, e)| {
            let t = pure::lerp_clamped(0.0, 1.0, c, d, e);
            F32(b * t + a * (1.0 - t))
        },
    );
}

#[test]
fn rw_008d5180_code_for_selector() {
    check(
        "rw_008D5180",
        &words(4),
        |&s| rw::rw_008D5180(s),
        |&s| pure::code_for_selector(s),
        |&s| {
            if s == 5 {
                0x20
            } else {
                pure::code_for_selector(s)
            }
        },
    );
}

#[test]
fn rw_008d70e0_clamp_map() {
    let ins = inputs(5, N, |r| {
        (
            small_f32(r),
            small_f32(r),
            small_f32(r),
            small_f32(r),
            small_f32(r),
        )
    });
    check(
        "rw_008d70e0",
        &ins,
        |&(p, q, r, s, t)| F32(rw::rw_008d70e0(p, q, r, s, t)),
        |&(p, q, r, s, t)| F32(pure::clamp_map(p, q, r, s, t)),
        // Plausible mistake: multiplying before dividing.
        |&(p, q, r, s, t)| {
            if q > p {
                F32(s)
            } else if p > r {
                F32(t)
            } else {
                F32((t - s) * (p - q) / (r - q) + s)
            }
        },
    );
}

#[test]
fn rw_0091b3c0_map_char_byte() {
    check(
        "rw_0091b3c0",
        &bytes_with_junk(6),
        |&arg| {
            let out = rw::rw_0091b3c0(arg);
            // Pin the residue the lift narrows away.
            let residue = if (arg & 0xFF) < 0x8E { 0x00FF_FFFF } else { 0 };
            assert_eq!(out >> 8, residue, "residue shape for {arg:#x}");
            out as u8
        },
        |&arg| pure::map_char_byte(arg as u8),
        |&arg| {
            if arg as u8 == 0xD3 {
                0xB8
            } else {
                pure::map_char_byte(arg as u8)
            }
        },
    );
}

#[test]
fn rw_00925e50_input_buffer_size() {
    check(
        "rw_00925E50",
        &words(7),
        |&c| rw::rw_00925E50(c),
        |&c| pure::input_buffer_size(c),
        |&c| {
            if c == 0x100 {
                0
            } else {
                pure::input_buffer_size(c)
            }
        },
    );
}

#[test]
fn rw_009253f0_input_flag_class() {
    const BITS: [u32; 8] = [0, 1, 0x100, 0x400, 0x500, 0x101, 0x401, 0xFFFF_FFFF];
    let ins = inputs(8, N, |r| {
        let m = if r.one_in(4) { r.u32() } else { r.pick(&BITS) };
        let v = if r.one_in(4) { r.u32() } else { r.pick(&BITS) };
        (m, v)
    });
    check(
        "rw_009253F0",
        &ins,
        |&(m, v)| rw::rw_009253F0(m, v),
        |&(m, v)| pure::input_flag_class(m, v),
        |&(m, v)| {
            if m & 0x100 == 0 && v & 0x500 != 0 {
                1
            } else {
                pure::input_flag_class(m, v)
            }
        },
    );
}

#[test]
fn rw_0094b8c0_bank_slot_index() {
    let ins = inputs(9, N, |r| (r.edge_u32(), r.edge_u32(), r.edge_u32()));
    check(
        "rw_0094b8c0",
        &ins,
        |&(obj, idx, alt)| rw::rw_0094b8c0(obj as usize as *const u8, idx, alt),
        |&(obj, idx, alt)| {
            element_addr(
                obj,
                pure::bank_slot_index(idx, alt as u8 != 0),
                pure::BANK_SLOT_STRIDE,
            )
        },
        // Plausible mistake: testing the whole bank word.
        |&(obj, idx, alt)| {
            element_addr(
                obj,
                pure::bank_slot_index(idx, alt != 0),
                pure::BANK_SLOT_STRIDE,
            )
        },
    );
}

#[test]
fn rw_00952630_size_class() {
    check(
        "rw_00952630",
        &words(10),
        |&c| rw::rw_00952630(c),
        |&c| pure::size_class(c as i32),
        // Plausible mistake: comparing unsigned.
        |&c| {
            if c >= 0x780 {
                0x3000
            } else {
                pure::size_class(c as i32)
            }
        },
    );
}

#[test]
fn rw_009529e0_kind_flag_bit() {
    check(
        "rw_009529e0",
        &words(11),
        |&k| rw::rw_009529e0(k),
        |&k| pure::kind_flag_bit(k),
        |&k| if k == 0x98 { 0 } else { pure::kind_flag_bit(k) },
    );
}

#[test]
fn rw_009532a0_record_size_for_tag() {
    check(
        "rw_009532a0",
        &bytes_with_junk(12),
        |&t| rw::rw_009532a0(t),
        |&t| pure::record_size_for_tag(t as u8),
        |&t| {
            if t as u8 == 0x8E {
                0x120
            } else {
                pure::record_size_for_tag(t as u8)
            }
        },
    );
}

#[test]
fn rw_009535d0_band_index() {
    check(
        "rw_009535d0",
        &words(13),
        |&c| rw::rw_009535d0(c),
        |&c| pure::band_index(c),
        |&c| if c == 0xE1 { 0 } else { pure::band_index(c) },
    );
}

#[test]
fn rw_00953640_bucket_limit() {
    check(
        "rw_00953640",
        &words(14),
        |&b| rw::rw_00953640(b),
        |&b| pure::bucket_limit(b),
        |&b| if b == 8 { 0x64 } else { pure::bucket_limit(b) },
    );
}

#[test]
fn rw_0097b490_audio_channel_bucket() {
    check(
        "rw_0097b490",
        &words(15),
        |&i| rw::rw_0097b490(i),
        |&i| pure::audio_channel_bucket(i),
        |&i| {
            if i == 16 {
                1
            } else {
                pure::audio_channel_bucket(i)
            }
        },
    );
}

#[test]
fn rw_009b7600_record_stride() {
    check(
        "rw_009b7600",
        &pairs(16),
        |&(t, i)| rw::rw_009b7600(t, i),
        |&(t, i)| element_addr(t, i, pure::RECORD_0X84_STRIDE),
        |&(t, i)| element_addr(t, i, 0x80),
    );
}

#[test]
fn rw_009f62c0_code_to_float() {
    check(
        "rw_009f62c0",
        &words(17),
        |&c| F32(rw::rw_009f62c0(c)),
        |&c| F32(pure::code_to_float(c)),
        |&c| F32(if c == 7 { 60.0 } else { pure::code_to_float(c) }),
    );
}

#[test]
fn rw_00a71cf0_is_kind_4_to_6() {
    check(
        "rw_00a71cf0",
        &words(18),
        |&k| rw::rw_00a71cf0(k),
        |&k| u8::from(pure::is_kind_4_to_6(k)),
        |&k| u8::from(matches!(k, 3..=6)),
    );
}

#[test]
fn rw_00ab6f50_ids_match_or_null() {
    check(
        "rw_00ab6f50",
        &pairs(19),
        |&(a, b)| rw::rw_00ab6f50(a, b),
        |&(a, b)| u32::from(pure::ids_match_or_null(a, b)),
        |&(a, b)| u32::from(a == b || a == 0),
    );
}

#[test]
fn rw_00b31650_task_rate() {
    check(
        "rw_00b31650",
        &words(20),
        |&k| F32(rw::rw_00b31650(k)),
        |&k| F32(pure::task_rate(k)),
        |&k| F32(if k == 21 { 5.0 } else { pure::task_rate(k) }),
    );
}

#[test]
fn rw_00b79210_float_band() {
    check(
        "rw_00b79210",
        &inputs(21, N, small_f32),
        |&x| rw::rw_00b79210(x),
        |&x| pure::float_band(x),
        |&x| {
            if (0.0..1.5).contains(&x) {
                2
            } else {
                pure::float_band(x)
            }
        },
    );
}

#[test]
fn rw_00b79250_index_to_float() {
    check(
        "rw_00b79250",
        &words(22),
        |&i| F32(rw::rw_00b79250(i)),
        |&i| F32(pure::index_to_float(i)),
        |&i| F32(if i == 4 { 4.0 } else { pure::index_to_float(i) }),
    );
}

#[test]
fn rw_00be81d0_pair_rejected() {
    let ins = inputs(23, N, |r| {
        let a = if r.one_in(8) { r.u32() } else { r.below(6) };
        let b = if r.one_in(8) { r.u32() } else { r.below(6) };
        (a, b)
    });
    check(
        "rw_00BE81D0",
        &ins,
        |&(a, b)| rw::rw_00BE81D0(a, b),
        |&(a, b)| u32::from(pure::pair_rejected(a, b)),
        |&(a, b)| {
            u32::from(if matches!(a, 3 | 4) {
                b > 3
            } else {
                pure::pair_rejected(a, b)
            })
        },
    );
}

#[test]
fn rw_00d38c10_selector_code() {
    check(
        "rw_00d38c10",
        &words(24),
        |&s| rw::rw_00d38c10(s),
        |&s| pure::selector_code(s),
        |&s| if s == 8 { 0x1E } else { pure::selector_code(s) },
    );
}

#[test]
fn rw_00d740a0_render_mode_is_active() {
    check(
        "rw_00d740a0",
        &words(25),
        |&m| rw::rw_00d740a0(m),
        |&m| u32::from(pure::render_mode_is_active(m)),
        |&m| u32::from(matches!(m, 0 | 1 | 4)),
    );
}

#[test]
fn rw_00d740e0_render_mode_is_shadow() {
    check(
        "rw_00d740e0",
        &words(26),
        |&m| rw::rw_00d740e0(m),
        |&m| u32::from(pure::render_mode_is_shadow(m)),
        |&m| u32::from(matches!(m, 8..=11)),
    );
}

// ---------------------------------------------------------------------
// Forwarders.
// ---------------------------------------------------------------------

/// Marker for the objects the destructor family destroys.
struct Obj;

/// One object at `addr`, as the lift sees it: an arena handle bound to it.
fn bind_one<T>(addr: u32) -> (Handle<T>, AddressMap<T>) {
    let mut arena: Arena<()> = Arena::new();
    let handle = arena.insert(()).cast::<T>();
    let mut map = AddressMap::new();
    map.bind(addr, handle).expect("non-null address");
    (handle, map)
}

struct LifeFake<'a> {
    map: &'a AddressMap<Obj>,
}

impl Lifecycle<Obj> for LifeFake<'_> {
    fn destruct(&mut self, object: Handle<Obj>) {
        rt::record(1, &[self.map.addr_of(object).expect("bound")]);
    }
    fn release(&mut self, object: Handle<Obj>) {
        rt::record(2, &[self.map.addr_of(object).expect("bound")]);
    }
}

#[test]
fn deleting_destructors() {
    type Rewrite = fn(u32, u32) -> u32;
    let family: [(&str, Rewrite); 7] = [
        ("rw_009815f0", |t, f| rw::rw_009815f0(t, f)),
        ("rw_009856a0", |t, f| rw::rw_009856a0(t, f)),
        ("rw_00add1a0", |t, f| rw::rw_00add1a0(t, f)),
        ("rw_00add1c0", |t, f| rw::rw_00add1c0(t, f)),
        ("rw_00c6e1c0", |t, f| rw::rw_00c6e1c0(t, f)),
        ("rw_00ca4e40", |t, f| rw::rw_00ca4e40(t, f)),
        ("rw_00d8c690", |t, f| rw::rw_00d8c690(t, f)),
    ];
    let _s = rt::session();
    rt::set_stubs(&[(1, Sig::This1), (2, Sig::Cdecl1)]);
    let ins = inputs(30, N, |r| (r.edge_u32().max(1), r.edge_u32()));
    let lifted = |this: u32, release: bool| {
        let (h, map) = bind_one::<Obj>(this);
        rt::capture(|| {
            let out = forward::deleting_destructor(&mut LifeFake { map: &map }, h, release);
            map.addr_of(out).expect("bound")
        })
    };
    for (name, rewrite) in family {
        check(
            name,
            &ins,
            |&(this, flags)| rt::capture(|| rewrite(this, flags)),
            |&(this, flags)| lifted(this, flags & 1 != 0),
            // Plausible mistake: the wrong flag bit.
            |&(this, flags)| lifted(this, flags & 2 != 0),
        );
    }
}

/// Element ranges translated back to the rewrite's calls.
struct SortFake {
    first: u32,
    size: u32,
    guarded_slot: u32,
    unguarded_slot: u32,
    guarded_extra: u32,
    unguarded_extra: u32,
}

impl SortFake {
    fn addr(&self, index: usize) -> u32 {
        let index = u32::try_from(index).expect("small index");
        self.first.wrapping_add(index.wrapping_mul(self.size))
    }
    fn ends(&self, range: &Range<usize>) -> (u32, u32) {
        (self.addr(range.start), self.addr(range.end))
    }
}

impl InsertionSort for SortFake {
    fn insertion_sort(&mut self, range: Range<usize>) {
        let (a, b) = self.ends(&range);
        rt::record(self.guarded_slot, &[a, b, 0, self.guarded_extra]);
    }
    fn unguarded_insertion_sort(&mut self, range: Range<usize>) {
        let (a, b) = self.ends(&range);
        rt::record(self.unguarded_slot, &[a, b, 0, self.unguarded_extra]);
    }
}

/// A range input: start address, element count, comparator word.
fn range_inputs(seed: u64, size: u32) -> Vec<(u32, u32, u32)> {
    let max_len = 0x7FFF_FFFF / size;
    inputs(seed, N / 4, |r| {
        let first = r.below(0x4000_0000);
        let len = match r.below(3) {
            0 => r.pick(&[0, 1, 2, 15, 16, 17, 18, 31, 32]),
            1 => r.below(2000),
            _ => max_len - r.below(4),
        };
        (first, len, r.u32())
    })
}

#[test]
fn final_insertion_sorts() {
    struct Member {
        name: &'static str,
        rewrite: fn(u32, u32, u32),
        size: u32,
        guarded: u32,
        unguarded: u32,
        tail_extra_is_zero: bool,
    }
    let family = [
        Member {
            name: "rw_00abbda0",
            rewrite: |a, b, c| {
                rw::rw_00abbda0(a, b, c);
            },
            size: 4,
            guarded: 1,
            unguarded: 2,
            tail_extra_is_zero: false,
        },
        Member {
            name: "rw_00ade7a0",
            rewrite: |a, b, c| {
                rw::rw_00ade7a0(a, b, c, 0);
            },
            size: 4,
            guarded: 2,
            unguarded: 3,
            tail_extra_is_zero: false,
        },
        Member {
            name: "rw_00b05100",
            rewrite: |a, b, c| {
                rw::rw_00b05100(a, b, c);
            },
            size: 8,
            guarded: 1,
            unguarded: 2,
            tail_extra_is_zero: true,
        },
        Member {
            name: "rw_00b33d00",
            rewrite: |a, b, c| {
                rw::rw_00b33d00(a, b, c);
            },
            size: 28,
            guarded: 1,
            unguarded: 2,
            tail_extra_is_zero: false,
        },
        Member {
            name: "rw_00b33d70",
            rewrite: |a, b, c| {
                rw::rw_00b33d70(a, b, c);
            },
            size: 28,
            guarded: 1,
            unguarded: 2,
            tail_extra_is_zero: false,
        },
        Member {
            name: "rw_00b33de0",
            rewrite: |a, b, c| {
                rw::rw_00b33de0(a, b, c);
            },
            size: 16,
            guarded: 1,
            unguarded: 2,
            tail_extra_is_zero: false,
        },
        Member {
            name: "rw_00c6dbb0",
            rewrite: |a, b, c| rw::rw_00c6dbb0(a, b, c),
            size: 8,
            guarded: 1,
            unguarded: 2,
            tail_extra_is_zero: false,
        },
    ];
    let _s = rt::session();
    for (seed, m) in (40..).zip(&family) {
        rt::set_stubs(&[(m.guarded, Sig::Cdecl4), (m.unguarded, Sig::Cdecl4)]);
        let fake = |first: u32, extra: u32| SortFake {
            first,
            size: m.size,
            guarded_slot: m.guarded,
            unguarded_slot: m.unguarded,
            guarded_extra: extra,
            unguarded_extra: if m.tail_extra_is_zero { 0 } else { extra },
        };
        check(
            m.name,
            &range_inputs(seed, m.size),
            |&(first, len, extra)| {
                let last = first + len * m.size;
                rt::capture(|| (m.rewrite)(first, last, extra)).1
            },
            |&(first, len, extra)| {
                let mut sorter = fake(first, extra);
                rt::capture(|| forward::final_insertion_sort(&mut sorter, len as usize)).1
            },
            // Plausible mistake: splitting at fifteen.
            |&(first, len, extra)| {
                let mut s = fake(first, extra);
                let len = len as usize;
                rt::capture(|| {
                    if len > 15 {
                        s.insertion_sort(0..15);
                        s.unguarded_insertion_sort(15..len);
                    } else {
                        s.insertion_sort(0..len);
                    }
                })
                .1
            },
        );
    }
}

struct IntroFake {
    first: u32,
    comparator: u32,
}

impl IntroFake {
    fn ends(&self, range: &Range<usize>) -> (u32, u32) {
        let at = |i: usize| self.first + 4 * u32::try_from(i).expect("small index");
        (at(range.start), at(range.end))
    }
}

impl IntroSort for IntroFake {
    fn introsort_loop(&mut self, range: Range<usize>, depth_budget: u32) {
        let (a, b) = self.ends(&range);
        rt::record(2, &[a, b, 0, depth_budget, self.comparator]);
    }
    fn final_insertion_sort(&mut self, range: Range<usize>) {
        let (a, b) = self.ends(&range);
        rt::record(3, &[a, b, self.comparator]);
    }
}

#[test]
fn rw_00adebd0_sort() {
    let _s = rt::session();
    rt::set_stubs(&[(2, Sig::Cdecl5), (3, Sig::Cdecl3)]);
    let ins = inputs(50, N, |r| {
        let len = if r.one_in(2) {
            r.pick(&[0, 1, 2, 3, 4, 5, 7, 8, 9, 1000, 1024])
        } else {
            r.below(0x1000_0000)
        };
        (r.below(0x4000_0000), len, r.u32())
    });
    check(
        "rw_00adebd0",
        &ins,
        |&(first, len, comparator)| {
            rt::capture(|| rw::rw_00adebd0(first, first + 4 * len, comparator)).1
        },
        |&(first, len, comparator)| {
            let mut s = IntroFake { first, comparator };
            rt::capture(|| forward::sort(&mut s, len as usize)).1
        },
        // Plausible mistake: a depth budget of log2 instead of twice it.
        |&(first, len, comparator)| {
            let mut s = IntroFake { first, comparator };
            let len = len as usize;
            rt::capture(|| {
                if len > 0 {
                    s.introsort_loop(0..len, len.ilog2());
                    s.final_insertion_sort(0..len);
                }
            })
            .1
        },
    );
}

/// A scripted answer for `slot` that depends only on `seed`: zero, a word
/// whose low byte is zero, small values or random bits.
fn scripted(seed: u64, slot: u32) -> u32 {
    let mut r = Rng::new(seed ^ (u64::from(slot) << 40));
    match r.below(6) {
        0 => 0,
        1 => 0x100 * (1 + r.below(0xFF)),
        2 => r.below(4),
        3 => 0xFF,
        4 => 0x101,
        _ => r.u32(),
    }
}

struct GateFake;

impl Gates for GateFake {
    fn first(&mut self, subject: u32) -> bool {
        rt::record(1, &[subject]) as u8 != 0
    }
    fn second(&mut self) -> bool {
        rt::record(2, &[]) as u8 != 0
    }
    fn third(&mut self) -> bool {
        rt::record(3, &[]) as u8 != 0
    }
}

#[test]
fn rw_00a72820_any_gate_open() {
    let _s = rt::session();
    rt::set_stubs(&[(1, Sig::Cdecl1), (2, Sig::Cdecl0), (3, Sig::Cdecl0)]);
    let ins = inputs(60, N, |r| (r.edge_u32(), r.next_u64()));
    let answers = |seed: u64| rt::set_answers(move |slot, _| scripted(seed, slot));
    check(
        "rw_00a72820",
        &ins,
        |&(arg, seed)| {
            answers(seed);
            rt::capture(|| u32::from(rw::rw_00a72820(arg)))
        },
        |&(arg, seed)| {
            answers(seed);
            rt::capture(|| u32::from(forward::any_gate_open(&mut GateFake, arg)))
        },
        // Plausible mistake: testing the first gate's whole word.
        |&(arg, seed)| {
            answers(seed);
            rt::capture(|| {
                u32::from(rt::record(1, &[arg]) != 0 || GateFake.second() || GateFake.third())
            })
        },
    );
}

struct RankFake;

impl RankLookup for RankFake {
    fn primary(&mut self, a: u32, b: u32) -> u8 {
        rt::record(1, &[a, b]) as u8
    }
    fn secondary(&mut self, a: u32, b: u32, level: i32) -> u8 {
        rt::record(2, &[a, b, level as u32]) as u8
    }
}

#[test]
fn rw_00aba180_dual_rank_test() {
    let _s = rt::session();
    rt::set_stubs(&[(1, Sig::Cdecl2), (2, Sig::Cdecl3)]);
    let ins = inputs(61, N, |r| {
        let (rc, rd) = (r.u32(), r.u32());
        let c = r.pick(&[u32::MAX, 0, 1, 2, 5, 0x7F, 0x80, 0xFF, 0x100, rc]);
        let d = r.pick(&[0, u32::MAX, 1, 3, 0x100, rd]);
        (r.edge_u32(), r.edge_u32(), c, d, r.next_u64())
    });
    let answers = |seed: u64| rt::set_answers(move |slot, _| scripted(seed, slot));
    check(
        "rw_00aba180",
        &ins,
        |&(a, b, c, d, seed)| {
            answers(seed);
            rt::capture(|| rw::rw_00aba180(a, b, c, d))
        },
        |&(a, b, c, d, seed)| {
            answers(seed);
            rt::capture(|| {
                u32::from(forward::dual_rank_test(
                    &mut RankFake,
                    a,
                    b,
                    c as i32,
                    d as i32,
                ))
            })
        },
        // Plausible mistake: a strict comparison against the primary rank.
        |&(a, b, c, d, seed)| {
            answers(seed);
            rt::capture(|| {
                let (level, sub) = (c as i32, d as i32);
                u32::from(if level == -1 && sub == 0 {
                    true
                } else if level > i32::from(RankFake.primary(a, b)) {
                    false
                } else {
                    sub < i32::from(RankFake.secondary(a, b, level))
                })
            })
        },
    );
}

struct ProbeFake;

impl Probe for ProbeFake {
    fn probe(&mut self, subject: u32) -> u32 {
        rt::record(1, &[subject])
    }
}

#[test]
fn rw_009a3ea0_probe_nonzero() {
    let _s = rt::session();
    rt::set_stubs(&[(1, Sig::Cdecl1)]);
    let ins = inputs(62, N, |r| (r.edge_u32(), r.next_u64()));
    let answers = |seed: u64| rt::set_answers(move |slot, _| scripted(seed, slot));
    check(
        "rw_009a3ea0",
        &ins,
        |&(arg, seed)| {
            answers(seed);
            rt::capture(|| rw::rw_009a3ea0(arg))
        },
        |&(arg, seed)| {
            answers(seed);
            rt::capture(|| u32::from(forward::probe_nonzero(&mut ProbeFake, arg)))
        },
        // Plausible mistake: testing only the low byte.
        |&(arg, seed)| {
            answers(seed);
            rt::capture(|| u32::from(ProbeFake.probe(arg) & 0xFF != 0))
        },
    );
}

struct EffectFake<'a> {
    map: &'a AddressMap<Effect>,
}

impl EffectBase for EffectFake<'_> {
    fn process(&mut self, effect: Handle<Effect>, a: u32, b: u32) -> bool {
        let this = self.map.addr_of(effect).expect("bound");
        // Inferred: the base step answers a boolean in its low byte.
        rt::record(1, &[this, a, b]) & 0xFF != 0
    }
}

#[test]
fn rw_008ac690_effect_process() {
    let _s = rt::session();
    rt::set_stubs(&[(1, Sig::This3)]);
    let ins = inputs(63, N, |r| {
        (
            r.edge_u32().max(1),
            r.edge_u32(),
            r.edge_u32(),
            r.next_u64(),
        )
    });
    let answers = |seed: u64| rt::set_answers(move |slot, _| scripted(seed, slot));
    check(
        "rw_008ac690",
        &ins,
        |&(this, a, b, seed)| {
            answers(seed);
            let (out, calls) = rt::capture(|| rw::rw_008ac690(this, a, b));
            // Pin the residue the lift narrows away.
            assert_eq!(
                out & !0xFF,
                scripted(seed, 1) & !0xFF,
                "residue for seed {seed}"
            );
            (out & 0xFF, calls)
        },
        |&(this, a, b, seed)| {
            answers(seed);
            let (h, map) = bind_one::<Effect>(this);
            rt::capture(|| {
                u32::from(forward::effect_process(
                    &mut EffectFake { map: &map },
                    h,
                    a,
                    b,
                ))
            })
        },
        // Plausible mistake: testing the whole answer word.
        |&(this, a, b, seed)| {
            answers(seed);
            rt::capture(|| u32::from(rt::record(1, &[this, a, b]) != 0))
        },
    );
}

// ---------------------------------------------------------------------
// The slot table (globals).
// ---------------------------------------------------------------------

/// The slot table's callees, translated back to the rewrites' calls.
struct SlotFake;

impl SlotTableOps for SlotFake {
    fn next_serial(&mut self, _st: &mut SlotTableState) -> u16 {
        rt::record(1, &[]) as u16
    }
    fn current_stamp(&mut self, _st: &mut SlotTableState) -> u32 {
        rt::record(1, &[])
    }
    fn refresh_entry(&mut self, _st: &mut SlotTableState, index: u16) {
        rt::record(1, &[rt::relocated(layout::entry_addr(index))]);
    }
}

/// One slot-table input: a whole starting state, an argument word and the
/// seed both were generated from (printed instead of the state on failure).
struct SlotInput {
    seed: u64,
    state: SlotTableState,
    arg: u32,
}

impl fmt::Debug for SlotInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SlotInput {{ seed: {:#x}, arg: {:#x} }}",
            self.seed, self.arg
        )
    }
}

/// A random starting state that reaches every branch of the cluster.
fn slot_state(r: &mut Rng) -> SlotTableState {
    let mut st = SlotTableState::new(r.edge_f32());
    let density = r.pick(&[0, 50, 99, 100, 100]);
    for (used, entry) in st.used.iter_mut().zip(st.entries.iter_mut()) {
        *used = r.below(100) < density;
        let flags = if r.one_in(2) { 0 } else { r.u32() as u8 };
        *entry = Entry {
            value: r.u32(),
            flags,
        };
    }
    if density == 100 && r.one_in(2) {
        st.used[r.below(ENTRY_COUNT as u32) as usize] = false;
    }
    let any_cursor = r.below(1501);
    st.cursor = r.pick(&[0, 1, 1499, 1500, any_cursor]) as u16;
    let serials = [0, 1, 0x7FFD, 0x7FFE, 0x7FFF, 0xFFFF, r.u32() as u16];
    st.serial_a = r.pick(&serials);
    st.serial_b = r.pick(&serials);
    st.clock_base = r.edge_u32();
    st.clock_mark = if r.one_in(2) {
        st.clock_base.wrapping_add(r.below(100_000))
    } else {
        r.edge_u32()
    };
    st.clock_ticks = r.edge_u32();
    st.stamp_base = r.edge_u32();
    st.pending_scale = r.edge_f32();
    // A few keys repeat so first-match order matters.
    let pool = [r.u32(), r.u32(), r.u32(), 7];
    for key in st.keys.iter_mut() {
        let k = match r.below(4) {
            0 | 1 => None,
            2 => Some(r.pick(&pool)),
            _ => Some(r.u32()).filter(|&k| k != u32::MAX),
        };
        *key = KeyEntry {
            key: k,
            value: r.u32(),
        };
    }
    st
}

fn slot_inputs(
    seed: u64,
    n: usize,
    mut arg: impl FnMut(&mut Rng, &SlotTableState) -> u32,
) -> Vec<SlotInput> {
    let mut r = Rng::new(seed);
    (0..n)
        .map(|_| {
            let input_seed = r.next_u64();
            let mut g = Rng::new(input_seed);
            let state = slot_state(&mut g);
            let arg = arg(&mut g, &state);
            SlotInput {
                seed: input_seed,
                state,
                arg,
            }
        })
        .collect()
}

/// An entry index: in range, at the edges, past the table, or with junk in
/// the upper half.
fn entry_arg(r: &mut Rng, _st: &SlotTableState) -> u32 {
    let (inside, any) = (r.below(1500), r.u32());
    r.pick(&[0, 1499, 1500, 0xFFFF, 0x1_0005, inside, any])
}

/// A key: one the table holds, one it may not, or the empty marker.
fn key_arg(r: &mut Rng, st: &SlotTableState) -> u32 {
    match r.below(3) {
        0 => st.keys[r.below(KEY_COUNT as u32) as usize].key.unwrap_or(7),
        1 => u32::MAX,
        _ => {
            let any = r.u32();
            r.pick(&[7, any])
        }
    }
}

fn no_arg(_r: &mut Rng, _st: &SlotTableState) -> u32 {
    0
}

/// What one run of a slot-table function did.
#[derive(PartialEq)]
struct Outcome<O> {
    ret: O,
    calls: Vec<Call>,
    /// The modelled regions after the run.
    regions: Vec<u8>,
    /// True when the whole image equals the reference run's whole image
    /// (catches writes outside the modelled regions).
    whole_image_agrees: bool,
}

impl<O: fmt::Debug> fmt::Debug for Outcome<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sum = self.regions.iter().fold(0u64, |h, &b| {
            h.wrapping_mul(0x100_0000_01B3).wrapping_add(u64::from(b))
        });
        write!(
            f,
            "Outcome {{ ret: {:?}, calls: {:?}, regions: {:#018x}, whole image agrees: {} }}",
            self.ret, self.calls, sum, self.whole_image_agrees
        )
    }
}

type SlotLift<O> = fn(&mut SlotTableState, &mut SlotFake, u32) -> O;

/// Runs one slot-table case through [`check`].
///
/// The reference and the lift each have an image, filled with the same
/// pattern (memory nothing models). Each run stores the input state over
/// its image, scripts the stamp callee from the input's seed (equal to the
/// mark clock half the time), and runs. The lift's resulting state is
/// stored back over its image, and the two images are compared over the
/// modelled regions and in whole. `check` always runs the reference for an
/// input immediately before the lift (or the wrong lift) for it, which the
/// whole-image comparison relies on.
fn run_slot<O: PartialEq + fmt::Debug>(
    name: &str,
    ins: &[SlotInput],
    stubs: &[(u32, Sig)],
    reference: impl Fn(u32) -> O,
    lifted: SlotLift<O>,
    wrong: SlotLift<O>,
) {
    let _s = rt::session();
    rt::set_stubs(stubs);
    let base = layout::span_start() & !7;
    let len = (layout::span_end() - base) as usize;
    let mut pattern = VaImage::new(base, len);
    pattern.fill_pattern(0x5EED);
    let ref_image = RefCell::new(pattern.clone());
    let lift_image = RefCell::new(pattern);
    let answers = |input: &SlotInput| {
        let (seed, mark) = (input.seed, input.state.clock_mark);
        rt::set_answers(move |_, _| {
            let mut r = Rng::new(seed);
            if r.one_in(2) { mark } else { r.u32() }
        });
    };
    let run_lift = |input: &SlotInput, f: SlotLift<O>| {
        let mut image = lift_image.borrow_mut();
        layout::store(&input.state, &mut *image).expect("state fits the image");
        answers(input);
        let mut st = input.state.clone();
        // The lift runs with the image installed too, so the fake's
        // relocated addresses match the rewrite's.
        let (ret, calls) = rt::with_image(&mut image, || {
            rt::capture(|| f(&mut st, &mut SlotFake, input.arg))
        });
        layout::store(&st, &mut *image).expect("state fits the image");
        Outcome {
            ret,
            calls,
            regions: image.regions(&layout::REGIONS),
            whole_image_agrees: image.bytes() == ref_image.borrow().bytes(),
        }
    };
    check(
        name,
        ins,
        |input| {
            let mut image = ref_image.borrow_mut();
            layout::store(&input.state, &mut *image).expect("state fits the image");
            answers(input);
            let (ret, calls) = rt::with_image(&mut image, || rt::capture(|| reference(input.arg)));
            Outcome {
                ret,
                calls,
                regions: image.regions(&layout::REGIONS),
                whole_image_agrees: true,
            }
        },
        |input| run_lift(input, lifted),
        |input| run_lift(input, wrong),
    );
}

/// Inputs per slot-table case (each carries a whole table).
const SLOT_N: usize = 150;

#[test]
fn rw_00952db0_bump_serial_a() {
    run_slot(
        "rw_00952db0",
        &slot_inputs(70, SLOT_N, no_arg),
        &[],
        |_| rw::rw_00952db0() & 0xFFFF,
        |st, _, _| u32::from(slot_table::bump_serial_a(st)),
        // Plausible mistake: wrapping at 0x8000.
        |st, _, _| {
            let next = st.serial_a.wrapping_add(1);
            st.serial_a = if next == 0x8000 { 0 } else { next };
            u32::from(st.serial_a)
        },
    );
}

#[test]
fn rw_00952e60_bump_serial_b() {
    run_slot(
        "rw_00952e60",
        &slot_inputs(71, SLOT_N, no_arg),
        &[],
        |_| rw::rw_00952e60() & 0xFFFF,
        |st, _, _| u32::from(slot_table::bump_serial_b(st)),
        // Plausible mistake: bumping the other counter.
        |st, _, _| u32::from(slot_table::bump_serial_a(st)),
    );
}

#[test]
fn rw_00952de0_allocate() {
    run_slot(
        "rw_00952de0",
        &slot_inputs(72, SLOT_N, no_arg),
        &[(1, Sig::Cdecl0)],
        |_| rw::rw_00952de0(),
        |st, ops, _| slot_table::allocate(st, ops).map_or(u32::MAX, slot_table::Ticket::pack),
        // Plausible mistake: no wrap-round scan from the start.
        |st, ops, _| {
            let cursor = usize::from(st.cursor);
            match (cursor..ENTRY_COUNT).find(|&i| !st.used[i]) {
                None => u32::MAX,
                Some(i) => {
                    st.used[i] = true;
                    st.cursor = i as u16 + 1;
                    let serial = ops.next_serial(st);
                    slot_table::Ticket {
                        index: i as u16,
                        serial,
                    }
                    .pack()
                }
            }
        },
    );
}

#[test]
fn rw_00953110_entry_value() {
    run_slot(
        "rw_00953110",
        &slot_inputs(73, SLOT_N, entry_arg),
        &[(1, Sig::This1)],
        |arg| rw::rw_00953110(arg),
        |st, ops, arg| slot_table::entry_value(st, ops, arg as u16),
        // Plausible mistake: checking the index before narrowing it.
        |st, ops, arg| {
            if arg > 0x5DB {
                0
            } else {
                slot_table::entry_value(st, ops, arg as u16)
            }
        },
    );
}

#[test]
fn rw_00953210_entry_row() {
    run_slot(
        "rw_00953210",
        &slot_inputs(74, SLOT_N, entry_arg),
        &[],
        |arg| rw::rw_00953210(arg),
        |_, _, arg| {
            slot_table::entry_row(arg as u16).map_or(0, |k| rt::relocated(layout::entry_addr(k)))
        },
        |_, _, arg| {
            let key = arg as u16;
            if key < 1499 {
                rt::relocated(layout::entry_addr(key))
            } else {
                0
            }
        },
    );
}

/// 32-bit only: this rewrite turns a relocated address into a pointer
/// itself, which the host stand-in cannot give it.
#[cfg(target_arch = "x86")]
#[test]
fn rw_00e63c90_clear_entries() {
    run_slot(
        "rw_00e63c90",
        &slot_inputs(75, SLOT_N, no_arg),
        &[],
        |_| rw::rw_00e63c90(),
        |st, _, _| {
            slot_table::clear_entries(st);
            rt::relocated(layout::ENTRIES + 1500 * layout::ENTRY_STRIDE)
        },
        // Plausible mistake: clearing the flag bytes only.
        |st, _, _| {
            st.entries.iter_mut().for_each(|e| e.flags = 0);
            rt::relocated(layout::ENTRIES + 1500 * layout::ENTRY_STRIDE)
        },
    );
}

#[test]
fn rw_00953900_clock_span() {
    run_slot(
        "rw_00953900",
        &slot_inputs(76, SLOT_N, no_arg),
        &[],
        |_| rw::rw_00953900(),
        |st, _, _| slot_table::clock_span(st),
        |st, _, _| st.clock_base.wrapping_sub(st.clock_mark),
    );
}

#[test]
fn rw_00953910_clock_span_seconds() {
    run_slot(
        "rw_00953910",
        &slot_inputs(77, SLOT_N, no_arg),
        &[],
        |_| F32(rw::rw_00953910()),
        |st, _, _| F32(slot_table::clock_span_seconds(st)),
        // Plausible mistake: dividing by a thousand, which rounds differently.
        |st, _, _| F32(slot_table::clock_span(st) as f32 / 1000.0),
    );
}

#[test]
fn rw_00952700_stamp_sum() {
    run_slot(
        "rw_00952700",
        &slot_inputs(78, SLOT_N, no_arg),
        &[],
        |_| rw::rw_00952700(),
        |st, _, _| slot_table::stamp_sum(st),
        |st, _, _| st.clock_ticks.wrapping_add(st.clock_mark),
    );
}

#[test]
fn rw_009526d0_scaled_ticks() {
    run_slot(
        "rw_009526d0",
        &slot_inputs(79, SLOT_N, no_arg),
        &[],
        |_| F64(rw::rw_009526d0()),
        |st, _, _| F64(slot_table::scaled_ticks(st)),
        // Plausible mistake: scaling in double precision.
        |st, _, _| F64(f64::from(st.clock_ticks) * f64::from(st.tick_scale)),
    );
}

#[test]
fn rw_00953160_take_pending_scale() {
    run_slot(
        "rw_00953160",
        &slot_inputs(80, SLOT_N, no_arg),
        &[],
        |_| F64(rw::rw_00953160()),
        |st, _, _| F64(slot_table::take_pending_scale(st)),
        // Plausible mistake: reading without clearing.
        |st, _, _| F64(f64::from(st.pending_scale)),
    );
}

#[test]
fn rw_009526b0_stamped_base() {
    run_slot(
        "rw_009526b0",
        &slot_inputs(81, SLOT_N, no_arg),
        &[(1, Sig::Cdecl0)],
        |_| rw::rw_009526b0(),
        |st, ops, _| slot_table::stamped_base(st, ops),
        // Plausible mistake: comparing against the other clock word.
        |st, ops, _| {
            let stamp = ops.current_stamp(st);
            if stamp == st.clock_base {
                st.stamp_base.wrapping_add(1)
            } else {
                st.stamp_base
            }
        },
    );
}

#[test]
fn rw_00952660_key_value() {
    run_slot(
        "rw_00952660",
        &slot_inputs(82, SLOT_N, key_arg),
        &[(1, Sig::Cdecl0)],
        |arg| rw::rw_00952660(arg),
        |st, ops, arg| slot_table::key_value(st, ops, arg),
        // Plausible mistake: the last match instead of the first.
        |st, ops, arg| match st.keys.iter().rev().find(|e| e.key == Some(arg)) {
            Some(e) => e.value,
            None => slot_table::stamped_base(st, ops),
        },
    );
}

// ---------------------------------------------------------------------
// Coverage.
// ---------------------------------------------------------------------

/// Every rewrite with a case above.
const CASES: [&str; 58] = [
    "rw_0088bcd0",
    "rw_008a6a00",
    "rw_008aadb0",
    "rw_008D5180",
    "rw_008d70e0",
    "rw_0091b3c0",
    "rw_00925E50",
    "rw_009253F0",
    "rw_0094b8c0",
    "rw_00952630",
    "rw_009529e0",
    "rw_009532a0",
    "rw_009535d0",
    "rw_00953640",
    "rw_0097b490",
    "rw_009b7600",
    "rw_009f62c0",
    "rw_00a71cf0",
    "rw_00ab6f50",
    "rw_00b31650",
    "rw_00b79210",
    "rw_00b79250",
    "rw_00BE81D0",
    "rw_00d38c10",
    "rw_00d740a0",
    "rw_00d740e0",
    "rw_009815f0",
    "rw_009856a0",
    "rw_00add1a0",
    "rw_00add1c0",
    "rw_00c6e1c0",
    "rw_00ca4e40",
    "rw_00d8c690",
    "rw_00abbda0",
    "rw_00ade7a0",
    "rw_00b05100",
    "rw_00b33d00",
    "rw_00b33d70",
    "rw_00b33de0",
    "rw_00c6dbb0",
    "rw_00adebd0",
    "rw_00a72820",
    "rw_00aba180",
    "rw_009a3ea0",
    "rw_008ac690",
    "rw_00952db0",
    "rw_00952e60",
    "rw_00952de0",
    "rw_00953110",
    "rw_00953210",
    "rw_00e63c90",
    "rw_00953900",
    "rw_00953910",
    "rw_00952700",
    "rw_009526d0",
    "rw_00953160",
    "rw_009526b0",
    "rw_00952660",
];

#[test]
fn every_lifted_record_has_a_case() {
    let mut registered: Vec<&str> = LIFTED.iter().map(|r| r.rewrite).collect();
    let mut cased: Vec<&str> = CASES.to_vec();
    registered.sort_unstable();
    cased.sort_unstable();
    assert_eq!(registered, cased);
}
