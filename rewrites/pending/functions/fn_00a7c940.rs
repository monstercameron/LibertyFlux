// original: 0x00a7c940 find_flagged_node
// Find the first node in the +0x120 chain (starting from the head the
// callee returns) whose flags read (+0x13c & 0x0C) == 0x0C with bit 1
// clear. Returns the node address, or null when the chain ends first.
export!(thiscall, rw_s13_00a7c940(this: u32) -> u32 {
    unsafe {
        let head: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let mut node = head(this);
        while node != 0 {
            let flags = *((node + 0x13C) as *const u8);
            if flags & 0x04 != 0 && flags & 0x08 != 0 && flags & 0x02 == 0 {
                return node;
            }
            node = *((node + 0x120) as *const u32);
        }
        0
    }
});
