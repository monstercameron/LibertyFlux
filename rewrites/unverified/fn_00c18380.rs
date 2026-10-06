// original: 0x00c18380 init_copy_with_two_helpers

/// Fill a destination record from a source record through two helpers.
///
/// Calls the first helper (thiscall: object `src`, one scratch-buffer
/// argument) and the second (cdecl: one scratch-buffer argument, 64-bit
/// result), stores that result at `+0x40/+0x44` of `dst`, and copies the
/// three dwords at `+0x30/+0x34/+0x38` of `src` to `+0x34/+0x38/+0x3c` of
/// `dst`. Returns the last copied dword (left in `eax` by the original).
/// Both helpers take a pointer to the caller's own scratch area; the contract
/// skips those pointer arguments and snapshots the (zero-filled) contents.
///
/// Original: 0x00C18380 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_00c18380(dst: u32, src: u32) -> u32 {
    unsafe {
        const HELPER_THIS: u32 = 1;
        const HELPER_U64: u32 = 2;
        const RESULT_OFF: u32 = 0x40;
        const COPY_DST: u32 = 0x34;
        const COPY_SRC: u32 = 0x30;
        let mut scratch = [0u32; 8];
        let buf = scratch.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(HELPER_THIS, u32, src, buf);
        let v: u64 = lf_checker_rt::callee_cdecl!(HELPER_U64, u64, buf);
        ((dst + RESULT_OFF) as *mut u32).write_unaligned(v as u32);
        ((dst + RESULT_OFF + 4) as *mut u32).write_unaligned((v >> 32) as u32);
        for i in 0..3u32 {
            let w = ((src + COPY_SRC + i * 4) as *const u32).read_unaligned();
            ((dst + COPY_DST + i * 4) as *mut u32).write_unaligned(w);
        }
        ((src + COPY_SRC + 8) as *const u32).read_unaligned()
    }
});
