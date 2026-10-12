// original: 0x00DB3620 UIFrame::vf121

/// Walk the receiver's child array in order and return the first zero-based
/// index whose child reports the requested value from its virtual method at
/// vtable slot +0x4C. Return 0xFFFFFFFF when no child matches. The child count
/// is an unsigned 16-bit value at +0x1D8; the array pointer is at +0x1D4.
lf_checker_rt::export!(thiscall, rw_00db3620(this: u32, requested_value: u32) -> u32 {
    unsafe {
        const CHILD_ARRAY: u32 = 0x1d4;
        const CHILD_COUNT: u32 = 0x1d8;
        const VTABLE_SLOT: u32 = 0x4c;
        let array = ((this + CHILD_ARRAY) as *const u32).read_unaligned();
        let count = ((this + CHILD_COUNT) as *const u16).read_unaligned() as u32;
        let end = array.wrapping_add(count.wrapping_mul(4));
        let mut cursor = array;
        let mut index = 0u32;
        while cursor != end {
            let child = (cursor as *const u32).read_unaligned();
            let vtable = (child as *const u32).read_unaligned();
            let method_address = (vtable.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned();
            let get_value: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(method_address as usize);
            if get_value(child) == requested_value {
                return index;
            }
            cursor = cursor.wrapping_add(4);
            index = index.wrapping_add(1);
        }
        0xFFFFFFFF
    }
});
