// original: 0x00898560 input_node_flag_test_b

/// Tests a node of the input graph, as the sibling test does, with the kind read from bits 2 and 3
/// of the 32-bit word at +0x4C. Kind 2 is a link: its 16-bit index at +0x50 names a node in the node
/// table, whose base is the global at 0x115F810; 0xFFFF means no node and the result is false. The
/// linked node is tested with the same function (one recursive step, answered by the checker). Kind 1
/// is true, and every other kind is false. The result is a boolean in AL. The index is signed (16 bits,
/// sign extended) and the node stride is 0x70 bytes.
lf_checker_rt::export!(thiscall, rw_00898560(this: u32) -> u8 {
    unsafe {
        const FLAGS_FIELD: u32 = 0x4C;
        const INDEX_FIELD: u32 = 0x50;
        const KIND_SHIFT: u32 = 2;
        const KIND_NODE: u32 = 2;
        const KIND_LEAF: u32 = 1;
        const NODE_STRIDE: i32 = 0x70;
        const NODE_BASE: u32 = 0x0115_F810;
        let kind = ((this.wrapping_add(FLAGS_FIELD) as *const u32).read_unaligned() >> KIND_SHIFT) & 3;
        if kind == KIND_NODE {
            // The index is a signed 16-bit field; 0xFFFF means no node.
            let index_bits = (this.wrapping_add(INDEX_FIELD) as *const u16).read_unaligned();
            if index_bits == 0xFFFF {
                0
            } else {
                let index = i32::from(index_bits as i16);
                let base = lf_checker_rt::global::<u32>(NODE_BASE).read();
                let child = base.wrapping_add(index.wrapping_mul(NODE_STRIDE) as u32);
                let found = lf_checker_rt::callee_thiscall!(1, u8, child);
                (found != 0) as u8
            }
        } else {
            (kind == KIND_LEAF) as u8
        }
    }
});
