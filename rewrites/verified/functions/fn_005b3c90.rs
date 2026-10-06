// original: 0x005B3C90 sample_palette_block

/// Resample a palette table: jitter one global colour, then refresh five
/// table entries plus two epilogue entries from random bytes.
///
/// Calling convention: thiscall with no stack arguments (`ecx` carries a
/// flag in its low byte and arbitrary upper bits, all of which flow into
/// call arguments). No observed return value (the single caller ignores
/// `eax`, so the contract compares no return channel).
///
/// Behaviour in order:
/// 1. Prologue. When the flag byte is non-zero, jitter global colour A:
///    replace its third byte with a random byte and bump the shift by the
///    mode switch (mode 1/2/3 adds 3/6/9, anything else adds nothing; the
///    switch bound is an UNSIGNED above-3). When the flag byte is zero,
///    jitter global colour B instead (no shift bump), seeding the random
///    byte through callee 2.
/// 2. Five sample blocks. Each block picks a table index from the flag bit
///    (block 1: 0x33/0x3a; block 2: 0x44/0x3f; block 3: 0x34/0x3b; block 4:
///    0x45/0x46; block 5: 0x47/0x48), unpacks that entry's colour bytes,
///    draws a small random integer through callee 2, and gates the rest on
///    callee 3's SIGNED non-positive answer (a non-positive answer skips to
///    the store). Blocks 3-5 run callee 3 twice and gate a second fetch
///    pair on the second answer's SIGNED value. Each fetch pair (callees 5
///    and 6) fills a (pointer, count) slot; a non-zero count draws a second
///    integer (block 1 additionally picks one byte out of the pointed-to
///    list), and a non-null pointer is submitted through virtual slot 3 of
///    the TLS factory object. Each block finally packs its drawn integer
///    over its own entry's three colour bytes back into its table entry.
/// 3. Epilogue. Two mini-blocks (indices 0x35/0x3c and 0x36/0x3d) run callee
///    7, draw one more random byte scaled by the answer-plus-one, and pack
///    it as the top byte over the entry's own low three bytes.
///
/// The random idiom everywhere is `((rand & 0xffff) as f32) * K1 * N`,
/// truncated to int (`K1` = 2^-15, `N` a small count), except the epilogue
/// which multiplies in the opposite order (`N * (rand*K1)`); both orders
/// are reproduced exactly through order-pinned helpers. All float inputs
/// are finite and non-negative with results below 510, so no NaN, infinity
/// or saturation is possible and truncation equals `as i32`. Colour unpack
/// uses arithmetic right shifts, reproduced with signed shifts.
///
/// The stack slots are modelled as variables initialised to the contract's
/// `stack_fill` (0), including the cross-block staleness the original
/// relies on: blocks 3-5 pass block 1's pickup slot (fetch byte or drawn
/// integer) as call arguments, and block 2 overwrites block 1's index slot
/// with its own entry word. The two jittered globals sit inside the table
/// itself (colour B is entry 0x32, colour A entry 0x39), so the prologue
/// store and the block stores all land in one tracked region.
use lf_checker_rt::{callee_cdecl, callee_fastcall, relocated, tls_slot};
use core::hint::black_box;

pub const COL_A: u32 = 0x01160D2C;
pub const COL_B: u32 = 0x01160D10;
pub const MODE_SW: u32 = 0x01160EC4;
pub const EDI_SRC: u32 = 0x011D6FD4;
pub const TABLE: u32 = 0x01160C48;
/// 2^-15: scales a u16 random draw into [0, 2).
pub const K1: f32 = 3.0517578125e-05;
pub const K2: f32 = 3.0;

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

/// Original-order float multiply: (r & 0xffff) * K1 first, then * N.
#[inline(always)]
fn mul_ord(a: f32, b: f32) -> f32 {
    black_box(a) * black_box(b)
}

#[inline(always)]
fn rand_unit(rand: u32) -> f32 {
    mul_ord((rand & 0xFFFF) as f32, black_box(K1))
}

/// Truncated ((rand & 0xffff) * K1 * n) as the original computes it.
#[inline(always)]
fn scaled(rand: u32, n: u32) -> u32 {
    mul_ord(rand_unit(rand), black_box(n as f32)) as i32 as u32
}

/// Epilogue order: n * ((rand & 0xffff) * K1).
#[inline(always)]
fn scaled_rev(rand: u32, n: u32) -> u32 {
    mul_ord(black_box(n as f32), rand_unit(rand)) as i32 as u32
}

