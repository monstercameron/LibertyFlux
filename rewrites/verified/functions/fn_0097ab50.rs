// original: 0x0097ab50 audio_listener_acquire
/// Acquire a listener slot for an incoming audio request.
///
/// A class-6 request reuses the active class-6 slot when one exists.
/// Otherwise the free slot is used when there is one; with no free slot
/// the lowest-class active slot below the request's class is released
/// and reused. Returns the chosen slot, or null when none qualifies.
export!(cdecl, rw_0097ab50(arg: *const u8) -> u32 {
    unsafe {
        let table = global::<u32>(0x12312D4);
        let class = *arg.add(0xc);
        if class == 6 {
            for i in 0..3usize {
                let e = *table.add(i);
                if e != 0
                    && *((e as *const u8).add(0x28)) == 1
                    && *((e as *const u8).add(0x1c)) == 6
                {
                    return e;
                }
            }
        }
        let free = callee_cdecl!(1, u32,);
        let mut slot = free;
        if free == 0 {
            let mut best = 9u32;
            let mut besti = 0usize;
            let mut i = 0usize;
            while i < 3 {
                let e = *table.add(i);
                if e != 0 && *((e as *const u8).add(0x28)) == 1 {
                    let c = *((e as *const u8).add(0x1c)) as u32;
                    if c < best {
                        best = c;
                        besti = i;
                    }
                }
                if best <= 1 {
                    break;
                }
                i += 1;
            }
            let e = *table.add(besti);
            if e == 0 || *((e as *const u8).add(0x1c)) >= class {
                return slot;
            }
            callee_thiscall!(2, u32, e);
            slot = *table.add(besti);
            if slot == 0 {
                return 0;
            }
        }
        callee_thiscall!(3, u32, slot, arg as u32);
        slot
    }
});
