// original: 0x0065a780 rage::fragHeapAllocator::vf18
use lf_checker_rt::{export, global};

/// Return true when the candidate address lies in any allocator-owned node
/// interval. The global slot points to a manager whose first word is the
/// root node. Each node stores its unsigned end at +0x04, next link at +0x10,
/// and start at its own address plus +0x20. ECX is unused and the one stack
/// argument is returned as AL; the callee removes that argument.
export!(thiscall, rw_0065a780(_this: *mut u8, candidate: u32) -> u8 {
    const ROOT_SLOT_VA: u32 = 0x01BB_37D4;
    const NODE_END: u32 = 0x04;
    const NODE_NEXT: u32 = 0x10;
    const NODE_START: u32 = 0x20;
    const ROOT_NODE_OFFSET: u32 = 0x00;

    unsafe {
        let manager = global::<u32>(ROOT_SLOT_VA).read_unaligned();
        let mut node = (manager.wrapping_add(ROOT_NODE_OFFSET) as *const u32).read_unaligned();
        while node != 0 {
            let start = node.wrapping_add(NODE_START);
            if candidate < start {
                node = (node.wrapping_add(NODE_NEXT) as *const u32).read_unaligned();
                continue;
            }
            let end = (node.wrapping_add(NODE_END) as *const u32).read_unaligned();
            if candidate < end {
                return 1;
            }
            node = (node.wrapping_add(NODE_NEXT) as *const u32).read_unaligned();
        }
        0
    }
});
