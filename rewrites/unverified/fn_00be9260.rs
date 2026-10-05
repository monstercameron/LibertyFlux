// original: 0x00be9260 task_dispatch_on_kind (proposed)

/// Dispatch a task object `this` on its kind word at `+0x10`.
///
/// Resolves a handle for the id at `this+8` through callee 1; a null
/// handle, or one whose id at `+0xa4` disagrees, is returned as is. The
/// handle and the sub-block at `this+0x0c` go through callee 2 then the
/// kind word selects: 0x9b takes the attach path (callee 3 maps the
/// sub-block's target to an object, whose kind field at `+0x28` masked
/// with 0x3c0 must be 0x80 or the object is returned; otherwise callee 4
/// builds a context from the object (the original pushes five words but
/// the callee pops one, so only that word is observed), its slot at
/// `+0xbf8` is cleared, callee 5 renders a fixed address, the handle's
/// virtual slot 2
/// consumes the result, and callee 7 finalises with three zero words whose
/// answer is returned); 0x55 takes the float path (the float at `this+0x30`
/// and a fixed address go to the handle's virtual slot 3, then callees 9
/// and 10 each take a zero word); any other kind takes only the common
/// tail of callee 7 with three zero words, whose answer is returned.
///
/// The two virtual calls load their targets from the handle's table at
/// `+8` and `+0x0c`, exactly like the original; the checker plants its
/// stubs there. Returns a handle, an object, or callee 7's answer.
///
/// Original: 0x00be9260 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00be9260(this: u32) -> u32 {
    unsafe {
        const KIND_ATTACH: u32 = 0x9b;
        const KIND_FLOAT: u32 = 0x55;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_WANT: u32 = 0x80;
        const CTX_SLOT: u32 = 0xbf8;
        const VT_SLOT_A: u32 = 8;
        const VT_SLOT_B: u32 = 0x0c;
        const ADDR_C: u32 = 0x00eb9808;
        const ADDR_D: u32 = 0x00eb97fc;

        #[inline(always)]
        unsafe fn rd32(x: u32) -> u32 {
            unsafe { (x as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(x: u32, v: u32) {
            unsafe { (x as *mut u32).write_unaligned(v) }
        }

        let h: u32 = lf_checker_rt::callee_cdecl!(1, u32, rd32(this + 8));
        if h == 0 {
            return h;
        }
        if rd32(h + 0xa4) != rd32(this + 8) {
            return h;
        }
        lf_checker_rt::callee_cdecl!(2, u32, h, this.wrapping_add(0x0c));
        let kind = rd32(this + 0x10);
        if kind == KIND_ATTACH {
            let p: u32 = lf_checker_rt::callee_cdecl!(3, u32, 1u32, rd32(this + 0x0c));
            if rd32(p + 0x28) & KIND_MASK != KIND_WANT {
                return p;
            }
            // The original pushes five words here but the callee pops exactly
            // one (proved by the balanced return: five pushed minus one
            // popped minus the caller's 0x10 cleanup leaves the frame
            // exact). Only the consumed word is passed on.
            let q: u32 = lf_checker_rt::callee_stdcall!(4, u32, p);
            wr32(q + CTX_SLOT, 0);
            let v = rd32(h);
            let r: u32 =
                lf_checker_rt::callee_cdecl!(5, u32, lf_checker_rt::relocated(ADDR_C), 0u32);
            let slot_a: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(v + VT_SLOT_A) as usize);
            slot_a(h, r);
            let ans: u32 = lf_checker_rt::callee_thiscall!(7, u32, h, 0u32, 0u32, 0u32);
            return ans;
        }
        if kind == KIND_FLOAT {
            let f = rd32(this + 0x30);
            let v = rd32(h);
            let slot_b: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(v + VT_SLOT_B) as usize);
            slot_b(h, lf_checker_rt::relocated(ADDR_D), f);
            lf_checker_rt::callee_thiscall!(9, u32, h, 0u32);
            lf_checker_rt::callee_thiscall!(10, u32, h, 0u32);
        }
        let ans: u32 = lf_checker_rt::callee_thiscall!(7, u32, h, 0u32, 0u32, 0u32);
        ans
    }
});
