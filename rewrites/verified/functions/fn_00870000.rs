// original: 0x00870000 rage::crmtComposerData::vf1

/// Release the composer's data block and reset the header at `this`.
///
/// When the block pointer at `this` + 4 is non-null it is freed through the
/// thread-local manager (TLS slot 0 points at the thread block, whose word
/// at +8 is the manager object; the free routine is the vtable slot at +0xC
/// of the manager's table, taking the block as its stack argument). The
/// header is then reset: words +4 through +0x10 and +0x18 to zero, +0x14 to
/// all-ones (empty marker). The null check on the block is an exact
/// null-versus-non-null test, not a signed comparison. Returns zero.
///
/// Original: 0x00870000 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00870000(this: u32) -> u32 {
    const BLOCK_OFF: u32 = 4;
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const EMPTY_MARK: u32 = 0xffff_ffff;
    unsafe {
        let block = ((this + BLOCK_OFF) as *const u32).read_unaligned();
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
    }
    0
});
