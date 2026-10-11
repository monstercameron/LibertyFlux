// original: 0x0063D4E0 rage::ptxModel::vf9

/// Visit nodes whose blend value does not exceed the thread's threshold and
/// link each eligible node into the per-thread chain stored in `context`.
///
/// `context[0]` is the source-list head. The thread index is the byte at
/// offset `1` in the record reached through the global thread-record pointer.
/// For that index, `context` stores a count at `0x14 + index*4`, the first
/// eligible node at `4 + index*4`, and the latest node at `0x0c + index*4`.
/// Each node links to the next source node at offset `0`, has its threshold
/// value at `0xf0`, and receives a per-thread link at `8 + index*4`. The
/// helper is called through the checker runtime with scripted writes. The
/// method's return value is not compared.
lf_checker_rt::export!(thiscall, rw_0063d4e0(_unused_this: u32, context: u32) -> u32 {
    const THREAD_RECORD_POINTER: u32 = 0x01BB6678;
    const BLEND_THRESHOLD: u32 = 0x00FE88E8;
    const NODE_BLEND_VALUE: u32 = 0xf0;
    const NODE_THREAD_LINK: u32 = 8;

    unsafe {
        let thread_record = lf_checker_rt::global::<u32>(THREAD_RECORD_POINTER).read();
        let threshold = lf_checker_rt::global::<f32>(BLEND_THRESHOLD).read();
        let mut node = (context as *const u32).read();
        while node != 0 {
            let next_node = (node as *const u32).read();
            let blend_value = (node.wrapping_add(NODE_BLEND_VALUE) as *const f32).read();
            if !blend_value.is_nan() && !(threshold < blend_value) {
                let _ = lf_checker_rt::callee_stdcall!(1, u32, node);
                let thread_index = (thread_record.wrapping_add(1) as *const u8).read() as u32;
                let index_bytes = thread_index.wrapping_mul(4);
                let count = context.wrapping_add(0x14).wrapping_add(index_bytes) as *mut u32;
                count.write((*count).wrapping_add(1));

                let first_node = context.wrapping_add(4).wrapping_add(index_bytes) as *mut u32;
                if first_node.read() == 0 {
                    (context.wrapping_add(0x0c).wrapping_add(index_bytes) as *mut u32).write(0);
                    first_node.write(node);
                }
                let last_node = (context.wrapping_add(0x0c).wrapping_add(index_bytes) as *const u32).read();
                if last_node != 0 {
                    (last_node.wrapping_add(index_bytes).wrapping_add(NODE_THREAD_LINK) as *mut u32).write(node);
                }
                (node.wrapping_add(index_bytes).wrapping_add(NODE_THREAD_LINK) as *mut u32).write(0);
                (context.wrapping_add(0x0c).wrapping_add(index_bytes) as *mut u32).write(node);
            }
            node = next_node;
        }
        0
    }
});
