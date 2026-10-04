// original: 0x008AAE20 audio_effect_bind_params
/// Bind this effect object's parameter slots by name.
///
/// The object holds a sub-object ("binder") at offset 0x1D4. Behaviour:
/// gate on the first call's low byte, then bind sixteen named slots through
/// four binding call groups (two string-keyed, three, six conditional with
/// the slot pointer recorded back into the object, four), then finalise.
/// Returns nonzero on success, zero when the gate call fails.
export!(thiscall, rw_008aae20(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        // Offset of the binder sub-object within the effect object.
        const BINDER_OFF: u32 = 0x1d4;
        // Name strings live in read-only data; resolve per load base.
        const NAME_UP: u32 = 0x00e7bef8;
        const NAME_FWD: u32 = 0x00e7bf08;
        const NAME_SCALE: u32 = 0x00e7bf1c;
        const NAME_P3: u32 = 0x00e7bf2c;
        const NAME_P4: u32 = 0x00e7bf4c;
        const NAME_S0: u32 = 0x00e7bf5c;
        const NAME_S1: u32 = 0x00e7bf6c;
        const NAME_S2: u32 = 0x00e7bf7c;
        const NAME_S3: u32 = 0x00e7bf90;
        const NAME_S4: u32 = 0x00e7bfa0;
        const NAME_S5: u32 = 0x00e7bfb0;
        const NAME_T0: u32 = 0x00e7bfc8;
        const NAME_T1: u32 = 0x00e7bfdc;
        const NAME_T2: u32 = 0x00e7bfe8;
        const NAME_T3: u32 = 0x00e7c008;

        let base = this as u32;
        let binder = base.wrapping_add(BINDER_OFF);

        // Gate: only the low byte of the answer matters.
        let gate: u32 = callee_thiscall!(1, u32, binder, arg);
        if gate & 0xff == 0 {
            return 0;
        }

        // Callee ids: 2 = first binder, 3 = second, 4 = conditional slot
        // binder, 5 = tail binder, 6 = finalise.
        let _: u32 = callee_thiscall!(2, u32, binder, relocated(NAME_UP), base);
        let _: u32 = callee_thiscall!(2, u32, binder, relocated(NAME_FWD), base.wrapping_add(0x10));
        let _: u32 = callee_thiscall!(3, u32, binder, relocated(NAME_SCALE), base.wrapping_add(0x1a0));
        let _: u32 = callee_thiscall!(3, u32, binder, relocated(NAME_P3), base.wrapping_add(0x1a8));
        let _: u32 = callee_thiscall!(3, u32, binder, relocated(NAME_P4), base.wrapping_add(0x1a4));

        // Six conditional slots: on a nonzero low byte the slot pointer is
        // recorded in the object at running offsets 0x1AC..0x1C0.
        let slots: [(u32, u32, u32); 6] = [
            (NAME_S0, 0x020, 0x1ac),
            (NAME_S1, 0x060, 0x1b0),
            (NAME_S2, 0x0a0, 0x1b4),
            (NAME_S3, 0x0e0, 0x1b8),
            (NAME_S4, 0x120, 0x1bc),
            (NAME_S5, 0x160, 0x1c0),
        ];
        for (name, slot_off, rec_off) in slots {
            let ok: u32 = callee_thiscall!(4, u32, binder, relocated(name), base.wrapping_add(slot_off));
            if ok & 0xff != 0 {
                *(base.wrapping_add(rec_off) as *mut u32) = base.wrapping_add(slot_off);
            }
        }

        let _: u32 = callee_thiscall!(5, u32, binder, relocated(NAME_T0), base.wrapping_add(0x1c4));
        let _: u32 = callee_thiscall!(5, u32, binder, relocated(NAME_T1), base.wrapping_add(0x1c8));
        let _: u32 = callee_thiscall!(5, u32, binder, relocated(NAME_T2), base.wrapping_add(0x1cc));
        let _: u32 = callee_thiscall!(5, u32, binder, relocated(NAME_T3), base.wrapping_add(0x1d0));
        let _: u32 = callee_thiscall!(6, u32, binder);
        1
    }
});
