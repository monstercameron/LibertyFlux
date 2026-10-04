// original: 0x00ae28d0 input_ui_register_record

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};
// relocated is used for every absolute game address (the worker maps the
// image at a different base, so raw file-VA constants would mismatch).

// Callee ids for fn_00ae28d0 (see contract).
const C_INIT: u32 = 1; // thiscall/0: frame initializer (opaque, encrypted region)
const C_ZERO: u32 = 2; // cdecl/3: memset-like (dst, 0, 0x94)
const C_REG: u32 = 3; // cdecl/4: conditional registrar, answer stored into table
const C_COOKIE: u32 = 4; // thiscall/0: CRT security-cookie check (scripted 0)

// Globals (file VAs) touched by fn_00ae28d0.
const G_COUNT: u32 = 0x15B2B84; // table fill count, incremented when < 199
const G_TICKS: u32 = 0x15AE648; // incremented on every call
const B_FLAG: u32 = 0x1593311; // byte flag selecting the 0x2f/0x3f mode word
const G_SEQ: u32 = 0x15C3F14; // sequence counter with /50 bucket step
const T_BASE: u32 = 0x15C3BD8; // registrar-answer table (199 entries)
const F_PTR: u32 = 0x15B2BA0; // constant pointer stored into the frame
const F_BASE: u32 = 0x15AE650; // constant base added to (seq << 7)

/// Registration record builder: fills a 0x94-byte frame record from its
/// integer arguments and a few globals, bumps two counters, and — while the
/// fill count is below 199 — hands the record to the registrar and stores
/// the answer into the table.
///
/// Only arguments 1, 3, 4 and 5 are read; 0 and 2 are ignored. The return
/// value is whatever the trailing cookie check answers (scripted 0).
export!(cdecl, rw_00ae28d0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    let _ = (a0, a2);
    let mut frame = [0u32; 38];
    let fp = frame.as_mut_ptr() as u32;
    let _ = callee_thiscall!(C_INIT, u32, fp);
    let _ = callee_cdecl!(C_ZERO, u32, fp, 0, 0x94);
    // Pack: high half from the count, low half from a1 (dead store: the mode
    // word below overwrites this slot before anything can observe it, but the
    // original computes it, so so do we).
    let count = unsafe { global::<u32>(G_COUNT).read() };
    let ticks = global::<u32>(G_TICKS);
    unsafe { ticks.write(ticks.read().wrapping_add(1)) };
    frame[0x58 / 4] = a1.wrapping_add(0x400) | count.wrapping_shl(16);
    let lo = a4.wrapping_add(0x400) & 0xffff;
    frame[0x54 / 4] = a3.wrapping_add(0x400).wrapping_shl(16) ^ lo;
    // Mode word from the flag byte.
    let flag = unsafe { global::<u8>(B_FLAG).read() };
    frame[0x58 / 4] = (if flag != 0 { 0x1f } else { 0x0f }) | 0x20;
    frame[0x5c / 4] = a5;
    // Sequence slot and counter step (signed /50 via magic multiply).
    let seq = unsafe { global::<u32>(G_SEQ).read() };
    frame[0x0c / 4] = seq.wrapping_shl(7).wrapping_add(relocated(F_BASE));
    let step = seq.wrapping_add(1);
    let prod = (step as i32 as i64).wrapping_mul(0x99999999u32 as i32 as i64);
    let quot = ((prod >> 32) as i32) >> 5;
    let quot = quot.wrapping_add((((quot as u32) >> 31)) as i32);
    let seq_new = step.wrapping_add((quot as u32).wrapping_mul(5).wrapping_shl(4));
    frame[0x3c / 4] = 8;
    frame[0x48 / 4] = 0;
    frame[0x50 / 4] = relocated(F_PTR);
    frame[0x60 / 4] = 0;
    unsafe { global::<u32>(G_SEQ).write(seq_new) };
    frame[0x08 / 4] = 0x80;
    if (count as i32) >= 0xc7 {
        return callee_thiscall!(C_COOKIE, u32, 0);
    }
    let ans = callee_cdecl!(C_REG, u32, relocated(0xea75a8), relocated(0xae6ca0), fp, 0);
    let count2 = unsafe { global::<u32>(G_COUNT).read() };
    let slot = relocated(T_BASE).wrapping_add(count2.wrapping_mul(4)) as *mut u32;
    unsafe { slot.write(ans) };
    unsafe { global::<u32>(G_COUNT).write(count2.wrapping_add(1)) };
    callee_thiscall!(C_COOKIE, u32, 0)
});
