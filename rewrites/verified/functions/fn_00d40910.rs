// original: 0x00d40910 jump_window_trace_accept (proposed)

/// Run the timed window check for a jump task and accept its result.
///
/// `this` is the task, `owner` (arg0) the ped. The window byte at
/// `WINDOW` (+0x50) being clear ends at once. When the rearm byte at
/// `REARM` (+0x51) is set, the start word at `START` (+0x48) is reloaded
/// from the tick global at `TICK` and `REARM` cleared. The window closes
/// unless `SPAN` (+0x4c) plus `START` (wrapping) is at most `TICK`
/// (signed). An open window calls the owner's gather slot (virtual slot
/// +0xec, thiscall on `owner` with a frame struct pointer); the answer
/// points to a record whose float at +8 must be at most -20.0 (ordered,
/// so NaN closes the window). Then the accept callee runs (thiscall on
/// `owner`+0x570 with the same frame struct: word 0 is 20.0f as bits,
/// word 1 is (owner byte at +0x210 shifted to bits 16-23) or 2, the byte
/// written earlier at offset 6 being part of that word), and `WINDOW` is
/// cleared.
///
/// Both calls take a pointer into the caller's own frame; the contract
/// skips the addresses and snapshots the struct words, with a zero stack
/// fill so the one never-written byte compares deterministically.
///
/// Original: 0x00d40910 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d40910(this: u32, owner: u32) -> u32 {
    unsafe {
        const WINDOW: u32 = 0x50;
        const REARM: u32 = 0x51;
        const START: u32 = 0x48;
        const SPAN: u32 = 0x4c;
        const TICK: u32 = 0x0117_35b4;
        const GATHER: u32 = 1;
        const GATHER_SLOT: u32 = 0xec;
        const ACCEPT: u32 = 2;
        const C_FLOOR: u32 = 0x00e7_e870;
        const ACCEPT_ARG: u32 = 0x41a0_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        if ((this + WINDOW) as *const u8).read() == 0 {
            return 0;
        }
        if ((this + REARM) as *const u8).read() != 0 {
            let tick = lf_checker_rt::global::<u32>(TICK).read_unaligned();
            ((this + START) as *mut u32).write_unaligned(tick);
            ((this + REARM) as *mut u8).write(0);
        }
        let span = rd32(this + SPAN);
        let start = rd32(this + START);
        let tick = lf_checker_rt::global::<u32>(TICK).read_unaligned();
        if (span.wrapping_add(start) as i32) > tick as i32 {
            return 0;
        }
        let mut frame = [0u32, 0u32];
        let vtable = rd32(owner);
        let slot = rd32(vtable + GATHER_SLOT);
        let gather: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        let rec = gather(owner, frame.as_mut_ptr() as u32);
        let floor = f32::from_bits(rd32(lf_checker_rt::relocated(C_FLOOR)));
        if !(rdf(rec + 8) <= floor) {
            return 0;
        }
        frame[0] = ACCEPT_ARG;
        let tag = ((owner + 0x210) as *const u8).read() as u32;
        frame[1] = 2 | (tag << 16);
        lf_checker_rt::callee_thiscall!(ACCEPT, u32, owner.wrapping_add(0x570), frame.as_mut_ptr() as u32);
        ((this + WINDOW) as *mut u8).write(0);
        0
    }
});
