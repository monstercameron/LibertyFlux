// original: 0x00C798E0 module_registry_register

/// Append a module record to the registry at `this`. Copies the NUL-terminated name to slot count*60 + 0xc, stores the four words at the slot's +0x0, +0x4, +0x8 and +0x38, bumps the count at +0x12c and returns the previous count. Original: thiscall, five stack words.
lf_checker_rt::export!(thiscall, rw_c798e0(this: u32, name: u32, w0: u32, w1: u32, w2: u32, w3: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr16(address: u32, value: u16) { unsafe { (address as *mut u16).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
        #[inline(always)]
        unsafe fn wr8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }

        const SLOT: u32 = 60;
                const NAMES: u32 = 0x0c;
                const COUNT: u32 = 0x12c;
                let count = rd32(this.wrapping_add(COUNT));
                let slot = this.wrapping_add(count.wrapping_mul(SLOT));
                let mut off = 0u32;
                loop {
                    let b = rd8(name.wrapping_add(off));
                    wr8(slot.wrapping_add(NAMES).wrapping_add(off), b);
                    if b == 0 { break; }
                    off += 1;
                }
                wr32(slot.wrapping_add(0x00), w0);
                wr32(slot.wrapping_add(0x04), w1);
                wr32(slot.wrapping_add(0x08), w2);
                wr32(slot.wrapping_add(0x38), w3);
                wr32(this.wrapping_add(COUNT), count.wrapping_add(1));
                count
        }
});
