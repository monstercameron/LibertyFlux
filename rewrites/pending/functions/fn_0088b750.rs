// original: 0x0088b750 audio_update_slot_fields
/// Refreshes per-slot fields of every audio entity row.
///
/// Walks `count` rows of the global entity table (stride `0x6F40`) and, for
/// each of the 192 slots whose flag byte has both bits 0 and 3 set and
/// passes the mode/flag gates, recomputes the slot's value word from the
/// argument scaled by the slot gain, clamps it into range, and merges the
/// low 29 bits into the stored word. Slots with bit 0 set but bit 3 clear
/// just get bit 3 set. Returns nothing meaningful.
export!(cdecl, rw_0088b750(arg: u32) -> u32 {
    unsafe {
        let count = *(relocated(0x0115D96C) as *const u32);
        if count == 0 {
            return 0;
        }
        let table = *(relocated(0x0115D988) as *const u32) as *mut u8;
        let mut row_idx: u32 = 0;
        let mut row_off: u32 = 0;
        while row_idx < count {
            let row = table.add(row_off as usize);
            let mut slot: u32 = 0;
            while slot < 0xC0 {
                let flag = *row.add(slot as usize);
                if flag & 1 != 0 {
                    if flag & 8 == 0 {
                        *row.add(slot as usize) = flag | 8;
                    } else {
                        let gain_ptr = row.add(0xC0 + (slot as usize) * 0x70);
                        if *gain_ptr.add(0x18) & 4 == 0 {
                            let words =
                                row.add(0x54C4 + (slot as usize) * 0x20) as *mut u32;
                            let mode = *words.add(3) & 3;
                            if mode == 0 || mode == 2 {
                                let gain = *(gain_ptr as *const f32);
                                let prod = (arg as f32) * gain;
                                // cvttss2si semantics: truncate, indefinite on
                                // overflow/NaN (Rust `as` would saturate).
                                let scaled: i32 = if prod.is_nan()
                                    || prod >= 2147483648.0
                                    || prod < -2147483648.0
                                {
                                    0x80000000u32 as i32
                                } else {
                                    prod as i32
                                };
                                let center = *words.add(1);
                                let base_sub =
                                    ((center.wrapping_mul(8) as i32) >> 3) as u32;
                                let limit = *words.sub(1);
                                let mut v = (scaled as u32).wrapping_add(base_sub);
                                if v >= limit {
                                    let lo = *words;
                                    if lo == 0xFFFFFFFF {
                                        v = 0xFFFFFFFF;
                                    } else if limit == lo {
                                        v = lo;
                                    } else {
                                        v = (v.wrapping_sub(lo) % limit.wrapping_sub(lo))
                                            .wrapping_add(lo);
                                    }
                                }
                                let old = *words.add(1);
                                *words.add(1) = (old & !0x1FFFFFFF) | (v & 0x1FFFFFFF);
                            }
                        }
                    }
                }
                slot += 1;
            }
            row_idx += 1;
            row_off = row_off.wrapping_add(0x6F40);
        }
        0
    }
});
