// original: 0x00be43f0 CTaskComplexWaitForBus::vf18 (symbols)

/// Maintain this task's bus wait: mark, count down, or rebuild the waiter.
///
/// Asks the waiter's (`this + WAITER_OFF`, 0x8) kind through its third
/// virtual. An `IDLE_KIND` (0x2de) waiter marks the incoming argument (two
/// flag bits at `+ FLAG_A`, 0x260, and `+ FLAG_B`, 0x24) and then follows the
/// countdown chain (`this + COUNT_OFF`, 0x14, then `+ CHAIN_OFF`, 0xf50):
/// a missing link ends here; a live chain whose ready byte
/// (`+ READY_OFF`, 0x219) is set and whose counter (`+ COUNTER_OFF`, 0x228)
/// is present gains five (`+ COUNT_AT`, 0x480), while a missing counter
/// faults exactly as the original does (a null-page write the harness
/// compares for parity).
///
/// A `GONE_KIND` (0xee) waiter first refreshes the countdown link from the
/// waiter's own (`+ COUNT_OFF`), then takes a pool slot and constructs the
/// replacement waiter there with (link, -4, 27, 0, 0), returning it, or zero
/// when the pool is empty. Any other kind returns zero at once.
///
/// Original: 0x00be43f0 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be43f0(this: u32, arg: u32) -> u32 {
    unsafe {
        const WAITER_OFF: u32 = 0x08;
        const KIND_SLOT: u32 = 0x0c;
        const IDLE_KIND: u32 = 0x2de;
        const GONE_KIND: u32 = 0xee;
        const FLAG_A: u32 = 0x260;
        const FLAG_A_BIT: u32 = 0x10000000;
        const FLAG_B: u32 = 0x24;
        const FLAG_B_BIT: u32 = 0x400;
        const COUNT_OFF: u32 = 0x14;
        const CHAIN_OFF: u32 = 0xf50;
        const READY_OFF: u32 = 0x219;
        const COUNTER_OFF: u32 = 0x228;
        const COUNT_AT: u32 = 0x480;
        const COUNT_STEP: u32 = 5;
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const REBUILD_D: u32 = 27;
        const ALLOC: u32 = 2;
        const REBUILD: u32 = 3;
        // The kind query runs through the waiter's own vtable (stub id 1 in
        // the contract), exactly like the original: no ctable use here.
        let waiter = (this.wrapping_add(WAITER_OFF) as *const u32).read_unaligned();
        let vtable = (waiter as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(KIND_SLOT) as *const u32).read_unaligned();
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let kind = kind_of(waiter);
        if kind == IDLE_KIND {
            let a = (arg.wrapping_add(FLAG_A) as *const u32).read_unaligned();
            (arg.wrapping_add(FLAG_A) as *mut u32).write_unaligned(a | FLAG_A_BIT);
            let b = (arg.wrapping_add(FLAG_B) as *const u32).read_unaligned();
            (arg.wrapping_add(FLAG_B) as *mut u32).write_unaligned(b | FLAG_B_BIT);
            let link = (this.wrapping_add(COUNT_OFF) as *const u32).read_unaligned();
            let chain = (link.wrapping_add(CHAIN_OFF) as *const u32).read_unaligned();
            if chain == 0 {
                return 0;
            }
            let ready = (chain.wrapping_add(READY_OFF) as *const u8).read();
            if ready == 0 {
                return 0;
            }
            let counter = (chain.wrapping_add(COUNTER_OFF) as *const u32).read_unaligned();
            if counter == 0 {
                // The original jumps to a null-page write on this path
                // rather than adding: mirror its fault address exactly so
                // both sides fault alike (compared for parity).
                const NULL_PAGE_AT: u32 = 0x410;
                let at = (NULL_PAGE_AT as *const u32).read_unaligned();
                (NULL_PAGE_AT as *mut u32).write_unaligned(at.wrapping_add(COUNT_STEP));
                return 0;
            }
            let at = (counter.wrapping_add(COUNT_AT) as *const u32).read_unaligned();
            (counter.wrapping_add(COUNT_AT) as *mut u32)
                .write_unaligned(at.wrapping_add(COUNT_STEP));
            return 0;
        }
        if kind != GONE_KIND {
            return 0;
        }
        let waiter_link =
            (waiter.wrapping_add(COUNT_OFF) as *const u32).read_unaligned();
        (this.wrapping_add(COUNT_OFF) as *mut u32).write_unaligned(waiter_link);
        let pool = (lf_checker_rt::global::<u32>(POOL_GLOBAL) as *const u32).read_unaligned();
        let got = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if got == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(REBUILD, u32, got, waiter_link, 0xfffffffc, REBUILD_D, 0, 0)
    }
});
