// original: 0x008923D0 audsound_init_object
/// Forwards four arguments to the base initializer, then inits the fields.
///
/// Calls the base callee (thiscall, four stack words) with (`this`, a, b, c,
/// d), whose answer is discarded. Then copies the dword at `this+4` to
/// `this+0x7c`; zeroes the word at 0x4c, the dwords at 0x50, 0x54, 0x58, 0x5c,
/// 0x60 and 0x78 and the byte at 0x4e; sets the dwords at 0x68 and 0x6c to
/// all-ones; writes all-ones at 0x70 and 0x74 with zero dwords at 0x80 and
/// 0x84 (two loop iterations); sets the dword at 0x28 to 2, the byte at 0x48
/// to 1, the dwords at 0x24 to 1 and at 0x98 and 0x94 to 0. Returns `this`.
/// Original: 0x008923D0 (thiscall, four stack words; true size 167 bytes,
/// the batch list says 164).
export!(thiscall, rw_008923D0(this: *mut u8, a: u32, b: u32, c: u32, d: u32) -> u32 {
    unsafe {
        const BASE_INIT: u32 = 1;
        let _: u32 = callee_thiscall!(BASE_INIT, u32, this as u32, a, b, c, d);
        let v4 = *(this.add(4) as *const u32);
        *(this.add(0x7c) as *mut u32) = v4;
        *(this.add(0x4c) as *mut u16) = 0;
        *(this.add(0x68) as *mut u32) = 0xffff_ffff;
        *(this.add(0x6c) as *mut u32) = 0xffff_ffff;
        *(this.add(0x50) as *mut u32) = 0;
        *(this.add(0x54) as *mut u32) = 0;
        *(this.add(0x58) as *mut u32) = 0;
        *(this.add(0x5c) as *mut u32) = 0;
        *(this.add(0x60) as *mut u32) = 0;
        *this.add(0x4e) = 0;
        *(this.add(0x78) as *mut u32) = 0;
        for k in 0..2usize {
            *(this.add(0x70 + k * 4) as *mut u32) = 0xffff_ffff;
            *(this.add(0x80 + k * 4) as *mut u32) = 0;
        }
        *(this.add(0x28) as *mut u32) = 2;
        *this.add(0x48) = 1;
        *(this.add(0x24) as *mut u32) = 1;
        *(this.add(0x98) as *mut u32) = 0;
        *(this.add(0x94) as *mut u32) = 0;
        this as u32
    }
});
