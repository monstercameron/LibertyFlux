// original: 0x009f9890 tagged_mode_dispatch
use lf_k2_rt::{export, callee_cdecl, global};

/// Bits of 1.0f, pushed as the float argument to the notify calls.
const ONE_BITS: u32 = 0x3F800000;

/// Tagged mode dispatch (cdecl/2 -> al).
///
/// Returns 0 when the stored tag already equals `tag`; otherwise stores the
/// tag and fires the notify step for modes 0, 1, 2 and 4 through a jump
/// table (modes 3 and above 4 just return 1). Only AL is defined: the upper
/// bits keep the caller's entry garbage, so the contract compares AL.
export!(cdecl, rw_s18f13(tag: u32, mode: u32) -> u32 {
    unsafe {
        let tag = tag as u16;
        if *global::<u16>(0x12B627C) == tag {
            return 0;
        }
        *global::<u16>(0x12B627C) = tag;
        if mode > 4 {
            return 1;
        }
        // Jump-table order read from the binary.
        match mode {
            0 => {
                callee_cdecl!(1, u32, 0x112, ONE_BITS);
                1
            }
            1 => {
                callee_cdecl!(1, u32, 0x113, ONE_BITS);
                1
            }
            2 => {
                callee_cdecl!(1, u32, 0x114, ONE_BITS);
                1
            }
            3 => 1,
            _ => {
                callee_cdecl!(1, u32, 0x115, ONE_BITS);
                1
            }
        }
    }
});
