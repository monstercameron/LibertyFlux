// original: 0x00627550 network_record_forward_range_chunks

/// Forward a byte range to the range parser in 1 KiB chunks. The initial pointer and range end
/// arrive in ECX and EDX; the third input is a stack word whose low byte selects the parser mode.
/// A signed, 64-byte-rounded difference chooses the short parser path or the 1 KiB chunk path.
/// The short path returns the mode byte directly when the pointers are equal, and otherwise
/// returns the parser's low-byte result. The chunk path parses the first 1 KiB, then forwards the
/// remaining range together with the original pointer and mode. The function exposes AL.
lf_checker_rt::export!(fastcall, rw_00627550(start: u32, end: u32, mode_word: u32) -> u32 {
    unsafe {
        const CHUNK_BYTES: u32 = 0x400;
        const ROUND_MASK: u32 = !0x3F;
        const PARSE_CHUNK: u32 = 1;
        const FORWARD_REMAINDER: u32 = 2;

        let aligned_distance = end.wrapping_sub(start) & ROUND_MASK;
        if (aligned_distance as i32) <= CHUNK_BYTES as i32 {
            if start == end {
                return mode_word & 0xFF;
            }

            let local_mode_word = (start & !0xFF) | (mode_word & 0xFF);
            let mode_address = (&local_mode_word as *const u32 as usize) as u32;
            return lf_checker_rt::callee_fastcall!(
                PARSE_CHUNK,
                u32,
                start,
                end,
                mode_address
            );
        }

        let first_end = start.wrapping_add(CHUNK_BYTES);
        let local_mode_word = mode_word;
        let mode_address = (&local_mode_word as *const u32 as usize) as u32;
        let _ = lf_checker_rt::callee_fastcall!(
            PARSE_CHUNK,
            u32,
            start,
            first_end,
            mode_address
        );
        lf_checker_rt::callee_fastcall!(
            FORWARD_REMAINDER,
            u32,
            first_end,
            end,
            start,
            mode_word
        )
    }
});
