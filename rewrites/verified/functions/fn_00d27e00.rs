// original: 0x00d27e00 targetting_base_construct (proposed)

/// Construct the targetting base object: size the list, clear the 8 slot
/// records, set defaults and register with the manager.
///
/// Calls the list-sizing helper (intercepted) with 9 on `this + 0x10`, then
/// installs the (relocated) vtable address and clears the 8 slot records at
/// `+0x20 + i * 0x40` via the slot-clear helper (intercepted). Sets the word
/// at `+0x228` to all-ones, the byte at `+0x220`, the words at `+0x224`,
/// `+0x22c`, `+0x230`, `+0x234`, `+0x238` and the byte at `+0x23c` to zero,
/// and finally calls the register helper (intercepted) with `this + 0x10`.
/// Returns `this`.
///
/// Original: 0x00D27E00 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d27e00(this: u32) -> u32 {
    unsafe {
        const LIST_OFF: u32 = 0x10;
        const LIST_SIZE: u32 = 9;
        const VTABLE_VA: u32 = 0x00ee_1bc4;
        const SLOT_BASE: u32 = 0x20;
        const SLOT_STRIDE: u32 = 0x40;
        const SLOT_COUNT: u32 = 8;
        let list = this + LIST_OFF;
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, list, LIST_SIZE);
        unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_VA)) };
        let mut i = 0u32;
        while i < SLOT_COUNT {
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this + SLOT_BASE + i * SLOT_STRIDE);
            i += 1;
        }
        unsafe { ((this + 0x228) as *mut u16).write_unaligned(0xffff) };
        unsafe { ((this + 0x220) as *mut u8).write(0) };
        for off in [0x224u32, 0x22c, 0x230] {
            unsafe { ((this + off) as *mut u32).write_unaligned(0) };
        }
        unsafe { ((this + 0x234) as *mut u16).write_unaligned(0) };
        unsafe { ((this + 0x238) as *mut u32).write_unaligned(0) };
        unsafe { ((this + 0x23c) as *mut u8).write(0) };
        let _: u32 = lf_checker_rt::callee_cdecl!(3, u32, list);
        this
    }
});
