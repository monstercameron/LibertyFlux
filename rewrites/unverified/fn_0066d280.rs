// original: 0x0066D280 rb_tree_lower_bound

/// Return the first tree node whose unsigned key is at least the requested
/// key. `this` points to a tree header whose root is at offset zero; nodes
/// store their right link at `+4`, their left link at `+8`, and their key at
/// `+0x10`. The low bit on a right link is a tag and is cleared before
/// following it. The first stack argument points to a three-word output
/// record, the second points to the requested key, and the third is unused.
///
/// The output is `(key, node, tree)` for a match, `(0, 0, tree)` when no key
/// is large enough. The function returns the output-record pointer and uses
/// thiscall with three callee-cleaned stack words.
lf_checker_rt::export!(thiscall, rw_0066d280(this: u32, output: u32, query_ptr: u32, _unused: u32) -> u32 {
    unsafe {
        const TREE_ROOT: u32 = 0;
        const NODE_RIGHT_TAGGED: u32 = 4;
        const NODE_LEFT: u32 = 8;
        const NODE_KEY: u32 = 0x10;
        const RIGHT_LINK_TAG: u32 = 1;
        const OUTPUT_KEY: u32 = 0;
        const OUTPUT_NODE: u32 = 4;
        const OUTPUT_TREE: u32 = 8;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let query = read_word(query_ptr);
        let mut candidate = 0u32;
        let mut node = read_word(this + TREE_ROOT);

        while node != 0 {
            let node_key = read_word(node + NODE_KEY);
            if node_key >= query {
                candidate = node;
                node = read_word(node + NODE_LEFT);
            } else {
                node = read_word(node + NODE_RIGHT_TAGGED) & !RIGHT_LINK_TAG;
            }
        }

        write_word(output + OUTPUT_KEY, 0);
        write_word(output + OUTPUT_NODE, 0);
        write_word(output + OUTPUT_TREE, this);
        if candidate != 0 {
            write_word(output + OUTPUT_KEY, read_word(candidate + NODE_KEY));
            write_word(output + OUTPUT_NODE, candidate);
        }
        output
    }
});