#[inline(always)]
fn sar(x: u32, n: u32) -> u32 {
    ((x as i32) >> n) as u32
}

/// Pack four bytes big-end-first (b0 is the top byte).
#[inline(always)]
fn pack8(b0: u32, b1: u32, b2: u32, b3: u32) -> u32 {
    ((b0 & 0xFF) << 24) | ((b1 & 0xFF) << 16) | ((b2 & 0xFF) << 8) | (b3 & 0xFF)
}

type SlotFn = extern "thiscall" fn(u32, u32) -> u32;

unsafe fn vslot(obj: u32, arg: u32) -> u32 {
    unsafe {
        let holder = ((obj + 8) as *const u32).read_unaligned();
        let vtable = (holder as *const u32).read_unaligned();
        let slot = ((vtable + 0xC) as *const u32).read_unaligned();
        let f: SlotFn = core::mem::transmute(slot as usize);
        f(holder, arg)
    }
}

unsafe fn table_get(table: u32, idx: u32) -> u32 {
    unsafe { ((table + idx.wrapping_mul(4)) as *const u32).read_unaligned() }
}
unsafe fn table_put(table: u32, idx: u32, v: u32) {
    unsafe { ((table + idx.wrapping_mul(4)) as *mut u32).write_unaligned(v) }
}

unsafe fn rand_call() -> u32 {
    unsafe { callee_cdecl!(1, u32,) }
}
/// Callee 2 (first site only): the original zeroes dl, leaving the
/// upper 24 bits of edx as entry leftover, so the contract compares ecx
/// alone. Only the dl byte is passed meaningfully here.
unsafe fn probe_first(mode: u32, dl: u32, edi: u32) -> u32 {
    unsafe { callee_fastcall!(2, u32, mode, dl & 0xFF, edi) }
}
/// Later probe sites (callee 9): the original only sets dl, leaving the
/// upper 24 bits as entry leftover, so the contract compares ecx alone.
unsafe fn probe(mode: u32, dl: u32, edi: u32) -> u32 {
    unsafe { callee_fastcall!(9, u32, mode, dl & 0xFF, edi) }
}

/// Callee 6 takes (extra, ebp, a2, edi, ptr): the original pushes
/// ptr first and the extra flag last, so C argument order is the
/// reverse of push order (this holds for every multi-argument call:
/// the stub logs top-of-stack first).
unsafe fn submit(edi: u32, a2: u32, ebp: u32, out_ptr: u32, extra: u32) {
    unsafe {
        callee_cdecl!(6, u32, extra, ebp, a2, edi, out_ptr);
    }
}

/// Draw an index into a non-empty list and pick one byte from it.
/// Returns (index, picked_byte).
unsafe fn pick(rand: u32, count16: u16, ptr: u32) -> (u32, u32) {
    unsafe {
        let index = scaled(rand, count16 as u32);
        let byte = ((ptr + index.wrapping_mul(2) + 1) as *const u8).read();
        (index, byte as u32)
    }
}

