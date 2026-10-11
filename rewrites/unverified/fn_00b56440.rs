// original: 0x00B56440 crmtManagerPriority_find_empty_active_node

/// Scans the manager's linked list at offset `0x1A30` for the first node
/// whose 16-bit state at offset `0x48` equals one and whose result link at
/// node offset `0x44` is zero. It advances the manager head to that node's
/// successor and returns the embedded result pointer at node offset four.
/// If no node matches, it returns zero and leaves the head unchanged. This
/// is a thiscall method with the manager in ECX and no stack arguments.
lf_checker_rt::export!(thiscall, rw_00b56440(manager: u32) -> u32 {
    const HEAD: u32 = 0x1a30;
    const NODE_STATE: u32 = 0x48;
    const RESULT_LINK: u32 = 0x40;
    const NEXT: u32 = 0x8c;
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }

    let mut node = unsafe { read_u32(manager.wrapping_add(HEAD)) };
    while node != 0 {
        let result = node.wrapping_add(4);
        let next = unsafe { read_u32(node.wrapping_add(NEXT)) };
        let active = unsafe { read_u16(node.wrapping_add(NODE_STATE)) } == 1;
        let link_is_empty = unsafe { read_u32(result.wrapping_add(RESULT_LINK)) } == 0;
        if active && link_is_empty {
            unsafe { write_u32(manager.wrapping_add(HEAD), next) };
            return result;
        }
        node = next;
    }
    0
});
