// original: 0x00A6CC90 global_list_clear

/// Walks the process-wide linked list, marks each node's second word, and
/// calls the node cleanup helper.
///
/// Each node stores the helper's first argument at offset `0` and the next
/// node at `+8`; the word at `+4` is set to `u32::MAX` before the call. The
/// helper receives the first word and a pointer to `+4` as two cdecl stack
/// arguments. The function returns the last helper result, or leaves the
/// incoming result unspecified for an empty list; its checked contract uses
/// a nonempty list. ECX is not read.
lf_checker_rt::export!(thiscall, rw_00a6cc90(this: u32) -> u32 {
    unsafe {
        const LIST_HEAD_VA: u32 = 0x12F9_D3C;
        const NODE_VALUE: u32 = 0;
        const NODE_MARKER: u32 = 4;
        const NODE_NEXT: u32 = 8;
        const CLEAN_NODE: u32 = 1;

        let mut node = lf_checker_rt::global::<u32>(LIST_HEAD_VA).read();
        let mut last_result = 0;
        while node != 0 {
            let value = (node as *const u32).add((NODE_VALUE / 4) as usize).read_unaligned();
            let marker = node.wrapping_add(NODE_MARKER);
            (marker as *mut u32).write_unaligned(u32::MAX);
            last_result = lf_checker_rt::callee_cdecl!(CLEAN_NODE, u32, value, marker);
            node = ((node + NODE_NEXT) as *const u32).read_unaligned();
        }
        let _ = this;
        last_result
    }
});
