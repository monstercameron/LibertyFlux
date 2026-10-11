// original: 0x00a09e20 radar_blip_sweep_active (proposed)
/// Sweep the 256 radar-blip slots and re-arm the live ones.
///
/// Asks the cleanup gate first; when it says no, nothing happens. Otherwise
/// each slot whose tag byte is nonzero and whose flag at +0xc is 1 is
/// handled by tag (1, 2 and 4 resolve their handle through pools A, B and
/// C and validate it; 7 runs the pair check and skips the slot when it
/// answers negative; 3, 5 and 6 go straight on) and every handled slot is
/// re-armed with (handle, tag, 0, 1). Returns the last answer seen.
/// Thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00a09e20(this: u32) -> u32 {
    unsafe {
        const POOL_A: u32 = 0x012e22a4;
        const POOL_B: u32 = 0x018b6f1c;
        const POOL_C: u32 = 0x01632c60;
        const GATE: u32 = 0;
        const LOOKUP: u32 = 1;
        const VAL_A: u32 = 2;
        const VAL_B: u32 = 3;
        const VAL_C: u32 = 4;
        const PAIR: u32 = 5;
        const COMMIT: u32 = 6;
        const REARM: u32 = 7;
        const COUNT: u32 = 0x100;
        const STRIDE: u32 = 0x2c;
        let gate: u32 = lf_checker_rt::callee_cdecl!(GATE, u32,);
        if (gate & 0xff) == 0 {
            return gate;
        }
        let pool_a = (lf_checker_rt::global::<u32>(POOL_A) as *const u32).read_unaligned();
        let pool_b = (lf_checker_rt::global::<u32>(POOL_B) as *const u32).read_unaligned();
        let pool_c = (lf_checker_rt::global::<u32>(POOL_C) as *const u32).read_unaligned();
        let mut last: u32 = gate;
        let mut slot = this + 4;
        let mut i = 0u32;
        while i < COUNT {
            let tag = (slot as *const u8).read();
            let flag = ((slot + 0x0c) as *const u32).read_unaligned();
            if tag != 0 && flag == 1 {
                let handle = ((slot + 4) as *const u32).read_unaligned();
                let mut go = true;
                match tag {
                    1 | 2 | 4 => {
                        let (pool, val) = match tag {
                            1 => (pool_a, VAL_A),
                            2 => (pool_b, VAL_B),
                            _ => (pool_c, VAL_C),
                        };
                        let obj = lf_checker_rt::callee_thiscall!(LOOKUP, u32, pool, handle);
                        if obj != 0 {
                            last = lf_checker_rt::callee_cdecl!(val, u32, obj);
                        }
                    }
                    7 => {
                        let r: u32 = lf_checker_rt::callee_cdecl!(PAIR, u32, handle, 8u32);
                        last = r;
                        if (r as i32) < 0 {
                            go = false;
                        } else {
                            last = lf_checker_rt::callee_cdecl!(COMMIT, u32, r, 1u32);
                        }
                    }
                    _ => {}
                }
                if go {
                    last = lf_checker_rt::callee_thiscall!(
                        REARM, u32, this, handle, tag as u32, 0u32, 1u32);
                }
            }
            i += 1;
            slot += STRIDE;
        }
        last
    }
});
