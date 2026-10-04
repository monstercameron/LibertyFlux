// original: 0x009e71e0 ped_value_route (proposed)

/// Copy a strided record, or resolve and emit an indexed value.
///
/// When `[this + 0x7b4]` is null or its word at +0x64 is zero, copies
/// 12 of 15 words from `[this + 0x20]` to the destination `a1`,
/// skipping words 3, 7 and 11, and returns the last word copied.
/// Otherwise resolves the table (`thiscall` on the head, no stack
/// words), indexes `[[t + 0xd4] + a2 * 4]`, sign-extends the word at
/// +0xe, maps it (`thiscall` on this) and emits the result
/// (`thiscall` on `a1`), returning the emit answer. `thiscall`, two
/// stack words (pointer, index).
lf_checker_rt::export!(thiscall, rw_009e71e0(this: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x7b4;
        const SRC: u32 = 0x20;
        const FIRST: u32 = 1;
        const INDEX: u32 = 2;
        const EMIT: u32 = 3;
        let head = ((this + HEAD) as *const u32).read_unaligned();
        let live = head != 0 && ((head + 0x64) as *const u32).read_unaligned() != 0;
        if !live {
            let src = ((this + SRC) as *const u32).read_unaligned();
            let mut last = 0u32;
            // Words 0,1,2,4,5,6,8,9,10,12,13,14; 3, 7, 11 skipped.
            let mut i = 0u32;
            while i < 15 {
                if i & 3 != 3 {
                    last = ((src + i * 4) as *const u32).read_unaligned();
                    ((a1 + i * 4) as *mut u32).write_unaligned(last);
                }
                i += 1;
            }
            return last;
        }
        let t = lf_checker_rt::callee_thiscall!(FIRST, u32, head);
        let base = ((t + 0xd4) as *const u32).read_unaligned();
        let ent = ((base + a2.wrapping_mul(4)) as *const u32).read_unaligned();
        let w = ((ent + 0xe) as *const u16).read_unaligned() as i16 as i32 as u32;
        let v = lf_checker_rt::callee_thiscall!(INDEX, u32, this, w);
        lf_checker_rt::callee_thiscall!(EMIT, u32, a1, v)
    }
});
