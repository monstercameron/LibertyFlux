// original: 0x00d19ea0 ped_task_dispatcher (proposed)

/// Dispatch one ped task slot: latch the arguments into the dispatcher,
/// resolve the handler chain for the slot, and run it.
///
/// `this` is the dispatcher (argument latch at `+0x1d3c`, a 128-entry handler
/// table at `+0x1b20`, flag byte at `+0x1d25`, allocator state at `+0x14b4`).
/// The seven stack words are latched into the dispatcher; `a3` and `a6`
/// contribute only their low bytes. When `a6` is nonzero a notification
/// callee runs before and after the dispatch.
///
/// The handler source at `[this+0x1d28]` (or a fallback global when `a5` is
/// null) feeds two intercepted lookups; an indirect handler predicate through
/// `[this+0x1d2c]` (a planted stub) gates three intercepted handler calls.
/// The loop walks the handler table (at most 128 entries) until a null entry,
/// a repeat of `a0`, or a handler accepting the slot. When the allocator
/// state is clear, two intercepted stack-allocator callees reserve scratch
/// (modelled as no-ops: the only in-frame use of the reservation is an
/// address passed on to another intercepted callee) and a teardown callee
/// runs at the end. Returns the teardown callee's answer.
///
/// The indirect predicate is called from two sites with different cleanup:
/// the null-source site adds 4 after the call, leaking 4 bytes of stack per
/// hit on the original side. The rewrite does not reproduce the leak: nothing
/// in the loop reads stack relatively and the epilogue restores the pointer,
/// so it is unobservable. The security-cookie check runs natively, unpatched.
///
/// Original: 0x00d19ea0 (thiscall, seven stack words).
lf_checker_rt::export!(thiscall, rw_00d19ea0(
    this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32,
) -> u32 {
    unsafe {
        const LATCH0: u32 = 0x1d3c;
        const LATCH1: u32 = 0x1d40;
        const LATCH2: u32 = 0x1d44;
        const LATCH5: u32 = 0x1d50;
        const LATCH3B: u32 = 0x1d48;
        const TABLE: u32 = 0x1b20;
        const TABLE_LEN: i32 = 0x80;
        const FLAG: u32 = 0x1d25;
        const ALLOC_STATE: u32 = 0x14b4;
        const SRC: u32 = 0x1d28;
        const FALLBACK_GLOBAL: u32 = 0x018b896c;
        const NOTIFY_OBJ: u32 = 0x01b4a8b0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        wr32(this.wrapping_add(LATCH0), a0);
        wr8(this.wrapping_add(LATCH3B), a3 as u8);
        let mut src = a5;
        wr32(this.wrapping_add(LATCH1), a1);
        wr32(this.wrapping_add(LATCH2), a2);
        wr32(this.wrapping_add(LATCH5), a5);
        let notify = a6 as u8;
        if notify != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, NOTIFY_OBJ);
        }
        // The control block the original passes to its setup and lookup
        // callees is a separate frame region the original never writes, so
        // on the original side it holds the contract's zero fill; the model
        // passes equivalent zeros. (The original also initialises a second,
        // dead frame struct and overwrites parts of it with argument words;
        // those stores hit below-ESP frame, are never read back, and are
        // omitted here.)
        let mut block = [0u32; 8];
        lf_checker_rt::callee_thiscall!(2, u32, block.as_mut_ptr() as u32);
        if rd8(this.wrapping_add(FLAG)) == 0 {
            lf_checker_rt::callee_thiscall!(3, u32, this, block.as_mut_ptr() as u32);
        }
        lf_checker_rt::callee_thiscall!(4, u32, this);
        if src == 0 {
            src = rd32(lf_checker_rt::relocated(FALLBACK_GLOBAL));
        }
        let mut teardown = false;
        if rd32(this.wrapping_add(ALLOC_STATE)) == 0 {
            lf_checker_rt::callee_cdecl!(5, u32,);
            lf_checker_rt::callee_cdecl!(6, u32,);
            // The reservation's contents are the contract's zero fill on the
            // original side (the allocator callees are modelled as no-ops);
            // the model passes equivalent zeros.
            let framebuf = [0u32; 8];
            let sp = framebuf.as_ptr() as u32;
            lf_checker_rt::callee_thiscall!(7, u32, this, sp, 0x2000, sp, 1);
            teardown = true;
        }
        let mut ebx: i32 = 0;
        let mut esi: u32 = 0;
        if rd8(this.wrapping_add(FLAG)) == 0 {
            esi = lf_checker_rt::callee_thiscall!(8, u32, src, block.as_mut_ptr() as u32);
            wr32(this.wrapping_add(TABLE), esi);
        } else {
            esi = rd32(this.wrapping_add(TABLE));
        }
        loop {
            if esi == 0 {
                break;
            }
            if esi != a0 {
                let pred_this = rd32(this.wrapping_add(SRC));
                let pred: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::callee_addr(9) as usize);
                let ok = pred(pred_this, esi);
                if ok as u8 != 0 {
                    lf_checker_rt::callee_thiscall!(10, u32, this, esi.wrapping_add(0x10));
                    let inner = rd32(esi.wrapping_add(4));
                    lf_checker_rt::callee_thiscall!(11, u32, this, rd32(inner.wrapping_add(0xc)), esi);
                    if lf_checker_rt::callee_thiscall!(12, u32, this, esi.wrapping_add(0x10), 0) as u8 != 0 {
                        break;
                    }
                }
            }
            if rd8(this.wrapping_add(FLAG)) == 0 {
                esi = lf_checker_rt::callee_thiscall!(13, u32, src, block.as_mut_ptr() as u32);
                ebx = ebx.wrapping_add(1);
                let idx = if ebx < 0x7f { ebx } else { 0x7f } as u32;
                wr32(this.wrapping_add(TABLE).wrapping_add(idx.wrapping_mul(4)), esi);
            } else {
                ebx = ebx.wrapping_add(1);
                if ebx >= TABLE_LEN {
                    break;
                }
                esi = rd32(this.wrapping_add(TABLE).wrapping_add((ebx as u32).wrapping_mul(4)));
            }
        }
        if teardown {
            lf_checker_rt::callee_thiscall!(14, u32, this, 0, 0, 0, 0);
        }
        let ret = lf_checker_rt::callee_thiscall!(15, u32, this);
        if notify != 0 {
            lf_checker_rt::callee_thiscall!(16, u32, NOTIFY_OBJ);
        }
        ret
    }
});
