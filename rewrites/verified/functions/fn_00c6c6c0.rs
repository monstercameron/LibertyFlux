// original: 0x00c6c6c0 anim_dict_dtor (proposed)

/// Dictionary destructor: release the shared member, every entry, and
/// both buffers, then run the base destructor (tail call).
///
/// The member at `+8` is released through its vtable slot 0 when its
/// count at `+0xC` drops to zero. Each of the `count` entries at the
/// array in `+0x18` has the halfword at its offset 4 decremented and
/// is released through slot 0 on reaching zero. The buffers at `+0x18`
/// and `+0x10` are freed when the halfwords at `+0x1E` and `+0x16` are
/// nonzero. The vtable slot is tagged first and `+8` cleared.
///
/// Original: thiscall with no stack words, two indirect calls, two
/// direct calls, and a tail call.
lf_checker_rt::export!(thiscall, rw_00c6c6c0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EC_C104;
        const RELEASE: u32 = 1;
        const FREE: u32 = 2;
        const BASE_DTOR: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }

        let shared = rd32(this.wrapping_add(8));
        wr32(this, lf_checker_rt::relocated(VTABLE));
        if shared != 0 {
            let rc = rd32(shared.wrapping_add(0xC)).wrapping_sub(1);
            wr32(shared.wrapping_add(0xC), rc);
            if rc == 0 {
                let vt = rd32(shared);
                let slot = rd32(vt);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    unsafe { core::mem::transmute(slot as usize) };
                f(shared, 1);
            }
        }
        wr32(this.wrapping_add(8), 0);
        let mut i = 0u32;
        while i < rd16(this.wrapping_add(0x1C)) {
            let arr = rd32(this.wrapping_add(0x18));
            let elem = rd32(arr.wrapping_add(i.wrapping_mul(4)));
            let rc =
                ((elem.wrapping_add(4) as *const u16).read_unaligned()).wrapping_add(0xFFFF);
            unsafe { ((elem.wrapping_add(4)) as *mut u16).write_unaligned(rc) };
            if rc == 0 {
                let vt = rd32(elem);
                let slot = rd32(vt);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    unsafe { core::mem::transmute(slot as usize) };
                f(elem, 1);
            }
            i = i.wrapping_add(1);
        }
        if rd16(this.wrapping_add(0x1E)) != 0 {
            lf_checker_rt::callee_cdecl!(FREE, u32, rd32(this.wrapping_add(0x18)));
        }
        if rd16(this.wrapping_add(0x16)) != 0 {
            lf_checker_rt::callee_cdecl!(FREE, u32, rd32(this.wrapping_add(0x10)));
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
