// original: 0x00e632e0 fill_neg1_118dd98
/// Fills 0x49 dwords at 0x118DD98 with 0xFFFFFFFF.
///
/// Single `rep stosd`. Returns 0xFFFFFFFF, the value the original leaves
/// in EAX.
export!(cdecl, rw_00e632e0() -> u32 {
    unsafe {
        core::ptr::write_bytes(global::<u32>(0x118DD98), 0xFF, 0x49);
        0xFFFF_FFFF
    }
});
