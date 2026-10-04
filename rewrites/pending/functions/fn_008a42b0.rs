// original: 0x008a42b0 rage::audVariableCurveSound::vf5
/// Deleting destructor of `audVariableCurveSound`.
///
/// Original 0x008A42B0: same shape as rw_008a37f0 without the +0x39 gate.
export!(thiscall, rw_008a42b0(this: u32, flag: u32) -> u32 {    unsafe {
        (this as *mut u32).write_unaligned(relocated(0xe7af9c));
    }
    let id = unsafe { ((this + 0x48) as *const u8).read_unaligned() } as u32;
    if id != 0xFF {
        let bank = unsafe { ((this + 0x40) as *const u8).read_unaligned() } as u32;
        if ({
        let __stride = unsafe { global::<u32>(0x115d964).read_unaligned() };
        let __base = unsafe { global::<u32>(0x115d988).read_unaligned() };
        let __entry = unsafe {
            (__base
                .wrapping_add((bank).wrapping_mul(0x6f40))
                .wrapping_add(0x6f10) as *const u32)
                .read_unaligned()
        };
        __entry.wrapping_add(__stride.wrapping_mul(id))
    }) != 0 {
            callee_thiscall!(1, u32, this, 0);
        }
    }
    callee_thiscall!(2, u32, this);
    if flag & 1 != 0 {
        let bank = unsafe { ((this + 0x40) as *const u8).read_unaligned() } as u32;
        callee_thiscall!(3, u32, relocated(0x115d8a0), this, bank);
    }
    this
});
