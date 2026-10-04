// original: 0x00d27e90 targetting_teardown (proposed)

/// Tear down the handle area, then tail-call the base teardown.
///
/// Installs the (relocated) teardown vtable address, clears the word at
/// `+0x24c` when nonzero, releases each nonzero word of the 8-word table at
/// `+0x250` with the release helper (intercepted) and clears it, releases the
/// word at `+0x248` the same way, then tail-calls the base teardown
/// (intercepted) on `this`. The rewrite forwards the tail as a call.
///
/// Original: 0x00D27E90 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d27e90(this: u32) -> u32 {
    unsafe {
        const VTABLE_VA: u32 = 0x00ee_1c00;
        const FLAG_OFF: u32 = 0x24c;
        const TABLE_BASE: u32 = 0x250;
        const TABLE_COUNT: u32 = 8;
        const REF_OFF: u32 = 0x248;
        unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_VA)) };
        if unsafe { ((this + FLAG_OFF) as *const u32).read_unaligned() } != 0 {
            unsafe { ((this + FLAG_OFF) as *mut u32).write_unaligned(0) };
        }
        let mut i = 0u32;
        while i < TABLE_COUNT {
            let slot = this + TABLE_BASE + i * 4;
            if unsafe { (slot as *const u32).read_unaligned() } != 0 {
                let _: u32 = lf_checker_rt::callee_stdcall!(1, u32, slot);
                unsafe { (slot as *mut u32).write_unaligned(0) };
            }
            i += 1;
        }
        let slot = this + REF_OFF;
        if unsafe { (slot as *const u32).read_unaligned() } != 0 {
            let _: u32 = lf_checker_rt::callee_stdcall!(1, u32, slot);
            unsafe { (slot as *mut u32).write_unaligned(0) };
        }
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
