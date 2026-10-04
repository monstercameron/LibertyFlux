// original: 0x00d8a8e0 scan_box_vquery_emit
//
// Scan the entity list at `head` and emit every entity inside the box
// whose vtable slot 0x64 query scores above the threshold.
//
// The list, stamp, flag, box and height gates match scan_box_gate_emit;
// there is no excluded entity, no flag byte and no gate call. Each
// survivor is passed through the thiscall query at vtable slot 0x64 and
// emitted only when the query result's float at +8 strictly exceeds the
// threshold (NaN fails).

use lf_checker_rt::{callee_cdecl, export, global};

const F2_EMIT: u32 = 2; // cdecl/4: emit call for entities passing the query

/// Visit-stamp counter shared by the entity-scan functions.
const G_STAMP: u32 = 0x011A8908;
/// Z-distance tolerance (8.0).
const G_ZTOL8: u32 = 0x00FE8AFC;
/// Query-score threshold (0.9).
const G_SCORE_MIN: u32 = 0x00FE88BC;

#[inline(always)]
unsafe fn rd8(addr: u32) -> u8 {
    unsafe { (addr as *const u8).read() }
}

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rf32(addr: u32) -> f32 {
    unsafe { (addr as *const f32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write_unaligned(v) }
}

/// Absolute value exactly as the original's `comiss 0,d / jbe / xorps sign`
/// sequence: negatives are negated, everything else (including NaN and
/// signed zero) passes through unchanged.
#[inline(always)]
fn abs_scan(d: f32) -> f32 {
    if d < 0.0 { -d } else { d }
}

#[allow(clippy::too_many_arguments)]
fn scan_box_vquery_emit(
    head: u32, obj: u32,
    xlo: f32, ylo: f32, xhi: f32, yhi: f32,
    a7: u32, a8: u32,
) -> u32 {
    let mut link = unsafe { rd32(head) };
    if link == 0 {
        return 0;
    }
    let stamp = unsafe { global::<u16>(G_STAMP).read_unaligned() } as u32;
    let ztol = unsafe { global::<f32>(G_ZTOL8).read_unaligned() };
    let smin = unsafe { global::<f32>(G_SCORE_MIN).read_unaligned() };
    loop {
        let entity = unsafe { rd32(link) };
        link = unsafe { rd32(link.wrapping_add(4)) };
        if unsafe { rd32(entity.wrapping_add(0x3C)) } == stamp {
            if link == 0 { return 0; }
            continue;
        }
        if unsafe { rd8(entity.wrapping_add(0x24)) } & 1 == 0 {
            if link == 0 { return 0; }
            continue;
        }
        unsafe { wr32(entity.wrapping_add(0x3C), stamp) };
        let pos_base = unsafe { rd32(entity.wrapping_add(0x20)) };
        let pos = if pos_base == 0 {
            entity.wrapping_add(0x10)
        } else {
            pos_base.wrapping_add(0x30)
        };
        let px = unsafe { rf32(pos) };
        if !(px > xlo) {
            if link == 0 { return 0; }
            continue;
        }
        if !(xhi > px) {
            if link == 0 { return 0; }
            continue;
        }
        let py = unsafe { rf32(pos.wrapping_add(4)) };
        if !(py > ylo) {
            if link == 0 { return 0; }
            continue;
        }
        if !(yhi > py) {
            if link == 0 { return 0; }
            continue;
        }
        // The original loads the reference height here, after the box.
        let zref_base = unsafe { rd32(obj.wrapping_add(0x20)) };
        let zref = unsafe { rf32(zref_base.wrapping_add(0x38)) };
        let pz = unsafe { rf32(pos.wrapping_add(8)) };
        if !(ztol > abs_scan(pz - zref)) {
            if link == 0 { return 0; }
            continue;
        }
        // Vtable slot 0x64 query, called exactly like the original so
        // both sides land on the same planted stub.
        let vtable = unsafe { rd32(entity) };
        let target = unsafe { rd32(vtable.wrapping_add(0x64)) };
        let query: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        let res = query(entity);
        let score = unsafe { rf32(res.wrapping_add(8)) };
        if !(score > smin) {
            if link == 0 { return 0; }
            continue;
        }
        callee_cdecl!(F2_EMIT, u32, entity, obj, a7, a8);
        if link == 0 {
            return 0;
        }
    }
}

export!(cdecl, rw_d8a8e0(
    head: u32, obj: u32,
    xlo: f32, ylo: f32, xhi: f32, yhi: f32,
    a7: u32, a8: u32,
) -> u32 {
    scan_box_vquery_emit(head, obj, xlo, ylo, xhi, yhi, a7, a8)
});
