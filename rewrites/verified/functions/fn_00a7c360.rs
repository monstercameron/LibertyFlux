// original: 0x00a7c360 bu_task_collect_subtree
/// Writes the node into the output slot, then recurses over its child chain,
/// appending each descendant. Returns the number of nodes collected.
/// (The checker stubs the recursive call; both sides observe the same
/// scripted child counts, so the accumulation logic itself is what is proven.)
export!(thiscall, rw_00a7c360(obj: *mut u8, out: *mut u32) -> u32 {
    unsafe {
        *out = obj as u32;
        let mut node = *((obj as *const u8).add(0x124) as *const u32);
        let mut count: u32 = 1;
        while node != 0 {
            let slot = (out as u32).wrapping_add(count.wrapping_mul(4));
            let n = callee_stdcall!(1, u32, slot);
            node = *((node + 0x11c) as *const u32);
            count = count.wrapping_add(n);
        }
        count
    }
});
