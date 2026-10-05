// original: 0x00888D60 stream_bank_attach (proposed)

/// Attach a stream bank the first time it is seen, under a guard.
///
/// Opens the guard (callee 1) on a stack scratch word. When the attached
/// flag global is already set, or the bank index `arg1` is 8 or more,
/// closes the guard (callee 3) and returns the guard answer with its low
/// byte cleared. Otherwise records the index, copies `arg1 * 8` bytes from
/// `arg0` to the bank name slot (callee 2), stamps the attach time
/// (callee 4), sets the flag and returns the guard answer with its low
/// byte set to 1.
///
/// The two scratch words are only ever passed to the guard entries and
/// never read back, so the contract does not compare them.
///
/// Original: 0x00888D60 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00888D60(arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const GUARD_ARG_FILE_VA: u32 = 0x0115_a4f4;
        const NAME_SLOT_FILE_VA: u32 = 0x0115_a488;
        const INDEX_GLOBAL: u32 = 0x0115_a478;
        const STAMP_GLOBAL: u32 = 0x0115_a474;
        const AUX_GLOBAL: u32 = 0x0115_a47c;
        const FLAG_GLOBAL: u32 = 0x0115_a480;
        const FLAG2_GLOBAL: u32 = 0x0115_a481;
        const MAX_BANK: u32 = 8;
        const OPEN: u32 = 1;
        const COPY: u32 = 2;
        const CLOSE: u32 = 3;
        const STAMP: u32 = 4;
        let mut scratch = [0u32; 2];
        let sp = &mut scratch as *mut u32 as u32;
        let ga = lf_checker_rt::relocated(GUARD_ARG_FILE_VA);
        lf_checker_rt::callee_thiscall!(OPEN, u32, sp, ga);
        let flag = (lf_checker_rt::relocated(FLAG_GLOBAL) as *const u8).read();
        let done = if flag != 0 || arg1 >= MAX_BANK {
            0u32
        } else {
            (lf_checker_rt::relocated(INDEX_GLOBAL) as *mut u32)
                .write_unaligned(arg1);
            let dst = lf_checker_rt::relocated(NAME_SLOT_FILE_VA);
            lf_checker_rt::callee_cdecl!(COPY, u32, dst, arg0, arg1.wrapping_mul(8));
            let t = lf_checker_rt::callee_cdecl!(STAMP, u32,);
            (lf_checker_rt::relocated(STAMP_GLOBAL) as *mut u32).write_unaligned(t);
            (lf_checker_rt::relocated(AUX_GLOBAL) as *mut u32).write_unaligned(0);
            (lf_checker_rt::relocated(FLAG_GLOBAL) as *mut u8).write(1);
            (lf_checker_rt::relocated(FLAG2_GLOBAL) as *mut u8).write(0);
            1u32
        };
        let g = lf_checker_rt::callee_thiscall!(CLOSE, u32, sp.wrapping_add(4));
        (g & 0xffff_ff00) | done
    }
});
