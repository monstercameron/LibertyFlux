// original: 0x008e6200 room_tone_bank_update
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};
#[inline]
unsafe fn rd8(base: u32, off: u32) -> u8 {
    unsafe { ((base.wrapping_add(off)) as *const u8).read() }
}

#[inline]
unsafe fn rd32(base: u32, off: u32) -> u32 {
    unsafe { ((base.wrapping_add(off)) as *const u32).read() }
}

#[inline]
unsafe fn wr8(base: u32, off: u32, v: u8) {
    unsafe { ((base.wrapping_add(off)) as *mut u8).write(v) }
}

#[inline]
unsafe fn wr16(base: u32, off: u32, v: u16) {
    unsafe { ((base.wrapping_add(off)) as *mut u16).write(v) }
}

#[inline]
unsafe fn wr32(base: u32, off: u32, v: u32) {
    unsafe { ((base.wrapping_add(off)) as *mut u32).write(v) }
}

/// Pinned entry-EAX value, returned on paths where no call ran.
const ENTRY_EAX: u32 = 0x1234_5678;
/// Refresh a four-slot room-tone bank and restart it when fully idle.
///
/// Builds a three-word frame vector from globals (first slot plus a constant
/// bias, then the middle slot, then the third) and offers it to each live
/// handle in the four slots at +0x404. When
/// the stored generation at +0x414 differs from the engine generation it
/// re-registers the bank through the twelve-argument registrar. If every
/// slot is empty, the generation is live and new, and the stored timestamp
/// plus 500ms has passed the incoming timestamp, it stamps the new
/// generation and either starts all four voices (matching voice generation)
/// or reports the mismatch through the fallback reporter. Returns the last
/// callee answer on the path, or this+0x410 (a lea every exit lies past)
/// when no call ran; the entry EAX is never observed.
const FN3_G0: u32 = 0x0116_5E00;
const FN3_G1: u32 = 0x0116_5E04;
const FN3_G2: u32 = 0x0116_5E08;
const FN3_BIAS: u32 = 0x00FE_8830;
const FN3_EDI: u32 = 0x0116_88C8;
const FN3_GEN1: u32 = 0x017A_CC5C;
const FN3_GEN2: u32 = 0x0117_6D44;
const FN3_R0: u32 = 0x00E8_30B0;
const FN3_R1: u32 = 0x00E8_30D0;
const FN3_R2: u32 = 0x00E8_30F0;
const FN3_R3: u32 = 0x00E8_3110;

export!(thiscall, rw_008E6200(this: u32, arg: u32) -> u32 {
    let mut eax = ENTRY_EAX;
    let g0 = f32::from_bits(unsafe { global::<u32>(FN3_G0).read() });
    // The middle global is stored to the pushed-EDI slot between the two
    // vector words, so it rides along in the pointer the handle calls take.
    let g1 = unsafe { global::<u32>(FN3_G1).read() };
    let g2 = f32::from_bits(unsafe { global::<u32>(FN3_G2).read() });
    let bias = f32::from_bits(unsafe { global::<u32>(FN3_BIAS).read() });
    let v = [(g0 + bias).to_bits(), g1, g2.to_bits()];
    let edi = unsafe { global::<u32>(FN3_EDI).read() };
    for i in 0u32..4 {
        let h = unsafe { rd32(this, 0x404 + i * 4) };
        if h != 0 {
            eax = callee_thiscall!(1, u32, h, core::ptr::addr_of!(v) as u32);
        }
    }
    // Every exit lies past the lea: with no calls yet, EAX is this+0x410.
    eax = this.wrapping_add(0x410);
    if unsafe { rd32(this, 0x414) } != edi {
        let h0 = unsafe { rd32(this, 0x404) };
        let h1 = unsafe { rd32(this, 0x408) };
        let h2 = unsafe { rd32(this, 0x40C) };
        let h3 = unsafe { rd32(this, 0x410) };
        eax = callee_thiscall!(2, u32, this, h0, h1, h2, h3, 0, 0, 0, 0, 0, 0, 0, 0);
    }
    if unsafe { rd32(this, 0x404) } != 0 {
        return eax;
    }
    if unsafe { rd32(this, 0x408) } != 0 {
        return eax;
    }
    if unsafe { rd32(this, 0x40C) } != 0 {
        return eax;
    }
    if unsafe { rd32(this, 0x410) } != 0 {
        return eax;
    }
    if edi == 0 {
        return eax;
    }
    if edi == unsafe { global::<u32>(FN3_GEN1).read() } {
        return eax;
    }
    // The deadline check leaves its sum in EAX on the early exit.
    let t = unsafe { rd32(this, 0x418) }.wrapping_add(0x1F4);
    eax = t;
    if t >= arg {
        return eax;
    }
    let mut s = [0u32; 24];
    let sptr = core::ptr::addr_of_mut!(s) as u32;
    let _b = callee_thiscall!(3, u32, sptr);
    unsafe { ((sptr + 0x14) as *mut u32).write(core::ptr::addr_of!(v) as u32) };
    unsafe { wr32(this, 0x418, arg) };
    unsafe { wr32(this, 0x414, edi) };
    if edi == unsafe { global::<u32>(FN3_GEN2).read() } {
        eax = callee_thiscall!(
            4, u32, this, relocated(FN3_R0),
            this.wrapping_add(0x404), sptr, 0xFFFF_FFFF, 0, 0
        );
        eax = callee_thiscall!(
            4, u32, this, relocated(FN3_R1),
            this.wrapping_add(0x408), sptr, 0xFFFF_FFFF, 0, 0
        );
        eax = callee_thiscall!(
            4, u32, this, relocated(FN3_R2),
            this.wrapping_add(0x410), sptr, 0xFFFF_FFFF, 0, 0
        );
        eax = callee_thiscall!(
            4, u32, this, relocated(FN3_R3),
            this.wrapping_add(0x40C), sptr, 0xFFFF_FFFF, 0, 0
        );
    } else {
        eax = callee_thiscall!(
            5, u32, this, edi,
            this.wrapping_add(0x404), sptr, 0xFFFF_FFFF, 0, 0
        );
    }
    eax
});
