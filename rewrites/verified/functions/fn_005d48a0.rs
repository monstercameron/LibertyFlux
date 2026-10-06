// original: 0x005d48a0 CViewportPrimaryOrtho::vf12
//
// Refresh the four overlay slots of a primary-ortho viewport.
//
// `this` is the viewport and the stack argument is forwarded to a setup
// callee with `this`. Four blocks follow, each requesting an overlay
// object from the game's allocator (TLS slot 0, `+8` object, vtable slot
// `+8`) with a size tag (0x950, 0x940, then 0x950 or 0x940, then 0x950)
// and the constant arguments `0x10, 0`, then registering the result with
// the slot callee at `this + 0x400`: a null answer registers null, while
// a live answer is first adapted by a per-block adapter callee (whose
// answer is what registers, except in the third block, which registers
// the allocator answer itself after stamping it with a vtable and a
// constant word). Whether the third block uses the 0x940 or the 0x950
// tag, and whether a fourth allocation happens at all, is decided by a
// global mode byte.
//
// Original: 0x005d48a0 (thiscall, one stack word; returns the last slot
// callee's answer).
lf_checker_rt::export!(thiscall, rw_005d48a0(this: u32, arg: u32) -> u32 {
    unsafe {
        const ID_SETUP: u32 = 1;
        const ID_ADAPT1: u32 = 3;
        const ID_SLOT: u32 = 4;
        const ID_ADAPT2: u32 = 5;
        const ID_ADAPT3: u32 = 6;
        const ID_ADAPT4: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(ID_SETUP, u32, this, arg);
        let tls0 = lf_checker_rt::tls_slot(0);
        let alloc = rd32(tls0.wrapping_add(8));
        let request: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(alloc).wrapping_add(8)) as usize);
        let slot_obj = this.wrapping_add(0x400);
        let r1 = request(alloc, 0x950, 0x10, 0);
        let v1 = if r1 != 0 {
            lf_checker_rt::callee_thiscall!(ID_ADAPT1, u32, r1, this)
        } else {
            0
        };
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_SLOT, u32, slot_obj, v1);
        let r2 = request(rd32(tls0.wrapping_add(8)), 0x940, 0x10, 0);
        let v2 = if r2 != 0 {
            lf_checker_rt::callee_thiscall!(ID_ADAPT2, u32, r2)
        } else {
            0
        };
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_SLOT, u32, slot_obj, v2);
        let mode = ((lf_checker_rt::relocated(0x011609f6)) as *const u8).read();
        let r3 = if mode == 0 {
            request(rd32(tls0.wrapping_add(8)), 0x950, 0x10, 0)
        } else {
            let r = request(rd32(tls0.wrapping_add(8)), 0x940, 0x10, 0);
            if r != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(ID_ADAPT3, u32, r, this);
                wr32(r, lf_checker_rt::relocated(0x00fe1054));
                wr32(r.wrapping_add(0x8f4), 7);
                let _: u32 = lf_checker_rt::callee_thiscall!(ID_SLOT, u32, slot_obj, r);
            } else {
                let _: u32 = lf_checker_rt::callee_thiscall!(ID_SLOT, u32, slot_obj, 0);
            }
            request(rd32(tls0.wrapping_add(8)), 0x950, 0x10, 0)
        };
        let v4 = if r3 != 0 {
            lf_checker_rt::callee_thiscall!(ID_ADAPT4, u32, r3, this)
        } else {
            0
        };
        lf_checker_rt::callee_thiscall!(ID_SLOT, u32, slot_obj, v4)
    }
});
