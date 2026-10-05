// original: 0x00a453a0 vehicle_copy_extra_state
/// Copy a block of tuning/state fields from the source object to `dest`.
///
/// `this` is the source, the stack word is the destination (thiscall, one
/// stack word). First calls the destination's slot-1 virtual (`[dest+4]`,
/// callee 1, thiscall, three stack words: source, 0, 0), intercepted through
/// a planted vtable. Then copies bytes `+0x41..0x44` to `+0xF94..0xF97`,
/// calls the refresher (callee 2, thiscall, no stack arguments) on `dest`,
/// then copies word `+0x46` to `+0xF48`, byte `+0x48` to `+0x1070`, dwords
/// `+0x4C/0x50/0x54` to `+0x1088/0x1078/0x107C`, qword `+0x58` to `+0xF14`,
/// dword `+0x60` to `+0xF1C` and word `+0x64` to `+0xF20`. Returns the last
/// copied word, zero-extended.
export!(thiscall, rw_00a453a0(this: u32, dest: u32) -> u32 {
    unsafe {
        const SRC_B4: u32 = 0x41;
        const DST_B4: u32 = 0xf94;
        const SRC_W1: u32 = 0x46;
        const DST_W1: u32 = 0xf48;
        const SRC_B1: u32 = 0x48;
        const DST_B1: u32 = 0x1070;
        const VT_SLOT: u32 = 4;
        let vt = (dest as *const u32).read_unaligned();
        let faddr = (vt.wrapping_add(VT_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(faddr as usize);
        f(dest, this, 0, 0);
        ((dest.wrapping_add(DST_B4)) as *mut u8).write(((this.wrapping_add(SRC_B4)) as *const u8).read());
        ((dest.wrapping_add(DST_B4 + 1)) as *mut u8)
            .write(((this.wrapping_add(SRC_B4 + 1)) as *const u8).read());
        ((dest.wrapping_add(DST_B4 + 2)) as *mut u8)
            .write(((this.wrapping_add(SRC_B4 + 2)) as *const u8).read());
        ((dest.wrapping_add(DST_B4 + 3)) as *mut u8)
            .write(((this.wrapping_add(SRC_B4 + 3)) as *const u8).read());
        let _: u32 = callee_thiscall!(2, u32, dest);
        ((dest.wrapping_add(DST_W1)) as *mut u16)
            .write_unaligned(((this.wrapping_add(SRC_W1)) as *const u16).read_unaligned());
        ((dest.wrapping_add(DST_B1)) as *mut u8).write(((this.wrapping_add(SRC_B1)) as *const u8).read());
        ((dest.wrapping_add(0x1088)) as *mut u32)
            .write_unaligned(((this.wrapping_add(0x4c)) as *const u32).read_unaligned());
        ((dest.wrapping_add(0x1078)) as *mut u32)
            .write_unaligned(((this.wrapping_add(0x50)) as *const u32).read_unaligned());
        ((dest.wrapping_add(0x107c)) as *mut u32)
            .write_unaligned(((this.wrapping_add(0x54)) as *const u32).read_unaligned());
        ((dest.wrapping_add(0xf14)) as *mut u64)
            .write_unaligned(((this.wrapping_add(0x58)) as *const u64).read_unaligned());
        ((dest.wrapping_add(0xf1c)) as *mut u32)
            .write_unaligned(((this.wrapping_add(0x60)) as *const u32).read_unaligned());
        let tail = ((this.wrapping_add(0x64)) as *const u16).read_unaligned();
        ((dest.wrapping_add(0xf20)) as *mut u16).write_unaligned(tail);
        tail as u32
    }
});
