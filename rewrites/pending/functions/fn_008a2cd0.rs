// original: 0x008a2cd0 rage::audSimpleSound::vf9
/// Voice setup of `rage::audSimpleSound` (vf9).
///
/// Resolves a loop helper through two helpers (cdecl/1 then cdecl/2,
/// stubbed); when that fails it sets flag bit 0 and returns 0. Otherwise it
/// programs the slot voice's fields from the helper's descriptor and sets or
/// clears its loop flag from the descriptor's sign. A 0xff slot faults
/// writing through the null voice, exactly like the original.
export!(thiscall, rw_008a2cd0(this: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        let o = this as *mut u8;
        let w = *(o.add(0x3c) as *const i16) as i32 as u32;
        let r1: u32 = callee_cdecl!(1, u32, w);
        let b0 = *(o.add(0xb0) as *const u32);
        let desc: u32 = callee_cdecl!(2, u32, r1, b0);
        if desc == 0 {
            *o.add(0x39) |= 1;
            return 0;
        }
        let slot = *o.add(0x48);
        let bank = *o.add(0x40);
        let voice = voice_ptr(bank, slot);
        let rate = core::ptr::read_unaligned(((desc.wrapping_add(0x1a))) as *const u16);
        core::ptr::write_unaligned(((voice.wrapping_add(0xe4))) as *mut u16, rate);
        let pitch = core::ptr::read_unaligned(((desc.wrapping_add(0x18))) as *const u16) as u32;
        let base = core::ptr::read_unaligned(((desc.wrapping_add(0x10))) as *const u32);
        let r3: u32 = callee_cdecl!(3, u32, base, pitch);
        core::ptr::write_unaligned(((voice.wrapping_add(0xe0))) as *mut u32, r3);
        let tail = core::ptr::read_unaligned(((desc.wrapping_add(0x14))) as *const u32);
        let fl = (voice.wrapping_add(0xee)) as *mut u8;
        if (tail as i32) < 0 {
            *fl &= !4;
        } else {
            *fl |= 4;
        }
        r3
    }
});
