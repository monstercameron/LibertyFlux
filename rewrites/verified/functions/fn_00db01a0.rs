// original: 0x00DB01A0 network_buffer_has_suffix

/// Compare the bytes before a candidate's final NUL with the end of a buffer.
///
/// The receiver stores a byte-buffer pointer at `+0` and its 16-bit length at
/// `+4`; `candidate` is a NUL-terminated byte string. The comparison uses the
/// candidate's full non-NUL length, returns false when that prefix is longer than
/// the buffer, and otherwise compares the prefix against the buffer tail in
/// reverse index order. An empty prefix succeeds without reading buffer data.
/// The original uses a signed length comparison and returns its boolean in AL.
lf_checker_rt::export!(thiscall, rw_00db01a0(buffer: u32, candidate: u32) -> u32 {
    const BUFFER_DATA_OFFSET: u32 = 0;
    const BUFFER_LENGTH_OFFSET: u32 = 4;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    let mut candidate_len = 0u32;
    loop {
        let byte = unsafe { read_u8(candidate.wrapping_add(candidate_len)) };
        if byte == 0 {
            break;
        }
        candidate_len = candidate_len.wrapping_add(1);
    }

    let compared_len = candidate_len;
    let buffer_len = unsafe { read_u16(buffer.wrapping_add(BUFFER_LENGTH_OFFSET)) as u32 };
    if (compared_len as i32) > (buffer_len as i32) {
        return 0;
    }

    let buffer_data = unsafe { read_u32(buffer.wrapping_add(BUFFER_DATA_OFFSET)) };
    let tail_start = buffer_data.wrapping_add(buffer_len.wrapping_sub(compared_len));
    let mut index = compared_len.wrapping_sub(1);
    loop {
        if (index as i32) < 0 {
            return 1;
        }
        let candidate_byte = unsafe { read_u8(candidate.wrapping_add(index)) };
        let buffer_byte = unsafe { read_u8(tail_start.wrapping_add(index)) };
        if candidate_byte != buffer_byte {
            return 0;
        }
        index = index.wrapping_sub(1);
    }
});
