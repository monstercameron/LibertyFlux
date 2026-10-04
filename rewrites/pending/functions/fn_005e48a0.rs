// original: 0x005e48a0 slot_effect_dispatch
// rw_005e48a0: fire a timed slot's effect or its fallback.
//
// Each slot has a live flag; a live slot replays its stored effect through
// the effect callback, caching a nonzero slot handle first. A dead slot runs
// the lookup step instead and, when the lookup yields a ready record, hands
// it to the bind callback. Returns the callback answer.
export!(thiscall, rw_005e48a0(this: u32, arg: u32) -> u32 {
    unsafe {
        let obj = this as *const u8;
        let at = |off: u32| obj.add(arg.wrapping_mul(4) as usize + off as usize);
        let flag = *(at(0xB8) as *const u32);
        if flag != 0 {
            let slot = *(at(0x30) as *const u32);
            if slot != 0 {
                *((this as *mut u8).add(0xB4) as *mut u32) = slot;
            }
            let amount = *(at(0x1B8) as *const u32);
            let param = *(at(0x138) as *const u32);
            let cached = *((this as *const u8).add(0xB4) as *const u32);
            let ctx = *((this as *const u8).add(0x20) as *const u32);
            callee_thiscall!(1, u32, ctx, cached, flag, param, amount)
        } else {
            let slot = *(at(0x30) as *const u32);
            let found: u32 = callee_cdecl!(
                3,
                u32,
                slot,
                0,
                relocated(0x114E780),
                relocated(0x114E798),
                0
            );
            if *((found.wrapping_add(0xD8)) as *const u32) == 0x15 {
                let ctx = *((this as *const u8).add(0x20) as *const u32);
                callee_thiscall!(2, u32, ctx, found)
            } else {
                found
            }
        }
    }
});
