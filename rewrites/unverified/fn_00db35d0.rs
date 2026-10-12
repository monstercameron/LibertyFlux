// original: 0x00DB35D0 UIFrame::vf120

/// Ask the receiver's virtual child-list predicate, then return the child
/// pointer at the requested unsigned index when it is below the 16-bit child
/// count. The child array pointer is at +0x1D4 and the count at +0x1D8. The
/// predicate is called through the receiver's vtable at slot +0x1D4.
lf_checker_rt::export!(thiscall, rw_00db35d0(this: u32, target_index: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 0x1d4;
        const CHILD_ARRAY: u32 = 0x1d4;
        const CHILD_COUNT: u32 = 0x1d8;
        let vtable = (this as *const u32).read_unaligned();
        let predicate_address = (vtable.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned();
        let predicate: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(predicate_address as usize);
        if predicate(this) == 0 {
            return 0;
        }

        let child_array = ((this + CHILD_ARRAY) as *const u32).read_unaligned();
        let child_count = ((this + CHILD_COUNT) as *const u16).read_unaligned() as u32;
        let mut child_index = 0u32;
        while child_index < child_count {
            if child_index == target_index {
                let slot = child_array.wrapping_add(child_index.wrapping_mul(4));
                return (slot as *const u32).read_unaligned();
            }
            child_index = child_index.wrapping_add(1);
        }
        0
    }
});
