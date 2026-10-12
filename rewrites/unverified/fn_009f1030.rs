// original: 0x009F1030 CPed::clear_speech_slot (proposed)

/// Clear the pending speech marker and either detach the current timing node
/// or queue that detach while the current thread is inside the protected
/// timing section. The node slot is at `this + 0xD20`; the marker is at
/// `this + 0xD24`. The thread-local control word is read at offset `0x8D0`
/// from the pointer selected by the process's TLS index. When its bit 3 is
/// clear, the detach helper receives the node and the address of its slot.
/// When bit 3 is set, a three-word deferred record is appended unless the
/// unsigned queue count has reached 256. This is a 32-bit thiscall method.
lf_checker_rt::export!(thiscall, rw_009F1030(this: u32) -> () {
    unsafe {
        const NODE_SLOT_OFFSET: usize = 0xD20;
        const MARKER_OFFSET: usize = 0xD24;
        const TLS_FLAGS_OFFSET: usize = 0x8D0;
        const TLS_INDEX_VA: u32 = 0x017A_BA14;
        const QUEUE_COUNT_VA: u32 = 0x0116_1914;
        const QUEUE_RECORDS_VA: u32 = 0x0116_1918;
        const MAX_QUEUED_DETACHES: u32 = 0x100;
        const QUEUE_RECORD_WORDS: usize = 3;
        const ACTIVE_TIMING_BIT: u32 = 1 << 3;
        const DETACH_CALLEE_ID: u32 = 1;

        let ped = this as *mut u8;
        let node_slot = ped.add(NODE_SLOT_OFFSET) as *mut u32;
        let marker = ped.add(MARKER_OFFSET) as *mut u32;

        if marker.read() != 0 {
            marker.write(0);
        }

        let node = node_slot.read();
        if node == 0 {
            return;
        }

        let tls_index = lf_checker_rt::global::<u32>(TLS_INDEX_VA).read() as usize;
        let thread_data = lf_checker_rt::tls_slot(tls_index);
        let thread_flags = (thread_data as *const u8)
            .add(TLS_FLAGS_OFFSET)
            .cast::<u32>()
            .read();

        if thread_flags & ACTIVE_TIMING_BIT == 0 {
            let _ = lf_checker_rt::callee_thiscall!(
                DETACH_CALLEE_ID,
                u32,
                node,
                node_slot as u32,
            );
            return;
        }

        let queue_count = lf_checker_rt::global::<u32>(QUEUE_COUNT_VA);
        let count = queue_count.read();
        if count >= MAX_QUEUED_DETACHES {
            return;
        }

        let record = lf_checker_rt::global::<u32>(QUEUE_RECORDS_VA)
            .add(count as usize * QUEUE_RECORD_WORDS);
        record.write(node);
        record.add(1).write(node_slot as u32);
        (record.add(2) as *mut u8).write(1);
        queue_count.write(count + 1);
    }
});
