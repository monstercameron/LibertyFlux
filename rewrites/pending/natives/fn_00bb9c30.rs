// original: 0x00BB9C30 TASK_GO_TO_OBJECT
/// Tasks a ped to go to an object (three words plus a float).
///
/// Forwards the three argument words and the trailing float word to the
/// engine task function.
lf_rn26_rt::export!(cdecl, rw_00BB9C30(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        let v = f32::from_bits(*a.add(3)).to_bits();
        lf_rn26_rt::callee_cdecl!(1, u32, *a, *a.add(1), *a.add(2), v)
    }
});
