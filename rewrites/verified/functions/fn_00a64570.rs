// original: 0x00a64570 intel_gated_poll
/// Polls the gated five-slot table, then runs the context tail check.
///
/// Unless the context's gate object (+0x40, +0x6C) exists with a set flag
/// byte at +0xE, takes the first live entry of the table at +0x44, walks
/// its +0x8 link chain to the tail, probes the tail through handler slot 2
/// and, when the probe reports true, notifies it through handler slot 19
/// with the context. Then, when the context flag byte at +0x26C has bit 2
/// set with a live tail object at +0xB30 whose state dword at +0x1300 reads
/// 1, reports the context through the tail callee. Returns nothing.
export!(thiscall, rw_00a64570(this: u32) -> u32 {
    unsafe {
        let ctx = *((this + 0x40) as *const u32);
        let gate = *((ctx + 0x6C) as *const u32);
        let gated = gate != 0 && *((gate + 0xE) as *const u8) != 0;
        if !gated {
            let mut obj = 0u32;
            let mut i = 0u32;
            while i < 5 {
                let cand = *((this + 0x44 + i.wrapping_mul(4)) as *const u32);
                if cand != 0 {
                    obj = cand;
                    break;
                }
                i += 1;
            }
            if obj != 0 {
                let mut tail = obj;
                loop {
                    let next = *((tail + 8) as *const u32);
                    if next == 0 {
                        break;
                    }
                    tail = next;
                }
                type Probe = extern "thiscall" fn(u32) -> u32;
                type Notify = extern "thiscall" fn(u32, u32) -> u32;
                let vtable = *(tail as *const u32);
                let probe: Probe =
                    core::mem::transmute(*((vtable + 8) as *const u32) as usize);
                if (probe(tail) as u8) != 0 {
                    let notify: Notify =
                        core::mem::transmute(*((vtable + 0x4C) as *const u32) as usize);
                    notify(tail, ctx);
                }
            }
        }
        let ctx = *((this + 0x40) as *const u32);
        if *((ctx + 0x26C) as *const u8) & 4 == 0 {
            return 0;
        }
        let tail_obj = *((ctx + 0xB30) as *const u32);
        if tail_obj == 0 {
            return 0;
        }
        if *((tail_obj + 0x1300) as *const u32) != 1 {
            return 0;
        }
        callee_thiscall!(3, u32, tail_obj, ctx);
    }
    0
});
