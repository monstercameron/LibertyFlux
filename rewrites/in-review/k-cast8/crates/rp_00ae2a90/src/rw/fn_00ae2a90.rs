// original: 0x00ae2a90 input_ui_list_refresh

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

// Callee ids for fn_00ae2a90 (see contract).
const D_ALLOC: u32 = 1; // cdecl/2: 12-byte allocator, sequenced answers
const D_CTOR_H: u32 = 2; // thiscall/1: head object setup
const D_V28: u32 = 3; // planted +0x28 on the head global object
const D_V8H: u32 = 4; // planted +0x8 on the head/element object
const D_FWD: u32 = 5; // cdecl/4: sibling handler (kind == 8 forward)
const D_GATE: u32 = 6; // cdecl/5: early-out notifier
const D_PREP_A: u32 = 7; // cdecl/2: mode-3 preamble, immediates only
const D_PREP_B: u32 = 8; // cdecl/2: mode-3 preamble, one out-slot
const D_PREP_C: u32 = 9; // cdecl/3: mode-3 preamble, two byte-slots
const D_CTOR_1: u32 = 10; // thiscall/1: element-allocator setup
const D_SINK: u32 = 11; // cdecl/1: result sink
const D_CTOR_0A: u32 = 12; // thiscall/1: mode-4/5 setup (mid)
const D_PROBE: u32 = 13; // cdecl/1: mode-4/5 element probe
const D_V58: u32 = 14; // planted +0x58 float source (f32 on ST0)
const D_TEST: u32 = 15; // cdecl/1: element acceptance test
const D_EMIT: u32 = 16; // cdecl/2: float emitter, one out-slot
const D_SEND3: u32 = 17; // cdecl/3: accepted-element sender
const D_FILL4: u32 = 18; // cdecl/4: rejected-element filler A
const D_FILL4B: u32 = 19; // cdecl/4: rejected-element filler B
const D_V8C: u32 = 20; // planted +0x8c element commit
const D_CTOR_0B: u32 = 21; // thiscall/1: mode-4/5 setup (end)
const D_POST_A: u32 = 23; // cdecl/2: mode-3 epilogue, one byte-slot
const D_POST_B: u32 = 24; // cdecl/1: mode-3 epilogue sender

// Globals (file VAs) read by fn_00ae2a90.
const H_HEAD_OBJ: u32 = 0x12FB1B8; // head object pointer
const T_ROWS: u32 = 0x1614C90; // row table base (0x54-byte rows)
const B_GATE: u32 = 0x1593312; // early-out gate byte
const B_COND: u32 = 0x103F6E7; // sub-4 conjunction byte
const B_MODE1: u32 = 0x15B0E7D; // mode-1 skip byte
const B_BLEND: u32 = 0x15AE64C; // float-blend gate byte
const G_THRESH: u32 = 0x103F734; // acceptance threshold
const F_ONE: u32 = 0xFE88E8; // float constant 1.0
const F_SCALE: u32 = 0xFE86E8; // float scale constant

/// Truncating float-to-int exactly like `cvttss2si`: NaN and out-of-range
/// values produce `i32::MIN` (Rust `as` would saturate instead).
#[inline(always)]
fn cvttss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 9.223372e18f32 || x < -9.223372e18f32 {
        i32::MIN
    } else {
        x as i32
    }
}

/// The shared `% 16` / `/ 16` bitfield step: two sampled values are folded
/// into bits 13..24 of the word at `obj + 4`. Returns the folded bits.
#[inline(always)]
unsafe fn fold16(obj: u32, t1: u32, t2: u32) -> u32 {
    let r1 = (t1 as i32) % 16;
    let s = 0x10i32.wrapping_sub(r1) % 16;
    let q = (t2 as i32).wrapping_add(s) / 16;
    let cell = unsafe { ((obj + 4) as *const u32).read() };
    let bits = ((q as u32).wrapping_shl(14) ^ cell) & 0x01ffc000;
    unsafe { ((obj + 4) as *mut u32).write(cell ^ bits) };
    bits
}

fn null_fault() -> u32 {
    unsafe { (0 as *const u32).read_volatile() }
}

