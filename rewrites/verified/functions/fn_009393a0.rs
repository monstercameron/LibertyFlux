// original: 0x009393A0 stream_id_resolve (proposed)

/// Resolve a streaming index to its record, past a range gate.
///
/// When `checked` is nonzero and `index` is at or past the counter, answers
/// the overflow record without searching. Otherwise looks the index up;
/// a null lookup answers null for an unchecked call and the missing record
/// for a checked one. A found object is measured and the node list is
/// searched for a node with that id, answering the node plus `0x20E`.
lf_checker_rt::export!(cdecl, rw_009393a0(index: u32, checked: u32) -> u32 {
    unsafe {
        const COUNTER: u32 = 1;
        const LOOKUP: u32 = 2;
        const MEASURE: u32 = 3;
        const HEAD_GLOBAL: u32 = 0x11A4EE0;
        const NEXT: u32 = 0x00;
        const ID: u32 = 0x0C;
        const RECORD_SKIP: u32 = 0x20E;
        const OVERFLOW_RECORD: u32 = 0xE87D30;
        const MISSING_RECORD: u32 = 0xE87D38;
        let flag = (checked & 0xFF) != 0;
        if flag {
            let cap: u32 = lf_checker_rt::callee_cdecl!(COUNTER, u32,);
            if index >= cap {
                return lf_checker_rt::relocated(OVERFLOW_RECORD);
            }
        }
        let obj: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, index);
        if obj == 0 {
            if !flag {
                return 0;
            }
            return lf_checker_rt::relocated(MISSING_RECORD);
        }
        let mut node = lf_checker_rt::global::<u32>(HEAD_GLOBAL).read_unaligned();
        loop {
            // NOTE: the measure call is inside the loop: once per node.
            let id: u32 = lf_checker_rt::callee_thiscall!(MEASURE, u32, obj);
            if ((node + ID) as *const u32).read_unaligned() == id {
                return node.wrapping_add(RECORD_SKIP);
            }
            node = ((node + NEXT) as *const u32).read_unaligned();
        }
    }
});
