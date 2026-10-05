// original: 0x00870110 crmt_composer_optimized_reset (proposed)

/// Reset the optimized composer at `this`, then tail-jump to the shared
/// finaliser.
///
/// Installs the composer vtable, frees the block at `this` + 4 through the
/// thread-local manager when non-null (TLS slot 0 leads to the manager at
/// thread + 8; the free routine is its vtable slot +0xC), zeroes the header
/// words (+4 through +0x10 and +0x18, with +0x14 set to the all-ones empty
/// marker), then initialises 64 slots descending from `this` + 0x1B1C
/// (stride 0x34, each getting the slot vtable at +0 and zero at +0x2C; the
/// counter runs 63 down to 0 compared as signed). Control passes to the
/// finaliser with `this`, whose result is returned. The block test is an
/// exact null check.
///
/// Original: 0x00870110 (thiscall, no stack arguments; ends in a tail call).
lf_checker_rt::export!(thiscall, rw_00870110(this: u32) -> u32 {
    const VTABLE: u32 = 0x00fe7ec8;
    const SLOT_VTABLE: u32 = 0x00e86afc;
    const SLOTS_TOP: u32 = 0x1b1c;
    const SLOT_STRIDE: u32 = 0x34;
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const EMPTY_MARK: u32 = 0xffff_ffff;
    const FREE: u32 = 1;
    const FINALISE: u32 = 2;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let block = ((this + 4) as *const u32).read_unaligned();
        if block != 0 {
            let thread = lf_checker_rt::tls_slot(0);
            let manager = ((thread + MANAGER_OFF) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable + FREE_SLOT) as *const u32).read_unaligned();
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            free(manager, block);
        }
        ((this + 0x04) as *mut u32).write_unaligned(0);
        ((this + 0x08) as *mut u32).write_unaligned(0);
        ((this + 0x0c) as *mut u32).write_unaligned(0);
        ((this + 0x10) as *mut u32).write_unaligned(0);
        ((this + 0x14) as *mut u32).write_unaligned(EMPTY_MARK);
        ((this + 0x18) as *mut u32).write_unaligned(0);
        let mut slot = this + SLOTS_TOP;
        let mut remaining = 64u32;
        while remaining != 0 {
            slot -= SLOT_STRIDE;
            (slot as *mut u32).write_unaligned(lf_checker_rt::relocated(SLOT_VTABLE));
            ((slot + 0x2c) as *mut u32).write_unaligned(0);
            remaining -= 1;
        }
        lf_checker_rt::callee_thiscall!(FINALISE, u32, this)
    }
});
