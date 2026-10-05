// original: 0x00c3b5a0 train_kind_small_or_class_le3 (proposed)
/// Accept train kinds 4 and 5 outright, else classify through two helpers.
///
/// When `arg - 4 <= 1` (unsigned, i.e. arg is 4 or 5) returns 1 in AL over
/// the high bits of `arg - 4`. Otherwise calls helper id 1 (no arguments),
/// passes its answer as `this` to helper id 2, and returns 1 in AL when
/// that answer is `<= 3`, else 0, again over the answer's high bits.
///
/// Original: 0x00c3b5a0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00c3b5a0(arg: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 1;
        const SECOND: u32 = 2;
        const LIMIT: u32 = 3;
        let t = arg.wrapping_sub(4);
        if t <= 1 {
            return (t & 0xffff_ff00) | 1;
        }
        let mid: u32 = lf_checker_rt::callee_cdecl!(FIRST, u32,);
        let got: u32 = lf_checker_rt::callee_thiscall!(SECOND, u32, mid);
        (got & 0xffff_ff00) | ((got <= LIMIT) as u32)
    }
});
