// original: 0x00e040e0 validate_pe_headers
/// Validates a module image's DOS and NT headers, returning 1 if valid.
///
/// Returns 1 only when the word at the base is the MZ signature, the
/// long at base+0x3C leads to the PE signature, and the optional-header
/// magic there is 0x10B (32-bit); otherwise returns 0.
export!(cdecl, rw_00e040e0(base: u32) -> u32 {
    unsafe {
        const MZ: u16 = 0x5A4D;
        const PE_SIG: u32 = 0x4550;
        const MAGIC_32: u16 = 0x10B;
        if *((base) as *const u16) != MZ {
            return 0;
        }
        let nt = base.wrapping_add(*((base + 0x3C) as *const u32));
        if *((nt) as *const u32) != PE_SIG {
            return 0;
        }
        if *((nt + 0x18) as *const u16) == MAGIC_32 {
            1
        } else {
            0
        }
    }
});
