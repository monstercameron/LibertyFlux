// original: 0x00b5d630 BULLET_IMPACT_WATER
/// Bullet-impact-on-water dispatch (`this`, target, info, extra, scratch, flags).
///
/// Passes three global entry gates, notifies the impact handler (callee 1),
/// then, unless the flag byte is set, runs the gated body: weapon-row checks
/// (callee 2), a position solve into caller frame (callee 3), a threshold
/// select over the mode word (callee 4), an optional horn probe (callee 5),
/// more weapon-row gates, a multiplicative random draw against the threshold
/// (globals `G_RNG0`/`G_RNG1` updated), a spread call (callee 6), a geometry
/// solve (callee 7), a placement call over three differences (callee 8), an
/// owner lookup (callee 9), a rubber notify (callee 10), an object clear plus
/// two counter calls (callees 11-13), an effect dispatch (callee 14) and a
/// hash/register pair or a release (callees 15-16 or 17). The tail handles a
/// splash path (callees 18-19) and two further kind dispatches (callees
/// 19-20). No result; several frame stores the original makes are never read
/// and are omitted (the checker's stack comparison does not cover the
/// callee's own frame).
///
/// Cond: `this` points at the slot object (`+0x18` is the kind index), the
/// target and extra pointers are null or point at the documented objects.
export!(thiscall, rw_aq44_f1(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        let _ = a3;
        if *(global::<u32>(G_ENTRY_A)) == 1 {
            return 0;
        }
        if *(global::<u32>(G_ENTRY_B)) != *(global::<u32>(C_ENTRY_B)) {
            return 0;
        }
        if *(global::<u32>(G_ENTRY_C)) == ENTRY_C_EXIT {
            return 0;
        }
        let edi = a0;
        let this18 = *(this.wrapping_add(0x18) as *const u32);
        let _: u32 = callee_thiscall!(1, u32, relocated(OBJ1), this, a1, edi);
        let mode = *(global::<i32>(G_MODE));
        let flag15 = u32::from(!(mode < 2) && (this18 == 0x1E || this18 == 0x28));
        let mut body = (a4 as u8) == 0;
        if body {
            let w = callee_cdecl!(2, u32, this18);
            if ((*(w.wrapping_add(0x20) as *const u32) >> 5) & 1) == 0 {
                body = false;
            }
        }
        if body {
            let w = callee_cdecl!(2, u32, this18);
            if *(w.wrapping_add(0x0C) as *const u32) != 2 && flag15 == 0 {
                body = false;
            }
        }
        let mut o_b62 = [0u32; 4];
        let mut o_af1 = [0u32; 4];
        let mut o_af2 = [0u32; 4];
        let mut o_ad0 = [0u32; 4];
        let mut o_ad1 = [0u32; 4];
        let mut o_ad2 = [0u32; 4];
        let mut o_40e = [0u32; 4];
        let mut o_8e3 = [0u32; 8];
        let mut o_a7b = [0u32; 1];
        let mut o_jw2 = [0u32; 4];
        let mut slot08 = 0u32;
        if body {
            let _: u32 = callee_thiscall!(3, u32, this, o_b62.as_mut_ptr() as u32, a1, 3);
            slot08 = *(global::<u32>(C_SLOT_A));
            let al4: u32 = callee_cdecl!(4, u32,);
            if al4 as u8 != 0 {
                let mode2 = *(global::<i32>(G_MODE));
                slot08 = if mode2 == 2 {
                    *(global::<u32>(C_ONE))
                } else {
                    *(global::<u32>(C_SLOT_B))
                };
            }
            if edi != 0
                && (*(edi.wrapping_add(0x28) as *const u32) & 0x3C0) == 0xC0
                && *(edi.wrapping_add(0x219) as *const u8) != 0
            {
                let al5: u32 = callee_thiscall!(5, u32, edi);
                if al5 as u8 != 0 {
                    slot08 = *(global::<u32>(C_ONE));
                }
            }
            let w = callee_cdecl!(2, u32, this18);
            if *(w as *const u32) == 0x14 {
                slot08 = *(global::<u32>(C_ONE));
            } else if !(*(global::<i32>(G_MODE)) < 2) {
                let w2 = callee_cdecl!(2, u32, this18);
                if *(w2 as *const u32) == 0x27 {
                    slot08 = *(global::<u32>(C_ONE));
                }
            }
            let w = callee_cdecl!(2, u32, this18);
            if *(w as *const u32) == 0x10 && *(global::<i32>(G_MODE)) == 2 {
                slot08 = *(global::<u32>(C_ONE));
            }
            let w = callee_cdecl!(2, u32, this18);
            if *(w.wrapping_add(0x10) as *const u32) != 9 {
                let rng0 = *(global::<u32>(G_RNG0));
                let rng1 = *(global::<u32>(G_RNG1));
                let prod = (rng0 as u64).wrapping_mul(RNG_MULT as u64);
                let sum = (prod as u32 as u64) + rng1 as u64;
                let nlo = sum as u32;
                let nhi = ((prod >> 32) as u32).wrapping_add((sum >> 32) as u32);
                *(global::<u32>(G_RNG0)) = nlo;
                *(global::<u32>(G_RNG1)) = nhi;
                let scale = *(global::<u32>(C_RNG_SCALE));
                let rf = (nlo & RNG_KEEP_MASK) as f32;
                let rbits = fmul_pinned(rf.to_bits(), scale);
                if !comiss_jb(slot08, rbits) {
                    let w = callee_cdecl!(2, u32, this18);
                    let low = u32::from(
                        *(w as *const u32) == 0x10 && *(global::<i32>(G_MODE)) == 2,
                    );
                    slot08 = (slot08 & !0xFF) | (low & 0xFF);
                    let w = callee_cdecl!(2, u32, this18);
                    // The clear path zeroes only the low byte of the row
                    // pointer still in eax; the upper bytes survive.
                    let mut alv = w & !0xFF;
                    if *(w as *const u32) == 0x14 {
                        alv = 1;
                    } else if !(*(global::<i32>(G_MODE)) < 2) {
                        let w2 = callee_cdecl!(2, u32, this18);
                        if *(w2 as *const u32) == 0x27 {
                            alv = 1;
                        } else {
                            alv = w2 & !0xFF;
                        }
                    }
                    let _: u32 = callee_thiscall!(
                        6,
                        u32,
                        relocated(OBJ1),
                        o_af2.as_mut_ptr() as u32,
                        o_af1.as_mut_ptr() as u32,
                        edi,
                        alv,
                        slot08
                    );
                }
            }
        }
        if body {
            let w = callee_cdecl!(2, u32, this18);
            if *(w.wrapping_add(8) as *const u32) == 2 {
                body = false;
            }
        }
        if body {
            let al7: u32 = callee_cdecl!(
                7,
                u32,
                o_ad2.as_mut_ptr() as u32,
                o_ad1.as_mut_ptr() as u32,
                o_ad0.as_mut_ptr() as u32
            );
            if al7 as u8 == 0 {
                body = false;
            }
        }
        if body {
            o_40e[0] = fsub_pinned(o_ad0[0], *(a1.wrapping_add(0x30) as *const u32));
            o_40e[1] = fsub_pinned(o_ad0[1], *(a1.wrapping_add(0x34) as *const u32));
            o_40e[2] = fsub_pinned(o_ad0[2], *(a1.wrapping_add(0x38) as *const u32));
            o_40e[3] = o_ad2[3];
            let _: u32 = callee_thiscall!(8, u32, o_40e.as_mut_ptr() as u32);
            // Dead row reads the original still faults on: reproduced.
            let wd = callee_cdecl!(2, u32, this18);
            let _ = *(wd.wrapping_add(0x10) as *const u32);
            let w = callee_cdecl!(2, u32, this18);
            if *(w as *const u32) != 0x14 && *(global::<i32>(G_MODE)) >= 2 {
                let w2 = callee_cdecl!(2, u32, this18);
                let _ = *(w2 as *const u32);
            }
            let p = callee_cdecl!(9, u32, 0);
            let eq = u32::from(edi == p);
            let _: u32 =
                callee_thiscall!(10, u32, relocated(OBJ1), o_a7b.as_mut_ptr() as u32, eq);
            let _: u32 = callee_thiscall!(11, u32, o_8e3.as_mut_ptr() as u32);
            o_8e3[5] = o_ad0.as_ptr() as u32;
            let a12 = callee_cdecl!(12, u32,);
            slot08 = a12;
            let a13 = callee_cdecl!(13, u32, a12);
            let al14: u32 = callee_thiscall!(
                14,
                u32,
                relocated(OBJ2),
                relocated(ARG_C1),
                o_8e3.as_mut_ptr() as u32,
                slot08,
                a13,
                0
            );
            if al14 as u8 != 0 {
                o_jw2[0] = 0;
                o_jw2[1] = 0xFFFFFFFF;
                o_jw2[2] = 0xA9;
                o_jw2[3] = o_ad2[0];
                let a15 = callee_cdecl!(
                    15,
                    u32,
                    relocated(ARG_C2),
                    0,
                    0,
                    0,
                    1,
                    o_8e3.as_mut_ptr() as u32,
                    o_jw2.as_mut_ptr() as u32,
                    0
                );
                let _: u32 = callee_cdecl!(
                    16,
                    u32,
                    a15,
                    0,
                    0,
                    1,
                    o_8e3.as_mut_ptr() as u32,
                    o_jw2.as_mut_ptr() as u32,
                    0,
                    slot08
                );
            } else {
                let _: u32 = callee_cdecl!(17, u32, slot08);
            }
        }
        if *(global::<i32>(G_MODE)) >= 1
            && a2 != 0
            && *(a2 as *const u32) != 0
            && edi != 0
            && (*(edi.wrapping_add(0x28) as *const u32) & 0x3C0) == 0x80
            && *(edi.wrapping_add(0xF50) as *const u32) != 0
        {
            let ecx30 = *(edi.wrapping_add(0xF50) as *const u32);
            let al18: u32 = callee_thiscall!(18, u32, ecx30);
            if al18 as u8 != 0
                && *(edi.wrapping_add(0x1304) as *const u32) == 4
                && this18 == 0x14
            {
                let eap = a2.wrapping_add(0x10);
                let _: u32 = callee_cdecl!(
                    19,
                    u32,
                    0,
                    edi,
                    0x11,
                    0x3F800000,
                    eap,
                    0,
                    0,
                    1,
                    0xBF800000,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0xFFFFFFFF
                );
            }
        }
        if *(global::<i32>(G_MODE)) < 2 {
            return 0;
        }
        let w = callee_cdecl!(2, u32, this18);
        if *(w as *const u32) == 0x1E {
            if a2 == 0 {
                return 0;
            }
            if *(a2 as *const u32) == 0 {
                return 0;
            }
            let ecx33 = *(edi.wrapping_add(0x6C) as *const u32);
            if ecx33 != 0 && *(ecx33.wrapping_add(0x0E) as *const u8) != 0 {
                return 0;
            }
            let eap = a2.wrapping_add(0x10);
            let _: u32 = callee_cdecl!(
                19,
                u32,
                0,
                edi,
                0x10,
                0x3F800000,
                eap,
                1,
                0,
                1,
                0xBF800000,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0xFFFFFFFF
            );
            return 0;
        }
        let w = callee_cdecl!(2, u32, this18);
        if *(w as *const u32) != 0x28 {
            return 0;
        }
        let esi2 = a2;
        if esi2 == 0 {
            return 0;
        }
        if *(esi2 as *const u32) == 0 {
            return 0;
        }
        let al20: u32 = callee_thiscall!(20, u32, edi);
        if al20 as u8 != 0 {
            return 0;
        }
        let eap = esi2.wrapping_add(0x10);
        let _: u32 = callee_cdecl!(
            19,
            u32,
            0,
            edi,
            0x12,
            0x3F800000,
            eap,
            1,
            0,
            1,
            0xBF800000,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0xFFFFFFFF
        );
        0
    }
});

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

