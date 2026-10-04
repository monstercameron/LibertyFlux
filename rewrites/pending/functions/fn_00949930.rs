// original: 0x00949930 four_span_probe_8f
//! Four-span probe (8 floats in, four engine calls out).
//!
//! Builds one 21-word shared parameter table from three global floats plus
//! constants, then issues four identical-shape calls. Each call passes a
//! fresh 3-word span holding two of the eight input floats and the constant
//! -1000.0, the shared table, and the constants (1000.0, 0, 6, 4). Between
//! calls the original re-adds one table entry to a fourth global float and
//! stores the sum back into dead stack slots; the sums are never read again
//! (verified by symbolic execution of every stack access), so they are
//! reproduced here only as ordinary locals for fidelity. Exit EAX is the last
//! call's answer. Stack slots the original never initializes (table padding)
//! read the contract's defined stack fill, reproduced here as explicit zeros.

use lf_checker_rt::{callee_cdecl, export, global};

/// File VAs (image base 0x400000) of the globals this function reads.
const G_VEC_X: u32 = 0x01B4_B320;
const G_VEC_Y: u32 = 0x01B4_B324;
const G_VEC_Z: u32 = 0x01B4_B328;
const G_STEP: u32 = 0x00FE_8A24;

/// Call-shape constants, as raw bit patterns (moved, never interpreted).
const SCALE_UP: u32 = 0x447A_0000; // 1000.0f
const SCALE_DOWN: u32 = 0xC47A_0000; // -1000.0f
const TABLE_TAIL: u32 = 0x0000_FFFF;

/// Shared 21-word parameter table, in word order.
fn shared_table(gx: u32, gy: u32, gz: u32) -> [u32; 21] {
    let mut t = [0u32; 21];
    // Words 1,2,3 / 7 / 11 / 15 are padding the original never stores to;
    // with the contract's zero stack fill they read back as 0.
    t[4] = gx;
    t[5] = gy;
    t[6] = gz;
    t[8] = gx;
    t[9] = gy;
    t[10] = gz;
    t[12] = gx;
    t[13] = gy;
    t[14] = gz;
    t[19] = TABLE_TAIL;
    t
}

export!(cdecl, rw_00949930(
    a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32,
) -> u32 {
    let gx: u32 = unsafe { global::<u32>(G_VEC_X).read() };
    let gy: u32 = unsafe { global::<u32>(G_VEC_Y).read() };
    let gz: u32 = unsafe { global::<u32>(G_VEC_Z).read() };
    let step: u32 = unsafe { global::<u32>(G_STEP).read() };
    let table = shared_table(gx, gy, gz);
    let mut span = [0u32; 3];
    span[2] = SCALE_DOWN;

    // Dead running sum the original recomputes between calls; kept so the
    // same floating-point operations execute in the same order. The sums are
    // never read (in either binary), hence the throwaway bindings.
    let _acc0 = f32::from_bits(gz) + f32::from_bits(step);

    span[0] = a0;
    span[1] = a1;
    let _ans0 = callee_cdecl!(1, u32,
        span.as_ptr() as u32, SCALE_UP, 0, table.as_ptr() as u32, 6, 4);
    let _acc1 = f32::from_bits(table[6]) + f32::from_bits(step);

    span[0] = a2;
    span[1] = a3;
    let _ans1 = callee_cdecl!(1, u32,
        span.as_ptr() as u32, SCALE_UP, 0, table.as_ptr() as u32, 6, 4);
    let _acc2 = f32::from_bits(table[6]) + f32::from_bits(step);

    span[0] = a4;
    span[1] = a5;
    let _ans2 = callee_cdecl!(1, u32,
        span.as_ptr() as u32, SCALE_UP, 0, table.as_ptr() as u32, 6, 4);
    let _acc3 = f32::from_bits(table[6]) + f32::from_bits(step);

    span[0] = a6;
    span[1] = a7;
    // Exit EAX is the last call's answer, matching the original.
    callee_cdecl!(1, u32,
        span.as_ptr() as u32, SCALE_UP, 0, table.as_ptr() as u32, 6, 4)
});
