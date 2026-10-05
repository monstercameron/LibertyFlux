// original: 0x00a9cef0 stream_stage_descriptor (proposed)

/// Copy a descriptor into the staging table when the mode gate agrees, then
/// run the queue hooks.
///
/// The gate is the same three-condition global read as the neighbouring
/// loader (file addresses 0x011f7060, 0x012088b4, 0x00f1c040, 0x01037720
/// against 1, equal, 0x12). When the low byte of `mode` differs from the
/// gate, or the staged count at `this + 0x3010` has reached 128, the gate
/// (respectively the count) is the result and nothing happens. Otherwise
/// the words at `obj + 0x0` / `+0x10` / `+0x1c` / `+0x20` / `+0x24`, the
/// float bits at `+0x14` / `+0x18` and the byte at `+0x28` are copied into
/// slot `count` (slots of 48 bytes at `this + 0x3020`, fields at `+0x0`,
/// `+0x10`, `+0x14`, `+0x18`, `+0x1c`, `+0x20`, `+0x24`, `+0x28`), the slot
/// is announced (slot contents in ECX, slot address on the stack), and the
/// count is incremented. The gate is then re-read: a 0 runs the settlement
/// hook with (0, `obj`) — note the swapped order against the neighbouring
/// loader — and its answer is the result. As there, the middle two re-read
/// exits leave the B global itself in EAX.
///
/// Original: 0x00a9cef0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a9cef0(this: u32, obj: u32, mode: u32) -> u32 {
    unsafe {
        const GATE_A: u32 = 0x011f7060;
        const GATE_B: u32 = 0x012088b4;
        const GATE_C: u32 = 0x00f1c040;
        const GATE_D: u32 = 0x01037720;
        const GATE_D_WANT: u32 = 0x12;
        const COUNT_OFF: u32 = 0x3010;
        const SLOTS_OFF: u32 = 0x3020;
        const SLOT_STRIDE: u32 = 48;
        const CAPACITY: i32 = 0x80;
        const ANNOUNCE: u32 = 1;
        const SETTLE: u32 = 2;
        let gate = || -> u32 {
            unsafe {
                let a = lf_checker_rt::global::<u32>(GATE_A).read_unaligned();
                if a == 1 {
                    return 1;
                }
                let b = lf_checker_rt::global::<u32>(GATE_B).read_unaligned();
                let c = lf_checker_rt::global::<u32>(GATE_C).read_unaligned();
                if b != c {
                    return 1;
                }
                let d = lf_checker_rt::global::<u32>(GATE_D).read_unaligned();
                if d == GATE_D_WANT {
                    return 1;
                }
                0
            }
        };
        let g = gate();
        if mode as u8 != g as u8 {
            return g;
        }
        let count = ((this + COUNT_OFF) as *const u32).read_unaligned();
        if count as i32 >= CAPACITY {
            return count;
        }
        let slot = this
            .wrapping_add(SLOTS_OFF)
            .wrapping_add(count.wrapping_mul(SLOT_STRIDE));
        ((slot) as *mut u32)
            .write_unaligned((obj as *const u32).read_unaligned());
        ((slot + 0x10) as *mut u32)
            .write_unaligned(((obj + 0x10) as *const u32).read_unaligned());
        ((slot + 0x14) as *mut u32)
            .write_unaligned(((obj + 0x14) as *const u32).read_unaligned());
        ((slot + 0x18) as *mut u32)
            .write_unaligned(((obj + 0x18) as *const u32).read_unaligned());
        ((slot + 0x1c) as *mut u32)
            .write_unaligned(((obj + 0x1c) as *const u32).read_unaligned());
        ((slot + 0x20) as *mut u32)
            .write_unaligned(((obj + 0x20) as *const u32).read_unaligned());
        ((slot + 0x24) as *mut u32)
            .write_unaligned(((obj + 0x24) as *const u32).read_unaligned());
        ((slot + 0x28) as *mut u8).write(((obj + 0x28) as *const u8).read());
        let head = (slot as *const u32).read_unaligned();
        let mut answer: u32 = lf_checker_rt::callee_thiscall!(ANNOUNCE, u32, head, slot);
        ((this + COUNT_OFF) as *mut u32).write_unaligned(count.wrapping_add(1));
        let a2 = lf_checker_rt::global::<u32>(GATE_A).read_unaligned();
        if a2 != 1 {
            let b2 = lf_checker_rt::global::<u32>(GATE_B).read_unaligned();
            answer = b2;
            let c2 = lf_checker_rt::global::<u32>(GATE_C).read_unaligned();
            let d2 = lf_checker_rt::global::<u32>(GATE_D).read_unaligned();
            if b2 == c2 && d2 != GATE_D_WANT {
                answer = lf_checker_rt::callee_cdecl!(SETTLE, u32, 0, obj);
            }
        }
        answer
    }
});
