// original: 0x005E8920 CPedIntelligenceNY::vf4
const PED_ACTION_MODE_OFFSET: u32 = 4;
const PED_ACTION_FIRST_WORD: u32 = 0x1c;
const PED_ACTION_SECOND_WORD: u32 = 0x20;
const PED_ACTION_MODE_PAIR_A: u32 = 0x70;
const PED_ACTION_MODE_PAIR_B: u32 = 0x71;
const PED_ACTION_MODE_SIMPLE: u32 = 0x72;
const PED_ACTION_MANAGER_FILE_VA: u32 = 0x0167e2a0;

/// Routes the action record's mode to one of three action helpers. The
/// record is the single stack argument; its mode is at offset `0x04`, and
/// modes `0x70` and `0x71` also forward words at offsets `0x1c` and `0x20`.
/// The global manager pointer is passed in ECX to each lookup helper. The
/// first two modes call separate two-word methods, mode `0x72` calls a
/// simple method, and other modes or a null lookup return zero. This
/// thiscall method pops four bytes and returns the selected helper's EAX.
lf_checker_rt::export!(thiscall, rw_005e8920(_this: u32, action_record: u32) -> u32 {
    let mode = unsafe {
        (action_record.wrapping_add(PED_ACTION_MODE_OFFSET) as *const u32).read_unaligned()
    };
    let manager = unsafe {
        lf_checker_rt::global::<u32>(PED_ACTION_MANAGER_FILE_VA).read_unaligned()
    };

    match mode {
        PED_ACTION_MODE_PAIR_A => {
            let action = lf_checker_rt::callee_thiscall!(1, u32, manager);
            if action == 0 {
                0
            } else {
                let first = unsafe {
                    (action_record.wrapping_add(PED_ACTION_FIRST_WORD) as *const u32)
                        .read_unaligned()
                };
                let second = unsafe {
                    (action_record.wrapping_add(PED_ACTION_SECOND_WORD) as *const u32)
                        .read_unaligned()
                };
                lf_checker_rt::callee_thiscall!(4, u32, action, first, second)
            }
        }
        PED_ACTION_MODE_PAIR_B => {
            let action = lf_checker_rt::callee_thiscall!(1, u32, manager);
            if action == 0 {
                0
            } else {
                let first = unsafe {
                    (action_record.wrapping_add(PED_ACTION_FIRST_WORD) as *const u32)
                        .read_unaligned()
                };
                let second = unsafe {
                    (action_record.wrapping_add(PED_ACTION_SECOND_WORD) as *const u32)
                        .read_unaligned()
                };
                lf_checker_rt::callee_thiscall!(3, u32, action, first, second)
            }
        }
        PED_ACTION_MODE_SIMPLE => {
            let action = lf_checker_rt::callee_thiscall!(1, u32, manager);
            if action == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(2, u32, action)
            }
        }
        _ => 0,
    }
});
