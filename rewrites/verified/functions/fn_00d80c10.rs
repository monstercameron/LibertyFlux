// original: 0x00D80C10 refresh_cached_selection (proposed)

/// Refresh a cached two-word selection on an input object.
///
/// `obj` points to a large input-state object. The refresh proceeds only
/// when two global tick counters (scaled by an exact multiply-and-shift
/// divide) disagree for this object's key (u16 at `+KEY`); when they agree
/// nothing happens. Otherwise a mode byte (`+MODE`, shifted by 2) is
/// classified by a 31-entry table into "query" or "clear-and-exit".
///
/// On the query path callee 1 (cdecl, 3 args) fills two out words; when the
/// low word of the second is all-ones the function exits. The pair is then
/// compared against three cached words (`+ID0/ID1/ID2`); any of six matches
/// clears the retry counter (`+RETRY`) and exits. Otherwise the counter is
/// incremented and the function exits unless it overflowed past 4.
///
/// Past the counter, callee 2 rebuilds a sub-object (`+SUB_A`), then callee
/// 3 (thiscall, 18 args, fixed service object as `this`) is consulted
/// twice: the first answer carries a float (must be below a global limit)
/// and an integer (must reach 2); the pair is then stored to `+ID1/ID2` and
/// the second consultation runs. Three tail calls reset sub-object `+SUB_B`
/// and the retry counter is cleared.
///
/// Original: 0x00D80C10 (cdecl, one stack word, no return value).
export!(cdecl, rw_00D80C10(obj: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x2c;
        const MODE: u32 = 0xe6e;
        const RETRY: u32 = 0xeed;
        const ID0: u32 = 0xddc;
        const ID1: u32 = 0xde0;
        const ID2: u32 = 0xde4;
        const FLAG: u32 = 0x1304;
        const OPT: u32 = 0xe73;
        const INNER: u32 = 0x20;
        const SUB_A: u32 = 0xe48;
        const SUB_B: u32 = 0xdd4;
        const DIV_MAGIC: u64 = 0x10624dd3;
        const SERVICE: u32 = 0x01177a80;
        const LIMIT: u32 = 0x00ec5200;
        const TAG: u32 = 0x497423fe;
        // Mode table: 0 = query, 1 = clear-and-exit, indexed by (mode - 2).
        const MAP: [u8; 31] = [
            0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0,
        ];

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn scale(x: u32) -> u32 {
            (((DIV_MAGIC * x as u64) >> 32) as u32) >> 7
        }

        // Scratch slots mirroring the original's frame offsets (byte 0x14,
        // 0x18, 0x1c, 0x20): callee 1 writes [arg0] and [arg1 + 12].
        let mut slots = [0u32; 9];
        let base = slots.as_mut_ptr() as u32;

        let key = rd16(obj + KEY) as u32;
        let g_hi = lf_checker_rt::global::<u32>(0x011735b8).read_unaligned();
        let g_lo = lf_checker_rt::global::<u32>(0x011735b4).read_unaligned();
        if scale(g_hi.wrapping_add(key)) == scale(g_lo.wrapping_add(key)) {
            return 0;
        }
        let mode = rd8(obj + MODE) as i8 as i32 - 2;
        if mode as u32 > 0x1e || MAP[mode as usize] != 0 {
            wr8(obj + RETRY, 0);
            return 0;
        }

        let out0 = base + 0x18;
        let out1 = base + 0x14;
        (base as *mut u32).add(5).write_unaligned(0xffff_ffff);
        (base as *mut u32).add(6).write_unaligned(0xffff_ffff);
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, obj, out1, out0);
        let got0 = (base as *const u32).add(6).read_unaligned();
        let got1 = (base as *const u32).add(5).read_unaligned();
        if got1 & 0xffff == 0xffff {
            return 0;
        }
        let id1 = rd32(obj + ID1);
        let id2 = rd32(obj + ID2);
        let id0 = rd32(obj + ID0);
        if id1 == got1 && id2 == got0 {
            wr8(obj + RETRY, 0);
            return 0;
        }
        if id1 == got0 && id2 == got1 {
            wr8(obj + RETRY, 0);
            return 0;
        }
        if id0 == got1 && id1 == got0 {
            wr8(obj + RETRY, 0);
            return 0;
        }
        if id0 == got0 && id1 == got1 {
            wr8(obj + RETRY, 0);
            return 0;
        }
        if id1 == got0 {
            wr8(obj + RETRY, 0);
            return 0;
        }
        if id0 == got0 {
            wr8(obj + RETRY, 0);
            return 0;
        }
        let retry = rd8(obj + RETRY).wrapping_add(1);
        wr8(obj + RETRY, retry);
        if retry <= 4 {
            return 0;
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, obj.wrapping_add(SUB_A), base + 0x20, obj);
        let flag_eq2 = (rd32(obj + FLAG) == 2) as u32;
        let opt_bit = ((!((rd8(obj + OPT) >> 1) & 1)) & 1) as u32;
        let inner30 = rd32(obj + INNER).wrapping_add(0x30);
        let svc = lf_checker_rt::global::<u32>(SERVICE) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            3, u32, svc, inner30, got0, base + 0x20, 0, base + 0x14, 0,
            base + 0x18, TAG, 0, TAG, opt_bit, id1, 0, flag_eq2, 0, 0, 0, 0
        );
        let limit = f32::from_bits(lf_checker_rt::global::<u32>(LIMIT).read_unaligned());
        let measured = f32::from_bits((base as *const u32).add(6).read_unaligned());
        let count = (base as *const u32).add(5).read_unaligned() as i32;
        if !(limit > measured) || count < 2 {
            wr8(obj + RETRY, 0);
            return 0;
        }
        wr32(obj + ID1, got1);
        wr32(obj + ID2, got0);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            4, u32, svc, inner30, got0, base + 0x20, obj.wrapping_add(ID0),
            base + 0x1c, 0x0c, 0, TAG, 0, TAG, opt_bit, got1, 0, flag_eq2, 0, 0, 0, 0
        );
        let sub_b = obj.wrapping_add(SUB_B);
        let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, sub_b);
        let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, sub_b, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(7, u32, sub_b, 0);
        wr8(obj + RETRY, 0);
        0
    }
});
