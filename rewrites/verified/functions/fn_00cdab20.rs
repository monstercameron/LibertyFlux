// original: 0x00CDAB20 task_object_cleanup_40_50 (proposed)

/// Install the object's vtable; when `this+0x40` is non-null, pass the fixed negative-one-thousand float bits to the effect helper, notify the related object, set bit `0x4000` in its word at `+4`, and clear `this+0x40`. Release the optional pointer at `this+0x50`, then tail-call the base destructor. The zero-argument thiscall returns the tail-call result.
lf_checker_rt::export!(thiscall, rw_00cdab20(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        const OBJECT_VTABLE: u32 = 0x00edd68c;
        const EFFECT_TIME_BITS: u32 = 0xc47a0000;
        let effect = read_word(this + 0x40);
        write_word(this, lf_checker_rt::relocated(OBJECT_VTABLE));
        if effect != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, effect, EFFECT_TIME_BITS);
            let _ = lf_checker_rt::callee_thiscall!(2, u32, effect, this);
            let flags = read_word(effect + 4);
            write_word(effect + 4, flags | 0x4000);
            write_word(this + 0x40, 0);
        }
        let auxiliary = read_word(this + 0x50);
        if auxiliary != 0 {
            let _ = lf_checker_rt::callee_thiscall!(3, u32, auxiliary, this + 0x50);
        }
        lf_checker_rt::callee_thiscall!(4, u32, this)
    }
});
