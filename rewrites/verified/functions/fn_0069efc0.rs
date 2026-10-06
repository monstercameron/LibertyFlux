// original: 0x0069EFC0 init_dual_and_table
/// Run two init callees (plus a conditional third) and seed four table words.
///
/// Installs a handler address (a relocated code pointer) into a global
/// slot, calls callee 0 with the
/// register pair `(a0_lowbyte != 0, a1_lowbyte)` (fastcall-style, no stack
/// words), then callee 1 unconditionally and callee 2 when a flag byte is
/// nonzero. Finally adds two constant quads pairwise (`paddd`) and stores
/// the four sums to four global words. Only the low bytes of the two stack
/// words are read. Returns the last scripted callee answer.
/// The incoming ecx/edx upper bytes flow into callee 0's registers as
/// caller leftovers; the contract fixes them to 0 (a Rust rewrite cannot
/// read incoming general registers), so the compared registers carry exactly
/// the two computed bytes. Original: cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_0069efc0(a0: u32, b0: u32) -> u32 {
    unsafe {
        const HANDLER_SLOT: u32 = 0x018B_9B9C;
        const HANDLER: u32 = 0x006A_5AE0;
        const FLAG: u32 = 0x0105_C7CF;
        const QUAD_A: u32 = 0x00FE_8E70;
        const QUAD_B: u32 = 0x00FE_8E20;
        const OUT0: u32 = 0x019F_2388;
        const OUT1: u32 = 0x019F_23C0;
        const OUT2: u32 = 0x019F_23F8;
        const OUT3: u32 = 0x019F_2430;
        let cl = if (a0 as u8) != 0 { 1u32 } else { 0 };
        let dl = (b0 as u8) as u32;
        lf_checker_rt::global::<u32>(HANDLER_SLOT)
            .write_unaligned(lf_checker_rt::relocated(HANDLER));
        let _: u32 = lf_checker_rt::callee_fastcall!(0, u32, cl, dl);
        let mut last: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        let flag = lf_checker_rt::global::<u8>(FLAG).read_unaligned();
        if flag != 0 {
            last = lf_checker_rt::callee_cdecl!(2, u32,);
        }
        let qa = lf_checker_rt::relocated(QUAD_A) as *const u32;
        let qb = lf_checker_rt::relocated(QUAD_B) as *const u32;
        lf_checker_rt::global::<u32>(OUT0)
            .write_unaligned(qa.read_unaligned().wrapping_add(qb.read_unaligned()));
        lf_checker_rt::global::<u32>(OUT1).write_unaligned(
            qa.add(1).read_unaligned().wrapping_add(qb.add(1).read_unaligned()),
        );
        lf_checker_rt::global::<u32>(OUT2).write_unaligned(
            qa.add(2).read_unaligned().wrapping_add(qb.add(2).read_unaligned()),
        );
        lf_checker_rt::global::<u32>(OUT3).write_unaligned(
            qa.add(3).read_unaligned().wrapping_add(qb.add(3).read_unaligned()),
        );
        last
    }
});