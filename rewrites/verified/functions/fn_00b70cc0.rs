// original: 0x00b70cc0 CTaskComplexGoToCarDoorAndStandStill::vf5
/// Veto-or-forward gate for a car-door subtask.
///
/// `this` points to the task; three stack words (`a0`, `a1`, `a2`) are the
/// forwarded arguments. First gate: when the flag byte at `+0x74` is set, the
/// child at `+0x8` is non-null, its slot-3 method (callee 1, thiscall/0)
/// answers 0xca, `a2` is non-null, and `a2`'s slot-1 method (callee 2,
/// thiscall/0) answers 0x42 on the first call or 0x21 on the second, the
/// function returns 0.
///
/// Otherwise the child's slot-5 method (callee 3, thiscall/3: child, a0, a1,
/// a2) runs unless bit 0 of the child's `+0xc` flags is already set; a zero
/// low byte in its answer returns 0, otherwise bit 1 of the flags is set.
/// Finally the notifier (callee 4, thiscall/0, `a0` in ECX) runs and the
/// function returns 1. Callees 1-3 are reached through fabricated objects
/// exactly like the original (planted stub addresses in heap vtables).
///
/// Only AL is meaningful (every return sets just AL), so the contract
/// compares `al`. A null child faults on both sides at the flags read.
///
/// Original: 0x00b70cc0 (thiscall, three stack arguments).
lf_checker_rt::export!(thiscall, rw_00b70cc0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x74;
        const CHILD_OFF: u32 = 0x08;
        const SLOT_TYPE: u32 = 0x0c;
        const SLOT_KIND: u32 = 0x04;
        const SLOT_RUN: u32 = 0x14;
        const FLAGS_OFF: u32 = 0x0c;
        const WANT_TYPE: u32 = 0xca;
        const WANT_KIND_A: u32 = 0x42;
        const WANT_KIND_B: u32 = 0x21;
        if ((this + FLAG_OFF) as *const u8).read() != 0 {
            let c = ((this + CHILD_OFF) as *const u32).read_unaligned();
            if c != 0 {
                let vt = (c as *const u32).read_unaligned();
                let f_type: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                    ((vt + SLOT_TYPE) as *const u32).read_unaligned() as usize);
                if f_type(c) == WANT_TYPE && a2 != 0 {
                    let vt2 = (a2 as *const u32).read_unaligned();
                    let f_kind: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                        ((vt2 + SLOT_KIND) as *const u32).read_unaligned() as usize);
                    if f_kind(a2) == WANT_KIND_A {
                        return 0;
                    }
                    let vt3 = (a2 as *const u32).read_unaligned();
                    let f_kind2: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                        ((vt3 + SLOT_KIND) as *const u32).read_unaligned() as usize);
                    if f_kind2(a2) == WANT_KIND_B {
                        return 0;
                    }
                }
            }
        }
        let c = ((this + CHILD_OFF) as *const u32).read_unaligned();
        if ((c + FLAGS_OFF) as *const u8).read() & 1 == 0 {
            let vt = (c as *const u32).read_unaligned();
            let f_run: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(
                ((vt + SLOT_RUN) as *const u32).read_unaligned() as usize);
            let ans = f_run(c, a0, a1, a2);
            if (ans & 0xff) == 0 {
                return 0;
            }
            let p = (c + FLAGS_OFF) as *mut u32;
            p.write_unaligned(p.read_unaligned() | 2);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, a0);
        1
    }
});

