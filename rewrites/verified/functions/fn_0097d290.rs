// original: 0x0097d290 MOBILE_TWO_WAY_GARBLED
//! Gated audio-request helper (thiscall/0, no stack args, plain ret).
//!
//! Behaviour: three global gates must all pass (run flag set, generation
//! counter equal, mode word not 0x12) and the object's request slot at
//! this+0x134 must be empty. It then builds a 9-word work struct (filled by
//! callee 1, patched with two object fields), asks callee 2 to allocate the
//! request (which fills the slot), and, when a request was produced, issues
//! it through callees 3-5 and records the answers at offsets 0xA4/0xA8/0xAC
//! of the request before notifying callee 6.
//!
//! Returns void: exit EAX is the last callee answer or entry residue, so the
//! contract compares no return channel; behaviour is verified through the
//! heap writes, the global gates and the outgoing calls.
export!(thiscall, rw_0097d290(this: u32) -> u32 {
    unsafe {
        if *global::<u32>(0x011F7060) == 1 {
            return 0;
        }
        if *global::<u32>(0x012088B4) != *global::<u32>(0x00F1C040) {
            return 0;
        }
        if *global::<u32>(0x01037720) == 0x12 {
            return 0;
        }
        let slot = (this + 0x134) as *mut u32;
        if *slot != 0 {
            return 0;
        }
        // Work struct: callee 1 fills words 0..4 (ECX struct init); the
        // caller then patches word 3 (base+0x780 link) and word 8 (flags).
        let mut work = [0u32; 9];
        callee_thiscall!(1, u32, work.as_mut_ptr() as u32);
        let base120 = *((this + 0x120) as *const u32);
        work[3] = base120.wrapping_add(0x780);
        work[8] = *((this + 8) as *const u32);
        // Callee 2 allocates the request and stores it through `slot`.
        callee_thiscall!(2, u32, this, relocated(0x00E8CBE4), slot as u32,
            work.as_mut_ptr() as u32, 0xFFFFFFFF, 0, 0);
        if *slot == 0 {
            return 0;
        }
        // Fixed 3-word issue descriptor.
        let args = [0u32, 0xFFFFFFFF, 0x4D];
        let r = callee_cdecl!(3, u32, relocated(0x00E8CC04), 0, 0, 1, 1,
            work.as_mut_ptr() as u32, args.as_ptr() as u32, base120, 0xFFFFFFFF);
        let s = callee_cdecl!(4, u32, r);
        let t = callee_cdecl!(5, u32, s);
        let obj = *slot as *mut u8;
        *(obj.add(0xA4) as *mut u32) = s;
        *(obj.add(0xA8) as *mut u32) = t;
        *(obj.add(0xAC) as *mut u32) = 0;
        callee_thiscall!(6, u32, *slot, 0, 0, 0);
        0
    }
});
