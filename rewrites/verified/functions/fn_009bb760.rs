// original: 0x009bb760 guarded_input_dispatch_b (proposed)

/// Clear the input-shutdown words unless the holder chain is already empty.
///
/// `this` points to a holder whose first word links to an inner record.
/// When the link is null the function returns at once. Otherwise control
/// continues into the shared tail routine (inlined here, since a rewrite
/// cannot jump into original code): when the inner record's first word is
/// zero it also returns at once; otherwise, unless the global flag byte at
/// 0x017ED94B is zero, it calls helper id 1 (cdecl, no arguments), then
/// zeroes the two global words at 0x017F58E0 and 0x017F58E4. Returns nothing
/// (the original never sets EAX).
///
/// Edge cases: null link and zero inner word both return early with no
/// writes and no call; the helper fires only when both are non-zero and the
/// flag byte is non-zero.
///
/// Original: thiscall, `this` in ECX, no stack arguments, one callee inside
/// the inlined tail routine.
lf_checker_rt::export!(thiscall, rw_009bb760(this: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x017ed94b;
        const WORD0: u32 = 0x017f58e0;
        const WORD1: u32 = 0x017f58e4;
        const HELPER: u32 = 1;
        let inner = (this as *const u32).read_unaligned();
        if inner == 0 {
            return 0;
        }
        if (inner as *const u32).read_unaligned() == 0 {
            return 0;
        }
        if ((lf_checker_rt::global::<u8>(FLAG) as *const u8).read() != 0) {
            let _ans: u32 = lf_checker_rt::callee_cdecl!(HELPER, u32,);
        }
        lf_checker_rt::global::<u32>(WORD0).write_unaligned(0);
        lf_checker_rt::global::<u32>(WORD1).write_unaligned(0);
        0
    }
});
