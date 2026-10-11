// original: 0x009B80F0 NativeImpl_SET_CAMERA_AUTO_SCRIPT_ACTIVATION
/// Stores the low-byte activation setting at instance offset 0x534.
lf_checker_rt::export!(thiscall, rw_009b80f0(this: u32, arg0: u32) -> u32 {
    const FLAG_OFFSET: u32 = 0x534;
    let value: u8 = (arg0 as u8) as u8;
    unsafe { (this.wrapping_add(FLAG_OFFSET) as *mut u8).write(value); }
    0
});
