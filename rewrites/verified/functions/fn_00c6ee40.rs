// original: 0x00c6ee40 parse_driveby_flags
/// Tokenize a driveby-flags string into a bit mask and two floats.
///
/// Takes three output pointers on the stack: the flags word, then two float
/// slots. Pulls tokens from the shared tokenizer until it answers null.
/// Each token is compared against twenty-one known words in fixed order;
/// the first match sets its bit in the flags word. The nineteenth word
/// additionally pulls two more tokens and parses one float from each into
/// the float slots. Tokens matching nothing are skipped. The result is the
/// accumulated mask; the tokenizer and the float parser are intercepted
/// calls whose answers the contract scripts.
const C6EE40_FLAG_STRS: [u32; 21] = [
    0x00ED3018, 0x00ED3030, 0x00ED3048, 0x00ED3084, 0x00ED3098, 0x00ED30A8, 0x00ED30E4, 0x00ED30F4,
    0x00ED3104, 0x00ED3140, 0x00ED3154, 0x00ED316C, 0x00ED3198, 0x00ED31BC, 0x00ED31D8, 0x00ED3204,
    0x00ED3228, 0x00ED3240, 0x00ED3270, 0x00ED3290, 0x00ED32D0,
];
const C6EE40_FLAG_BITS: [u32; 21] = [
    0x000001, 0x000002, 0x000004, 0x000008, 0x000010, 0x000020, 0x000040, 0x000080,
    0x000100, 0x000200, 0x000400, 0x000800, 0x001000, 0x002000, 0x004000, 0x008000,
    0x010000, 0x020000, 0x040000, 0x080000, 0x100000,
];
const C6EE40_DELIMS: u32 = 0x00ED2FF0;
const C6EE40_FMT0: u32 = 0x00ED32B0;
const C6EE40_FMT1: u32 = 0x00ED32B4;
const C6EE40_FLOAT_WORD: usize = 19;
#[inline(always)]
fn c6ee40_streq(tok: u32, imgstr: u32) -> bool {
    unsafe {
        let mut i: u32 = 0;
        loop {
            let a = *((tok.wrapping_add(i)) as *const u8);
            let b = *((imgstr.wrapping_add(i)) as *const u8);
            if a != b {
                return false;
            }
            if a == 0 {
                return true;
            }
            i = i.wrapping_add(1);
        }
    }
}
export!(cdecl, rw_00c6ee40(flags_out: u32, fp0_out: u32, fp1_out: u32) -> u32 {
    unsafe {
        let delims = relocated(C6EE40_DELIMS);
        let mut tok = callee_cdecl!(1, u32, 0, delims);
        while tok != 0 {
            let mut i: usize = 0;
            while i < C6EE40_FLAG_STRS.len() {
                if c6ee40_streq(tok, relocated(C6EE40_FLAG_STRS[i])) {
                    *(flags_out as *mut u32) |= C6EE40_FLAG_BITS[i];
                    if i == C6EE40_FLOAT_WORD {
                        // The original forwards its zeroed accumulator as the
                        // first pull's start token; the value is zero either way.
                        let t1 = callee_cdecl!(1, u32, 0, delims);
                        callee_cdecl!(2, u32, t1, relocated(C6EE40_FMT0), fp0_out);
                        let t2 = callee_cdecl!(1, u32, 0, delims);
                        callee_cdecl!(2, u32, t2, relocated(C6EE40_FMT1), fp1_out);
                    }
                    break;
                }
                i += 1;
            }
            tok = callee_cdecl!(1, u32, 0, delims);
        }
        0
    }
});
