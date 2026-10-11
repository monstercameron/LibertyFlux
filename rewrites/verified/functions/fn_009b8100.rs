// original: 0x009B8100 NativeImpl_SET_CAMERA_BEGIN_CAM_COMMANDS_REQUIRED
/// Stores one when the low byte is zero at instance offset 0x533.
lf_checker_rt::export!(thiscall, rw_009b8100(this: u32, arg0: u32) -> u32 {
    const FLAG_OFFSET: u32 = 0x533;
    let value: u8 = (u32::from((arg0 as u8) == 0)) as u8;
    unsafe { (this.wrapping_add(FLAG_OFFSET) as *mut u8).write(value); }
    0
});
