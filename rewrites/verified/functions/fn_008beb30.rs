// original: 0x008BEB30 input_mode_flag_update (proposed)

/// Update the input-mode flag byte and the derived mode dwords.
///
/// If either guard byte is nonzero the function does nothing and returns the
/// incoming eax untouched. Otherwise it stores the low byte of its stack
/// argument into the mode flag byte and, when that byte is nonzero, sets the
/// mode dword to 9; when it is zero and a second guard byte is also zero, it
/// sets the mode dword to 3 instead. Either store also writes all-ones to a
/// companion global. The return value is the incoming eax with its low byte
/// replaced by the argument byte (the original only ever writes al), so the
/// contract pins incoming eax to zero and compares the full word (cdecl, one
/// stack word, only the low byte read).
lf_checker_rt::export!(cdecl, rw_008BEB30(arg: u32) -> u32 {
    unsafe {
        /// Guard bytes: when either is nonzero the function is a no-op.
        const GUARD_A: u32 = 0x01160C35;
        const GUARD_B: u32 = 0x01160C36;
        /// Mode flag byte written from the argument.
        const MODE_FLAG: u32 = 0x01160C33;
        /// Second guard, consulted only when the argument byte is zero.
        const GUARD_C: u32 = 0x01160C3D;
        /// Derived mode dword.
        const MODE_DWORD: u32 = 0x01160C40;
        /// Companion global set to all-ones on either store path.
        const COMPANION: u32 = 0x01030BA8;
        /// Mode values for the nonzero and zero argument paths.
        const MODE_NONZERO: u32 = 9;
        const MODE_ZERO: u32 = 3;
        let rd8 = |a: u32| (lf_checker_rt::relocated(a) as *const u8).read();
        if rd8(GUARD_A) != 0 || rd8(GUARD_B) != 0 {
            // Incoming eax passes through; the contract pins it to zero.
            return 0;
        }
        let flag = (arg & 0xFF) as u8;
        (lf_checker_rt::relocated(MODE_FLAG) as *mut u8).write(flag);
        if flag != 0 {
            lf_checker_rt::global::<u32>(MODE_DWORD).write_unaligned(MODE_NONZERO);
            lf_checker_rt::global::<u32>(COMPANION).write_unaligned(0xFFFF_FFFF);
        } else if rd8(GUARD_C) == 0 {
            lf_checker_rt::global::<u32>(MODE_DWORD).write_unaligned(MODE_ZERO);
            lf_checker_rt::global::<u32>(COMPANION).write_unaligned(0xFFFF_FFFF);
        }
        u32::from(flag)
    }
});
