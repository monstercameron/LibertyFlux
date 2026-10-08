// original: 0x008B4B30 FRONTEND_MENU_TOGGLE_ON
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Enable or close one frontend menu item selected by the first argument.
///
/// The entry path checks the master byte, readiness helpers, mode dispatch,
/// secondary gate, and shared readiness function. A set latch calls its
/// provider and clear helper, clears the latch byte, and returns zero.
/// Otherwise five provider lookups prepare two selectors, two object slots,
/// and the setup byte before the index dispatches to one of 25 cases.
///
/// Cases 0 through 3 use selector windows, provider-record byte pairs,
/// signed record limits, and a shared counter/limit update. Cases 2 and 3
/// also consult the alternate-state byte and their case-specific resolver.
/// Cases 4 through 7 compare provider fields with the prepared object
/// slots. Cases 8 through 10 and 17 through 22 validate record-byte
/// agreements. Case 11 combines the setup byte, the sixth argument, record
/// pairs, resolver/checker calls, two global input words, and a counter
/// gate. Case 12 follows the setup and mode state through its resolver
/// calls. Cases 13 through 16 compare prepared slots or provider fields;
/// cases 23 and 24 call the checker on their provider records.
///
/// A successful case reports its case key when the second argument's low
/// byte is nonzero, always calls the shared close helper, and returns one.
/// Every failed gate or failed case returns zero. The two early returns in
/// case 12 preserve its original distinction: successful resolver checks
/// close without reporting, while the last failed resolver returns zero
/// directly.
const MASTER_ENABLE: u32 = 0x0103_0B9E;
const MODE_STATE: u32 = 0x0116_0C24;
const LATCH_BYTE: u32 = 0x0116_0B85;
const COUNTER: u32 = 0x0116_0C2C;
const LIMIT: u32 = 0x0117_3594;
const CASE_SWITCH: u32 = 0x0116_09F6;
const CASE_COUNTER: u32 = 0x0116_0C40;
const FILTER_A: u32 = 0x018B_7A84;
const FILTER_B: u32 = 0x018B_7A88;
const TAIL_THIS: u32 = 0x0117_6888;

fn provider() -> u32 {
    callee_cdecl!(4, u32, 1)
}

fn byte_at(base: u32, offset: u32) -> u8 {
    unsafe { (base.wrapping_add(offset) as *const u8).read() }
}

fn dword_at(base: u32, offset: u32) -> u32 {
    unsafe { (base.wrapping_add(offset) as *const u32).read() }
}

fn record_byte(offset: u32) -> u8 {
    byte_at(provider(), offset)
}

fn record_dword(offset: u32) -> u32 {
    dword_at(provider(), offset)
}

fn pair_xor(left: u32, right: u32) -> u8 {
    let base = provider();
    byte_at(base, left) ^ byte_at(base, right)
}

fn global_byte(address: u32) -> u8 {
    unsafe { global::<u8>(address).read() }
}

fn global_dword(address: u32) -> u32 {
    unsafe { global::<u32>(address).read() }
}

fn write_global_byte(address: u32, value: u8) {
    unsafe { global::<u8>(address).write(value) }
}

fn write_global_dword(address: u32, value: u32) {
    unsafe { global::<u32>(address).write(value) }
}

fn mode_route(mode: u32) -> u8 {
    match mode {
        0 | 19..=31 | 33..=34 | 36..=48 | 53..=57 | 71 => 0,
        65 => 1,
        _ => 2,
    }
}

fn bump_counter_when_below_limit() -> bool {
    let count = global_dword(COUNTER);
    let limit = global_dword(LIMIT);
    if count.wrapping_add(0x42) < limit {
        write_global_dword(COUNTER, limit);
        true
    } else {
        false
    }
}

fn resolver_at(offset: u32) -> bool {
    let object = provider().wrapping_add(offset);
    (callee_thiscall!(11, u32, object) & 0xFF) != 0
}

fn checker_at(offset: u32) -> bool {
    let object = provider().wrapping_add(offset);
    (callee_thiscall!(12, u32, object) & 0xFF) != 0
}

fn global_filter_matches() -> bool {
    let a = global_dword(FILTER_A);
    let b = global_dword(FILTER_B);
    ((a ^ b) & b & 8) != 0
}

fn selected_tail(select_word: u32, key: u32) -> u32 {
    if (select_word & 0xFF) != 0 {
        callee_thiscall!(9, u32, relocated(TAIL_THIS), relocated(key));
    }
    callee_thiscall!(10, u32, relocated(TAIL_THIS));
    1
}