/// Full behaviour.
unsafe fn body(input: u32) {
    unsafe {
        let table = relocated(TABLE);
        let bl = input & 0xFF;
        let ebx = input;
        let mut edi = rd32(relocated(EDI_SRC));
        let mut ebp: u32;
        // Cross-block stack slots, all zero-filled at entry.
        let mut v_e40: u32 = 0;
        let mut v_e36: u32 = 0;
        let mut v_e32: u32 = 0;
        let mut v_e28: u32 = 0;
        let mut v_e24: u32 = 0;
        let mut v_e20: u32 = 0;
        let mut v_e16: u32 = 0;
        if bl != 0 {
            let col = rd32(relocated(COL_A));
            let r = rand_call();
            ebp = mul_ord(rand_unit(r), black_box(K2)) as i32 as u32 & 0xFF;
            let packed = pack8(ebp, sar(col, 16), sar(col, 8), col);
            (relocated(COL_A) as *mut u32).write_unaligned(packed);
            match rd32(relocated(MODE_SW)) {
                1 => ebp = ebp.wrapping_add(3),
                2 => ebp = ebp.wrapping_add(6),
                3 => ebp = ebp.wrapping_add(9),
                _ => {}
            }
        } else {
            let col = rd32(relocated(COL_B));
            let probe_al = probe_first(0x32, 0, edi) & 0xFF;
            let r = rand_call();
            ebp = scaled(r, probe_al) & 0xFF;
            let packed = pack8(ebp, sar(col, 16), sar(col, 8), col);
            (relocated(COL_B) as *mut u32).write_unaligned(packed);
        }
        // Block 1: index 0x33/0x3a.
        {
            let idx = if bl != 0 { 0x3A } else { 0x33 };
            v_e28 = idx;
            let col = table_get(table, idx);
            v_e20 = col;
            v_e32 = sar(col, 16);
            v_e24 = sar(col, 8);
            let s_al = probe(1, bl, edi) & 0xFF;
            let fint = scaled(rand_call(), s_al);
            v_e36 = fint;
            v_e40 = fint;
            let a1 = callee_cdecl!(3, u32, 0, ebp, ebx, idx, edi);
            let skip = (a1 as i32) <= 0;
            if !skip {
                let mut out = [0u32; 2];
                callee_cdecl!(5, u32, 0, ebp, ebx, edi, 0x3A, out.as_mut_ptr() as u32);
                submit(edi, 1, ebp, out.as_mut_ptr() as u32, 0);
                let (ptr, count16) = (out[0], out[1] as u16);
                if count16 != 0 {
                    let (index, byte) = pick(rand_call(), count16, ptr);
                    v_e36 = index;
                    v_e40 = byte;
                }
                if ptr != 0 {
                    vslot(tls_slot(0), ptr);
                }
            }
            if idx != 0x7FFF_FFFF {
                table_put(table, idx, pack8(v_e36, v_e32, v_e24, v_e20));
            }
        }
        // Block 2: index 0x44/0x3f; second fetch arg is const 0x3f.
        {
            let idx = if bl != 0 { 0x3F } else { 0x44 };
            let s_al = probe(7, bl, edi) & 0xFF;
            let fint = scaled(rand_call(), s_al);
            v_e36 = fint;
            let col = table_get(table, idx);
            v_e28 = col;
            v_e20 = sar(col, 16);
            v_e24 = sar(col, 8);
            let a = callee_cdecl!(3, u32, 0, ebp, ebx, 0x3F, edi);
            if (a as i32) > 0 {
                let mut out = [0u32; 2];
                callee_cdecl!(5, u32, 0, ebp, ebx, edi, idx, out.as_mut_ptr() as u32);
                submit(edi, ebx, ebp, out.as_mut_ptr() as u32, 0);
                let (ptr, count16) = (out[0], out[1] as u16);
                if count16 != 0 {
                    v_e36 = scaled(rand_call(), count16 as u32);
                }
                if ptr != 0 {
                    vslot(tls_slot(0), ptr);
                }
            }
            if idx != 0x7FFF_FFFF {
                table_put(table, idx, pack8(v_e36, v_e20, v_e24, v_e28));
            }
        }
        // Block 3: index 0x34/0x3b; stores into its own drawn integer.
        {
            let idx = if bl != 0 { 0x3B } else { 0x34 };
            let s_al = probe(2, bl, edi) & 0xFF;
            let fint = scaled(rand_call(), s_al);
            v_e28 = fint;
            let col = table_get(table, idx);
            v_e16 = col;
            v_e36 = idx;
            v_e20 = sar(col, 16);
            v_e24 = sar(col, 8);
            let a1 = callee_cdecl!(3, u32, 0, ebp, ebx, idx, edi);
            let a2 = callee_cdecl!(4, u32, 1, v_e40, ebx, idx, edi);
            let esi_first = a1;
            let esi = v_e36;
            v_e32 = a2;
            if (esi_first as i32) > 0 {
                let mut out = [0u32; 2];
                callee_cdecl!(5, u32, 0, ebp, ebx, edi, esi, out.as_mut_ptr() as u32);
                if (v_e32 as i32) > 0 {
                    let esi2 = v_e40;
                    callee_cdecl!(5, u32, 1, esi2, ebx, edi, esi, out.as_mut_ptr() as u32);
                }
                submit(edi, ebx, ebp, out.as_mut_ptr() as u32, 0);
                if (v_e32 as i32) > 0 {
                    submit(edi, ebx, v_e40, out.as_mut_ptr() as u32, 1);
                }
                let (ptr, count16) = (out[0], out[1] as u16);
                if count16 != 0 {
                    v_e28 = scaled(rand_call(), count16 as u32);
                }
                if ptr != 0 {
                    vslot(tls_slot(0), ptr);
                }
            }
            let store_idx = v_e36;
            if store_idx != 0x7FFF_FFFF {
                table_put(table, store_idx, pack8(v_e28, v_e20, v_e24, v_e16));
            }
        }
        // Block 4: index 0x45/0x46.
        {
            let idx = if bl != 0 { 0x46 } else { 0x45 };
            v_e36 = idx;
            let col = table_get(table, idx);
            v_e24 = col;
            v_e16 = sar(col, 16);
            v_e20 = sar(col, 8);
            let s_al = probe(3, bl, edi) & 0xFF;
            let fint = scaled(rand_call(), s_al);
            v_e32 = fint;
            let a1 = callee_cdecl!(3, u32, 0, ebp, ebx, idx, edi);
            let a2 = callee_cdecl!(4, u32, 1, v_e40, ebx, idx, edi);
            let esi_first = a1;
            let esi = v_e36;
            v_e28 = a2;
            if (esi_first as i32) > 0 {
                let mut out = [0u32; 2];
                callee_cdecl!(5, u32, 0, ebp, ebx, edi, esi, out.as_mut_ptr() as u32);
                if (v_e28 as i32) > 0 {
                    callee_cdecl!(5, u32, 1, v_e40, ebx, edi, esi, out.as_mut_ptr() as u32);
                }
                submit(edi, ebx, ebp, out.as_mut_ptr() as u32, 0);
                if (v_e28 as i32) > 0 {
                    submit(edi, ebx, v_e40, out.as_mut_ptr() as u32, 1);
                }
                let (ptr, count16) = (out[0], out[1] as u16);
                if count16 != 0 {
                    v_e32 = scaled(rand_call(), count16 as u32);
                }
                if ptr != 0 {
                    vslot(tls_slot(0), ptr);
                }
            }
            if esi != 0x7FFF_FFFF {
                table_put(table, esi, pack8(v_e32, v_e16, v_e20, v_e24));
            }
        }
        // Block 5: index 0x47/0x48; count arrives through bp.
        {
            let idx = if bl != 0 { 0x48 } else { 0x47 };
            v_e36 = idx;
            let col = table_get(table, idx);
            v_e24 = col;
            v_e16 = sar(col, 16);
            v_e20 = sar(col, 8);
            let s_al = probe(4, bl, edi) & 0xFF;
            let fint = scaled(rand_call(), s_al);
            v_e32 = fint;
            let a1 = callee_cdecl!(3, u32, 0, ebp, ebx, idx, edi);
            let a2 = callee_cdecl!(4, u32, 1, v_e40, ebx, idx, edi);
            let esi_first = a1;
            let esi = v_e36;
            v_e28 = a2;
            if (esi_first as i32) > 0 {
                let mut out = [0u32; 2];
                callee_cdecl!(5, u32, 0, ebp, ebx, edi, esi, out.as_mut_ptr() as u32);
                if (v_e28 as i32) > 0 {
                    callee_cdecl!(5, u32, 1, v_e40, ebx, edi, esi, out.as_mut_ptr() as u32);
                }
                submit(edi, ebx, ebp, out.as_mut_ptr() as u32, 0);
                if (v_e28 as i32) > 0 {
                    submit(edi, ebx, v_e40, out.as_mut_ptr() as u32, 1);
                }
                let (ptr, count16) = (out[0], out[1] as u16);
                ebp = (ebp & 0xFFFF_0000) | count16 as u32;
                if count16 != 0 {
                    v_e32 = scaled(rand_call(), count16 as u32);
                }
                if ptr != 0 {
                    vslot(tls_slot(0), ptr);
                }
            }
            if esi != 0x7FFF_FFFF {
                table_put(table, esi, pack8(v_e32, v_e16, v_e20, v_e24));
            }
        }
        // Epilogue: two fixed mini-blocks.
        for (idx, extra) in [(if bl != 0 { 0x3C } else { 0x35 }, 1), (if bl != 0 { 0x3D } else { 0x36 }, 0)] {
            let col = table_get(table, idx);
            v_e16 = col;
            let al = callee_cdecl!(7, u32, extra, ebx, edi) & 0xFF;
            let grown = al.wrapping_add(1);
            if extra == 1 {
                ebp = grown;
            } else {
                edi = grown;
            }
            let top = scaled_rev(rand_call(), grown) & 0xFF;
            if idx != 0x7FFF_FFFF {
                let d = v_e16;
                table_put(table, idx, pack8(top, sar(d, 16), sar(d, 8), d));
            }
        }
    }
}

lf_checker_rt::export!(thiscall, rw_005B3C90(this: u32) -> u32 {
    unsafe { body(this) };
    0
});
