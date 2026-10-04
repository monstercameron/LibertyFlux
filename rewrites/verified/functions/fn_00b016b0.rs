// original: 0x00b016b0 reinit_two_subobjects
/// Re-initialise the two strided sub-objects of this object in order.
export!(thiscall, rw_00b016b0(this: u32) -> u32 {
    unsafe {
        let mut slot = this.wrapping_add(0x424);
        let mut i = 0;
        while i < 2 {
            callee_thiscall!(1, u32, slot);
            slot = slot.wrapping_add(0x24);
            i += 1;
        }
        0
    }
});