// Game-data addresses and constants this function uses (file VAs).
const G_ENTRY_A: u32 = 0x11F7060;
const G_ENTRY_B: u32 = 0x12088B4;
const C_ENTRY_B: u32 = 0xF1C040;
const G_ENTRY_C: u32 = 0x1037720;
const ENTRY_C_EXIT: u32 = 0x12;
const G_MODE: u32 = 0x11D6FD4;
const G_RNG0: u32 = 0x11101A0;
const G_RNG1: u32 = 0x11101A4;
const RNG_MULT: u32 = 0x5CDCFAA7;
const RNG_KEEP_MASK: u32 = 0x7FFFFF;
const C_RNG_SCALE: u32 = 0xFE864C;
const C_SLOT_A: u32 = 0xFE87B4;
const C_SLOT_B: u32 = 0xFE881C;
const C_ONE: u32 = 0xFE88E8;
const OBJ1: u32 = 0x13B6798;
const OBJ2: u32 = 0x12831E4;
const ARG_C1: u32 = 0xEB10D8;
const ARG_C2: u32 = 0xEB10EC;

/// Ordered-compare branch condition: true when the hardware below flag would
/// be set (first operand below the second, or either operand unordered).
#[inline(always)]
fn comiss_jb(a_bits: u32, b_bits: u32) -> bool {
    let a = f32::from_bits(a_bits);
    let b = f32::from_bits(b_bits);
    a.is_nan() || b.is_nan() || a < b
}

/// Scalar float subtract with pinned operand order.
#[inline(always)]
fn fsub_pinned(a_bits: u32, b_bits: u32) -> u32 {
    let a = f32::from_bits(a_bits);
    let b = f32::from_bits(b_bits);
    (core::hint::black_box(a) - core::hint::black_box(b)).to_bits()
}

/// Scalar float multiply with pinned operand order.
#[inline(always)]
fn fmul_pinned(a_bits: u32, b_bits: u32) -> u32 {
    let a = f32::from_bits(a_bits);
    let b = f32::from_bits(b_bits);
    (core::hint::black_box(a) * core::hint::black_box(b)).to_bits()
}