/// Shared logic for cases 0 through 3. The resolver variants (cases 2 and 3)
/// call it directly on the ungated miss; cases 0 and 1 use the pair checks.
fn selector_case(
    selector_a: u32,
    selector_b: u32,
    case_gate: u8,
    pair_base: u32,
    record_offset: u32,
    threshold: i32,
    resolver_offset: Option<u32>,
) -> bool {
    if selector_a.wrapping_add(9) > 0x12 {
        return false;
    }

    let mut counter_bumped = false;
    if case_gate != 0 && pair_xor(pair_base + 2, pair_base) > 0x7F {
        counter_bumped = bump_counter_when_below_limit();
    }

    let field = record_dword(record_offset) as i32;
    let selector_b = selector_b as i32;
    let record_join = if threshold < 0 {
        field > threshold && selector_b < threshold
    } else {
        field < threshold && selector_b > threshold
    };
    if record_join || counter_bumped {
        return true;
    }

    if case_gate != 0 {
        return false;
    }
    if let Some(offset) = resolver_offset {
        return resolver_at(offset);
    }
    if pair_xor(pair_base + 2, pair_base) > 0x7F {
        return false;
    }
    if pair_xor(pair_base + 3, pair_base) <= 0x7F {
        return false;
    }
    true
}

fn two_pair_case(
    select_word: u32,
    first: u32,
    second: u32,
    base: u32,
    first_must_exceed: bool,
    second_must_exceed: bool,
    key: u32,
) -> u32 {
    let first_exceeds = pair_xor(first, base) > 0x7F;
    if first_exceeds != first_must_exceed {
        return 0;
    }
    let second_exceeds = pair_xor(second, base) > 0x7F;
    if second_exceeds != second_must_exceed {
        return 0;
    }
    selected_tail(select_word, key)
}

