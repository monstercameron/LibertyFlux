// original: 0x00a29d30 camera_record_has_both_enable_flags

/// Return one when both bytes at offsets 0x1191 and 0x1190 are nonzero, and
/// return zero when either byte is zero.
lf_checker_rt::export!(thiscall, rw_00a29d30(this: u32) -> u32 {
    let later_flag = unsafe { (this.wrapping_add(0x1191) as *const u8).read() };
    let earlier_flag = unsafe { (this.wrapping_add(0x1190) as *const u8).read() };
    u32::from(later_flag != 0 && earlier_flag != 0)
});
