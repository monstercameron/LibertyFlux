// original: 0x00BBE060 group_assign_task_29_to_leader_and_members (proposed)

/// Assign a freshly built task (slot 0x29) to a group leader and its members.
///
/// `group` is a script handle resolved through the vehicle/group pool
/// (global at `GROUP_POOL`) into an entity. When the start gate (callee 1)
/// answers zero and the entity's leader slot (`+0xF50`) holds a live ped
/// (flag byte `+0x211` clear), a task object is allocated from the task pool
/// (callee 3), initialised by the base task constructor (callee 4) with the
/// vtable `TASK_VTABLE`, a zeroed timer word and unit half/byte fields, and
/// assigned with slot `TASK_SLOT` through the pointer-to-handle lookup
/// (callee 5) and the assign call (callee 6). A failed allocation assigns a
/// null task instead; the calls still happen in the same order.
///
/// The entity's member count (byte at `+0x1070`) then drives a loop over the
/// member slots (`+0xF54`, one dword each): each live member gets the same
/// task, except the timer word is `round_index * 500 - delay + 0x190`, where
/// `delay` truncates `rand16 * RAND_SCALE * DELAY_GAIN` toward zero (`rand16`
/// is the low half of callee 7). The count is reloaded from the entity every
/// iteration and the loop compares signed. The float multiplies run in the
/// original's operand order with the order pinned.
///
/// Original: 0x00BBE060 (cdecl, one stack word, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00BBE060(group: u32) -> u32 {
    unsafe {
        const GROUP_POOL: u32 = 0x012E22A4;
        const TASK_POOL: u32 = 0x0167E2A0;
        const PED_POOL: u32 = 0x018B6F1C;
        const RAND_SCALE: u32 = 0x00FE8680;
        const DELAY_GAIN: u32 = 0x00E90950;
        const LEADER_SLOT: u32 = 0xF50;
        const MEMBER_SLOTS: u32 = 0xF54;
        const MEMBER_COUNT: u32 = 0x1070;
        const PED_DEAD_FLAG: u32 = 0x211;
        const TASK_VTABLE: u32 = 0x00E89044;
        const TASK_TIMER: u32 = 0x14;
        const TASK_UNIT_HALF: u32 = 0x18;
        const TASK_UNIT_BYTE: u32 = 0x1A;
        const TASK_SLOT: u32 = 0x29;
        const ROUND_STRIDE: u32 = 0x1F4;
        const TIMER_BIAS: u32 = 0x190;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn const_f32(file_va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(file_va))) }
        }

        let gate: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        if gate & 0xFF != 0 {
            return 0;
        }
        let group_pool = rd32(lf_checker_rt::relocated(GROUP_POOL));
        let ent: u32 = lf_checker_rt::callee_thiscall!(2, u32, group_pool, group);
        let task_pool = rd32(lf_checker_rt::relocated(TASK_POOL));
        let ped_pool = rd32(lf_checker_rt::relocated(PED_POOL));
        let leader = rd32(ent.wrapping_add(LEADER_SLOT));
        if leader != 0 && rd8(leader.wrapping_add(PED_DEAD_FLAG)) == 0 {
            let alloc: u32 = lf_checker_rt::callee_thiscall!(3, u32, task_pool);
            let mut task = 0u32;
            if alloc != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, alloc);
                wr32(alloc, lf_checker_rt::relocated(TASK_VTABLE));
                wr32(alloc.wrapping_add(TASK_TIMER), 0);
                wr16(alloc.wrapping_add(TASK_UNIT_HALF), 1);
                wr8(alloc.wrapping_add(TASK_UNIT_BYTE), 1);
                task = alloc;
            }
            let leader_again = rd32(ent.wrapping_add(LEADER_SLOT));
            let handle: u32 =
                lf_checker_rt::callee_thiscall!(5, u32, ped_pool, leader_again);
            let _: u32 = lf_checker_rt::callee_cdecl!(6, u32, handle, task, TASK_SLOT);
        }
        if rd8(ent.wrapping_add(MEMBER_COUNT)) == 0 {
            return 0;
        }
        let scale = const_f32(RAND_SCALE);
        let gain = const_f32(DELAY_GAIN);
        let mut index: i32 = 0;
        let mut round: u32 = 0;
        loop {
            let count = rd8(ent.wrapping_add(MEMBER_COUNT)) as i32;
            if !(index < count) {
                break;
            }
            let mem = rd32(ent.wrapping_add(MEMBER_SLOTS).wrapping_add((index as u32).wrapping_mul(4)));
            if mem != 0 && rd8(mem.wrapping_add(PED_DEAD_FLAG)) == 0 {
                let r: u32 = lf_checker_rt::callee_cdecl!(7, u32,);
                let f1 = mul((r & 0xFFFF) as f32, scale);
                let alloc: u32 = lf_checker_rt::callee_thiscall!(3, u32, task_pool);
                let mut task = 0u32;
                if alloc != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, alloc);
                    let f2 = mul(f1, gain);
                    let delay = f2 as i32;
                    wr32(alloc, lf_checker_rt::relocated(TASK_VTABLE));
                    wr16(alloc.wrapping_add(TASK_UNIT_HALF), 1);
                    let timer = round.wrapping_sub(delay as u32).wrapping_add(TIMER_BIAS);
                    wr32(alloc.wrapping_add(TASK_TIMER), timer);
                    wr8(alloc.wrapping_add(TASK_UNIT_BYTE), 1);
                    task = alloc;
                }
                let mem_again = rd32(ent.wrapping_add(MEMBER_SLOTS).wrapping_add((index as u32).wrapping_mul(4)));
                let handle: u32 =
                    lf_checker_rt::callee_thiscall!(5, u32, ped_pool, mem_again);
                let _: u32 = lf_checker_rt::callee_cdecl!(6, u32, handle, task, TASK_SLOT);
            }
            index += 1;
            round = round.wrapping_add(ROUND_STRIDE);
        }
        0
    }
});
