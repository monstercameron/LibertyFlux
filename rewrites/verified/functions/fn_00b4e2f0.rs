// original: 0x00b4e2f0 ped_task_alloc_variant_a (proposed)

/// Build a small or large task object for `this`, then report it.
///
/// `this` is a state object with a vtable, a signed 16-bit pool index at
/// `+0x2e`, an info pointer at `+0x34`, two tag bytes at `+0x5b`/`+0x63`,
/// and a fallback pointer at `+0x100`. Two state-getter calls (vtable
/// `+0xd0`) open: the first feeds a flag that is dead on every path (a
/// scratch byte overwritten before any read), the second's bit 4 picks the
/// path. With the bit set and `arg0` nonzero, the getter result itself is
/// returned.
///
/// Otherwise a buffer is allocated (`0x1c` bytes on the small path, `0xa0`
/// on the large; a null buffer reports 0 through the reporter and returns
/// its answer), the selector (vtable `+0xa0`, retried once; null selects
/// the fallback, else the selected object's getter at `+0xe0` supplies a
/// handle) picks a handle, and a constructor runs: five words on the small
/// path (handle, 0, second tag, first tag, `arg1`), fourteen on the large
/// (index, third getter result, handle, info word, `arg0`, tags, predicate
/// byte, one word of the original's own uninitialized scratch that is not
/// compared, `arg1`, four zeroes). The small path writes its incoming
/// `arg0` slot (dead, callee-popped) with the first tag, so the stack check
/// is off and the tag is verified through the constructor call instead; the
/// three tag words carry only a meaningful low byte and compare masked. The
/// constructor's result goes through the reporter and its answer is
/// returned. Thiscall, two stack words.
lf_checker_rt::export!(thiscall, rw_00b4e2f0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const VT: u32 = 0x00;
        const INDEX: u32 = 0x2e;
        const INFO: u32 = 0x34;
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
        const SMALL_SIZE: u32 = 0x1C;
        const LARGE_SIZE: u32 = 0xA0;
        const GETTER: u32 = 1;
        const SELECTOR: u32 = 2;
        const HANDLE: u32 = 3;
        const ALLOC: u32 = 4;
        const PREDICATE: u32 = 5;
        const CONSTRUCT_SMALL: u32 = 6;
        const REPORT: u32 = 7;
        const CONSTRUCT_LARGE: u32 = 8;

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
        // Dead flag: computed, then overwritten before any read on both
        // paths. Kept as reads only, so the pool chain stays valid.
        let _flag: u32 = if rd8(st1 + ST_BYTE) & 8 != 0 {
            1
        } else {
            let q = rd32(pool + Q_WORD);
            if rd8(q + Q_BYTE) != 0 { 0 } else { 1 }
        };
        let st2 = get1(this);
        if rd8(st2 + ST_BYTE) & 0x10 == 0 {
            // Large path.
            let saved = rd32(rd32(this + INFO) + 4);
            let buf: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, LARGE_SIZE, 0);
            if buf == 0 {
                return lf_checker_rt::callee_cdecl!(REPORT, u32, 0);
            }
            let tag_a = rd8(this + TAG_A);
            let tag_b = rd8(this + TAG_B);
            let sel_fn: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(this + VT) + SLOT_SELECTOR) as usize);
            let sel = sel_fn(this);
            let handle: u32 = if sel == 0 {
                rd32(this + FALLBACK)
            } else {
                let sel2 = sel_fn(this);
                let h_fn: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(sel2 + VT) + SLOT_HANDLE) as usize);
                h_fn(sel2)
            };
            let st3 = get1(this);
            let pred: u32 = lf_checker_rt::callee_thiscall!(PREDICATE, u32, this);
            let built: u32 = lf_checker_rt::callee_thiscall!(
                CONSTRUCT_LARGE, u32, buf, index, st3, handle, saved, arg0,
                tag_b, tag_a, pred & 0xFF, 0, arg1, 0, 0, 0, 0
            );
            return lf_checker_rt::callee_cdecl!(REPORT, u32, built);
        }
        if arg0 != 0 {
            return st2;
        }
        // Small path.
        let buf: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, SMALL_SIZE, 0);
        if buf == 0 {
            return lf_checker_rt::callee_cdecl!(REPORT, u32, 0);
        }
        let tag_a = rd8(this + TAG_A);
        let tag_b = rd8(this + TAG_B);
        let sel_fn: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(this + VT) + SLOT_SELECTOR) as usize);
        let sel = sel_fn(this);
        let handle: u32 = if sel == 0 {
            rd32(this + FALLBACK)
        } else {
            let sel2 = sel_fn(this);
            let h_fn: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(sel2) + SLOT_HANDLE) as usize);
            h_fn(sel2)
        };
        let built: u32 = lf_checker_rt::callee_thiscall!(
            CONSTRUCT_SMALL, u32, buf, handle, 0, tag_b, tag_a, arg1
        );
        lf_checker_rt::callee_cdecl!(REPORT, u32, built)
    }
});
