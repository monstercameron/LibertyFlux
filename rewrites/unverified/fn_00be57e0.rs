// original: 0x00BE57E0 task_child_value_for_type_111

/// Return the child object's word at offset 0x18 when its virtual type query
/// returns 0x111. A null child or any other type returns the all-bits-set
/// sentinel. The type method is reached through the child's planted vtable.
lf_checker_rt::export!(thiscall, rw_00be57e0(this: u32) -> u32 {
    unsafe {
        const CHILD_POINTER: u32 = 8;
        const TYPE_SLOT: u32 = 0x0c;
        const EXPECTED_TYPE: u32 = 0x111;
        const CHILD_VALUE: u32 = 0x18;
        let child = ((this + CHILD_POINTER) as *const u32).read_unaligned();
        if child == 0 {
            return u32::MAX;
        }
        let vtable = (child as *const u32).read_unaligned();
        let method = ((vtable + TYPE_SLOT) as *const u32).read_unaligned();
        let get_type: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(method as usize);
        if get_type(child) != EXPECTED_TYPE {
            u32::MAX
        } else {
            ((child + CHILD_VALUE) as *const u32).read_unaligned()
        }
    }
});
