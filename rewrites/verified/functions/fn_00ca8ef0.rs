// original: 0x00ca8ef0 CEventHandler::vf72
/// Pick a response to a shocking event: allocate a task slot from the task
/// pool and construct one of four specialised response tasks into it.
///
/// `handler` points to the event handler (`+0x04` holds its ped, `+0x0c`
/// receives the new task or zero). `event` points to the event record:
/// `+0x0c` is the response selector (1 Watch, 2 Goto, 3 HurryAway, 4 Flee),
/// `+0x36` a flag byte, `+0x10..0x1c` a three-float vector, `+0x38` a status
/// word. The second and third stack arguments are not read.
///
/// Guards, in order: a ped suppress flag (byte at ped `+0x211`) returns the
/// ped pointer with nothing stored; a set flag byte with a clear status word,
/// or a clear flag byte with a set status word, returns the ped pointer with
/// its low byte replaced by the flag (nothing stored); a clear flag byte with
/// a clear status word additionally requires the vector's squared length to
/// exceed 0.05 (compared as `jbe`, so NaN fails).
///
/// On dispatch the pool pointer is read from its global, one slot is
/// allocated (a null slot stores zero and returns zero), and the selected
/// task constructor runs with the slot as `this` and `event+0x10` as its
/// argument; its result is stored at `handler+0x0c` and returned. A
/// selector outside 1..=4 returns the decremented selector itself: the
/// dispatch loads it into the return register before the range check.
///
/// Original: 0x00ca8ef0 (thiscall, three stack words; the second and third
/// are not read).
lf_checker_rt::export!(thiscall, rw_00ca8ef0(handler: u32, event: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        const HANDLER_PED: u32 = 0x04;
        const HANDLER_TASK: u32 = 0x0c;
        const PED_SUPPRESS: u32 = 0x211;
        const EVT_SELECTOR: u32 = 0x0c;
        const EVT_PARAMS: u32 = 0x10;
        const EVT_VEC_X: u32 = 0x20;
        const EVT_VEC_Y: u32 = 0x24;
        const EVT_VEC_Z: u32 = 0x28;
        const EVT_FLAG: u32 = 0x36;
        const EVT_STATUS: u32 = 0x38;
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const LEN2_LIMIT: u32 = 0x00fe876c;
        const ALLOC: u32 = 1;
        const CTOR_WATCH: u32 = 2;
        const CTOR_GOTO: u32 = 3;
        const CTOR_HURRY: u32 = 4;
        const CTOR_FLEE: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let ped = rd32(handler + HANDLER_PED);
        if rd8(ped + PED_SUPPRESS) != 0 {
            return ped;
        }
        let flag = rd8(event + EVT_FLAG);
        let status = rd32(event + EVT_STATUS);
        // Paths that return without storing leave eax holding the ped
        // pointer with its low byte replaced by the flag (the `(an instruction of the original)`).
        let plain = (ped & 0xffff_ff00) | flag as u32;
        if flag != 0 {
            if status == 0 {
                return plain;
            }
        } else {
            if status != 0 {
                return plain;
            }
            let x = rdf(event + EVT_VEC_X);
            let y = rdf(event + EVT_VEC_Y);
            let z = rdf(event + EVT_VEC_Z);
            let len2 = add(add(mul(x, x), mul(y, y)), mul(z, z));
            let limit = f32::from_bits(rd32(lf_checker_rt::relocated(LEN2_LIMIT)));
            // `jbe` after `comiss`: dispatch only when strictly above.
            if !(core::hint::black_box(len2) > core::hint::black_box(limit)) {
                return plain;
            }
        }
        // `movzx` then `dec`: the selector check returns this value, not
        // the flag-filled one, when it falls through to the shared exit.
        let sel = (rd8(event + EVT_SELECTOR) as u32).wrapping_sub(1);
        if sel > 3 {
            return sel;
        }
        let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
        let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if slot == 0 {
            ((handler + HANDLER_TASK) as *mut u32).write_unaligned(0);
            return 0;
        }
        let params = event + EVT_PARAMS;
        let task: u32 = match sel {
            0 => lf_checker_rt::callee_thiscall!(CTOR_WATCH, u32, slot, params),
            1 => lf_checker_rt::callee_thiscall!(CTOR_GOTO, u32, slot, params),
            2 => lf_checker_rt::callee_thiscall!(CTOR_HURRY, u32, slot, params),
            _ => lf_checker_rt::callee_thiscall!(CTOR_FLEE, u32, slot, params),
        };
        ((handler + HANDLER_TASK) as *mut u32).write_unaligned(task);
        task
    }
});
