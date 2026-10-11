// original: 0x00627970 network_record_copy_chunks_with_mode

/// Copy each 64-byte record after the owner's 64-byte header into a by-value helper call, with
/// one mode byte supplied through the caller's stack pointer. ECX is the owner, EDX is the end
/// pointer, and the third argument points to the mode byte. The loop processes complete 64-byte
/// chunks until the source pointer equals the end pointer. Three 16-bit fields are zero-extended;
/// untouched high halves in the outgoing words come from the defined zero stack fill. The helper
/// receives the owner and current source in ECX and EDX, followed by sixteen payload words and the
/// zero-extended mode. The check compares every outgoing word and both sides' heap state; its
/// stack-content check is disabled because the function uses a 64-byte local call buffer.
lf_checker_rt::export!(fastcall, rw_00627970(owner: u32, end: u32, mode_pointer: u32) -> () {
    unsafe {
        const HEADER_BYTES: u32 = 0x40;
        const CHUNK_BYTES: u32 = 0x40;
        const RECORD_COPY: u32 = 1;

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let mode = (mode_pointer as *const u8).read_unaligned() as u32;
        let mut source = owner.wrapping_add(HEADER_BYTES);
        while source != end {
            let mut payload = [0u32; 16];
            payload[0] = read_u32(source);
            payload[1] = read_u32(source.wrapping_add(4));
            payload[2] = read_u32(source.wrapping_add(8));
            payload[3] = read_u32(source.wrapping_add(12));
            payload[4] = read_u32(source.wrapping_add(16));
            payload[5] = read_u32(source.wrapping_add(20));
            payload[6] = read_u16(source.wrapping_add(24)) as u32;
            payload[7] = read_u32(source.wrapping_add(28));
            payload[8] = read_u16(source.wrapping_add(32)) as u32;
            payload[9] = read_u32(source.wrapping_add(36));
            payload[10] = read_u16(source.wrapping_add(40)) as u32;
            payload[11] = read_u32(source.wrapping_add(44));
            payload[12] = read_u32(source.wrapping_add(48));
            payload[13] = read_u32(source.wrapping_add(52));
            payload[14] = read_u32(source.wrapping_add(56));
            payload[15] = read_u32(source.wrapping_add(60));

            let _: u32 = lf_checker_rt::callee_fastcall!(
                RECORD_COPY, u32, owner, source,
                payload[0], payload[1], payload[2], payload[3],
                payload[4], payload[5], payload[6], payload[7],
                payload[8], payload[9], payload[10], payload[11],
                payload[12], payload[13], payload[14], payload[15], mode
            );
            source = source.wrapping_add(CHUNK_BYTES);
        }
    }
});
