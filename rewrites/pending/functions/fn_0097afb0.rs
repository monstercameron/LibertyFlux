// original: 0x0097AFB0 audio_bank_stamp_if_flagged
/// Stamp the bank record when the object's flag word asks for it (tail call).
///
/// Takes the shared path only when the flag bit is set and the bank link is
/// non-null. A null object faults on the flag read, like the original.
/// cdecl(arg0, obj).
export!(cdecl, rw_s103_97afb0(_a0: u32, a1: *const u8) -> u32 {
    unsafe {
        if (*a1.add(0x26C) & 4) != 0 {
            let t = *(((a1 as usize) + 0xB30) as *const u32);
            if t != 0 {
                return callee_thiscall!(1, u32, t.wrapping_add(0x210));
            }
        }
        a1 as usize as u32
    }
});
