// original: 0x00b00550 VehicleBudgetFlagUpdate
/// Bump one of five budget counters up or down (saturating at zero) based on
/// the object's kind byte and a secondary flag. Kinds 3 and 4 share a counter.
export!(cdecl, rw_00b00550(obj: u32, sub: u32) -> u32 {
    unsafe {
        let kind = (*((obj + 0x10b8) as *const u8) as u32).wrapping_sub(1);
        let secondary = *((obj + 0xf1d) as *const u8) & 4 != 0;
        let dec_sat = |slot: *mut u32| {
            let next = (*slot).wrapping_sub(1);
            *slot = if (next as i32) < 0 { 0 } else { next };
        };
        if sub == 0 {
            match kind {
                0 => *global::<u32>(0x1600154) =
                    (*global::<u32>(0x1600154)).wrapping_add(1),
                1 => {
                    if secondary {
                        *global::<u32>(0x160015c) =
                            (*global::<u32>(0x160015c)).wrapping_add(1);
                    } else {
                        *global::<u32>(0x1600158) =
                            (*global::<u32>(0x1600158)).wrapping_add(1);
                    }
                }
                2 => *global::<u32>(0x1600160) =
                    (*global::<u32>(0x1600160)).wrapping_add(1),
                3 | 4 => *global::<u32>(0x1600164) =
                    (*global::<u32>(0x1600164)).wrapping_add(1),
                _ => {}
            }
        } else {
            match kind {
                0 => dec_sat(global::<u32>(0x1600154)),
                1 => {
                    let kept = *global::<u32>(0x160015c);
                    let alt = *global::<u32>(0x1600158);
                    if secondary {
                        let next = kept.wrapping_sub(1);
                        *global::<u32>(0x160015c) =
                            if (next as i32) < 0 { 0 } else { next };
                        if (alt as i32) < 0 {
                            *global::<u32>(0x1600158) = 0;
                        }
                    } else {
                        let next = alt.wrapping_sub(1);
                        *global::<u32>(0x1600158) = next;
                        *global::<u32>(0x160015c) =
                            if (kept as i32) < 0 { 0 } else { kept };
                        if (next as i32) < 0 {
                            *global::<u32>(0x1600158) = 0;
                        }
                    }
                }
                2 => dec_sat(global::<u32>(0x1600160)),
                3 | 4 => dec_sat(global::<u32>(0x1600164)),
                _ => {}
            }
        }
        0
    }
});
