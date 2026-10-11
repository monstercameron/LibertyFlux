// original: 0x00A1F0E0 ped_task_update_large
/// Specification: When the initial eligibility query returns false and the
/// current-item query returns null, the routine exits without changing state.
/// This proof does not cover later geometry work or the double-angle path.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export};

const LOW_BYTE_MASK: u32 = 0xFF;

export!(thiscall, rw_fn_00a1f0e0(_this: u32, _arg0: u32, _arg1: u32, _arg2: u32) -> u32 {
    let eligible = callee_thiscall!(1, u32, _this);
    if eligible & LOW_BYTE_MASK != 0 {
        return eligible;
    }
    let current_item = callee_cdecl!(2, u32,);
    if current_item == 0 { 0 } else { current_item }
});