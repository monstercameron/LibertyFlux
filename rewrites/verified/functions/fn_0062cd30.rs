// original: 0x0062CD30 rage::ProceduralTextureVerletWater::vf0 (symbols)

/// Deleting destructor releasing two owned pointers, then base and free.
///
/// Stamps the class vtable pointer at `+0x00`, then releases each non-null
/// member at `MEMBER_A` (`+0x84`) and `MEMBER_B` (`+0x88`) through its slot-0
/// release with argument 1, clearing the slot. Then the base destructor runs
/// (patched callee) and, when the flag's low bit is set, the object is freed
/// through the thread-local allocator's free slot (`+0x0c`). Returns `this`
/// (thiscall, one argument).
lf_checker_rt::export!(thiscall, rw_0062cd30(this: u32, flag: u32) -> u32 {
    unsafe {
        const MEMBER_A: u32 = 0x84;
        const MEMBER_B: u32 = 0x88;
        const VTABLE: u32 = 0xFE23DC;
        const CALLEE_BASE: u32 = 3;
        unsafe fn release_one(member: u32) {
            unsafe {
                let rvt = (member as *const u32).read_unaligned();
                let rtgt = (rvt as *const u32).read_unaligned();
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rtgt as usize);
                release(member, 1);
            }
        }
        ((this + 0x00) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let a = ((this + MEMBER_A) as *const u32).read_unaligned();
        if a != 0 {
            release_one(a);
            ((this + MEMBER_A) as *mut u32).write_unaligned(0);
        }
        let b = ((this + MEMBER_B) as *const u32).read_unaligned();
        if b != 0 {
            release_one(b);
            ((this + MEMBER_B) as *mut u32).write_unaligned(0);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_BASE, u32, this);
        if flag & 1 == 0 {
            return this;
        }
        let holder = lf_checker_rt::tls_slot(0);
        let frobj = ((holder + 8) as *const u32).read_unaligned();
        let fvt = (frobj as *const u32).read_unaligned();
        let ftgt = ((fvt + 0x0c) as *const u32).read_unaligned();
        let free: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(ftgt as usize);
        free(frobj, this);        this
    }
});
