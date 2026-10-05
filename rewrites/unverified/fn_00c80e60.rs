// original: 0x00c80e60 scenario_kind_report (proposed)

/// Classify scenario entry `idx` and report its kind word through `out`.
///
/// Looks the entry up with `c1(idx)`. A null entry, or one whose flag byte at
/// `+0x50` has bit 0 clear, yields 0. Otherwise the kind at `+0x14` decides:
/// 0 and 2 store `0x15e`, 3 and 4 store `0x15f`, any other kind stores
/// nothing; all three cases return 1. Only AL carries the result.
///
/// Original: cdecl, two stack words (plain `ret`).
lf_checker_rt::export!(cdecl, rw_00c80e60(idx: u32, out: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x50;
        const KIND_OFF: u32 = 0x14;
        const KIND_A: u32 = 0x15e;
        const KIND_B: u32 = 0x15f;
        const C1: u32 = 1;
        let p: u32 = lf_checker_rt::callee_cdecl!(C1, u32, idx);
        if p == 0 {
            return 0;
        }
        if ((p + FLAG_OFF) as *const u8).read() & 1 == 0 {
            return 0;
        }
        let kind = ((p + KIND_OFF) as *const u32).read_unaligned();
        if kind == 0 || kind == 2 {
            (out as *mut u32).write_unaligned(KIND_A);
        } else if kind == 3 || kind == 4 {
            (out as *mut u32).write_unaligned(KIND_B);
        }
        1
    }
});
