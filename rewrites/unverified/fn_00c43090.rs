// original: 0x00C43090 CIntermezzoEventVaultWall::init_state
/// Initialize the observed vault-wall event fields: zero the timer, counters,
/// flags and links, and set three float-valued words to the exact bits for 1.0.
/// The thiscall method has no stack parameters or outgoing calls.
lf_checker_rt::export!(thiscall, rw_00c43090(this: u32) -> u32 {
    const ONE_BITS: u32 = 0x3f800000;
    unsafe {
        for (offset, value) in [
            (0x74, 0), (0x30, ONE_BITS), (0x34, 0), (0x38, 0),
            (0x40, 0), (0x44, ONE_BITS), (0x48, 0), (0x50, 0),
            (0x54, 0), (0x58, ONE_BITS), (0x68, 0), (0x64, 0),
            (0x60, 0), (0x70, 0), (0x14, 0), (0x18, 0), (0x1c, 0),
        ] {
            ((this + offset) as *mut u32).write_unaligned(value);
        }
        ((this + 0x20) as *mut u8).write(0);
    }
    0
});
