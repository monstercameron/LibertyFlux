// original: 0x00a477b0 CVehicle::vf75
/// Bit 5 of the flag byte at `this+0xF1D`, gated by two intercepted calls.
///
/// Calls the readiness check (callee 1, thiscall, no stack arguments); a
/// nonzero low byte means 0. Then a word at `this+0xDE4` of 0xFFFF means 0.
/// Then calls the filler (callee 2, cdecl, two stack words: `this` and a
/// scratch pointer the original never reads back); a nonzero low byte means
/// 0. Else returns bit 5 of `this+0xF1D` (thiscall, no stack arguments).
/// Only AL is compared. The scratch address is skipped in the call log.
export!(thiscall, rw_00a477b0(this: u32) -> u32 {
    unsafe {
        const READY_OFF: u32 = 0xde4;
        const READY_SKIP: u16 = 0xffff;
        const FLAG_OFF: u32 = 0xf1d;
        const FLAG_BIT: u32 = 5;
        let r1: u32 = callee_thiscall!(1, u32, this);
        if (r1 & 0xff) != 0 {
            return 0;
        }
        if (this.wrapping_add(READY_OFF) as *const u16).read_unaligned() == READY_SKIP {
            return 0;
        }
        let mut scratch = 0u32;
        let r2: u32 = callee_cdecl!(2, u32, this, core::ptr::addr_of_mut!(scratch) as u32);
        if (r2 & 0xff) != 0 {
            return 0;
        }
        (((this.wrapping_add(FLAG_OFF)) as *const u8).read() >> FLAG_BIT) as u32 & 1
    }
});
