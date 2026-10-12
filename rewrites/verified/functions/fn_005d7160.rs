// original: 0x005d7160 html_subtree_set_link_active
/// Store the low byte of `active` at byte offset +0xcd on `node`, then walk
/// its child pointer array at +0x0c for the u16 child count at +0x10. Each
/// child is processed by the same method with the original context pointer,
/// child pointer and active word. The method returns the child count in EAX
/// (zero when the node has no children) and thiscall pops both stack words.
lf_checker_rt::export!(thiscall, rw_005d7160(context: u32, node: u32, active: u32) -> u32 {
    const CHILD_LIST_OFFSET: u32 = 0x0c;
    const CHILD_COUNT_OFFSET: u32 = 0x10;
    const ACTIVE_BYTE_OFFSET: u32 = 0xcd;
    const SELF_CALLEE: u32 = 1;

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write_unaligned(value) }
    }

    unsafe {
        write_u8(node.wrapping_add(ACTIVE_BYTE_OFFSET), active as u8);
        let child_count = u32::from(read_u16(node.wrapping_add(CHILD_COUNT_OFFSET)));
        if child_count == 0 {
            return 0;
        }

        let child_list = read_u32(node.wrapping_add(CHILD_LIST_OFFSET));
        let mut child_index = 0u32;
        while child_index < child_count {
            let child = read_u32(child_list.wrapping_add(child_index.wrapping_mul(4)));
            let _ = lf_checker_rt::callee_thiscall!(
                SELF_CALLEE,
                u32,
                context,
                child,
                active
            );
            child_index = child_index.wrapping_add(1);
        }
        child_count
    }
});
