// original: 0x008734c0 rage::crmtRequest::vf1
use lf_checker_rt::{callee_thiscall, export};

/// `rage::crmtRequest::vf1`: release the payload at +0xc, if attached.
///
/// Passes this+4 as the helper's context word. Returns the helper's
/// answer, or zero when there is no payload.
export!(thiscall, rw_008734c0(this_: u32) -> u32 {
    unsafe {
        let child = *((this_.wrapping_add(0xc)) as *const u32);
        if child != 0 {
            callee_thiscall!(1, u32, child, this_.wrapping_add(4))
        } else {
            0
        }
    }
});
