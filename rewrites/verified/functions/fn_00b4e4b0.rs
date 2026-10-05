// original: 0x00b4e4b0 ped_task_alloc_variant_b (proposed)

/// Build a task object for `this` when its state allows, then report it.
///
/// `this` is a state object with a vtable, a signed 16-bit pool index at
/// `+0x2e`, two tag bytes at `+0x5b`/`+0x63`, and a fallback pointer at
/// `+0x100`. The pool slot feeds a qualifier object. Two calls to the state
/// getter (vtable `+0xd0`) drive the gates: bit 5 of the first result's byte
/// at `+0x72` decides whether anything is built (a clear bit returns the
/// second result, or the qualifier's word at `+0x128` when bit 3 of it is
/// clear), and bit 3, or the qualifier's byte being zero, sets a flag word
/// for the constructor.
///
/// When set, a `0xc0`-byte buffer is allocated (a null buffer reports 0
/// through the reporter and returns its answer). Otherwise a third getter
/// call, the selector (vtable `+0xa0`, retried once; on a null selector the
/// fallback pointer is used, else the selected object's own getter at
/// `+0xe0` supplies a handle), and a direct predicate call taking no
/// stack words feed a ten-word constructor call together
/// with the pool index, the stack words, the flag, the third getter
/// result plus `0x80`, and both tags. Three of its words carry only a
/// meaningful low byte (the flag, the two tags) and compare masked. The
/// constructor's result goes through the reporter and its
/// answer is returned. Thiscall, two stack words.
lf_checker_rt::export!(thiscall, rw_00b4e4b0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const VT: u32 = 0x00;
        const INDEX: u32 = 0x2e;
        const TAG_A: u32 = 0x5b;
        const TAG_B: u32 = 0x63;
        const FALLBACK: u32 = 0x100;
        const SLOT_GETTER: u32 = 0xd0;
        const SLOT_SELECTOR: u32 = 0xa0;
        const SLOT_HANDLE: u32 = 0xe0;
        const ST_BYTE: u32 = 0x72;
        const POOL: u32 = 0x1295CD8;
        const Q_WORD: u32 = 0x128;
        const Q_BYTE: u32 = 0x02;
        const ALLOC_SIZE: u32 = 0xC0;
        const GETTER: u32 = 1;
        const SELECTOR: u32 = 2;
        const HANDLE: u32 = 3;
        const ALLOC: u32 = 4;
        const PREDICATE: u32 = 5;
        const CONSTRUCT: u32 = 6;
        const REPORT: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u32 {
            unsafe { (a as *const u8).read() as u32 }
        }
        let vtab = rd32(this + VT);
        let index = rd16(this + INDEX) as u16 as i16 as i32 as u32;
        let pool = rd32(
            lf_checker_rt::relocated(POOL)
                .wrapping_add(index.wrapping_mul(4)),
        );
        let get1: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtab + SLOT_GETTER) as usize);
        let st1 = get1(this);
        let build = (rd8(st1 + ST_BYTE) >> 5) & 1;
        let st2 = get1(this);
        let flag: u32 = if rd8(st2 + ST_BYTE) & 8 != 0 {
            1
        } else {
            let q = rd32(pool + Q_WORD);
            if rd8(q + Q_BYTE) != 0 { 0 } else { 1 }
        };
        if build == 0 {
            if rd8(st2 + ST_BYTE) & 8 != 0 {
                return st2;
            }
            return rd32(pool + Q_WORD);
        }
        let buf: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, ALLOC_SIZE, 0);
        if buf == 0 {
            return lf_checker_rt::callee_cdecl!(REPORT, u32, 0);
        }
        let tag_a = rd8(this + TAG_A);
        let tag_b = rd8(this + TAG_B);
        let st3 = get1(this);
        let sel1_fn: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(this + VT) + SLOT_SELECTOR) as usize);
        let sel = sel1_fn(this);
        let handle: u32 = if sel == 0 {
            rd32(this + FALLBACK)
        } else {
            let sel2 = sel1_fn(this);
            let h_fn: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(sel2 + VT) + SLOT_HANDLE) as usize);
            h_fn(sel2)
        };
        let pred: u32 = lf_checker_rt::callee_thiscall!(PREDICATE, u32, this);
        let pred_byte = pred & 0xFF;
        let built: u32 = lf_checker_rt::callee_thiscall!(
            CONSTRUCT, u32, buf, index, handle, st3.wrapping_add(0x80), arg0,
            tag_b, tag_a, pred_byte, flag, arg1, 0
        );
        lf_checker_rt::callee_cdecl!(REPORT, u32, built)
    }
});
