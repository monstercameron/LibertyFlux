// original: 0x00DA6080 CTaskComplexShockingEventFlee::vf18

/// Reaction update: unless every gate passes, fall back to the dispatch
/// helper and report 0; otherwise allocate and build the reaction object,
/// set its mark bit, and clear the wide-range bit when the flag word at
/// `+0x20` is below 0x1b.
///
/// The gates in order: virtual slot 0x0c of the subtask at `+0x08` must
/// answer 0x16e; the entity at `+0x48` must be set with kind bits
/// (`+0x28` masked by 0x3c0) equal to 0xc0; the probe callee, called with
/// the entity and the word at caller-arg `+0x224` as object, must refuse.
/// Allocation failure stores through null and faults, exactly like the
/// original. The build passes (entity, 0). Original: thiscall, one stack
/// word, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da6080(this: u32, arg: u32) -> u32 {
    unsafe {
        const TYPE_CHECK: u32 = 1;
        const PROBE: u32 = 2;
        const ALLOC: u32 = 3;
        const BUILD: u32 = 4;
        const DISPATCH: u32 = 5;
        const MEMMGR_SLOT: u32 = 0x0167E2A0;
        const WANT_TYPE: u32 = 0x16E;
        const KIND_MASK: u32 = 0x3C0;
        const WANT_KIND: u32 = 0xC0;
        const MARK_BIT: u32 = 8;
        const WIDE_MASK: u32 = 0xFFFB_FFFF;
        const FLAG_LIMIT: u32 = 0x1B;

        let fallback = |this: u32, arg: u32| -> u32 {
            lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, arg);
            0
        };
        let sub = ((this + 0x08) as *const u32).read();
        let table = (sub as *const u32).read();
        let slot = ((table + 0x0C) as *const u32).read();
        let type_check: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if type_check(sub) != WANT_TYPE {
            return fallback(this, arg);
        }
        let entity = ((this + 0x48) as *const u32).read();
        if entity == 0 {
            return fallback(this, arg);
        }
        if ((entity + 0x28) as *const u32).read() & KIND_MASK != WANT_KIND {
            return fallback(this, arg);
        }
        let probe_obj = ((arg + 0x224) as *const u32).read();
        let refused: u32 = lf_checker_rt::callee_thiscall!(PROBE, u32, probe_obj, entity);
        if (refused & 0xFF) != 0 {
            return fallback(this, arg);
        }
        let manager = (lf_checker_rt::relocated(MEMMGR_SLOT) as *const u32).read();
        let fresh: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, manager);
        if fresh == 0 {
            // Null allocation: the mark store faults, like the original.
            ((0x60) as *mut u32).write(
                ((0x60) as *const u32).read() | MARK_BIT);
            return 0;
        }
        let built: u32 = lf_checker_rt::callee_thiscall!(BUILD, u32, fresh, entity, 0);
        let marks = (built + 0x60) as *mut u32;
        marks.write(marks.read() | MARK_BIT);
        if ((this + 0x20) as *const u32).read() < FLAG_LIMIT {
            marks.write(marks.read() & WIDE_MASK);
        }
        built
    }
});
