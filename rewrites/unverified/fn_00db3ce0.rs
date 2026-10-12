// original: 0x00DB3CE0 UIGalleryFileViewer::vf28

/// Store the incoming value in a word at +0x14 of the receiver, then forward the full 32-bit argument to the corresponding virtual method of every child. The child pointer array starts at +0x1D4 and its unsigned 16-bit count is at +0x1D8. A null child faults when its vtable is read, matching the original. This thiscall method pops its one stack argument and returns the computed end of the child array in EAX.
lf_checker_rt::export!(thiscall, rw_00db3ce0(this: u32, value: u32) -> u32 {
    unsafe {
        const FIELD: u32 = 0x14;
        const CHILD_ARRAY: u32 = 0x1d4;
        const CHILD_COUNT: u32 = 0x1d8;
        const CHILD_VTABLE_SLOT: u32 = 0x70;

        ((this + FIELD) as *mut u32).write_unaligned(value);
        let children = ((this + CHILD_ARRAY) as *const u32).read_unaligned();
        let count = ((this + CHILD_COUNT) as *const u16).read_unaligned() as u32;
        let end = children.wrapping_add(count.wrapping_mul(4));
        let mut cursor = children;
        while cursor != end {
            let child = (cursor as *const u32).read_unaligned();
            let vtable = (child as *const u32).read_unaligned();
            let target = ((vtable + CHILD_VTABLE_SLOT) as *const u32).read_unaligned();
            let callback: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let _ = callback(child, value);
            cursor = cursor.wrapping_add(4);
        }
        end
    }
});
