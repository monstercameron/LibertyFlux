// original: 0x00a9ce40 stream_push_loader (proposed)

/// Append `obj` to the loader queue when the mode gate agrees, then run the
/// queue hooks.
///
/// The gate reads 1 when any of three global conditions holds (dword at
/// file address 0x011f7060 equals 1, dword at 0x012088b4 differs from dword
/// at 0x00f1c040, or dword at 0x01037720 equals 0x12), else 0. When the low
/// byte of `mode` differs from the gate, or the queue count at `this + 0x0`
/// has reached 128, the gate (respectively the count) is the result and
/// nothing happens. Otherwise `obj` is appended at slot `count` (slots of
/// 96 bytes at `this + 0x10`) through the appender, the slot is announced
/// through the announcer (slot contents in ECX, slot address on the stack),
/// and the count is incremented. The gate is then re-read: a 0 runs the
/// settlement hook with (`obj`, 0), and a `-1` word at `obj + 0x44` asks
/// the object's virtual slot `+0xa0` (object in ECX) for the final answer,
/// which is the result. When neither of those runs, the result is whatever
/// the re-read left in EAX: the announcer's answer on the first gate exit,
/// the B global itself on the middle two.
///
/// Original: 0x00a9ce40 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a9ce40(this: u32, obj: u32, mode: u32) -> u32 {
    unsafe {
        const GATE_A: u32 = 0x011f7060;
        const GATE_B: u32 = 0x012088b4;
        const GATE_C: u32 = 0x00f1c040;
        const GATE_D: u32 = 0x01037720;
        const GATE_D_WANT: u32 = 0x12;
        const COUNT_OFF: u32 = 0;
        const SLOTS_OFF: u32 = 0x10;
        const SLOT_STRIDE: u32 = 96;
        const CAPACITY: i32 = 0x80;
        const HOOK_OFF: u32 = 0x44;
        const ASK_SLOT: u32 = 0xa0;
        const APPEND: u32 = 1;
        const ANNOUNCE: u32 = 2;
        const SETTLE: u32 = 3;
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
        lf_checker_rt::callee_thiscall!(APPEND, u32, slot, obj);
        let head = (slot as *const u32).read_unaligned();
        let mut answer: u32 = lf_checker_rt::callee_thiscall!(ANNOUNCE, u32, head, slot);
        ((this + COUNT_OFF) as *mut u32).write_unaligned(count.wrapping_add(1));
        // The re-read gate is only normalised on two of its four exits: the
        // middle two leave the B global itself in EAX, and that leftover is
        // the result when neither the settlement nor the ask runs.
        let a2 = lf_checker_rt::global::<u32>(GATE_A).read_unaligned();
        if a2 != 1 {
            let b2 = lf_checker_rt::global::<u32>(GATE_B).read_unaligned();
            answer = b2;
            let c2 = lf_checker_rt::global::<u32>(GATE_C).read_unaligned();
            let d2 = lf_checker_rt::global::<u32>(GATE_D).read_unaligned();
            if b2 == c2 && d2 != GATE_D_WANT {
                answer = lf_checker_rt::callee_cdecl!(SETTLE, u32, obj, 0);
            }
        }
        if ((obj + HOOK_OFF) as *const u32).read_unaligned() == 0xffff_ffff {
            let target = (obj as *const u32).read_unaligned();
            let vtable = (target as *const u32).read_unaligned();
            let at = ((vtable + ASK_SLOT) as *const u32).read_unaligned();
            let ask: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(at as usize);
            answer = ask(target);
        }
        answer
    }
});
