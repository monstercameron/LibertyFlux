// original: 0x00DD1930 FRONTEND_MENU_MONTAGE_DROP_MT

/// Drop (release) one montage menu item: tag-gated dispatch to an early
/// notify exit, a row-update sequence, or a row fetch/insert sequence.
///
/// `this` is the container controller; `arg` is a registry key. The key is
/// resolved through the registry (callee 1, this = `REGISTRY`); a config
/// object is fetched (callee 2) and its byte at `+0x20b` set to 1. The
/// resolved object's tag (slot `+0x0`) is compared against the tag helper
/// (callee 3, cdecl/1 over a string address):
/// - match: a count is read through child `+0x1f8` (callee 4). A count at
///   or above `COUNT_LIMIT` (0x64, SIGNED `jl`: negative counts take the
///   long path) notifies (callee 5 with `NOTIFY_ARG`) and a global
///   listener (`GLOBAL_LISTENER`, callee 6), returning the listener's
///   answer (exit A1). Lower counts run the row-update sequence: a format
///   call (callee 8), an optional dismiss triple gated on byte
///   `+0x218` (slot `+0x120`, flag `+0x1f4`, callee 10), an optional
///   probe/pick/apply triple gated on slot `+0x1d4`, a flag store at
///   `+0x1f9`, index `-1`, the tag helper again, then the row fetch
///   (callee 15, thiscall/3: the stale pushed 1 sits below the tag
///   answer and the object), a touch (callee 16), refresh (slot `+0x4c`
///   into slot `+0x1e4`), two posts (callees 19, 20) and slot `+0x1b0`,
///   whose answer is returned (exit A2).
/// - mismatch: the tag is re-read and compared against a second string;
///   a second mismatch returns the helper's answer at once (exit B1),
///   a match runs the fetch/insert sequence: config + listener again,
///   two aux calls (callees 22, 23), a `ROW_SIZE`-byte block allocation
///   (callee 24; a null answer skips the build and faults on the first
///   use of the missing row, symmetrically on both sides), two cached
///   gets (slot `+0x48` twice, thiscall/0: the second call's two pushes
///   stay for the format call below), a format call (callee 27, cdecl/3
///   over string, second answer, sequence), the row build (callee 28,
///   thiscall/3: block, object, format answer, first cached value),
///   three refresh rounds (slot `+0x4c` on `this` into `+0x170/0x17c/
///   0x188` on the row), two single-word row updates (`+0x28`, `+0x44`),
///   a sequence increment, a negative-index recompute (SIGNED `jge`;
///   candidate lookup, one of two refresh/index sequences, `-1`
///   stored as 0 on the equal path), an array insert (slot `+0x1d0`
///   thrice, count decremented/restored around an upward shift, row
///   stored at the index), a sink announce (callee 41, thiscall/2 over
///   index and limit-or-limit+1 per the SIGNED `jg`), a float moved by
///   value from the sink answer `+0x44` to the row (`+0x224`), an
///   optional note (callee 42) gated on byte `+0x21c` == 1, a
///   still-negative index via slot `+0x1d4` (decremented) into the
///   three-word report (callee 43: object, row, word, zero), an x87
///   float (callee 44) stored to `+0x1f0`, a tail triple (slot `+0x21c`,
///   callees 46, 47), two posts (callees 20, 19), a finaliser (callee
///   48) and a touch (callee 16) whose answer is returned (exit B2).
///
/// All compared index/count values are SIGNED (jl/jge/jg); floats are
/// only moved, never computed, hence bit-exact. The original writes its
/// candidate pointer and the x87 result into the dead incoming-argument
/// slot as scratch; the rewrite keeps them in locals (stack check off,
/// values observed via compared call registers and heap stores).
///
/// Original: 0x00DD1930 (thiscall, one stack word, returns u32 in eax).
lf_checker_rt::export!(thiscall, rw_00dd1930(this: u32, arg: u32) -> u32 {
    unsafe {
        const REGISTRY: u32 = 0x0198_1A4C;
        const STR_CFG: u32 = 0x00EF_AE10;
        const STR_TAG1: u32 = 0x00EF_AE20;
        const STR_FMT: u32 = 0x00EF_AE2C;
        const STR_TAG3: u32 = 0x00EF_AE3C;
        const STR_TAG2: u32 = 0x00EF_AE48;
        const STR_CFG2: u32 = 0x00EF_AE58;
        const STR_AUX2: u32 = 0x00EF_AE68;
        const AUX2_THIS: u32 = 0x0117_6888;
        const STR_FMT2: u32 = 0x00EF_AE88;
        const AUX3_THIS: u32 = 0x0117_37D0;
        const ROW_SIZE: u32 = 0x31c;
        const STR_ROW: u32 = 0x00EF_AE98;
        const NOTIFY_ARG: u32 = 0x2a;
        const COUNT_LIMIT: u32 = 0x64;
        const GLOBAL_LISTENER: u32 = 0x018B_6C8C;
        const C_LOOKUP: u32 = 1;
        const C_CFG: u32 = 2;
        const C_TAG: u32 = 3;
        const C_COUNT: u32 = 4;
        const C_NOTIFY: u32 = 5;
        const C_LISTEN: u32 = 6;
        const C_FMT: u32 = 8;
        const C_HIDE: u32 = 10;
        const C_APPLY: u32 = 13;
        const C_FETCH: u32 = 15;
        const C_TOUCH: u32 = 16;
        const C_POSTA: u32 = 19;
        const C_POSTB: u32 = 20;
        const C_AUX2: u32 = 22;
        const C_AUX3: u32 = 23;
        const C_ALLOC: u32 = 24;
        const C_FMT3: u32 = 27;
        const C_BUILD: u32 = 28;
        const C_CAND: u32 = 37;
        const C_SINK2: u32 = 41;
        const C_NOTE: u32 = 42;
        const C_REPORT: u32 = 43;
        const C_FVAL: u32 = 44;
        const C_TAIL1: u32 = 46;
        const C_TAIL0: u32 = 47;
        const C_FIN0: u32 = 48;
        const C_TAG2: u32 = 49;
        const S_TAG: u32 = 0x0;
        const S_DISMISS: u32 = 0x120;
        const S_PROBE: u32 = 0x1d4;
        const S_SLOT: u32 = 0x1e0;
        const S_SELF4C: u32 = 0x4c;
        const S_SINK: u32 = 0x1e4;
        const S_DONE: u32 = 0x1b0;
        const S_GET48: u32 = 0x48;
        const S_SET170: u32 = 0x170;
        const S_SET17C: u32 = 0x17c;
        const S_SET188: u32 = 0x188;
        const S_SET28: u32 = 0x28;
        const S_SET44: u32 = 0x44;
        const S_SET224: u32 = 0x224;
        const S_LOOKUP: u32 = 0xf8;
        const S_ARR: u32 = 0x1d0;
        const S_TAIL: u32 = 0x21c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        /// Call through a fabricated object exactly like the original's
        /// `(an instruction of the original)`: load the vtable, load the slot, call it.
        #[inline(always)]
        unsafe fn icall0(obj: u32, off: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + off) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn icall1(obj: u32, off: u32, x: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + off) as usize);
                f(obj, x)
            }
        }


                let obj = lf_checker_rt::callee_thiscall!(
                    C_LOOKUP, u32, lf_checker_rt::relocated(REGISTRY), arg);
                let cfg = lf_checker_rt::callee_thiscall!(
                    C_CFG, u32, lf_checker_rt::relocated(REGISTRY),
                    lf_checker_rt::relocated(STR_CFG));
                wr8(cfg.wrapping_add(0x20b), 1);
                let tag = icall0(obj, S_TAG);
                let want = lf_checker_rt::callee_cdecl!(
                    C_TAG, u32, lf_checker_rt::relocated(STR_TAG1));
                if tag == want {
                    let count = lf_checker_rt::callee_thiscall!(
                        C_COUNT, u32, rd32(this.wrapping_add(0x1f8)));
                    if (count as i32) < COUNT_LIMIT as i32 {
                        lf_checker_rt::callee_cdecl!(
                            C_FMT, u32, lf_checker_rt::relocated(STR_FMT), 0);
                        if rd8(this.wrapping_add(0x218)) != 0 {
                            wr8(this.wrapping_add(0x218), 0);
                            icall1(rd32(this.wrapping_add(0x1e8)), S_DISMISS, 0);
                            let aux = rd32(this.wrapping_add(0x1ec));
                            wr8(aux.wrapping_add(0x1f4), 0);
                            lf_checker_rt::callee_thiscall!(
                                C_HIDE, u32, rd32(this.wrapping_add(0x1f0)), 0);
                        }
                        let list = rd32(this.wrapping_add(0x1e0));
                        if icall0(list, S_PROBE) != 0 {
                            let pick = icall0(list, S_PROBE).wrapping_sub(1);
                            let slot = icall1(list, S_SLOT, pick);
                            lf_checker_rt::callee_thiscall!(C_APPLY, u32, slot, 0);
                        }
                        wr8(rd32(list.wrapping_add(0x21c)).wrapping_add(0x1f9), 1);
                        wr32(this.wrapping_add(0x204), 0xffff_ffff);
                        let tag3 = lf_checker_rt::callee_cdecl!(
                            C_TAG, u32, lf_checker_rt::relocated(STR_TAG3));
                        let row = lf_checker_rt::callee_thiscall!(
                            C_FETCH, u32, this, obj, tag3, 1);
                        lf_checker_rt::callee_thiscall!(C_TOUCH, u32, this);
                        let r = icall0(row, S_SELF4C);
                        let w = icall1(list, S_SINK, r);
                        lf_checker_rt::callee_thiscall!(C_POSTA, u32, this, w);
                        lf_checker_rt::callee_thiscall!(C_POSTB, u32, this, row);
                        return icall0(this, S_DONE);
                    }
                    lf_checker_rt::callee_cdecl!(C_NOTIFY, u32, NOTIFY_ARG);
                    let listener: u32 =
                        lf_checker_rt::global::<u32>(GLOBAL_LISTENER).read();
                    return lf_checker_rt::callee_thiscall!(C_LISTEN, u32, listener);
                }
                let tag2 = icall0(obj, S_TAG);
                let want2 = lf_checker_rt::callee_cdecl!(
                    C_TAG2, u32, lf_checker_rt::relocated(STR_TAG2));
                if tag2 != want2 {
                    return want2;
                }
                let cfg2 = lf_checker_rt::callee_thiscall!(
                    C_CFG, u32, lf_checker_rt::relocated(REGISTRY),
                    lf_checker_rt::relocated(STR_CFG2));
                wr8(cfg2.wrapping_add(0x20b), 1);
                let listener: u32 =
                    lf_checker_rt::global::<u32>(GLOBAL_LISTENER).read();
                lf_checker_rt::callee_thiscall!(C_LISTEN, u32, listener);
                lf_checker_rt::callee_thiscall!(
                    C_AUX2, u32, lf_checker_rt::relocated(AUX2_THIS),
                    lf_checker_rt::relocated(STR_AUX2));
                lf_checker_rt::callee_cdecl!(
                    C_FMT, u32, lf_checker_rt::relocated(STR_FMT2), 0);
                lf_checker_rt::callee_thiscall!(
                    C_AUX3, u32, lf_checker_rt::relocated(AUX3_THIS), 0);
                let block = lf_checker_rt::callee_cdecl!(C_ALLOC, u32, ROW_SIZE);
                let list = rd32(this.wrapping_add(0x1e0));
                let row = if block != 0 {
                    let ansa = icall0(list, S_GET48);
                    let seq = rd32(this.wrapping_add(0x200));
                    let ansb = icall0(list, S_GET48);
                    let fmt = lf_checker_rt::callee_cdecl!(
                        C_FMT3, u32, lf_checker_rt::relocated(STR_ROW), ansb, seq);
                    lf_checker_rt::callee_thiscall!(C_BUILD, u32, block, obj, fmt, ansa)
                } else {
                    // The original skips the build and faults on the first use
                    // of the missing row; fault identically (volatile: the read
                    // must survive optimisation).
                    core::ptr::read_volatile(0 as *const u32)
                };
                // Touch the row exactly where the original does.
                core::ptr::read_volatile(row as *const u32);
                let t = icall0(this, S_SELF4C);
                icall1(row, S_SET170, t);
                let t = icall0(this, S_SELF4C);
                icall1(row, S_SET17C, t);
                let t = icall0(this, S_SELF4C);
                icall1(row, S_SET188, t);
                icall1(row, S_SET28, 1);
                icall1(row, S_SET44, 1);
                wr32(this.wrapping_add(0x200), rd32(this.wrapping_add(0x200)).wrapping_add(1));
                // Signed gate (`cmp x,0; jge`): negative indices recompute.
                if (rd32(this.wrapping_add(0x204)) as i32) < 0 {
                    let lookup = rd32(this.wrapping_add(0x1f4));
                    let cand = icall0(lookup, S_LOOKUP);
                    let cand_obj = rd32(rd32(cand).wrapping_add(8));
                    let z = lf_checker_rt::callee_thiscall!(C_CAND, u32, cand_obj);
                    // The original stores z into its dead incoming-arg slot here;
                    // the rewrite keeps it in a local (observed as the next ecx).
                    if rd32(cand_obj) == rd32(cand_obj.wrapping_add(0xc)) {
                        let r = icall0(z, S_SELF4C);
                        let idx = icall1(list, S_SINK, r);
                        if idx == 0xffff_ffff {
                            wr32(this.wrapping_add(0x204), 0);
                        } else {
                            wr32(this.wrapping_add(0x204), idx);
                        }
                    } else {
                        let r = icall0(z, S_SELF4C);
                        let idx = icall1(list, S_SINK, r);
                        wr32(this.wrapping_add(0x204), idx.wrapping_add(1));
                    }
                }
                let hold = rd32(rd32(this.wrapping_add(0x1e0)).wrapping_add(0x1e0));
                let arr = icall0(hold, S_ARR);
                wr16(arr.wrapping_add(4), rd16(arr.wrapping_add(4)).wrapping_add(0xffff));
                let hold = rd32(rd32(this.wrapping_add(0x1e0)).wrapping_add(0x1e0));
                let arr = icall0(hold, S_ARR);
                let idx = rd32(this.wrapping_add(0x204)) as i32;
                let mut pos = rd16(arr.wrapping_add(4)) as i32;
                let base = rd32(arr);
                while pos > idx {
                    pos -= 1;
                    wr32(
                        base.wrapping_add((pos as u32).wrapping_mul(4)).wrapping_add(4),
                        rd32(base.wrapping_add((pos as u32).wrapping_mul(4))),
                    );
                }
                wr16(arr.wrapping_add(4), rd16(arr.wrapping_add(4)).wrapping_add(1));
                let hold = rd32(rd32(this.wrapping_add(0x1e0)).wrapping_add(0x1e0));
                let arr = icall0(hold, S_ARR);
                let at = rd32(this.wrapping_add(0x204));
                wr32(rd32(arr).wrapping_add(at.wrapping_mul(4)), row);
                let cur = rd32(this.wrapping_add(0x204));
                let lim = rd32(this.wrapping_add(0x214));
                let vivo = if (cur as i32) > (lim as i32) { lim } else { lim.wrapping_add(1) };
                let sink = lf_checker_rt::callee_thiscall!(
                    C_SINK2, u32, rd32(this.wrapping_add(0x1f8)), vivo, cur);
                icall1(row, S_SET224, rd32(sink.wrapping_add(0x44)));
                if rd8(this.wrapping_add(0x21c)) == 1 {
                    lf_checker_rt::callee_thiscall!(
                        C_NOTE, u32, list, rd32(this.wrapping_add(0x204)), 1);
                }
                let cur2 = rd32(this.wrapping_add(0x204));
                let word = if (cur2 as i32) < 0 {
                    icall0(list, S_PROBE).wrapping_sub(1)
                } else {
                    cur2
                };
                lf_checker_rt::callee_thiscall!(
                    C_REPORT, u32, rd32(this.wrapping_add(0x1ec)), row, word, 0);
                let fval: f32 = lf_checker_rt::callee_thiscall!(
                    C_FVAL, f32, rd32(this.wrapping_add(0x1f8)));
                wr32(rd32(this.wrapping_add(0x1ec)).wrapping_add(0x1f0), fval.to_bits());
                let tail = icall0(hold, S_TAIL);
                lf_checker_rt::callee_thiscall!(
                    C_TAIL1, u32, rd32(this.wrapping_add(0x1ec)), tail);
                lf_checker_rt::callee_thiscall!(C_TAIL0, u32, rd32(this.wrapping_add(0x1ec)));
                lf_checker_rt::callee_thiscall!(C_POSTB, u32, this, row);
                lf_checker_rt::callee_thiscall!(
                    C_POSTA, u32, this, rd32(this.wrapping_add(0x204)));
                lf_checker_rt::callee_thiscall!(C_FIN0, u32, list);
                lf_checker_rt::callee_thiscall!(C_TOUCH, u32, this)
    
    }
});
