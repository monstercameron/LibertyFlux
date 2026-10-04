// original: 0x00d69950 maybe_refresh_and_forward
// s16f17: refresh when enabled, then forward to the slot record
// (thiscall/1).
//
// When this record's owner word is set and the flag byte is nonzero, probes
// the owner and runs the refresh step on a positive probe. Then, when a
// slot record is present, stores the flag byte on it and — still for a
// nonzero flag — runs the head-fetch step the original reaches through a
// tail jump. EAX is caller garbage on several paths, so unchecked.
export!(thiscall, rw_s16f17(this: *mut u8, flag: u32) -> u32 {
    unsafe {
        let flag_byte = (flag & 0xFF) as u8;
        if *((this.add(0xC)) as *const u32) != 0 && flag_byte != 0 {
            let probe: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(callee_addr(1) as usize);
            if probe(this as u32) & 0xFF != 0 {
                let refresh: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(callee_addr(2) as usize);
                refresh(this as u32, 0);
            }
        }
        let slot = *((this.add(8)) as *const u32);
        if slot == 0 {
            return 0; // original leaves entry EAX here; callers ignore it
        }
        *((slot.wrapping_add(0x48)) as *mut u8) = flag_byte;
        if flag_byte == 0 {
            // Original merges the flag byte into entry EAX here; unchecked.
            return 0;
        }
        let head_fetch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(3) as usize);
        // The tail target reloads ECX from [slot+4] before the step.
        let head = *((slot.wrapping_add(4)) as *const u32);
        head_fetch(head)
    }
});
