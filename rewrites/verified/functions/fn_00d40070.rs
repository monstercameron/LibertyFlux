// original: 0x00d40070 jump_trace_ground_toggle (proposed)

/// Trace the ground below a jump and toggle the task's ground flag.
///
/// `this` is the task, `owner` (arg0) the ped. The locator object at
/// `LOCATOR` (+0x224) is queried through its virtual slot at +0x1c
/// (thiscall on the locator); the answer feeds the classify callee
/// (thiscall, no stack words), whose 0x5d means "no ground" and returns
/// 0. Otherwise a three-float point is built from the position block at
/// `POS` (+0x20) (+0x30/+0x34 copied, +0x38 plus the image constant 1.0)
/// and passed by pointer with the constant 0.2f, two zeros, 0x8e and the
/// query answer slot to the trace callee (cdecl, six words): the pushed
/// query answer is overwritten by 0.2f before the call and is dead. The
/// trace answer's zero/nonzero is xored with bit 0 of the task flag byte
/// at `TASK_FLAGS` (+0x92), the bit is updated to that value, and the bit
/// is returned. Only al carries the result.
///
/// The trace call takes a pointer into the caller's own frame; the
/// contract skips the address and snapshots the three pointed-to words.
///
/// Original: 0x00d40070 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d40070(this: u32, owner: u32) -> u32 {
    unsafe {
        const LOCATOR: u32 = 0x224;
        const QUERY_SLOT: u32 = 0x1c;
        const CLASSIFY: u32 = 2;
        const NO_GROUND: u32 = 0x5d;
        const POS: u32 = 0x20;
        const TRACE: u32 = 3;
        const C_ONE: u32 = 0x00fe_88e8;
        const C_SMALL: u32 = 0x0105_4bc0;
        const TASK_FLAGS: u32 = 0x92;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        let locator = rd32(owner + LOCATOR);
        let vtable = rd32(locator);
        let slot = rd32(vtable + QUERY_SLOT);
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let probed: u32 = query(locator);
        let class: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, probed);
        if class == NO_GROUND {
            return 0;
        }
        let pos = rd32(owner + POS);
        let z = core::hint::black_box(rdf(pos + 0x38))
            + core::hint::black_box(f32::from_bits(rd32(
                lf_checker_rt::relocated(C_ONE),
            )));
        let mut point = [
            rd32(pos + 0x30),
            rd32(pos + 0x34),
            z.to_bits(),
        ];
        let small = rd32(lf_checker_rt::relocated(C_SMALL));
        let hit: u32 = lf_checker_rt::callee_cdecl!(
            TRACE, u32, point.as_mut_ptr() as u32, small, 0, 0x8e, 0, 0
        );
        let flag_byte = (this + TASK_FLAGS) as *mut u8;
        let toggled = ((hit != 0) as u8) ^ flag_byte.read();
        let bit = toggled & 1;
        flag_byte.write(flag_byte.read() ^ bit);
        (flag_byte.read() & 1) as u32
    }
});