/// Load a planted call target: the vtable pointer from the object, then the
/// slot content (the recorder-stub address the contract planted there).
#[inline(always)]
fn vslot(obj: u32, off: usize) -> u32 {
    let vt = unsafe { (obj as *const u32).read() } as usize;
    unsafe { ((vt + off) as *const u32).read() }
}

/// Table-list refresh dispatcher. The head samples a freshly allocated
/// object twice, folds the samples into its flag word, and either forwards
/// to the sibling handler (kind == 8) or reshuffles its arguments onto the
/// stack and falls into the table walk below, which visits every element of
/// the selected (row, sub) list and either blends, commits or skips it.
///
/// The reshuffle zeroes the caller's fourth argument slot when the filter is
/// nonzero and the mode is not 4 or 5; a rewrite cannot address its incoming
/// stack slots, so the stack channel is off for this contract (the r-n01
/// precedent) and the filtered value is verified through the walk it drives.
export!(cdecl, rw_00ae2a90(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    // Head: allocate, sample through the vtable, fold into [edi+4].
    let alloc_h = callee_cdecl!(D_ALLOC, u32, 0xc, 0);
    let edi = if alloc_h == 0 {
        null_fault();
        0u32
    } else {
        let hobj = unsafe { global::<u32>(H_HEAD_OBJ).read() };
        let f28: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(vslot(hobj, 0x28) as usize) };
        let v28 = f28(hobj);
        callee_thiscall!(D_CTOR_H, u32, alloc_h, v28)
    };
    let f8: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(vslot(edi, 8) as usize) };
    let t1 = f8(edi);
    let f8b: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(vslot(edi, 8) as usize) };
    let t2 = f8b(edi);
    unsafe { fold16(edi, t1, t2) };
    if a3 == 8 {
        return callee_cdecl!(D_FWD, u32, a0, a1, a2, a3);
    }
    // Reshuffle: the tail's filter is 0 unless the argument survives.
    let edx = if a4 != 0 && a2 != 4 && a2 != 5 { 0 } else { a4 };
    // Tail gate: forward the argument addresses and return early.
    if unsafe { global::<u8>(B_GATE).read() } != 0 {
        let mut c0 = [a0];
        let mut c1 = [a1];
        let mut c2 = [a2];
        let mut c3 = [a3];
        return callee_cdecl!(
            D_GATE, u32, relocated(0xae9240),
            c0.as_mut_ptr() as u32, c1.as_mut_ptr() as u32,
            c2.as_mut_ptr() as u32, c3.as_mut_ptr() as u32
        );
    }
    let row = relocated(T_ROWS).wrapping_add(a0.wrapping_mul(0x54));
    // Preamble by mode. Mode 3 runs its three setup calls (unless the kind
    // is zero) and then falls through into the shared kind dispatch below;
    // modes 4/5 run their own allocator branch instead.
    if a2 == 3 && a3 != 0 {
        let _ = callee_cdecl!(D_PREP_A, u32, relocated(0xae0c90), relocated(0xea7725));
        let mut slot = [1u32, 0];
        let _ = callee_cdecl!(D_PREP_B, u32, relocated(0xae0cb0), slot.as_mut_ptr() as u32);
        let mut s1 = [0u32; 1];
        let mut s2 = [0u32; 1];
        let _ = callee_cdecl!(
            D_PREP_C, u32, relocated(0xae4510),
            s1.as_mut_ptr() as u32, s2.as_mut_ptr() as u32
        );
    }
    if a2 == 4 || a2 == 5 {
        let ah = callee_cdecl!(D_ALLOC, u32, 0xc, 0);
        if ah == 0 {
            let _ = callee_cdecl!(D_SINK, u32, 0);
        } else {
            let t = callee_thiscall!(D_CTOR_1, u32, ah, 0);
            let _ = callee_cdecl!(D_SINK, u32, t);
        }
    } else if a3 == 4 || a3 == 0x10 || a3 == 0x20 {
        let ah = callee_cdecl!(D_ALLOC, u32, 0xc, 0);
        if ah == 0 {
            let _ = callee_cdecl!(D_SINK, u32, 0);
        } else {
            let t = callee_thiscall!(D_CTOR_1, u32, ah, 1);
            let _ = callee_cdecl!(D_SINK, u32, t);
        }
    } else {
        let ah = callee_cdecl!(D_ALLOC, u32, 0xc, 0);
        if ah == 0 {
            let _ = callee_cdecl!(D_SINK, u32, 0);
        } else {
            let t = callee_thiscall!(D_CTOR_1, u32, ah, 2);
            let _ = callee_cdecl!(D_SINK, u32, t);
        }
    }
    if a2 == 5 || a2 == 4 {
        let ah = callee_cdecl!(D_ALLOC, u32, 0xc, 0);
        if ah == 0 {
            let _ = callee_cdecl!(D_SINK, u32, 0);
        } else {
            let t = callee_thiscall!(D_CTOR_0A, u32, ah, 1);
            let _ = callee_cdecl!(D_SINK, u32, t);
        }
    }
    // Element walk.
    let sub = unsafe { ((row + a1.wrapping_mul(8) + 0x10) as *const u32).read() };
    let count = unsafe { ((row + a1.wrapping_mul(8) + 0x14) as *const u16).read() } as u32;
    let mut v10: u8 = 0;
    let mut v14: u32 = 0;
    // Frame slot shared by the emitter calls: the post-commit emitter writes
    // 1.0 there, and the pre-commit emitter's second word reads it back on a
    // later iteration. Starts zero (fresh stack fill), like the original's.
    let mut slot_x: u32 = 0;
    let mut idx: u32 = 0;
    while idx < count {
        let elem = unsafe { ((sub + idx.wrapping_mul(8)) as *const u32).read() };
        let flags = unsafe { ((elem + 0x24) as *const u32).read() };
        let flag8 = flags as u8;
        let enter = match edx {
            0 => true,
            4 => flag8 & 0x80 != 0,
            2 => flag8 & 0x40 != 0 && flag8 & 0x80 == 0,
            1 => flag8 & 0xc0 == 0,
            _ => true,
        };
        if !enter {
            idx += 1;
            continue;
        }
        // The sub-4 filter and the mode-1 skip both live under `a1 == 4`;
        // any other sub-index jumps straight to the mode gate below.
        if a1 == 4 {
            if unsafe { global::<u8>(B_COND).read() } == 1
                && a3 == 0
                && a2 == 0
                && flags & 0x40000000 != 0
            {
                idx += 1;
                continue;
            }
            if unsafe { global::<u8>(B_MODE1).read() } == 1 && a2 == 1 {
                idx += 1;
                continue;
            }
        }
        let mut v0e: u8 = 0;
        let enter2 = match a2 {
            0 => true,
            3 => flags & 0x4000 != 0,
            2 => flags & 0x2000 != 0,
            4 | 5 => flags & 0x8000 != 0,
            1 => flags & 0x1000 != 0,
            _ => false,
        };
        if !enter2 {
            idx += 1;
            continue;
        }
        // Per-element value. Kinds 0x10/0x20 force the byte and jump past
        // both the probe and the blend below, straight to the store.
        if a3 == 0x10 || a3 == 0x20 {
            v10 = 0xff;
        } else {
            if a1 == 4 {
                let eb = unsafe { ((elem + 0x63) as *const u8).read() };
                v14 = (a3 & 0xffffff00) | eb as u32;
                v10 = eb;
            } else {
                v14 = a3 | 0xff;
                v10 = 0xff;
            }
            if a2 == 5 || a2 == 4 {
                let cl = callee_cdecl!(D_PROBE, u32, elem) as u8;
                v0e = cl;
                if cl == 1 {
                    let o2 = unsafe { ((elem + 0x4c) as *const u32).read() };
                    let ob = unsafe { ((o2 + 0x63) as *const u8).read() };
                    let d = 0xffu8.wrapping_sub(ob);
                    let old = v14 as u8;
                    let m = if d < old { d } else { old };
                    v10 = m;
                    v14 = m as u32;
                }
            }
            if unsafe { global::<u8>(B_BLEND).read() } == 1 {
                let f = {
                    let fv: extern "thiscall" fn(u32) -> f32 =
                        unsafe { core::mem::transmute(vslot(elem, 0x58) as usize) };
                    fv(elem)
                };
                let rowf = unsafe { ((row + 0x50) as *const f32).read() };
                let subf = unsafe { ((sub + idx.wrapping_mul(8) + 4) as *const f32).read() };
                let x = subf + f;
                let mut z = (rowf - x) / f;
                if 0.0f32 > z {
                    z = 0.0;
                } else if z > unsafe { global::<f32>(F_ONE).read() } {
                    z = unsafe { global::<f32>(F_ONE).read() };
                }
                let s = (v14 as u8) as u32 as f32;
                v10 = cvttss2si(s * z) as u8;
            }
        }
        let v20 = v10 as u32;
        if (v20 as i32) < unsafe { global::<i32>(G_THRESH).read() } {
            idx += 1;
            continue;
        }
        if a2 == 4 || a2 == 5 {
            if v0e == 1 {
                let mut slot = [0u32, slot_x, 0];
                let _ = callee_cdecl!(D_EMIT, u32, relocated(0xac7320), slot.as_mut_ptr() as u32);
                vcomm(elem, a2, v10 as u32);
                let mut slot2 = [0x3f800000u32, 0, 0];
                let _ = callee_cdecl!(D_EMIT, u32, relocated(0xac7320), slot2.as_mut_ptr() as u32);
                slot_x = 0x3f800000;
                idx += 1;
                continue;
            }
            vcomm(elem, a2, v10 as u32);
            idx += 1;
            continue;
        }
        if a3 == 4 || a3 == 0x10 || a3 == 0x20 {
            vcomm(elem, a2, v10 as u32);
            idx += 1;
            continue;
        }
        let t = callee_cdecl!(D_TEST, u32, elem) as u8;
        if t != 0 {
            let scaled = (v20 as f32) * unsafe { global::<f32>(F_SCALE).read() };
            let mut slot = [scaled.to_bits(), 0, slot_x];
            let _ = callee_cdecl!(D_EMIT, u32, relocated(0xac6ef0), slot.as_mut_ptr() as u32);
            let _ = callee_cdecl!(D_SEND3, u32, elem, a2, 0);
            idx += 1;
            continue;
        }
        let shifted = 1u32 << (a2 & 31);
        let mut slot_a = [0u32, 0];
        let _ = callee_cdecl!(
            D_FILL4, u32, slot_a.as_mut_ptr() as u32, elem, shifted, 0
        );
        let se = unsafe { ((elem + 0x40) as *const i8).read() } as i32 as u32;
        let e48 = unsafe { ((elem + 0x48) as *const u32).read() };
        let mut slot_b = [0u32, 0];
        let _ = callee_cdecl!(
            D_FILL4B, u32, slot_b.as_mut_ptr() as u32, e48, se, 0
        );
        vcomm(elem, a2, v10 as u32);
        idx += 1;
    }
    // Post-loop.
    let mut pslot = [0x3f800000u32, 0, 0];
    let post_ans = callee_cdecl!(D_EMIT, u32, relocated(0xac6ef0), pslot.as_mut_ptr() as u32);
    if a2 == 5 || a2 == 4 {
        let ah = callee_cdecl!(D_ALLOC, u32, 0xc, 0);
        if ah == 0 {
            null_fault();
            return 0;
        }
        let aux = callee_thiscall!(D_CTOR_0B, u32, ah, 1);
        if aux == 0 {
            null_fault();
            return 0;
        }
        let g8: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(vslot(aux, 8) as usize) };
        let u1 = g8(aux);
        let g8b: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(vslot(aux, 8) as usize) };
        let u2 = g8b(aux);
        let bits = unsafe { fold16(aux, u1, u2) };
        return bits;
    }
    if a2 == 3 && a3 != 0 {
        let mut slot = [(v10 as u32) << 8];
        let _ = callee_cdecl!(D_POST_A, u32, relocated(0xae0c90), slot.as_mut_ptr() as u32);
        return callee_cdecl!(D_POST_B, u32, relocated(0xae44e0));
    }
    post_ans
});

/// Element commit through the +0x8c slot: (edi, 0, value, -1) with the
/// element in ECX, exactly like the original's indirect call.
#[inline(always)]
fn vcomm(elem: u32, edi: u32, val: u32) -> u32 {
    let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(vslot(elem, 0x8c) as usize) };
    f(elem, edi, 0, val, 0xffffffff)
}
