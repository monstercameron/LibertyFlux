// original: 0x00bb6620 REGISTER_ODDJOB_MISSION_PASSED
use lf_k2_rt::{callee_cdecl, export, global};
/// Script native `REGISTER_ODDJOB_MISSION_PASSED`.
/// Sets the latest odd job mission passed. The handler entry jumps here;
/// this body reads no script arguments: it fetches a float through one
/// engine call, feeds its bits to a second call, makes a third call with
/// constant arguments, then copies one engine global to another.
export!(cdecl, rw_00bb6620(ctx: u32) -> u32 {
    unsafe {
        let _ = ctx;
        let fetched: f32 = callee_cdecl!(1, f32, 0x1B0);
        let _: u32 = callee_cdecl!(2, u32, 0x1A5, fetched.to_bits());
        let _: u32 = callee_cdecl!(3, u32, 0x1B0, 0);
        let value = *global::<u32>(0x011735B4);
        *global::<u32>(0x011D78F4) = value;
        value
    }
});
