// original: 0x0094cd50 NativeImpl_IS_CHAR_STOPPED
/// Decide whether a character currently counts as stopped.
///
/// Returns 1 when any of three independent conditions holds, else 0:
/// - the detail flag (bit 2 of the status word at +0x26C) is set, a
///   position sub-object is attached at +0xB30, and its distance from the
///   reference point at +0x200 is within the scaled limit from the global
///   at 0x11735BC (times the constant 0.5);
/// - the checks below run and both velocity components at +0xE0/+0xE4
///   compare equal to zero (either sign of zero; NaN fails).
/// Between them, when the movement byte at +0x219 is non-zero, six state
/// queries (codes 0x30, 0x31, 0x53-0x56) are issued in order through the
/// helper at +0x78 and any non-zero answer means not stopped; the state
/// index at +0xB80 must be below 2 (signed), the mode bits 0x6000 of the
/// status word must be clear, and the active flag (bit 0) must be set.
/// The distance comparison treats any NaN input as out of range.
export!(cdecl, rw_0094cd50(obj: u32) -> u32 {
    unsafe {
        const STATUS_OFF: u32 = 0x26C;
        const DETAIL_BIT: u32 = 4;
        const ACTIVE_BIT: u32 = 1;
        const MODE_MASK: u32 = 0x6000;
        const POS_OBJ_OFF: u32 = 0xB30;
        const REF_OFF: u32 = 0x200;
        const ALT_POS_OFF: u32 = 0x10;
        const INDIR_OFF: u32 = 0x20;
        const INDIR_POS_OFF: u32 = 0x30;
        const LIMIT_GLOB: u32 = 0x11735BC;
        const LIMIT_SCALE: u32 = 0xFE8830;
        const MOVE_OFF: u32 = 0x219;
        const HELPER_OFF: u32 = 0x78;
        const STATE_OFF: u32 = 0xB80;
        const VEL_X_OFF: u32 = 0xE0;
        const VEL_Y_OFF: u32 = 0xE4;
        const QUERY_CODES: [u32; 6] = [0x30, 0x31, 0x53, 0x54, 0x55, 0x56];

        let status = *((obj.wrapping_add(STATUS_OFF)) as *const u32);
        if status & DETAIL_BIT != 0 {
            let sub = *((obj.wrapping_add(POS_OBJ_OFF)) as *const u32);
            if sub != 0 {
                let indir = *((sub.wrapping_add(INDIR_OFF)) as *const u32);
                let pos = if indir != 0 {
                    indir.wrapping_add(INDIR_POS_OFF)
                } else {
                    sub.wrapping_add(ALT_POS_OFF)
                };
                let dx = *((pos) as *const f32)
                    - *((sub.wrapping_add(REF_OFF)) as *const f32);
                let dy = *((pos.wrapping_add(4)) as *const f32)
                    - *((sub.wrapping_add(REF_OFF + 4)) as *const f32);
                let dz = *((pos.wrapping_add(8)) as *const f32)
                    - *((sub.wrapping_add(REF_OFF + 8)) as *const f32);
                // Addition order matches the original: (dy^2 + dx^2) + dz^2.
                let dist2 = dy * dy + dx * dx;
                let dist2 = dist2 + dz * dz;
                let dist = dist2.sqrt();
                let limit = *(global::<f32>(LIMIT_GLOB))
                    * *(relocated(LIMIT_SCALE) as *const f32);
                // Original is comiss+jb: out of range when less or unordered.
                if !(limit >= dist) {
                    return 0;
                }
                return 1;
            }
        }
        if *((obj.wrapping_add(MOVE_OFF)) as *const u8) != 0 {
            let helper = *((obj.wrapping_add(HELPER_OFF)) as *const u32);
            let mut id = 1u32;
            for code in QUERY_CODES {
                let answer: u32 = callee_thiscall!(id, u32, helper, code);
                if answer != 0 {
                    return 0;
                }
                id += 1;
            }
        }
        if *((obj.wrapping_add(STATE_OFF)) as *const i32) >= 2 {
            return 0;
        }
        if status & MODE_MASK != 0 {
            return 0;
        }
        if status & ACTIVE_BIT == 0 {
            return 0;
        }
        // Original compares with ucomiss: true only for ordered equality
        // with zero, so both signs of zero pass and NaN fails.
        if *((obj.wrapping_add(VEL_X_OFF)) as *const f32) == 0.0
            && *((obj.wrapping_add(VEL_Y_OFF)) as *const f32) == 0.0
        {
            1
        } else {
            0
        }
    }
});
