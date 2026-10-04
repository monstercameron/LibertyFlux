// original: 0x00b31760 subtask_state_probe (proposed)

/// Probe a task's two sub-tasks for a ready or matching state.
///
/// `task` points to a task record holding sub-task pointers at `+0x28` and
/// `+0x2c` (either may be null). Each sub-task carries flag bits at `+0x28`
/// and a marker byte at `+0x219`. The first sub-task, then the second, is
/// tested for the ready state (flag bits `0x3C0` equal `0xC0` with the marker
/// set), returning the masked flag bits with the low byte set to 1 when
/// found. When neither is ready, each in turn is tested for the matching
/// state (flag bits equal `0x80`) and polled through a callee; a non-zero
/// answer returns the answer with its low byte set to 1. When nothing
/// answers, the last value seen (the second sub-task's answer or masked
/// flags, or the first's when the second is null) comes back with its low
/// byte cleared. Both pointers null is left out: that exit returns the
/// unreadable incoming accumulator.
///
/// Original: 0x00b31760 (cdecl, one stack word; one no-argument callee
/// reached from two sites).
lf_checker_rt::export!(cdecl, rw_00b31760(task: u32) -> u32 {
    unsafe {
        const SUB_A: u32 = 0x28;
        const SUB_B: u32 = 0x2c;
        const FLAGS: u32 = 0x28;
        const MARKER: u32 = 0x219;
        const FLAG_MASK: u32 = 0x3c0;
        const READY_BITS: u32 = 0xc0;
        const MATCH_BITS: u32 = 0x80;
        const POLL: u32 = 1;
        let a = (task as *const u32).byte_add(SUB_A as usize).read_unaligned();
        let b = (task as *const u32).byte_add(SUB_B as usize).read_unaligned();
        if a != 0 {
            let masked =
                (a as *const u32).byte_add(FLAGS as usize).read_unaligned() & FLAG_MASK;
            if masked == READY_BITS && (a as *const u8).byte_add(MARKER as usize).read() != 0 {
                return masked & !0xFF | 1;
            }
        }
        if b != 0 {
            let masked =
                (b as *const u32).byte_add(FLAGS as usize).read_unaligned() & FLAG_MASK;
            if masked == READY_BITS && (b as *const u8).byte_add(MARKER as usize).read() != 0 {
                return masked & !0xFF | 1;
            }
        }
        let mut last_a = 0u32;
        let mut have_a = false;
        if a != 0 {
            let masked =
                (a as *const u32).byte_add(FLAGS as usize).read_unaligned() & FLAG_MASK;
            if masked == MATCH_BITS {
                let answer: u32 = lf_checker_rt::callee_thiscall!(POLL, u32, a);
                if answer & 0xFF != 0 {
                    return answer & !0xFF | 1;
                }
                last_a = answer;
            } else {
                last_a = masked;
            }
            have_a = true;
        }
        if b != 0 {
            let masked =
                (b as *const u32).byte_add(FLAGS as usize).read_unaligned() & FLAG_MASK;
            if masked == MATCH_BITS {
                let answer: u32 = lf_checker_rt::callee_thiscall!(POLL, u32, b);
                if answer & 0xFF != 0 {
                    return answer & !0xFF | 1;
                }
                return answer & !0xFF;
            }
            return masked & !0xFF;
        }
        if have_a {
            return last_a & !0xFF;
        }
        0
    }
});
