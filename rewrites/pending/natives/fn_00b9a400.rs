// original: 0x00b9a400 DEFINE_PED_GENERATION_CONSTRAINT_AREA
// rw_define_ped_generation_constraint_area: native DEFINE_PED_GENERATION_CONSTRAINT_AREA (handler 0x00B9A400).
//
// Forwards four float args (x, y, z, radius) to the ped-generation-area setter. No return slot.
export!(cdecl, rw_define_ped_generation_constraint_area(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3));
        ans
    }
});
