// original: 0x00b36a80 mode_flag_or_pending_count
/// Return 0 when the mode flag byte is set, else whether the pending count
/// is positive (signed). Leaf over two globals.
lf_checker_rt::export!(cdecl, rw_b36a80() -> u32 {
    unsafe {
        if lf_checker_rt::global::<u8>(0x16b7bf2).read() != 0 {
            return 0;
        }
        (lf_checker_rt::global::<u32>(0x16b7c98).read() as i32 > 0) as u32
    }
});
