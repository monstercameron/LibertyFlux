// original: 0x009a2b60 BUMP
/// Dispatch an audio event tag to the fixed ten-argument engine call.
///
/// Compares the tag against eleven global ids in order; on the first match,
/// calls the engine (stdcall/10, stubbed by the checker) with the matching
/// relocated selector, the constant words (1, 0, 0, -1, 0, 0), the float
/// 1.0 and two trailing zeros. Null objects and unmatched tags return with
/// no call.
///
/// Proven scope: the object is always non-null; the tag cycles through
/// the eleven distinct IDs pinned in the declared globals, exercising each
/// dispatch arm. The stdcall/10 engine callee is a stub; null and unmatched
/// tags are not covered.
export!(stdcall, rw_009a2b60(obj: u32, tag: u32) -> () {
    unsafe {
        if obj == 0 {
            return;
        }
        let g = |va: u32| *global::<u32>(va);
        let dest: u32 = if tag == g(0x128442C) {
            relocated(0xE909B4)
        } else if tag == g(0x128443C) {
            relocated(0xE909C0)
        } else if tag == g(0x1284410) {
            relocated(0xE909D0)
        } else if tag == g(0x128441C) {
            relocated(0xE909E0)
        } else if tag == g(0x1284520) {
            relocated(0xE909F0)
        } else if tag == g(0x1284578) {
            relocated(0xE90A00)
        } else if tag == g(0x1284494) {
            relocated(0xE90A0C)
        } else if tag == g(0x1284528) {
            relocated(0xE90B30)
        } else if tag == g(0x1284590) {
            relocated(0xE90A24)
        } else if tag == g(0x1284584) {
            relocated(0xE90B40)
        } else if tag == g(0x1284450) {
            relocated(0xE90B48)
        } else {
            return;
        };
        callee_stdcall!(
            1, u32, dest, 1, 0, 0, 0xFFFF_FFFFu32, 0, 0, 0x3F80_0000u32, 0, 0
        );
    }
});
