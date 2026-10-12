// original: 0x00c5bd00 intel_process_shocking_events

/// Process one ped's shocking-event stimulus: decide whether a nearby event
/// reaches the ped, then decay the ped's stored stimulus value.
///
/// `this` points to the task record (`+0`: expected slot remainder, `+4`:
/// slot index into the shared table). `obj` is the ped object (vtable at
/// `+0`, position block at `+0x20` with x/y/z floats at `+0x30/0x34/0x38`).
///
/// The helper callee (called with 0 up to three times) yields an event
/// source; when it answers null, or its status word (`+0x12ec`) is 0, the
/// ped is treated as out of range. Otherwise the squared distance between
/// the source position (`+0x20` block) and the ped position is compared
/// against the range table value: strictly below counts as near.
///
/// The event callee runs when the ped is near, or when the slot remainder
/// (shared dividend divided by the slot's divisor at table entry `+4`,
/// signed) equals the record's expected value.
///
/// The vtable slot at `+0xd0` then yields the ped's stimulus record on each
/// of up to nine calls. Its value word (`+0x180`) above the floor threshold
/// is stepped down by the shared decay step and stored back. A stepped value
/// of 0.0 or less (ordered; not-a-number exits) clears the record: value,
/// words `+0x170`/`+0x178` and flag byte `+0x17c` to 0, word `+0x174` to -1.
/// Returns the last vtable result.
///
/// Original: thiscall, one stack word. All comparisons are the original's:
/// `range > dist2` (false for NaN), `value > floor` (false for NaN), and
/// `stepped <= 0.0` (false for NaN, so NaN exits like the original's branch
/// on below). The divisor is always positive here, so the signed division
/// cannot fault.
lf_checker_rt::export!(thiscall, rw_00c5bd00(this: u32, obj: u32) -> u32 {
    unsafe {
        const HELPER: u32 = 1;
        const EVENT: u32 = 3;
        const SLOT_TABLE: u32 = 0x012B_60A0;
        const SLOT_DIVIDEND: u32 = 0x012B_41D4;
        const RANGE_LIMIT: u32 = 0x00FE_8C00;
        const VALUE_FLOOR: u32 = 0x00FE_8628;
        const DECAY_STEP: u32 = 0x0117_35BC;
        const THIS_EXPECT: u32 = 0x00;
        const THIS_INDEX: u32 = 0x04;
        const SRC_STATUS: u32 = 0x12EC;
        const SRC_POS: u32 = 0x20;
        const OBJ_POS: u32 = 0x20;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const POS_Z: u32 = 0x38;
        const VT_RESULT: u32 = 0xD0;
        const REC_VALUE: u32 = 0x180;
        const REC_A: u32 = 0x170;
        const REC_B: u32 = 0x174;
        const REC_C: u32 = 0x178;
        const REC_FLAG: u32 = 0x17C;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// Call the ped's result slot like the original: load the function
        /// pointer from the object's table and call it with the object.
        unsafe fn result_of(obj: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(obj) + VT_RESULT);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(obj)
            }
        }

        // Nearness of the event source to the ped.
        let mut near = false;
        if lf_checker_rt::callee_cdecl!(HELPER, u32, 0u32) != 0 {
            let probe = lf_checker_rt::callee_cdecl!(HELPER, u32, 0u32);
            if rd32(probe + SRC_STATUS) != 0 {
                let found = lf_checker_rt::callee_cdecl!(HELPER, u32, 0u32);
                let pa = rd32(found + SRC_POS);
                let pe = rd32(obj + OBJ_POS);
                let dx = sub(rdf(pa + POS_X), rdf(pe + POS_X));
                let dy = sub(rdf(pa + POS_Y), rdf(pe + POS_Y));
                let dz = sub(rdf(pa + POS_Z), rdf(pe + POS_Z));
                let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                let limit = rdf(lf_checker_rt::relocated(RANGE_LIMIT));
                near = limit > dist2;
            }
        }

        // Slot gate: signed remainder of the shared dividend.
        let idx = rd32(this + THIS_INDEX);
        let entry = rd32(lf_checker_rt::relocated(SLOT_TABLE)
            .wrapping_add(idx.wrapping_mul(4)));
        let dividend = rd32(lf_checker_rt::relocated(SLOT_DIVIDEND)) as i32;
        let divisor = rd32(entry.wrapping_add(4)) as i32;
        let rem = (dividend % divisor) as u32;
        if rem == rd32(this + THIS_EXPECT) || near {
            lf_checker_rt::callee_cdecl!(EVENT, u32, obj);
        }

        // Stimulus decay through the result slot.
        let floor = rdf(lf_checker_rt::relocated(VALUE_FLOOR));
        let r = result_of(obj);
        if !(rdf(r + REC_VALUE) > floor) {
            return r;
        }
        let r = result_of(obj);
        let stepped = sub(
            rdf(r + REC_VALUE),
            rdf(lf_checker_rt::relocated(DECAY_STEP)),
        );
        let r = result_of(obj);
        wrf(r + REC_VALUE, stepped);
        let r = result_of(obj);
        if rdf(r + REC_VALUE) <= 0.0 {
            let r = result_of(obj);
            wr32(r + REC_VALUE, 0);
            let r = result_of(obj);
            wr32(r + REC_A, 0);
            let r = result_of(obj);
            wr32(r + REC_B, 0xFFFF_FFFF);
            let r = result_of(obj);
            wr32(r + REC_C, 0);
            let r = result_of(obj);
            ((r + REC_FLAG) as *mut u8).write(0);
            return r;
        }
        r
    }
});
