// original: 0x00B63F10 veh_clamp_handle_5c
/// Clamp an argument against a resolver-provided signed cap, store at `[this+0x5c]`.
///
/// Calls the handle resolver (stubbed, cdecl/1) with `[this+0x18]`, reads a
/// signed 16-bit cap at offset 0x86 of the resolved object, stores
/// `min(a0, cap) & 0xFFFF` (signed comparison) to `[this+0x5c]` and returns it.
/// Thiscall, one stack word; entry registers except ECX are ignored.
export!(thiscall, rw_00b63f10(this: u32, a0: u32) -> u32 {
    unsafe {
        const HANDLE: u32 = 0x18;
        const CAP: u32 = 0x86;
        const SLOT: u32 = 0x5c;
        let h: u32 = callee_cdecl!(1, u32, ((this + HANDLE) as *const u32).read_unaligned());
        let cap = ((h + CAP) as *const i16).read_unaligned() as i32;
        let v = if (a0 as i32) > cap { cap } else { a0 as i32 };
        let out = (v as u32) & 0xFFFF;
        ((this + SLOT) as *mut u32).write_unaligned(out);
        out
    }
});
