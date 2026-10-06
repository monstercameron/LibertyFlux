// original: 0x00924DF0 input_level_check
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated};
/// Report whether the input level is usable: threshold plus a size probe.
///
/// Modes 1-3 of global `0x01045538` report false. Otherwise a helper pair
/// produces a 64-bit value that must clear 0x16800000 (with a negative high
/// half failing outright), and the shared size probe must reach 0x100 (compared signed: the
/// original's compare is followed by a signed jump, so a negative probe
/// answer counts as below the bound).
export!(cdecl, rw_00924DF0() -> u32 {
    unsafe {
        match *global::<u32>(0x1045538) {
            1 | 2 | 3 => return 0,
            _ => {}
        }
        let helper = callee_stdcall!(1, u32, 0);
        let v = callee_thiscall!(2, u64, helper);
        let (lo, hi) = (v as u32, (v >> 32) as i32);
        let ok = if hi < 0 {
            false
        } else if hi > 0 {
            true
        } else {
            lo >= 0x16800000
        };
        if !ok {
            return 0;
        }
        let probe: u32 = callee_cdecl!(3, u32,);
        if (probe as i32) < 0x100 {
            0
        } else {
            1
        }
    }
});
