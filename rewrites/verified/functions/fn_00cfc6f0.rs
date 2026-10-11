// original: 0x00CFC6F0 CTaskComplexNewUseCover::CTaskComplexNewUseCover

/// Initialize a new-use-cover task from an optional owner, a three-word source vector, and a byte mode. The vector is copied to 0x20, the owner and mode are stored, fixed defaults are written, and a component constructor runs at 0x54. When the owner is nonnull, a direct helper receives it and the owner field address. Returns `this`.
lf_checker_rt::export!(thiscall, rw_cfc6f0(this: u32, owner: u32, source: u32, mode_word: u32) -> u32 {
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

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
                const VTABLE_VA: u32 = 0x00EE042C;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                for lane in 0..3u32 { wr32(this.wrapping_add(0x20 + lane * 4), rd32(source.wrapping_add(lane * 4))); }
                let flag = rd8(this.wrapping_add(0x36)) & 0xfe;
                wr8(this.wrapping_add(0x36), flag);
                wr32(this.wrapping_add(0x30), owner);
                wr8(this.wrapping_add(0x34), mode_word as u8);
                wr8(this.wrapping_add(0x35), 0x48);
                for off in [0x38u32, 0x3c, 0x40] { wr32(this.wrapping_add(off), 0); }
                wr32(this.wrapping_add(0x44), 0x4479_c000);
                for off in [0x48u32, 0x4c] { wr32(this.wrapping_add(off), 0); }
                wr8(this.wrapping_add(0x50), 0);
                let _ = lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(0x54));
                for off in [0x60u32, 0x64, 0x68, 0x70, 0x80, 0x84, 0x88] { wr32(this.wrapping_add(off), 0); }
                wr8(this.wrapping_add(0x74), 0);
                if owner != 0 { let _ = lf_checker_rt::callee_thiscall!(3, u32, owner, this.wrapping_add(0x30)); }
                this
        }
});
