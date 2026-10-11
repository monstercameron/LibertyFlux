// original: 0x00A83ED0 CSequenceTaskInfo::append_sequence_state

/// Emit the sequence identifier at +0x14, sign-extended progress bytes at +0x18 and +0x19, and the unsigned recreation byte at +0x1A. Each value is forwarded to the logger in source order.
lf_checker_rt::export!(thiscall, rw_00a83ed0(this: u32, logger_context: u32) -> () {

    #[inline(always)]
    fn read_u8(base: u32, offset: u32) -> u8 {
        unsafe { core::ptr::read(base.wrapping_add(offset) as *const u8) }
    }
    #[inline(always)]
    fn read_u32(base: u32, offset: u32) -> u32 {
        unsafe { core::ptr::read_unaligned(base.wrapping_add(offset) as *const u32) }
    }
    #[inline(always)]
    fn double_words(value: f32) -> (u32, u32) {
        let bits = (value as f64).to_bits();
        (bits as u32, (bits >> 32) as u32)
    }

    const FIELD_SEQUENCE_ID: u32 = 0x14;
    const FIELD_PROGRESS_ONE: u32 = 0x18;
    const FIELD_PROGRESS_TWO: u32 = 0x19;
    const FIELD_RECREATE: u32 = 0x1A;
    const FORMAT_SEQUENCE_ID: u32 = 0x00EA0C54;
    const FORMAT_PROGRESS_ONE: u32 = 0x00EA0C68;
    const FORMAT_PROGRESS_TWO: u32 = 0x00EA0C7C;
    const FORMAT_RECREATE: u32 = 0x00EA0C90;
    let sequence_id = read_u32(this, FIELD_SEQUENCE_ID);
    let progress_one = read_u8(this, FIELD_PROGRESS_ONE) as i8 as i32 as u32;
    let progress_two = read_u8(this, FIELD_PROGRESS_TWO) as i8 as i32 as u32;
    let recreate = read_u8(this, FIELD_RECREATE) as u32;
    let _ = lf_checker_rt::callee_cdecl!(1, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_SEQUENCE_ID), sequence_id);
    let _ = lf_checker_rt::callee_cdecl!(1, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_PROGRESS_ONE), progress_one);
    let _ = lf_checker_rt::callee_cdecl!(1, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_PROGRESS_TWO), progress_two);
    let _ = lf_checker_rt::callee_cdecl!(1, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_RECREATE), recreate);
});
