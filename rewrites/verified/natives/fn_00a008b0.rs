// original: 0x00a008b0 CREATE_TEMPORARY_RADAR_BLIPS_FOR_PICKUPS_IN_AREA
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `CREATE_TEMPORARY_RADAR_BLIPS_FOR_PICKUPS_IN_AREA`: builds a three-float vector on its frame for ECX and redundantly passes the first element on the stack.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
export!(cdecl, rn24_create_temporary_radar_blips_for_pickups_in_area(ctx: u32) -> () {
        // The original builds a three-float vector on its frame, passes its
        // address in ECX (not part of the checker's cdecl call model), and
        // redundantly passes the vector's first element, script arg 0, on
        // the stack. Only the stack argument is compared.
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        let a0 = unsafe { (args as *const f32).read().to_bits() };
        callee_cdecl!(1, (), a0);
});
