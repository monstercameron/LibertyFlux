// original: 0x00b3b2b0 task_point_accepted (proposed)

/// Decide whether the point at `point` is accepted: when the gate callee's
/// low result byte is nonzero the point passes at once; otherwise the three
/// floats must lie inside the primary task-bounds box (`lo <= v <= hi` per
/// lane with the ordered greater test, so NaN passes a bound check) and the
/// primary-ready flag must be set. The result keeps the original's exact
/// register contents: the gate answer with its low byte replaced by 1 on
/// accept, 0 on reject. Original: 0x00b3b2b0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00b3b2b0(point: u32) -> u32 {
    unsafe {
        const GATE: u32 = 1;
        const READY_FLAG: u32 = 0x01662491;
        const MIN_SLOTS: u32 = 0x01662510;
        const MAX_SLOTS: u32 = 0x01662660;
        let gate: u32 = lf_checker_rt::callee_cdecl!(GATE, u32, point);
        if gate & 0xff != 0 {
            return (gate & 0xffff_ff00) | 1;
        }
        if lf_checker_rt::global::<u8>(READY_FLAG).read() == 0 {
            return gate & 0xffff_ff00;
        }
        for lane in 0..3u32 {
            let off = lane * 4;
            let lo = f32::from_bits(
                lf_checker_rt::global::<u32>(MIN_SLOTS + off).read(),
            );
            let hi = f32::from_bits(
                lf_checker_rt::global::<u32>(MAX_SLOTS + off).read(),
            );
            let v = f32::from_bits(
                ((point + off) as *const u32).read_unaligned(),
            );
            if lo > v || v > hi {
                return gate & 0xffff_ff00;
            }
        }
        (gate & 0xffff_ff00) | 1
    }
});
