// original: 0x00b0ada0 compare_slot_infos
/// Compare the info blocks of two slots through the shared resolver.
///
/// Resolves both slot ids; either missing fails. Dispatches on the first
/// block's kind word: a new block passes only when the second block is a
/// plain block with a smaller stamp; a linked block passes when the module
/// is in mode two, the second id is in range, and the second block is a
/// plain or wide block; a plain block passes when the second is plain; a
/// wide block passes when the second is plain or wide. Anything else fails.
export!(cdecl, rw_00b0ada0(a0: u32, a1: u32) -> u32 {
    unsafe {
        const KIND_OFF: usize = 4;
        const STAMP_OFF: usize = 0x10;
        const MODE: u32 = 0x011D_6FD4;
        const PLAIN: u32 = 2;
        const WIDE: u32 = 4;
        const MODE_OK: u32 = 2;
        const ID_MIN: i32 = 0x15;
        let first = callee_cdecl!(1, u32, a1);
        let second = callee_cdecl!(2, u32, a0);
        if first == 0 || second == 0 {
            return 0;
        }
        let kind = (*((first as usize + KIND_OFF) as *const u32)).wrapping_sub(PLAIN);
        if kind > 3 {
            return 0;
        }
        let skind = *((second as usize + KIND_OFF) as *const u32);
        match kind {
            0 => {
                if skind != PLAIN {
                    return 0;
                }
                let older = *((second as usize + STAMP_OFF) as *const i32);
                let newer = *((first as usize + STAMP_OFF) as *const i32);
                if older >= newer {
                    return 0;
                }
                1
            }
            1 => {
                if *global::<u32>(MODE) != MODE_OK {
                    return 0;
                }
                if (a1 as i32) < ID_MIN {
                    return 0;
                }
                if skind == PLAIN || skind == WIDE {
                    1
                } else {
                    0
                }
            }
            2 => {
                if skind != PLAIN {
                    return 0;
                }
                1
            }
            _ => {
                if skind == PLAIN || skind == WIDE {
                    1
                } else {
                    0
                }
            }
        }
    }
});
