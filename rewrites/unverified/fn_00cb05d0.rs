// original: 0x00cb05d0 CTaskComplexMoveBeInFormation::vf20

/// Decide the next step of the move-in-formation task: refresh the formation
/// slot timer, mirror the leader's goal into the sub-task, then poll three
/// movement pickers in order until one offers a live replacement task.
///
/// `this` is the task object; `ped` points at the pedestrian (mode nibble at
/// `+MODE`). Returns the sub-task at `+SUBTASK` on every quiet path, a
/// picker's answer when one offers a live replacement, or 0 after releasing.
///
/// Behaviour:
/// - When the pedestrian's mode nibble is 2 or more, return the sub-task.
///   Otherwise clear latch bit `LATCH` of `+FLAGS2`.
/// - Gate stage. If bit 0 of `+FLAGS2` is set, probe the slot object at
///   `[this+SLOT]+SLOT_BIAS` through callee id 1 (skipped when null); a true
///   answer jumps to the mirror stage. Otherwise, when bit 0 of `+FLAGS` is
///   set return 0, else run the release helper (virtual slot `+0x14`,
///   id 2): a false answer returns the sub-task, a true one sets bit 1 of
///   `+FLAGS` and returns 0.
/// - Mirror stage. When the sub-task exists with kind `KIND_FORM` (id 3) and
///   the locator (id 4) answers live, copy four words (`+COPY0..+COPY3`
///   from the locator's `+SRC0..+SRC3`) into the sub-task; the two middle
///   words move through vector registers but are plain bit copies.
/// - Latch stage. Read selector `c`: 1 when the locator answers null, else
///   the masked mode `([[r+LOC_MODE]+MODE_AT] & MODE_MASK)`. For `c != 1`
///   store `c` into `+SELECT` (latch bit stays clear). For `c == 1`, store 1
///   when `+SELECT <= 1` (signed), else store 1 and set the latch bit.
/// - Poll stage. Unless the validity callee (id 5) answers true, repeat the
///   release-helper sequence (bit 0 of `+FLAGS` set returns 0; helper true
///   sets bit 1 and returns 0; helper false continues). Then run the fetch
///   callee (id 6) and poll the three pickers (ids 7, 8, 9) in order, each
///   taking `(ped, frame_buf)`: the first answer that is neither null nor
///   the current sub-task is returned; otherwise the sub-task is returned.
///   `frame_buf` points at an uninitialised stack slot holding the defined
///   fill; only the pointed-to words are compared.
///
/// Original: 0x00cb05d0 (thiscall, one stack word). No floating-point
/// arithmetic (vector moves only) and no globals.
fn decide_00cb05d0<const KIND_BUMP: bool>(this: u32, ped: u32) -> u32 {
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read_unaligned() }
    }
    fn kind_of(obj: u32) -> u32 {
        unsafe {
            let vtable = (obj as *const u32).read_unaligned();
            let target = ((vtable as *const u8).add(0x0c) as *const u32).read_unaligned();
            let query: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            query(obj)
        }
    }
    fn release_helper(this: u32, ped: u32) -> u32 {
        unsafe {
            let vtable = rd32(this);
            let target = rd32(vtable + 0x14);
            let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            f(this, ped, 1, 0)
        }
    }
    fn store_select(this: u32, c: u32) {
        unsafe {
            wr32(this + 0x78, c);
            wr32(this + 0xb4, rd32(this + 0xb4) & !4);
        }
    }
    unsafe {
        const SUBTASK: u32 = 0x08;
        const FLAGS: u32 = 0x0c;
        const SLOT: u32 = 0x20;
        const SLOT_BIAS: u32 = 8;
        const SELECT: u32 = 0x78;
        const FLAGS2: u32 = 0xb4;
        const LATCH: u32 = 4;
        const MODE: u32 = 0x1e2;
        const MODE_MASK: u32 = 0x0f;
        const MODE_MAX: u8 = 2;
        const LOC_INNER: u32 = 0x20;
        const LOC_MODE: u32 = 0x224;
        const MODE_AT: u32 = 0x2e8;
        const SEL_MASK: u32 = 7;
        const KIND_FORM: u32 = 0x3b7;
        const COPY0: u32 = 0x20;
        const SRC0: u32 = 0x30;
        let kind_want = if KIND_BUMP { KIND_FORM + 1 } else { KIND_FORM };
        if rd8(ped + MODE) & (MODE_MASK as u8) >= MODE_MAX {
            return rd32(this + SUBTASK);
        }
        wr32(this + FLAGS2, rd32(this + FLAGS2) & !LATCH);
        if rd8(this + FLAGS2) & 1 != 0 {
            let q = rd32(this + SLOT).wrapping_add(SLOT_BIAS);
            let mut jump = false;
            if q != 0 {
                if lf_checker_rt::callee_thiscall!(1, u32, q, ped) as u8 != 0 {
                    jump = true;
                }
            }
            if !jump {
                if rd8(this + FLAGS) & 1 != 0 {
                    return 0;
                }
                if release_helper(this, ped) as u8 == 0 {
                    return rd32(this + SUBTASK);
                }
                wr32(this + FLAGS, rd32(this + FLAGS) | 2);
                return 0;
            }
        }
        let task = rd32(this + SUBTASK);
        if task != 0 && kind_of(task) == kind_want {
            let r: u32 = lf_checker_rt::callee_thiscall!(4, u32, this);
            if r != 0 {
                let src = rd32(r + LOC_INNER);
                let dst = rd32(this + SUBTASK);
                wr32(dst + COPY0, rd32(src + SRC0));
                wr32(dst + COPY0 + 4, rd32(src + SRC0 + 4));
                wr32(dst + COPY0 + 8, rd32(src + SRC0 + 8));
                wr32(dst + COPY0 + 12, rd32(src + SRC0 + 12));
            }
        }
        let r2: u32 = lf_checker_rt::callee_thiscall!(4, u32, this);
        if r2 == 0 {
            if (rd32(this + SELECT) as i32) <= 1 {
                store_select(this, 1);
            } else {
                wr32(this + SELECT, 1);
                wr32(this + FLAGS2, rd32(this + FLAGS2) | LATCH);
            }
        } else {
            let m = rd32(rd32(r2 + LOC_MODE) + MODE_AT) & SEL_MASK;
            if m == 1 {
                if (rd32(this + SELECT) as i32) <= 1 {
                    store_select(this, 1);
                } else {
                    wr32(this + SELECT, 1);
                    wr32(this + FLAGS2, rd32(this + FLAGS2) | LATCH);
                }
            } else {
                store_select(this, m);
            }
        }
        if lf_checker_rt::callee_thiscall!(5, u32, this, ped) as u8 == 0 {
            if rd8(this + FLAGS) & 1 != 0 {
                return 0;
            }
            if release_helper(this, ped) as u8 != 0 {
                wr32(this + FLAGS, rd32(this + FLAGS) | 2);
                return 0;
            }
        }
        let mut buf: u32 = 0;
        let buf_ptr = (&mut buf as *mut u32) as u32;
        lf_checker_rt::callee_thiscall!(6, u32, this, buf_ptr);
        let a7: u32 = lf_checker_rt::callee_thiscall!(7, u32, this, ped, buf_ptr);
        if a7 != 0 && a7 != rd32(this + SUBTASK) {
            return a7;
        }
        let a8: u32 = lf_checker_rt::callee_thiscall!(8, u32, this, ped, buf_ptr);
        if a8 != 0 && a8 != rd32(this + SUBTASK) {
            return a8;
        }
        lf_checker_rt::callee_thiscall!(9, u32, this, ped, buf_ptr);
        rd32(this + SUBTASK)
    }
}

lf_checker_rt::export!(thiscall, rw_00cb05d0(this: u32, ped: u32) -> u32 {
    decide_00cb05d0::<false>(this, ped)
});
