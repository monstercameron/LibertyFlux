// original: 0x00AF55F0 pad_is_device_enabled

/// Test the enabled byte for a device slot referenced by the controller. The
/// slot number is multiplied by the 0x98-byte device-record stride and added
/// to the relocated device table. The result is consumed from AL.
lf_checker_rt::export!(thiscall, rw_00af55f0(controller: u32) -> u32 {
    const DEVICE_SLOT: u32 = 0x00;
    const DEVICE_RECORD_STRIDE: u32 = 0x98;
    const DEVICE_ENABLED_TABLE: u32 = 0x018B7E48;

    unsafe {
        let slot = (controller.wrapping_add(DEVICE_SLOT) as *const u32).read_unaligned();
        let enabled_va = DEVICE_ENABLED_TABLE.wrapping_add(slot.wrapping_mul(DEVICE_RECORD_STRIDE));
        u32::from(lf_checker_rt::global::<u8>(enabled_va).read_volatile() != 0)
    }
});