export!(cdecl, rw_008b4b30(
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32
) -> u32 {
    let flags = (a2 & 0xFF) as u8;
    if global_byte(MASTER_ENABLE) == 0 && a0 == 11 {
        return 0;
    }
    if (callee_cdecl!(1, u32,) & 0xFF) != 0 && flags & 2 == 0 {
        return 0;
    }
    if (a3 & 0xFF) == 0 {
        let mode = global_dword(MODE_STATE);
        if mode > 0x47 {
            return 0;
        }
        match mode_route(mode) {
            0 => {}
            1 if flags & 8 != 0 => {}
            _ => return 0,
        }
    }
    if (callee_cdecl!(2, u32,) & 0xFF) != 0 && flags & 4 == 0 {
        return 0;
    }
    if a0 != 11 && a0 != 12 && (callee_cdecl!(3, u32,) & 0xFF) == 0 {
        return 0;
    }
    if global_byte(LATCH_BYTE) != 0 {
        let object = provider();
        callee_thiscall!(5, u32, object);
        write_global_byte(LATCH_BYTE, 0);
        return 0;
    }

    let provider_a = provider();
    let selector_object_a = callee_thiscall!(6, u32, provider_a, 0);
    let selector_a = callee_cdecl!(8, u32, selector_object_a);

    let provider_b = provider();
    let selector_object_b = callee_thiscall!(7, u32, provider_b, 0);
    let selector_b = callee_cdecl!(8, u32, selector_object_b);

    let provider_c = provider();
    let slot_c = callee_cdecl!(8, u32, provider_c.wrapping_add(0x2B48));

    let provider_d = provider();
    let slot_d = callee_cdecl!(8, u32, provider_d.wrapping_add(0x2B38));

    let setup_byte = record_byte(0x328D);
    if a0 > 24 {
        return 0;
    }

    match a0 {
        0 => {
            if selector_case(selector_a, selector_b, a6 as u8, 0x2AAC, 0x3A74, -10, None) {
                selected_tail(a1, 0x00E7_D2EC)
            } else {
                0
            }
        }
        1 => {
            if selector_case(selector_a, selector_b, a6 as u8, 0x2A9C, 0x3A74, 10, None) {
                selected_tail(a1, 0x00E7_D320)
            } else {
                0
            }
        }
        2 => {
            if selector_case(selector_b, selector_a, a6 as u8, 0x2ABC, 0x3A70, -10, Some(0x2AB8)) {
                let key = if global_byte(CASE_SWITCH) != 0 && (a4 & 0xFF) != 0 {
                    0x00E7_D35C
                } else {
                    0x00E7_D378
                };
                selected_tail(a1, key)
            } else {
                0
            }
        }
        3 => {
            if selector_case(selector_b, selector_a, a6 as u8, 0x2ACC, 0x3A70, 10, Some(0x2AC8)) {
                let key = if global_byte(CASE_SWITCH) != 0 && (a4 & 0xFF) != 0 {
                    0x00E7_D390
                } else {
                    0x00E7_D3A8
                };
                selected_tail(a1, key)
            } else {
                0
            }
        }
        4 => {
            if (record_dword(0x3A78) as i32) >= 10 || (slot_c as i32) <= 10 {
                0
            } else {
                selected_tail(a1, 0x00E7_D400)
            }
        }
        5 => {
            if (record_dword(0x3A7C) as i32) <= -10 || (slot_d as i32) >= -10 {
                0
            } else {
                selected_tail(a1, 0x00E7_D424)
            }
        }
        6 => {
            if (record_dword(0x3A7C) as i32) >= 10 || (slot_d as i32) <= 10 {
                0
            } else {
                selected_tail(a1, 0x00E7_D44C)
            }
        }
        7 => {
            if (record_dword(0x3A78) as i32) <= -10 || (slot_c as i32) >= -10 {
                0
            } else {
                selected_tail(a1, 0x00E7_D3E0)
            }
        }
        8 => {
            let first_exceeds = pair_xor(0x2B6E, 0x2B6C) > 0x7F;
            if first_exceeds != ((a5 & 0xFF) != 0) {
                return 0;
            }
            let second_exceeds = pair_xor(0x2B6F, 0x2B6C) > 0x7F;
            if second_exceeds == ((a5 & 0xFF) != 0) {
                return 0;
            }
            let key = if (a4 & 0xFF) != 0 {
                0x00E7_D464
            } else {
                0x00E7_D47C
            };
            selected_tail(a1, key)
        }
        9 => two_pair_case(a1, 0x2B8E, 0x2B8F, 0x2B8C, false, true, 0x00E7_D494),
        10 => two_pair_case(a1, 0x2B9E, 0x2B9F, 0x2B9C, false, true, 0x00E7_D4AC),
        11 => {
            let mut local_flag = false;
            if (a5 & 0xFF) != 0 {
                if setup_byte != 0 {
                    local_flag = checker_at(0x2B58) || global_filter_matches();
                } else {
                    let first_exceeds = pair_xor(0x2B7E, 0x2B7C) > 0x7F;
                    if first_exceeds {
                        let second_exceeds = pair_xor(0x2B7F, 0x2B7C) > 0x7F;
                        local_flag = !second_exceeds;
                    }
                    if !local_flag {
                        local_flag = global_filter_matches();
                    }
                }
            } else if setup_byte != 0 {
                local_flag = resolver_at(0x2B58) || global_filter_matches();
            } else {
                let first_exceeds = pair_xor(0x2B7E, 0x2B7C) > 0x7F;
                let pair_ok = !first_exceeds
                    && pair_xor(0x2B7F, 0x2B7C) > 0x7F;
                local_flag = pair_ok || global_filter_matches();
            }

            if global_dword(CASE_COUNTER) == 0x32 && (callee_cdecl!(1, u32,) & 0xFF) == 0 {
                return 0;
            }
            if !local_flag {
                return 0;
            }
            selected_tail(a1, 0x00E7_D4C4)
        }
        12 => {
            if setup_byte == 0 {
                if !resolver_at(0x2B58) {
                    return 0;
                }
                return selected_tail(0, 0);
            }
            if global_byte(CASE_SWITCH) == 0 && resolver_at(0x2B58) {
                return selected_tail(0, 0);
            }
            if global_dword(MODE_STATE) != 0x31 {
                return 0;
            }
            if resolver_at(0x2CB8) {
                selected_tail(0, 0)
            } else {
                0
            }
        }
        13 => {
            if (slot_c as i32) < -10 || pair_xor(0x2C6E, 0x2C6C) > 0x7F {
                selected_tail(a1, 0x00E7_D4D8)
            } else {
                0
            }
        }
        14 => {
            if (slot_c as i32) > 10 || pair_xor(0x2C7E, 0x2C7C) > 0x7F {
                selected_tail(a1, 0x00E7_D4F8)
            } else {
                0
            }
        }
        15 => {
            if (slot_d as i32) > 10 {
                selected_tail(a1, 0x00E7_D5C8)
            } else {
                0
            }
        }
        16 => {
            if (slot_d as i32) < -10 {
                selected_tail(a1, 0x00E7_D5E8)
            } else {
                0
            }
        }
        17 => two_pair_case(a1, 0x2BCE, 0x2BCF, 0x2BCC, false, true, 0x00E7_D550),
        18 => two_pair_case(a1, 0x2BDE, 0x2BDF, 0x2BDC, false, true, 0x00E7_D538),
        19 => two_pair_case(a1, 0x2BAE, 0x2BAF, 0x2BAC, false, true, 0x00E7_D568),
        20 => two_pair_case(a1, 0x2BBE, 0x2BBF, 0x2BBC, false, true, 0x00E7_D580),
        21 => {
            if pair_xor(0x2BCE, 0x2BCC) > 0x7F {
                selected_tail(a1, 0x00E7_D5B0)
            } else {
                0
            }
        }
        22 => {
            if pair_xor(0x2BDE, 0x2BDC) > 0x7F {
                selected_tail(a1, 0x00E7_D598)
            } else {
                0
            }
        }
        23 => {
            if checker_at(0x2CA8) {
                selected_tail(a1, 0x00E7_D608)
            } else {
                0
            }
        }
        24 => {
            if checker_at(0x2CB8) {
                selected_tail(a1, 0x00E7_D62C)
            } else {
                0
            }
        }
        _ => 0,
    }
});
