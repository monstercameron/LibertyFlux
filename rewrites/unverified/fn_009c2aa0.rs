// original: 0x009C2AA0 NativeImpl_GET_CUTSCENE_PED_POSITION

/// Clear the three output coordinates unless the request status permits a
/// lookup. For an enabled request, resolve the indexed record and copy its
/// position vector, following the nested vector pointer when present. If the
/// resolved record is absent, forward to the streaming vector helper. Return
/// the caller's output pointer on every path.
lf_checker_rt::export!(cdecl, rw_009c2aa0(output: u32, item_index: u32) -> u32 {
    unsafe {
        const REQUEST_STATUS: u32 = 0x0129_475c;
        const STREAMING_INDEX: u32 = 0x0129_4730;
        const CONTEXT_TABLE: u32 = 0x0129_3e58;
        const POSITION_TABLE: u32 = 0x0129_3e98;
        const RECORD_VECTOR: u32 = 0xfa98;
        const NESTED_VECTOR: u32 = 0x20;
        const NESTED_VECTOR_OFFSET: u32 = 0x30;
        const DIRECT_VECTOR: u32 = 0x10;
        const VECTOR_WORDS: usize = 3;

        let mut clear_output = || {
            let destination = output as usize as *mut u32;
            for word in 0..VECTOR_WORDS {
                destination.add(word).write_unaligned(0);
            }
        };

        let request_status = lf_checker_rt::global::<u32>(REQUEST_STATUS).read_unaligned();
        if request_status != 8 && request_status != 11 {
            clear_output();
            return output;
        }

        let streaming_index = lf_checker_rt::global::<u32>(STREAMING_INDEX).read_unaligned();
        let context_slot = CONTEXT_TABLE.wrapping_add(streaming_index.wrapping_mul(4));
        let context = lf_checker_rt::global::<u32>(context_slot).read_unaligned();
        let record_index_address = context
            .wrapping_add(RECORD_VECTOR)
            .wrapping_add(item_index.wrapping_mul(4));
        let record_index = (record_index_address as usize as *const u32).read_unaligned();

        if record_index == u32::MAX {
            clear_output();
            return output;
        }

        let position_slot = POSITION_TABLE.wrapping_add(record_index.wrapping_mul(4));
        let position_record = lf_checker_rt::global::<u32>(position_slot).read_unaligned();
        if position_record == 0 {
            let _ = lf_checker_rt::callee_cdecl!(1, u32, output, streaming_index);
            return output;
        }

        let nested = (position_record.wrapping_add(NESTED_VECTOR) as usize as *const u32)
            .read_unaligned();
        let vector = if nested != 0 {
            nested.wrapping_add(NESTED_VECTOR_OFFSET)
        } else {
            position_record.wrapping_add(DIRECT_VECTOR)
        };
        let source = vector as usize as *const u32;
        let destination = output as usize as *mut u32;
        for word in 0..VECTOR_WORDS {
            destination.add(word).write_unaligned(source.add(word).read_unaligned());
        }
        output
    }
});
