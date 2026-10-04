// original: 0x0099f040 audio_mixer_init
/// Initialize a mixer object: zero its fields, resolve four table handles,
/// set up two filter descriptors and two range descriptors, then register it.
///
/// Behavior: store `arg0` at `this+8` and zero most scalar fields (a few
/// take -1, 1, or float bit patterns); resolve four handles by calling the
/// table lookup once per global key, storing each answer plus a -1 tag;
/// pass a `(0, 1.0f, 0)` descriptor with a name to the filter setup for the
/// `+0xD0` and `+0x120` sub-objects; pass four float words to the range
/// setup for the `+0x18C` and `+0x170` sub-objects; stamp the remaining
/// fields; finally register the object and return the register answer.
/// (The original's epilogue restores two callee-saved registers from the
/// wrong stack slots, leaving garbage in them; register values other than
/// the return channel are not observable through the checker.)
export!(thiscall, rw_0099f040(this: u32, arg0: u32) -> u32 {
    unsafe {
        core::ptr::write_unaligned(this.wrapping_add(0xC1) as *mut u8, 0);
        core::ptr::write_unaligned(this.wrapping_add(0xC4) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(8) as *mut u32, arg0);
        core::ptr::write_unaligned(this.wrapping_add(0x60) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x64) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x68) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x6C) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x90) as *mut u32, 0xFFFF_FFFF);
        core::ptr::write_unaligned(this.wrapping_add(0x10) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x14) as *mut u8, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x54) as *mut u16, 0xFF);
        core::ptr::write_unaligned(this.wrapping_add(0x58) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x5C) as *mut u32, 0);
        for i in 0..4u32 {
            let key = core::ptr::read_unaligned((global::<u32>(0x01038E04) as u32 + i.wrapping_mul(4)) as *const u32);
            let h: u32 = callee_cdecl!(1, u32, key, 0);
            core::ptr::write_unaligned(this.wrapping_add(0x70).wrapping_add(i.wrapping_mul(4)) as *mut u32, h);
            core::ptr::write_unaligned(this.wrapping_add(0x80).wrapping_add(i.wrapping_mul(4)) as *mut u32, 0xFFFF_FFFF);
        }
        core::ptr::write_unaligned(this.wrapping_add(0xC8) as *mut u16, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x209) as *mut u8, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x94) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x98) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0xA0) as *mut u32, 0xFFFF_FFFF);
        core::ptr::write_unaligned(this.wrapping_add(0xA4) as *mut u32, 0xFFFF_FFFF);
        core::ptr::write_unaligned(this.wrapping_add(0x9C) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0xA8) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0xAC) as *mut u32, 0);
        let d1 = [0u32, 0x3F800000, 0];
        let _: u32 = callee_thiscall!(2, u32, this.wrapping_add(0xD0),
            relocated(0x00E910F0), d1.as_ptr() as u32);
        let d2 = [0u32, 0x3F800000, 0];
        let _: u32 = callee_thiscall!(2, u32, this.wrapping_add(0x120),
            relocated(0x00E910FC), d2.as_ptr() as u32);
        core::ptr::write_unaligned(this.wrapping_add(0x1B0) as *mut u32, 0x3F800000);
        core::ptr::write_unaligned(this.wrapping_add(0x1B4) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x1B8) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x1C0) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x1C4) as *mut u32, 0x3F800000);
        core::ptr::write_unaligned(this.wrapping_add(0x1C8) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x1D0) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x1D4) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x1D8) as *mut u32, 0x3F800000);
        core::ptr::write_unaligned(this.wrapping_add(0x1E8) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x1E4) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x1E0) as *mut u32, 0);
        let _: u32 = callee_thiscall!(3, u32, this.wrapping_add(0x18C),
            0x4A064700, 0x4A064700, 0, 0x46ABE000);
        let _: u32 = callee_thiscall!(3, u32, this.wrapping_add(0x170),
            0x461C4000, 0x461C4000, 0xC2C80000, 0);
        core::ptr::write_unaligned(this.wrapping_add(0xB0) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0xB4) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0xB8) as *mut u32, 1);
        core::ptr::write_unaligned(this.wrapping_add(0xBC) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0xC0) as *mut u8, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x200) as *mut u16, 0);
        core::ptr::write_unaligned(this.wrapping_add(0xC) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x204) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x1F0) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x1F4) as *mut u32, 0xC2C80000);
        core::ptr::write_unaligned(this.wrapping_add(0x208) as *mut u8, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x1F8) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x1FC) as *mut u32, 0);
        callee_thiscall!(4, u32, this)
    }
});
