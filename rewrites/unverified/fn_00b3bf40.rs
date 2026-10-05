// original: 0x00B3BF40 task_guard_enqueue_check (proposed)

/// Guard a task-list insertion for one object, then validate its sub-object.
///
/// `obj` points at the object's header. The function first checks three flag
/// fields (gate word at +0x28 masked with 0x3C0, bytes at +0x10B8/+0xF1E/
/// +0xF1D); when they select the object, a global queue slot is appended
/// (count at COUNT, slots of two words at QUEUE) holding `(0, obj)` and the
/// function returns 0 once the count reaches LIMIT. Otherwise the sub-object
/// pointer at +0x20 is created on demand through callees 1 and 2, its float
/// at +0x28 is compared against THRESH (unordered counts as not-above, so
/// NaN continues), and a sign flag at +0x24 selects an early return of 1.
/// Finally a signed index at +0x2E addresses a global pointer table; a null
/// entry returns 1, otherwise callee 3 yields an iteration bound, callee 4
/// yields one item per index, and each item's vtable slot at +4 is invoked:
/// an answer of 0x11 appends `(item, obj)` to the same queue (returning 0 at
/// the limit). Returns 1 on every other path. Cdecl, one stack word; the full
/// EAX value (including upper leftover bits) is part of the result.
lf_checker_rt::export!(cdecl, rw_00B3BF40(obj: u32) -> u32 {
    unsafe {
        const GATE_OFF: u32 = 0x28;
        const GATE_MASK: u32 = 0x3C0;
        const GATE_WANT: u32 = 0x80;
        const COUNT: u32 = 0x0166_24B0;
        const QUEUE: u32 = 0x0166_2668;
        const LIMIT: u32 = 0x0104_59B4;
        const THRESH: f32 = f32::from_bits(0x3F66_6666);
        const TABLE: u32 = 0x0129_5CD8;
        const CAL_CREATE: u32 = 1;
        const CAL_ATTACH: u32 = 2;
        const CAL_BOUND: u32 = 3;
        const CAL_ITEM: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn setg32(va: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(va).write_unaligned(v) }
        }

        // Path 1: flag gate, then queue append.
        if rd32(obj.wrapping_add(GATE_OFF)) & GATE_MASK == GATE_WANT {
            let by_flag = if rd8(obj.wrapping_add(0x10B8)) == 3 {
                true
            } else {
                rd8(obj.wrapping_add(0xF1E)) & 4 != 0
            };
            if by_flag && rd8(obj.wrapping_add(0xF1D)) & 0x80 == 0 {
                let c = g32(COUNT).wrapping_add(1);
                setg32(QUEUE.wrapping_add(c.wrapping_mul(8)), 0);
                setg32(QUEUE.wrapping_add(c.wrapping_mul(8)).wrapping_add(4), obj);
                setg32(COUNT, c);
                if !((c as i32) < (g32(LIMIT) as i32)) {
                    return c & 0xFFFF_FF00;
                }
            }
        }
        // Path 2: create the sub-object on demand.
        if rd32(obj.wrapping_add(0x20)) == 0 {
            lf_checker_rt::callee_thiscall!(CAL_CREATE, u32, obj);
            let slot = rd32(obj.wrapping_add(0x20));
            lf_checker_rt::callee_thiscall!(CAL_ATTACH, u32, obj.wrapping_add(0x10), slot);
        }
        // Path 3: threshold check (NaN continues, like comiss+jbe).
        let sub = rd32(obj.wrapping_add(0x20));
        if THRESH > rdf(sub.wrapping_add(0x28)) {
            return (sub & 0xFFFF_FF00) | 1;
        }
        // Path 4: sign-flag early return.
        if rd32(obj.wrapping_add(0x24)) & 0x8000_0000 == 0 {
            return (sub & 0xFFFF_FF00) | 1;
        }
        // Path 5: table lookup by signed index.
        let idx = rd16(obj.wrapping_add(0x2E)) as i16 as i32;
        let tab = g32(TABLE.wrapping_add((idx.wrapping_mul(4)) as u32));
        let mut last = idx as u32;
        if tab == 0 {
            return (last & 0xFFFF_FF00) | 1;
        }
        // Path 6: bounded item scan with vtable probe.
        let n0: u32 = lf_checker_rt::callee_thiscall!(CAL_BOUND, u32, tab);
        last = n0;
        if (n0 as i32) <= 0 {
            return (last & 0xFFFF_FF00) | 1;
        }
        let mut i = 0u32;
        loop {
            let item: u32 = lf_checker_rt::callee_thiscall!(CAL_ITEM, u32, tab, i);
            let slot: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(item).wrapping_add(4)) as usize);
            if slot(item) as u8 == 0x11 {
                let c = g32(COUNT).wrapping_add(1);
                setg32(QUEUE.wrapping_add(c.wrapping_mul(8)), item);
                setg32(QUEUE.wrapping_add(c.wrapping_mul(8)).wrapping_add(4), obj);
                setg32(COUNT, c);
                if (c as i32) >= (g32(LIMIT) as i32) {
                    return c & 0xFFFF_FF00;
                }
            }
            i = i.wrapping_add(1);
            let k: u32 = lf_checker_rt::callee_thiscall!(CAL_BOUND, u32, tab);
            last = k;
            if !((i as i32) < (k as i32)) {
                break;
            }
        }
        (last & 0xFFFF_FF00) | 1
    }
});
