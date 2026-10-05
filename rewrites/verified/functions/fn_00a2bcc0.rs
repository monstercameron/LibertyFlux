// original: 0x00a2bcc0 CPlayerPed::vf73

/// Player-ped update step: run the base step, then settle a wanted-heat
/// budget from the active task list.
///
/// `obj` (ECX) is the player ped. Callee 0 is the base-class step
/// (`CPed::vf73`). When the task driver byte at `+0x219` is clear the
/// function returns that answer untouched.
///
/// Otherwise the task chain hanging off `obj[0x224]+0x2e0` (links at
/// `+0x0c`) is walked twice: first for a node tagged `0x410`, then for one
/// tagged `0x41e`. Each walk compares the previous node's priority,
/// `([+0x08] >> 1) & 7`, against the current node's; when the previous is
/// strictly below the current and at least 2, the walk aborts (the first
/// walk falls through to the second, the second returns). Without either
/// tag the function returns the priority of the node it stopped on; a
/// missing chain returns the record pointer itself. Callee 1 then resolves
/// the ped's current item, and a null answer returns 0.
///
/// Callee 2 then classifies the task record (`obj[0x224]`) into an output
/// word and a flag byte: an output of 5 or 8 sets the flag, and 8 also sets
/// a hold marker. When task `0x413` is present (callee 3), the budget at
/// `obj+0xea0` is reduced by the frame cost (the game delta time from file
/// `0x11735bc` scaled by the constant at file `0xfe8c58`, truncated toward
/// zero exactly as the original's x87 `fistp`) and clamped at zero; when the
/// task is absent and the hold marker is clear, callee 4's award is added
/// instead and clamped at 10000. Finally the item's heat bytes (`+0x26dc`,
/// `+0x26de`, xored) are accepted when the flag is set and either the hold
/// marker is set or the heat lies strictly between `0x64` and the limit byte
/// from file `0x103ce48`. The old budget is always copied to `obj+0xea4`
/// and returned; a rejected heat also zeroes the budget.
///
/// The truncation assumes a finite non-negative frame cost, which holds for
/// game delta times; NaN or out-of-range costs would truncate differently
/// from the x87 path. The original reads its game data through relocated
/// absolute addresses; the rewrite reads the same addresses.
///
/// Original: 0x00a2bcc0 head plus its tail at 0xa2c320 (thiscall, no stack
/// words). The batch inventory counted only the head's one callee; the tail
/// adds four more.
lf_checker_rt::export!(thiscall, rw_00a2bcc0(obj: u32) -> u32 {
    unsafe {
        const DRIVER: u32 = 0x219;
        const TASKS: u32 = 0x224;
        const LIST: u32 = 0x2e0;
        const NODE_TAG: u32 = 4;
        const NODE_FLAGS: u32 = 8;
        const NODE_NEXT: u32 = 0x0c;
        const TAG_A: u32 = 0x410;
        const TAG_B: u32 = 0x41e;
        const TASK_BUDGET: u32 = 0x413;
        const OUT_5: u32 = 5;
        const OUT_8: u32 = 8;
        const BUDGET: u32 = 0xea0;
        const BUDGET_MAX: u32 = 10000;
        const HEAT_LO: u8 = 0x64;
        const HEAT_A: u32 = 0x26dc;
        const HEAT_B: u32 = 0x26de;
        const G_DT: u32 = 0x11735bc;
        const K_COST: u32 = 0xfe8c58;
        const G_HEAT_MAX: u32 = 0x103ce48;

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
        unsafe fn g32(ph: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(ph) as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn gf(ph: u32) -> f32 {
            unsafe { f32::from_bits(g32(ph)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// One tag walk over the chain. The loop keeps the previous node's
        /// priority in what is architecturally ecx (the back edge skips its
        /// reload) and aborts when the previous priority is strictly below
        /// the current one while at least 2. Returns whether the tag hit
        /// and the priority of the node it stopped on (what the original
        /// leaves in the return slot when the tag misses).
        #[inline(always)]
        unsafe fn walk(mut node: u32, tag: u32) -> (bool, u32) {
            unsafe {
                let mut prev: Option<u32> = None;
                let mut stopped = 0;
                while node != 0 {
                    let prio = (rd32(node.wrapping_add(NODE_FLAGS)) >> 1) & 7;
                    let ecx = prev.unwrap_or(prio);
                    if ecx < prio && ecx >= 2 {
                        return (false, prio);
                    }
                    if rd32(node.wrapping_add(NODE_TAG)) == tag {
                        return (true, prio);
                    }
                    prev = Some(prio);
                    stopped = prio;
                    node = rd32(node.wrapping_add(NODE_NEXT));
                }
                (false, stopped)
            }
        }

        let base = lf_checker_rt::callee_thiscall!(0, u32, obj);
        if rd8(obj.wrapping_add(DRIVER)) == 0 {
            return base;
        }
        let trec = rd32(obj.wrapping_add(TASKS));
        let head = rd32(trec.wrapping_add(LIST));
        if head == 0 {
            return trec;
        }
        let (found_a, _) = walk(head, TAG_A);
        if !found_a {
            let (found_b, prio) = walk(head, TAG_B);
            if !found_b {
                return prio;
            }
        }
        let item = lf_checker_rt::callee_thiscall!(1, u32, obj);
        if item == 0 {
            return 0;
        }
        let mut out: u32 = 0;
        let mut tag: u8 = 0;
        let mut flag: u8 = 1;
        let mut hold: u8 = 0;
        if lf_checker_rt::callee_thiscall!(
            2,
            u32,
            trec,
            &mut out as *mut u32 as u32,
            &mut tag as *mut u8 as u32
        ) as u8
            != 0
        {
            flag = 0;
            if out == OUT_5 || out == OUT_8 {
                flag = 1;
                if out == OUT_8 {
                    hold = 1;
                }
            }
        }
        let _ = tag;
        if lf_checker_rt::callee_thiscall!(3, u32, trec.wrapping_add(LIST), TASK_BUDGET, 0) as u8 != 0 {
            let cost = mul(gf(G_DT), gf(K_COST)).trunc() as i64 as i32 as u32;
            let next = rd32(obj.wrapping_add(BUDGET)).wrapping_sub(cost);
            wr32(obj.wrapping_add(BUDGET), if (next as i32) >= 0 { next } else { 0 });
        } else if hold == 0 {
            let award = lf_checker_rt::callee_cdecl!(4, u32,);
            let mut next = rd32(obj.wrapping_add(BUDGET)).wrapping_add(award);
            if next > BUDGET_MAX {
                next = BUDGET_MAX;
            }
            wr32(obj.wrapping_add(BUDGET), next);
        }
        let heat = rd8(item.wrapping_add(HEAT_B)) ^ rd8(item.wrapping_add(HEAT_A));
        let ok = if flag == 0 {
            false
        } else if hold != 0 {
            true
        } else if heat <= HEAT_LO {
            false
        } else {
            let max = (lf_checker_rt::global::<u8>(G_HEAT_MAX) as *const u8).read();
            heat < max
        };
        finish(obj, ok)
    }
});

/// Copy the old budget to the previous slot, zeroing it on rejection, and
/// return the old budget.
#[inline(always)]
unsafe fn finish(obj: u32, ok: bool) -> u32 {
    unsafe {
        let old = (obj.wrapping_add(0xea0) as *const u32).read_unaligned();
        if !ok {
            (obj.wrapping_add(0xea0) as *mut u32).write_unaligned(0);
        }
        (obj.wrapping_add(0xea4) as *mut u32).write_unaligned(old);
        old
    }
}
