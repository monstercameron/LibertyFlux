// original: 0x00963190 mode_switch_store
/// Run the mode-change side effects for a new mode, then store it.
///
/// Arguments 3..=17 select a case: 7 runs no side effect; 11..=17 poll the
/// readiness helper and, when it reports false, run the reset helper and set
/// the reset flag; every other value in range checks the guard words first
/// and runs the reset path unless the guards say the mode is already live.
/// Out-of-range arguments skip straight to the store. Every path ends by
/// storing the argument as the current mode, keeping the previous mode next
/// to it, and returning the previous mode.
export!(cdecl, rw_00963190(arg: u32) -> u32 {
    unsafe {
        const MEDIA_ABSENT: u32 = 0xFFFFFFFF; // .rdata const at 0xF1C040
        let case = arg.wrapping_sub(3);
        let mut run_reset = false;
        if case <= 0xE {
            let idx = if case == 4 {
                2
            } else if case >= 8 {
                1
            } else {
                0
            };
            match idx {
                0 => {
                    let live = *global::<u32>(0x11F7060) == 1
                        || *global::<u32>(0x12088B4) != MEDIA_ABSENT
                        || *global::<u32>(0x1037720) == 0x12;
                    if !live || *global::<u32>(0x11F66A0) == 0 {
                        run_reset = true;
                    }
                }
                1 => {
                    let ready: u32 = callee_cdecl!(1, u32,);
                    if (ready as u8) == 0 {
                        run_reset = true;
                    }
                }
                _ => {}
            }
        }
        if run_reset {
            callee_cdecl!(2, u32,);
            *global::<u8>(0x11F7077) = 1;
        }
        let old = *global::<u32>(0x1037720);
        *global::<u32>(0x1037720) = arg;
        *global::<u32>(0x1037724) = old;
        old
    }
});
