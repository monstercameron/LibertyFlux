// original: 0x00662820 session_task_init_from_params (proposed)

/// Initialise a session-task object from a parameter block, then validate it.
///
/// `this` is the task object. `mode` must be zero: any other value returns 0
/// at once and touches nothing. Otherwise the ten stack words are consumed as
/// follows (`src` is a 20-byte parameter header, `blob` a 16-byte key block,
/// `opt` an optional auxiliary pointer):
///
/// * the sub-object at `+OBJ_A` is initialised by callee 1 with the constant
///   table base `PARAM_TABLE` (the original forms `mode << 7 + PARAM_TABLE`,
///   which is the constant on the only live path);
/// * `FLAG_SLOT` is set to `mode` (zero); four dwords are copied from
///   `src[0..16]` to `+HDR_COPY`; the sub-object at `+OBJ_B` is initialised
///   by callee 2 with `src + 0x10`;
/// * `v0`, `v1`, `v2` land at `+VAL0`, `+VAL2`, `+VAL1` (note the order: the
///   second value goes to the higher slot);
/// * `opt` is stored at `+OPT_SLOT`; when it is non-zero callee 3 (cdecl) is
///   called as `callee3(this + AUX_BUF, blob, opt)`;
/// * `+STATE_A`/`+STATE_B` are set to -1/0 and the 16 bytes at `blob` are
///   copied to `+KEY_COPY` (the original moves them through vector registers
///   as plain 64-bit halves, no floating-point semantics);
/// * callee 4 runs as `callee4(this = blob, this + SCRATCH)` and its low byte
///   decides: zero returns 0, otherwise `+READY` is set to 1 and 1 is returned.
///
/// Original: thiscall, ten stack words; words 0, 6 and 9 are never read (the
/// sole caller pushes a stale register word first). Returns an 8-bit flag.
lf_checker_rt::export!(thiscall, rw_00662820(this: u32, _w0: u32, mode: u32, src: u32, v0: u32, v1: u32, v2: u32, _w6: u32, opt: u32, blob: u32, _w9: u32) -> u32 {
    unsafe {
        const PARAM_TABLE: u32 = 0x019f3230;
        const OBJ_A: u32 = 0x98;
        const FLAG_SLOT: u32 = 0x94;
        const HDR_COPY: u32 = 0x108;
        const OBJ_B: u32 = 0x118;
        const VAL0: u32 = 0x150;
        const VAL2: u32 = 0x158;
        const VAL1: u32 = 0x154;
        const OPT_SLOT: u32 = 0x35c;
        const AUX_BUF: u32 = 0x15c;
        const STATE_A: u32 = 0x360;
        const STATE_B: u32 = 0x364;
        const KEY_COPY: u32 = 0x588;
        const SCRATCH: u32 = 0xe0;
        const READY: u32 = 0x788;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if mode != 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(OBJ_A), PARAM_TABLE);
        wr32(this.wrapping_add(FLAG_SLOT), mode);
        for i in 0..4u32 {
            wr32(this.wrapping_add(HDR_COPY).wrapping_add(i * 4), rd32(src.wrapping_add(i * 4)));
        }
        lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(OBJ_B), src.wrapping_add(0x10));
        wr32(this.wrapping_add(VAL0), v0);
        wr32(this.wrapping_add(VAL2), v1);
        wr32(this.wrapping_add(VAL1), v2);
        wr32(this.wrapping_add(OPT_SLOT), opt);
        if opt != 0 {
            lf_checker_rt::callee_cdecl!(3, u32, this.wrapping_add(AUX_BUF), blob, opt);
        }
        wr32(this.wrapping_add(STATE_A), 0xffff_ffff);
        wr32(this.wrapping_add(STATE_B), 0);
        for i in 0..4u32 {
            wr32(this.wrapping_add(KEY_COPY).wrapping_add(i * 4), rd32(blob.wrapping_add(i * 4)));
        }
        let ok: u32 = lf_checker_rt::callee_thiscall!(4, u32, blob, this.wrapping_add(SCRATCH));
        if ok & 0xff == 0 {
            return 0;
        }
        wr32(this.wrapping_add(READY), 1);
        1
    }
});
