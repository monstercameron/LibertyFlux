// original: 0x00AF76F0 veh_handle_init (proposed)

/// Initialise a fresh handle object to its default state.
///
/// Notifies the owning pool through callee 1 (the pool-slot claim taking a
/// zero argument), then fills the fields: dwords at `+0x04`, `+0x08`,
/// `+0x0C`, `+0x10` and `+0x1C` to zero, `+0x18` to -1.0, `+0x20` to
/// `0x14001E`, `+0x24` to `0x14`, the word at `+0x28` to `0x400`, the byte at
/// `+0x2A` to zero, and the flag byte at `+0x2B` to bit 6 of its old value
/// with bit 1 set. Nothing is returned.
///
/// Original: 0x00AF76F0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00AF76F0(this: u32) -> u32 {
    unsafe {
        const CLAIM: u32 = 1;
        let flag = ((this + 0x2B) as *const u8).read();
        lf_checker_rt::callee_thiscall!(CLAIM, u32, this, 0u32);
        for off in [0x04u32, 0x08, 0x0C, 0x10, 0x1C] {
            ((this + off) as *mut u32).write_unaligned(0);
        }
        ((this + 0x18) as *mut u32).write_unaligned(0xBF80_0000);
        ((this + 0x20) as *mut u32).write_unaligned(0x14001E);
        ((this + 0x24) as *mut u32).write_unaligned(0x14);
        ((this + 0x28) as *mut u16).write_unaligned(0x400);
        ((this + 0x2A) as *mut u8).write(0);
        ((this + 0x2B) as *mut u8).write((flag & 0x40) | 2);
        0
    }
});
