// original: 0x00873710 rage::crmtNodeN::vf1
use lf_checker_rt::{callee_thiscall, export};

/// `rage::crmtNodeN::vf1`: tear down a blend node.
///
/// Releases the extra and target links at +0x124/+0x120 through their
/// vtables when present, clears both slots, runs the two teardown helpers,
/// then clears the state words at +8/+0xc/+0x10. Returns the second
/// helper's answer.
export!(thiscall, rw_00873710(this_: u32) -> u32 {
    unsafe {
        let extra = *((this_.wrapping_add(0x124)) as *const u32);
        if extra != 0 {
            let vt = *(extra as *const u32);
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(*((vt.wrapping_add(8)) as *const u32));
            release(extra);
        }
        let target = *((this_.wrapping_add(0x120)) as *const u32);
        *((this_.wrapping_add(0x124)) as *mut u32) = 0;
        if target != 0 {
            let vt = *(target as *const u32);
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(*((vt.wrapping_add(8)) as *const u32));
            release(target);
        }
        *((this_.wrapping_add(0x120)) as *mut u32) = 0;
        callee_thiscall!(2, u32, this_);
        let answer = callee_thiscall!(3, u32, this_);
        *((this_.wrapping_add(0xc)) as *mut u32) = 0;
        *((this_.wrapping_add(0x10)) as *mut u32) = 0;
        if *((this_.wrapping_add(8)) as *const u32) != 0 {
            *((this_.wrapping_add(8)) as *mut u32) = 0;
        }
        answer
    }
});
