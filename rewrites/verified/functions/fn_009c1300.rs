// original: 0x009C1300 task_system_tick (proposed)

use lf_checker_rt::{callee_cdecl, callee_thiscall, global, relocated};

// 0x009C1300 task_system_tick (proposed)
// ---------------------------------------------------------------------------

const F1_TABLE0: u32 = 0x1292450;
const F1_STRIDE: u32 = 0xd0;
const F1_COUNT: u32 = 32;
const F1_G_ACTIVE: u32 = 0x11f7060;
const F1_G_A: u32 = 0x12088b4;
const F1_G_B: u32 = 0x0f1c040;
const F1_G_MODE: u32 = 0x1037720;
const F1_G_SHIFT: u32 = 0x1292440;
const F1_G_CLOCK: u32 = 0x11735b4;
const F1_G_MULIN: u32 = 0x11735bc;
const F1_C_MUL: u32 = 0xfe8c58;
const F1_DST: u32 = 0x110db70;
const F1_KIND_MASK: u32 = 0x3c0;
const F1_KIND_WANT: u32 = 0x0c0;

#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}


#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}

#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}


/// Per-tick update of the whole task system: no arguments.
///
/// The 32 task records live in a global table (stride `0xd0`, first record
/// at `F1_TABLE0`), each laid out like the single records the other batch
/// functions take: object at `+0x10`, key at `+0x14`, mode at `+0x18`,
/// other object at `+0x1c`, counter at `+0x24`, flag byte at `+0xcb`.
///
/// Behaviour: return unless the three global guards pass. Shift one global
/// into its neighbour and zero it. Then, per record: when the other object
/// is set and the object is null, run callee 1; when both are set and the
/// other's kind bits match, poll callee 2 and, on a zero answer, report the
/// record cursor through callee 3 and clear the other object. When the
/// object is set, run callee 4 (the `0x9c1640` handler), accumulate the
/// truncated product of a global and a constant into the counter when the
/// flag is set, then either report through callee 5 (clock past a nonzero
/// counter) or, when the counter is reached or still zero, dispatch on the
/// mode (2 runs callee 7, the `0x9c1940`
/// handler; 3 runs callee 6, the `0x9c1b50` handler). Finally run callee 8
/// (the `0x9c1520` handler) and copy the base vector over the record head.
///
/// The counter product uses x87 `fistp` truncation of a 64-bit result kept
/// to its low 32 bits, including the indefinite-zero on overflow or NaN;
/// the rewrite emulates exactly that.
///
/// Original: 0x009C1300 (cdecl, no arguments or return).
lf_checker_rt::export!(cdecl, rw_009C1300() -> u32 {
    unsafe { f1_run() }
});

/// Shared body of the function.
unsafe fn f1_run() -> u32 {
    unsafe {
        if global::<u32>(F1_G_ACTIVE).read_unaligned() == 1 {
            return 0;
        }
        if global::<u32>(F1_G_A).read_unaligned()
            != global::<u32>(F1_G_B).read_unaligned()
        {
            return 0;
        }
        if global::<u32>(F1_G_MODE).read_unaligned() == 0x12 {
            return 0;
        }
        let sh = global::<u32>(F1_G_SHIFT).read_unaligned();
        global::<u32>(F1_G_SHIFT.wrapping_add(4)).write_unaligned(sh);
        global::<u32>(F1_G_SHIFT).write_unaligned(0);
        let clock = global::<u32>(F1_G_CLOCK).read_unaligned();
        let mulin = f32::from_bits(global::<u32>(F1_G_MULIN).read_unaligned());
        let mulc = f32::from_bits(global::<u32>(F1_C_MUL).read_unaligned());
        // Truncated `fistp` of the 64-bit product, low 32 bits kept; the
        // indefinite form (overflow, NaN, infinities) contributes zero.
        let prod = core::hint::black_box(mulin) * core::hint::black_box(mulc);
        let pd = prod as f64;
        let bump = if pd.is_nan() || pd >= 9223372036854775808.0 || pd <= -9223372036854775808.0 {
            0u32
        } else {
            (prod as i64) as i32 as u32
        };
        for i in 0..F1_COUNT {
            let t = relocated(F1_TABLE0.wrapping_add(i.wrapping_mul(F1_STRIDE)));
            let other = rd32(t.wrapping_add(0x1c));
            if other != 0 {
                if rd32(t.wrapping_add(0x10)) == 0 {
                    let _: u32 = callee_cdecl!(1, u32, t);
                } else if rd32(other.wrapping_add(0x28)) & F1_KIND_MASK == F1_KIND_WANT {
                    let a2: u32 = callee_thiscall!(2, u32, other);
                    if a2 & 0xff == 0 {
                        let o2 = rd32(t.wrapping_add(0x1c));
                        if o2 != 0 {
                            let _: u32 =
                                callee_thiscall!(3, u32, o2, t.wrapping_add(0x1c));
                            wr32(t.wrapping_add(0x1c), 0);
                        }
                    }
                }
            }
            if rd32(t.wrapping_add(0x10)) == 0 {
                continue;
            }
            let _: u32 = callee_cdecl!(4, u32, t);
            if rd32(t.wrapping_add(0x10)) == 0 {
                continue;
            }
            if rd8(t.wrapping_add(0xcb)) != 0 {
                let c = rd32(t.wrapping_add(0x24));
                wr32(t.wrapping_add(0x24), c.wrapping_add(bump));
            }
            let ctr = rd32(t.wrapping_add(0x24));
            // Callee 5 runs only past a nonzero counter; a zero counter
            // falls into the mode dispatch like a reached one.
            if clock > ctr && ctr != 0 {
                let _: u32 =
                    callee_cdecl!(5, u32, t, 0, relocated(F1_DST), 0);
            } else {
                let m = rd32(t.wrapping_add(0x18)).wrapping_sub(2);
                if m == 0 {
                    let _: u32 = callee_cdecl!(7, u32, t);
                } else if m.wrapping_sub(1) == 0 {
                    let _: u32 = callee_cdecl!(6, u32, t);
                }
            }
            if rd32(t.wrapping_add(0x10)) == 0 {
                continue;
            }
            let _: u32 = callee_cdecl!(8, u32, t);
            let ob = rd32(t.wrapping_add(0x10));
            let ln = rd32(ob.wrapping_add(0x20));
            let base = if ln != 0 {
                ln.wrapping_add(0x30)
            } else {
                ob.wrapping_add(0x10)
            };
            wr32(t, rd32(base));
            wr32(t.wrapping_add(4), rd32(base.wrapping_add(4)));
            wr32(t.wrapping_add(8), rd32(base.wrapping_add(8)));
            wr32(t.wrapping_add(0x0c), rd32(base.wrapping_add(0x0c)));
        }
        0
    }
}
