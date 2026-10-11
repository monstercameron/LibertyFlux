// original: 0x00B11EF0 CHeli::vf6

/// Release each queued entry from the entity's linked resource list. The
/// encoded state value 0x40 forwards the object directly to a tail helper.
/// Otherwise, each node may unlink a nested allocation, pass that allocation
/// to the first manager, unlink the node from the object's list, and pass the
/// node to the second manager. The two manager addresses and tail helper are
/// supplied by the checker as stand-ins. This is a no-argument thiscall.
lf_checker_rt::export!(thiscall, rw_00b11ef0(this: u32) -> () {
    const STATE: u32 = 0x28;
    const QUEUE_HEAD: u32 = 0x70;
    const FIRST_MANAGER_VA: u32 = 0x012B_4160;
    const SECOND_MANAGER_VA: u32 = 0x0179_D10C;
    const DIRECT_STATE: u32 = 0x40;
    const STATE_MASK: u32 = 0x3C0;

    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let state = read_u32(this.wrapping_add(STATE)) & STATE_MASK;
        if state == DIRECT_STATE {
            let _ = lf_checker_rt::callee_thiscall!(3, u32, this);
            return;
        }

        let mut node = read_u32(this.wrapping_add(QUEUE_HEAD));
        while node != 0 {
            let allocation_links = read_u32(node);
            let allocation = read_u32(node.wrapping_add(4));

            if read_u32(allocation_links) == allocation {
                write_u32(allocation_links, read_u32(allocation.wrapping_add(4)));
            }
            let allocation_next = read_u32(allocation.wrapping_add(8));
            if allocation_next != 0 {
                write_u32(
                    allocation_next.wrapping_add(4),
                    read_u32(allocation.wrapping_add(4)),
                );
            }
            let allocation_previous = read_u32(allocation.wrapping_add(4));
            if allocation_previous != 0 {
                write_u32(
                    allocation_previous.wrapping_add(8),
                    allocation_next,
                );
            }

            let first_manager = read_u32(lf_checker_rt::global::<u32>(FIRST_MANAGER_VA) as u32);
            let _ = lf_checker_rt::callee_thiscall!(1, u32, first_manager, allocation);

            let next_node = read_u32(node.wrapping_add(0x10));
            if read_u32(this.wrapping_add(QUEUE_HEAD)) == node {
                write_u32(this.wrapping_add(QUEUE_HEAD), next_node);
            }
            let previous_node = read_u32(node.wrapping_add(0x0C));
            if previous_node != 0 {
                write_u32(previous_node.wrapping_add(0x10), next_node);
            }
            if next_node != 0 {
                write_u32(next_node.wrapping_add(0x0C), previous_node);
            }

            let second_manager = read_u32(lf_checker_rt::global::<u32>(SECOND_MANAGER_VA) as u32);
            let _ = lf_checker_rt::callee_thiscall!(2, u32, second_manager, node);
            node = next_node;
        }
    }
});
