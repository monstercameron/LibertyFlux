// original: 0x00ae0180 present_input_record
//
// Presents one input record: an early arm publishes the record pointer
// to a global; otherwise an optional pre-call, a vtable call through a
// computed sub-object (faults when the computed offset equals the record
// stride, exactly like the original's null dereference), a measurement
// call, a 9-word render call whose float output gates an 8-word commit
// call, two global stamps, and an optional post-call. Nominally void;
// exit eax reproduced (early path: flag byte with entry eax pinned 0).

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};
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

// Callee ids (see contract).
const Q_PRE: u32 = 1; // 0xAD1240 thiscall/0: pre hook (ecx = record)
const _Q_VT: u32 = 2; // vtable+0x10 thiscall/0: sub-object hook (planted)
const Q_MEAS: u32 = 3; // 0xADFD50 thiscall/1: measurement (frame struct)
const Q_RENDER: u32 = 4; // 0x8DE1C0 cdecl/9: render
const Q_COMMIT: u32 = 5; // 0x8DE440 cdecl/8: commit
const Q_POST: u32 = 6; // 0xAD1820 thiscall/0: post hook (ambient ecx)

const G_SLOT: u32 = 0x01593B18;
const G_A0: u32 = 0x015DBDC0;
const G_A1: u32 = 0x015DBDC4;
const T_TAB: u32 = 0x01295CD8;
const K_LIM: u32 = 0x00FE8628;

export!(thiscall, rw_ae0180(this: u32, a0: u32, a1: u32) -> u32 {
    // SAFETY below: every address is contract-fabricated (record object,
    // vtable, frame locals) or a relocated global; offsets match the
    // original exactly.
    let f29 = unsafe { rd8(this.wrapping_add(0x29)) };
    if f29 & 0x20 != 0 && a0 == 1 && a1 != 0x40 {
        unsafe { global::<u32>(G_SLOT).write(this) };
        return f29 as u32;
    }
    if f29 & 0x10 != 0 {
        callee_thiscall!(Q_PRE, u32, this);
    }
    let idx = unsafe { rd16(this.wrapping_add(0x1a)) } as u32;
    let dead = unsafe { global::<u32>(T_TAB.wrapping_add(idx.wrapping_mul(4))).read() };
    core::hint::black_box(dead);
    let n = (unsafe { rd8(this.wrapping_add(0x26)) } as u32)
        .wrapping_sub(3 * ((unsafe { rd8(this.wrapping_add(0x28)) } & 1) as u32))
        .wrapping_add(4)
        << 4;
    let stride = unsafe { rd16(this.wrapping_add(0x18)) } as u32;
    let p = if n == stride { 0 } else { this.wrapping_add(n) };
    let vtable = unsafe { rd32(p) };
    let target = unsafe { rd32(vtable.wrapping_add(0x10)) };
    let hook: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(target as usize) };
    hook(p);
    let b27 = unsafe { rd8(this.wrapping_add(0x27)) };
    let edi = if b27 != 0 {
        this.wrapping_add((b27 as u32) << 4)
    } else {
        0
    };
    unsafe { global::<u32>(G_A1).write(a1) };
    unsafe { global::<u32>(G_A0).write(a0) };
    // Frame words the original passes by pointer: the measurement struct
    // (fill zeros, also handed to the render call) and the constant word
    // the original stores before the render call (-10.0f), which doubles
    // as the float the commit gate compares. (The saved edi/esi slots
    // are never read back, so entry edi/esi are irrelevant.)
    let frame = [0u32; 4];
    callee_thiscall!(Q_MEAS, u32, this, frame.as_ptr() as u32);
    let slot_e8 = 0xC120_0000u32;
    let w30 = unsafe { rd32(this.wrapping_add(0x30)) };
    let w34 = unsafe { rd32(this.wrapping_add(0x34)) };
    let w38 = unsafe { rd32(this.wrapping_add(0x38)) };
    let w3c = unsafe { rd32(this.wrapping_add(0x3c)) };
    let r4: u32 = callee_cdecl!(
        Q_RENDER, u32, dead, edi, frame.as_ptr() as u32,
        w30, w34, w38, w3c, &slot_e8 as *const u32 as u32, 0
    );
    // The original compares the constant word as a float against the
    // limit (comiss): below-or-unordered skips the commit call.
    let lim = unsafe { global::<f32>(K_LIM).read() };
    let gate = f32::from_bits(slot_e8);
    let below = gate < lim || gate.is_nan() || lim.is_nan();
    let mut last = r4;
    if !below {
        last = callee_cdecl!(
            Q_COMMIT, u32, dead, edi, w38, w3c, a0, 0, 0,
            &slot_e8 as *const u32 as u32
        );
    }
    unsafe { global::<u32>(G_A1).write(0xFF) };
    unsafe { global::<u32>(G_A0).write(0) };
    if f29 & 0x10 != 0 {
        last = callee_thiscall!(Q_POST, u32, this);
    }
    last
});
