// original: 0x00b2de20 garage_substate_step
// 0xB2DE20 garage_substate_step (thiscall/0).
//
// Advances the sub-state machine: kind 2 records move toward 3 (only from
// sub-states 0, 2 or 3), every other nonzero kind moves toward 2 (only from
// sub-states 1, 2 or 3). Anything else is left alone.
export!(thiscall, rw_00b2de20(rec: *mut u8) -> () {
    unsafe {
        let idx = (*rec.add(0x48)).wrapping_sub(1);
        if idx > 4 {
            return;
        }
        let sub = *rec.add(0x49);
        if idx == 1 {
            if sub == 0 || sub == 2 || sub == 3 {
                *rec.add(0x49) = 3;
            }
        } else if (1..=3).contains(&sub) {
            *rec.add(0x49) = 2;
        }
    }
});
