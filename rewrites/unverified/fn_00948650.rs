// original: 0x00948650 store_first_free_triple (proposed)

/// Find the first free 12-byte slot via callee 1 and store a triple into it.
///
/// `this` (ECX) holds 50 slots of 12 bytes. Calls the probe (callee 1)
/// with (`this`, index) for indices upward from 0: the first index whose
/// answer has a non-zero LOW byte wins (only `al` is tested). The winning
/// slot gets (`d0`, `d1`, low byte of `b`) at offsets 0, 4, 8 and the
/// return is 1; if no index answers non-zero in 50 tries the return is 0.
/// Only the low byte of the return is defined.
///
/// Original: 0x00948650 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00948650(this: u32, d0: u32, d1: u32, b: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 50;
        const STRIDE: u32 = 12;
        const PROBE: u32 = 1;
        let mut found = false;
        let mut i = 0u32;
        loop {
            if found {
                break;
            }
            let r = lf_checker_rt::callee_thiscall!(PROBE, u32, this, i);
            if (r & 0xFF) != 0 {
                found = true;
            } else {
                i += 1;
                if i >= COUNT {
                    break;
                }
            }
        }
        if !found {
            return 0;
        }
        if i >= COUNT {
            return 0;
        }
        let slot = this.wrapping_add(i.wrapping_mul(STRIDE));
        (slot as *mut u32).write_unaligned(d0);
        (slot.wrapping_add(4) as *mut u32).write_unaligned(d1);
        (slot.wrapping_add(8) as *mut u8).write(b as u8);
        1
    }
});
