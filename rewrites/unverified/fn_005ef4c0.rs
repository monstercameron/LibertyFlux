// original: 0x005EF4C0 shared_status_bit_clear (proposed)

/// Clear bit 0 of the shared object's status byte.
///
/// Loads the shared object pointer from image data and clears bit 0 of the
/// byte at offset 0x558, leaving the other bits untouched. The original
/// takes no arguments and returns nothing meaningful.
lf_checker_rt::export!(cdecl, rw_005ef4c0() -> u32 {
    unsafe {
        const OBJECT_GLOBAL: u32 = 0x0118D808;
        const STATUS_OFFSET: u32 = 0x558;
        const KEEP_MASK: u8 = 0xfe;
        let obj =
            (lf_checker_rt::global::<u32>(OBJECT_GLOBAL)).read_unaligned();
        let slot = obj.wrapping_add(STATUS_OFFSET) as *mut u8;
        slot.write_unaligned(slot.read_unaligned() & KEEP_MASK);
        0
    }
});
