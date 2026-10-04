// original: 0x00d294e0 target_entry_clear (proposed)

/// Release a target entry's reference and clear the entry.
///
/// When the word at `this + 0x14` is nonzero, passes its address to the
/// release helper (intercepted) and clears it; then zeroes the words at
/// `+0x00`, `+0x04`, `+0x08`, `+0x10`, `+0x18`, `+0x1c`, `+0x20`, `+0x24` and
/// `+0x30` and the byte at `+0x2d`, clears the low three bits of the word at
/// `+0x28` and bit 0 of the byte at `+0x2c`. The original leaves `eax`
/// untouched, so no return channel is compared.
///
/// Original: 0x00D294E0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d294e0(this: u32) -> u32 {
    unsafe {
        const REF_OFF: u32 = 0x14;
        const KEEP_MASK: u32 = 0xffff_fff8;
        const FLAG_MASK: u8 = 0xfe;
        let slot = this + REF_OFF;
        if unsafe { (slot as *const u32).read_unaligned() } != 0 {
            let _: u32 = lf_checker_rt::callee_stdcall!(1, u32, slot);
            unsafe { (slot as *mut u32).write_unaligned(0) };
        }
        unsafe { ((this + 0x28) as *mut u32).write_unaligned(((this + 0x28) as *const u32).read_unaligned() & KEEP_MASK) };
        unsafe { ((this + 0x2c) as *mut u8).write(((this + 0x2c) as *const u8).read() & FLAG_MASK) };
        for off in [0x18u32, 0x1c, 0x20, 0x24] {
            unsafe { ((this + off) as *mut u32).write_unaligned(0) };
        }
        unsafe { ((this + 0x2d) as *mut u8).write(0) };
        for off in [0x08u32, 0x04, 0x00, 0x10, 0x30] {
            unsafe { ((this + off) as *mut u32).write_unaligned(0) };
        }
        0
    }
});
