// original: 0x008731d0 child_release_if_attached
use lf_checker_rt::{callee_thiscall, export};

/// Release the child at +8 through the unlink helper, if one is attached.
///
/// Returns the helper's answer, or zero when there is no child.
export!(thiscall, rw_008731d0(this_: u32) -> u32 {
    unsafe {
        let child = *((this_.wrapping_add(8)) as *const u32);
        if child != 0 {
            callee_thiscall!(1, u32, child, this_)
        } else {
            0
        }
    }
});
