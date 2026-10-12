// original: 0x00AF55F0 pad_is_device_enabled

/// Test the enabled byte for the device record referenced by the controller.
/// A null record is disabled. Its
/// slot number is multiplied by the 0x98-byte record stride and added to the
/// relocated device table. The result is consumed from AL.
lf_checker_rt::export!(thiscall, rw_00af55f0(controller: u32) -> u32 {
    const DEVICE_RECORD_POINTER: u32 = 0x00;
    const DEVICE_SLOT: u32 = 0x00;
    const DEVICE_RECORD_STRIDE: u32 = 0x98;
    const DEVICE_ENABLED_TABLE: u32 = 0x018B7E48;

    unsafe {
        let device = (controller.wrapping_add(DEVICE_RECORD_POINTER) as *const u32).read_unaligned();
        if device == 0 {
            return 0;
        }
        let slot = (device.wrapping_add(DEVICE_SLOT) as *const u32).read_unaligned();
        let enabled_va = DEVICE_ENABLED_TABLE.wrapping_add(slot.wrapping_mul(DEVICE_RECORD_STRIDE));
        u32::from(lf_checker_rt::global::<u8>(enabled_va).read_volatile() != 0)
    }
});
