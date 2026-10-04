// original: 0x00d8a7b0 scan_box_gate_emit
//
// Scan the entity list at `head` and emit every entity inside the box.
//
// `head` points at a wrapper whose first word is the first link; each link
// holds an entity pointer and the next link. `obj` is the querying object
// (excluded from the results, and the source of the reference height via
// `[obj+0x20]+0x38`). Entities already stamped with the current counter,
// flagged off, equal to `excluded`, outside the (`xlo`..`xhi`,`ylo`..`yhi`)
// box (strict on all four sides, NaN fails), or further than the tolerance
// from the reference height are skipped. Each survivor is stamped, offered
// to the gate call, and, when the gate answers zero, passed to the emit
// call with the trailing arguments and two flag bits from the entity.

use lf_checker_rt::{callee_cdecl, export, global};

const F1_GATE: u32 = 1; // cdecl/2 (obj, entity): AL nonzero means handled
const F1_EMIT: u32 = 2; // cdecl/10: emit call for unhandled entities

/// Visit-stamp counter shared by the entity-scan functions.
const G_STAMP: u32 = 0x011A8908;
/// Z-distance tolerance (8.0).
const G_ZTOL8: u32 = 0x00FE8AFC;

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
fn scan_box_gate_emit(
    head: u32, obj: u32, excluded: u32,
    xlo: f32, ylo: f32, xhi: f32, yhi: f32,
    a8: u32, a9: u32, a10: u32, a11: u32, a12: u32, flag: u32,
) -> u32 {
    // Same access order as the original: head link, reference height,
    // then the flag byte.
    let mut link = unsafe { rd32(head) };
    let zref_base = unsafe { rd32(obj.wrapping_add(0x20)) };
    let zref = unsafe { rf32(zref_base.wrapping_add(0x38)) };
    if unsafe { rd8(flag) } != 0 {
        return 0;
    }
    if link == 0 {
        return 0;
    }
    let stamp = unsafe { global::<u16>(G_STAMP).read_unaligned() } as u32;
    let ztol = unsafe { global::<f32>(G_ZTOL8).read_unaligned() };
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
        if entity == excluded {
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
        let py = unsafe { rf32(pos.wrapping_add(4)) };
        let pz = unsafe { rf32(pos.wrapping_add(8)) };
        if !(px > xlo) {
            if link == 0 { return 0; }
            continue;
        }
        if !(xhi > px) {
            if link == 0 { return 0; }
            continue;
        }
        if !(py > ylo) {
            if link == 0 { return 0; }
            continue;
        }
        if !(yhi > py) {
            if link == 0 { return 0; }
            continue;
        }
        if !(ztol > abs_scan(pz - zref)) {
            if link == 0 { return 0; }
            continue;
        }
        if entity == obj {
            if link == 0 { return 0; }
            continue;
        }
        let handled = callee_cdecl!(F1_GATE, u32, obj, entity);
        if (handled & 0xFF) == 0 {
            let bit7 = (unsafe { rd8(entity.wrapping_add(0xF1F)) } >> 7) as u32;
            let bit0 = (unsafe { rd8(entity.wrapping_add(0xF20)) } & 1) as u32;
            callee_cdecl!(F1_EMIT, u32,
                entity, obj, a8, a9, bit0, bit7, a10, a11, a12, flag);
        }
        if link == 0 {
            return 0;
        }
    }
}

export!(cdecl, rw_d8a7b0(
    head: u32, obj: u32, excluded: u32,
    xlo: f32, ylo: f32, xhi: f32, yhi: f32,
    a8: u32, a9: u32, a10: u32, a11: u32, a12: u32, flag: u32,
) -> u32 {
    scan_box_gate_emit(head, obj, excluded, xlo, ylo, xhi, yhi,
        a8, a9, a10, a11, a12, flag)
});
