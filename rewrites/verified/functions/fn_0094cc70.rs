// original: 0x0094CC70 gated_slot0_probe (proposed)

/// Probe a nested float against a threshold, then ask a slot-0x1B0 function.
///
/// Reads the threshold word `THRESH` and the float at `[obj+0x20]+0x28`.
/// When the threshold is strictly above the float (ordered), the classifier
/// runs (callee 1, thiscall with `obj` in ECX); a SIGNED answer below 4
/// proceeds, anything else falls into the second test. That test re-reads
/// the float and proceeds only when it is strictly below 0.0 (ordered, so
/// NaN returns 0 on either compare). The final path calls the function at
/// slot 0x1B0 of `obj`'s table (callee 2, thiscall with `obj` in ECX and 0
/// on the stack; the contract plants the stub address in the slot) and
/// returns 1 when its low byte is nonzero, 0 otherwise. Only `al` is
/// defined.
///
/// Original: 0x0094CC70 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0094CC70(obj: u32) -> u32 {
    unsafe {
        const THRESH: u32 = 0xFE87E8;
        const CLASSIFY: u32 = 1;
        const SLOT: u32 = 2;
        const INNER: u32 = 0x20;
        const FIELD: u32 = 0x28;
        let t = f32::from_bits(
            (lf_checker_rt::relocated(THRESH) as *const u32).read(),
        );
        let p = (obj.wrapping_add(INNER) as *const u32).read();
        let f = f32::from_bits(
            (p.wrapping_add(FIELD) as *const u32).read(),
        );
        let thru = if t > f {
            let c = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, obj);
            if (c as i32) < 4 {
                true
            } else {
                let p2 = (obj.wrapping_add(INNER) as *const u32).read();
                let f2 = f32::from_bits(
                    (p2.wrapping_add(FIELD) as *const u32).read(),
                );
                0.0 > f2
            }
        } else {
            let p2 = (obj.wrapping_add(INNER) as *const u32).read();
            let f2 = f32::from_bits(
                (p2.wrapping_add(FIELD) as *const u32).read(),
            );
            0.0 > f2
        };
        if !thru {
            return 0;
        }
        let r = lf_checker_rt::callee_thiscall!(SLOT, u32, obj, 0);
        ((r as u8) != 0) as u32
    }
});
