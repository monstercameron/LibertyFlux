// original: 0x00878640 rage::crmtComposerOptimized::CombiningFilter::vf0

/// Initialise a combining filter and register it with the TLS owner.
///
/// `this` points to a combining-filter object. FS:0x2c gives the TLS
/// slot array, so slot 0 is read. The child count at `+0x2c`
/// is cleared and the combining-filter vtable installed. When the low bit
/// of `flags` is set, the filter registers itself: the thread-local owner
/// chain (TLS slot 0 holds a block whose word `+0` leads to a holder
/// whose word `+8` is the owner object) has its virtual at slot `+0xc`
/// invoked with `this`. Returns `this`.
///
/// Original: 0x00878640 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00878640(this: u32, flags: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x2c;
        const VTABLE_VA: u32 = 0x00e86afc;
        const TLS_SLOT: usize = 0;
        const HOLDER_OFF: u32 = 0x08;
        const REG_VT_SLOT: u32 = 0x0c;
        (this.wrapping_add(COUNT_OFF) as *mut u32).write_unaligned(0);
        (this as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE_VA));
        if (flags as u8) & 1 != 0 {
            let block = lf_checker_rt::tls_slot(TLS_SLOT);
            let holder = (block as *const u32).read_unaligned();
            let owner =
                (holder.wrapping_add(HOLDER_OFF) as *const u32).read_unaligned();
            let vtable = (owner as *const u32).read_unaligned();
            let reg: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
                core::mem::transmute(
                    ((vtable.wrapping_add(REG_VT_SLOT)) as *const u32).read_unaligned()
                        as usize,
                )
            };
            reg(owner, this);
        }
        this
    }
});
