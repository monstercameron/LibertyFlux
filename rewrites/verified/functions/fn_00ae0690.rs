// original: 0x00ae0690 dispatch_input_group
//
// Dispatches one input group: selects a pointer array from the object by
// (a0, a1), then walks it, gating each element through a predicate call
// and forwarding accepted elements to the per-element handler. The walk
// shape depends on a1 (direct walk, gated walk, or mode walk on a3), with
// notifier calls around the walk when a1 == 4 or a2 == 3. Nominally void;
// exit eax is reproduced exactly (the array end after a mode walk, else
// the last callee answer).

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

// Callee ids (see contract).
const N_NOTIFY: u32 = 1; // 0x432C20 cdecl/2: group notifier (return ignored)
const P_GATE: u32 = 2; // 0xAE4DB0 cdecl/2: element predicate, al==1/!=0 accepts
const F_SCALE: u32 = 3; // 0xAC6EF0 cdecl/1: float sink (return ignored)
const H_ELEM: u32 = 4; // 0xAE02A0 thiscall/2: per-element handler (return ignored)

/// Float factor the per-element byte is scaled by.
const K_FACTOR: u32 = 0x00FE86E8;
/// Stride table base inside the object: entry i is {array: u32, count: u16}.
const T_BASE: u32 = 0x140;
/// Entry stride in bytes.
const T_STRIDE: u32 = 8;

#[inline(always)]
unsafe fn rd8(addr: u32) -> u8 {
    unsafe { (addr as *const u8).read() }
}

#[inline(always)]
unsafe fn rd16(addr: u32) -> u16 {
    unsafe { (addr as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn factor() -> f32 {
    unsafe { global::<f32>(K_FACTOR).read() }
}

export!(thiscall, rw_ae0690(
    this: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
) -> u32 {
    let idx = a1.wrapping_add(a0.wrapping_mul(9));
    let slot = this.wrapping_add(T_BASE).wrapping_add(idx.wrapping_mul(T_STRIDE));
    // SAFETY: contract-fabricated object; offsets match the original exactly.
    let arr = unsafe { rd32(slot) };
    let count = unsafe { rd16(slot.wrapping_add(4)) } as u32;
    let end = arr.wrapping_add(count.wrapping_mul(4));

    if a1 == 4 || a2 == 3 {
        callee_cdecl!(N_NOTIFY, u32, 0x0d, 1);
        callee_cdecl!(N_NOTIFY, u32, 0x0e, 0);
    }

    if a1 == 4 {
        // Direct walk: optional flag pre-gate, predicate must return exactly 1,
        // then scale byte 0x20 and forward with (a2, 0).
        let mut p = arr;
        while p != end {
            // SAFETY: p walks the fabricated array exactly as the original does.
            let e = unsafe { rd32(p) };
            let mut accept = true;
            if a2 != 0 {
                if a2 == 3 {
                    accept = unsafe { rd8(e.wrapping_add(0x28)) } & 8 != 0;
                } else if a2 == 4 {
                    accept = unsafe { rd8(e.wrapping_add(0x28)) } & 0x20 != 0;
                } else {
                    accept = false;
                }
            }
            if accept {
                let key = unsafe { rd8(e.wrapping_add(0x24)) } as u32;
                let ok: u32 = callee_cdecl!(P_GATE, u32, a4, key);
                if (ok & 0xFF) == 1 {
                    let b = unsafe { rd8(e.wrapping_add(0x20)) } as f32;
                    let scaled = b * unsafe { factor() };
                    callee_cdecl!(F_SCALE, u32, scaled.to_bits());
                    callee_thiscall!(H_ELEM, u32, e, a2, 0);
                }
            }
            p = p.wrapping_add(4);
        }
        callee_cdecl!(F_SCALE, u32, 0x3F80_0000);
        let n: u32 = callee_cdecl!(N_NOTIFY, u32, 0x0d, 0);
        return n;
    }

    if a1 != 0 && a1 != 1 {
        // Gated walk: predicate nonzero forwards with (a2, a3).
        let mut p = arr;
        while p != end {
            let e = unsafe { rd32(p) };
            let key = unsafe { rd8(e.wrapping_add(0x24)) } as u32;
            let ok: u32 = callee_cdecl!(P_GATE, u32, a4, key);
            if (ok & 0xFF) != 0 {
                callee_thiscall!(H_ELEM, u32, e, a2, a3);
            }
            p = p.wrapping_add(4);
        }
    } else {
        // Mode walk on a3 (the predicate takes a4 in every loop).
        let mut p = arr;
        while p != end {
            let e = unsafe { rd32(p) };
            if a3 == 0 {
                let key = unsafe { rd8(e.wrapping_add(0x24)) } as u32;
                let ok: u32 = callee_cdecl!(P_GATE, u32, a4, key);
                if (ok & 0xFF) != 0 && unsafe { rd8(e.wrapping_add(0x25)) } == 4 {
                    callee_thiscall!(H_ELEM, u32, e, a2, 0);
                }
            } else if a3 == 2 {
                if unsafe { rd8(e.wrapping_add(0x28)) } & 2 != 0 {
                    let key = unsafe { rd8(e.wrapping_add(0x24)) } as u32;
                    let ok: u32 = callee_cdecl!(P_GATE, u32, a4, key);
                    if (ok & 0xFF) != 0 && unsafe { rd8(e.wrapping_add(0x25)) } != 4 {
                        callee_thiscall!(H_ELEM, u32, e, a2, 2);
                    }
                }
            } else if a3 == 1 {
                let key = unsafe { rd8(e.wrapping_add(0x24)) } as u32;
                let ok: u32 = callee_cdecl!(P_GATE, u32, a4, key);
                if (ok & 0xFF) != 0 && unsafe { rd8(e.wrapping_add(0x25)) } != 4 {
                    callee_thiscall!(H_ELEM, u32, e, a2, 1);
                }
            }
            p = p.wrapping_add(4);
        }
    }

    if a2 == 3 {
        let n: u32 = callee_cdecl!(N_NOTIFY, u32, 0x0d, 0);
        return n;
    }
    // Exit eax is a leftover: the mode walk reloads the array end into
    // eax every iteration and returns it when the walk ran; every other
    // path exits with 0 (last callee answer or the zero count).
    if (a1 == 0 || a1 == 1) && count > 0 {
        return end;
    }
    0
});
