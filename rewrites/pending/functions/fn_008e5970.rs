// original: 0x008e5970 probe_then_demote_state
/// Probe the worker (thiscall/0, stubbed); when it reports nonzero and the
/// state word at `this+0x46C` is 6, demote it to 4. Always clears the flag
/// byte first. Returns the probe answer.
export!(thiscall, rw_008e5970(this: *mut u8) -> u32 {
    unsafe {
        *global::<u8>(0x0115DC08) = 0;
        let answer = callee_thiscall!(1, u32, this as u32);
        if answer as u8 != 0 {
            let slot = this.add(0x46C) as *mut u32;
            if read_unaligned(slot) == 6 {
                write_unaligned(slot, 4);
            }
        }
        answer
    }
});
