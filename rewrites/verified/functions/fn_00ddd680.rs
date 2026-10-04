// original: 0x00ddd680 uitextfield_state_update (proposed)

/// Refresh a text field after its content or focus state changed.
///
/// `this` is the field object. Three child controls hang off it (dword
/// pointers at `+0x1e8`, `+0x1ec`, `+0x1f0`); the field keeps a cached value
/// at `+0x200`, a flag byte at `+0x209` and a state word at `+0x210`.
///
/// Behaviour: fetch the current text through virtual slot `+0x20c` of child
/// A and set the flag byte to 1. When the state word is not 7, offer the
/// text to the validate callee; skip reformatting when validation succeeds
/// (nonzero low byte). Otherwise measure the text length and hand an empty
/// scratch buffer plus the length to the format callee (the sibling function
/// at 0x00ddd4d0, scripted by the contract). Then poll the field: on a
/// nonzero answer enable child B's two slots with 1, disable child A's two
/// slots with 0, and clear the cached value; on zero enable child A's second
/// slot with 1 instead (the first call reuses the pushed 1 from the shared
/// argument slot on both paths). Always finish by disabling child C's two
/// slots with 0 and storing state 1. Returns the last call's answer.
///
/// Edge cases: an empty text measures length 0 and still reaches the format
/// callee; a state word of 7 skips validation entirely. The scratch buffer
/// handed to the format callee holds a zero first word; the second word is
/// unwritten stack scratch (the contract defines the fill as zero).
///
/// Original: 0x00ddd680 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00ddd680(this: u32) -> u32 {
    unsafe {
        const CHILD_A: u32 = 0x1e8;
        const CHILD_B: u32 = 0x1ec;
        const CHILD_C: u32 = 0x1f0;
        const CACHED_VALUE: u32 = 0x200;
        const DIRTY_FLAG: u32 = 0x209;
        const STATE: u32 = 0x210;
        const VT_GET_TEXT: u32 = 0x20c;
        const VT_SET_FIRST: u32 = 0x118;
        const VT_SET_SECOND: u32 = 0x120;
        const READY_STATE: u32 = 7;
        const CALLEE_VALIDATE: u32 = 1;
        const CALLEE_FORMAT: u32 = 2;
        const CALLEE_POLL: u32 = 3;
        const CALLEE_COOKIE: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        /// Call virtual `slot` on `child` with one stack argument.
        #[inline(always)]
        unsafe fn vcall1(child: u32, slot: u32, arg: u32) -> u32 {
            unsafe {
                let vt = rd32(child);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt + slot) as usize);
                f(child, arg)
            }
        }

        let child_a = rd32(this + CHILD_A);
        let vt_a = rd32(child_a);
        let get_text: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt_a + VT_GET_TEXT) as usize);
        let text = get_text(child_a);
        ((this + DIRTY_FLAG) as *mut u8).write(1);

        let settled = rd32(this + STATE) == READY_STATE;
        let mut need_format = settled;
        if !settled {
            let ok: u32 = lf_checker_rt::callee_thiscall!(CALLEE_VALIDATE, u32, this, text);
            need_format = (ok as u8) == 0;
        }
        if need_format {
            let mut p = text;
            let mut len = 0u32;
            while (p as *const u8).read() != 0 {
                p = p.wrapping_add(1);
                len = len.wrapping_add(1);
            }
            let buf = [0u32; 2];
            let buf_ptr = &buf as *const u32 as u32;
            lf_checker_rt::callee_thiscall!(CALLEE_FORMAT, u32, this, buf_ptr, len);
        }

        let child_b = rd32(this + CHILD_B);
        let child_c = rd32(this + CHILD_C);
        let live: u32 = lf_checker_rt::callee_thiscall!(CALLEE_POLL, u32, this);
        if (live as u8) != 0 {
            vcall1(child_b, VT_SET_FIRST, 1);
            vcall1(child_b, VT_SET_SECOND, 1);
            vcall1(child_a, VT_SET_FIRST, 0);
            vcall1(child_a, VT_SET_SECOND, 0);
            ((this + CACHED_VALUE) as *mut u32).write_unaligned(0);
        } else {
            vcall1(child_a, VT_SET_FIRST, 1);
            vcall1(child_a, VT_SET_SECOND, 1);
        }
        vcall1(child_c, VT_SET_FIRST, 0);
        let last = vcall1(child_c, VT_SET_SECOND, 0);
        ((this + STATE) as *mut u32).write_unaligned(1);
        // Mirror the original's stack-cookie check: the stub preserves all
        // registers, so the last call's answer is still the return value.
        lf_checker_rt::callee_cdecl!(CALLEE_COOKIE, u32,);
        last
    }
});
