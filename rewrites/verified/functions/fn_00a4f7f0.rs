// original: 0x00a4f7f0 vehicle_find_slot_by_ptr (proposed)

/// Find the first active slot holding `want`, returning its address or zero.
///
/// Scans the 256 flag bytes at `this`: an odd flag marks an active slot, and
/// the slot's identity word sits at +0x148 in its 0xe0-byte row. The first
/// active slot whose word equals `want` yields its row address (+0x100 base);
/// no match yields zero. Thiscall, one stack word, address in eax.
lf_checker_rt::export!(thiscall, rw_00a4f7f0(this: u32, want: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0xe0;
        const ID_BASE: u32 = 0x148;
        const ROW_BASE: u32 = 0x100;
        const NSLOTS: u32 = 0x100;
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut i: u32 = 0;
        loop {
            if rd8(this.wrapping_add(i)) & 1 != 0 {
                let id = rd32(this.wrapping_add(ID_BASE).wrapping_add(i.wrapping_mul(ROW_STRIDE)));
                if id == want {
                    return this.wrapping_add(ROW_BASE).wrapping_add(i.wrapping_mul(ROW_STRIDE));
                }
            }
            i = i.wrapping_add(1);
            if !(i < NSLOTS) {
                return 0;
            }
        }
    }
});
