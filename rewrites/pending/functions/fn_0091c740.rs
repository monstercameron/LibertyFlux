// original: 0x0091C740 compute_and_store_float
/// Run a two-float query through helpers and store the measured result.
///
/// Returns zero for a null query pointer. Otherwise initializes a 12-byte
/// local through the setup helper, runs the query helper with the two floats,
/// the query pointer and the local, and when the destination pointer is not
/// null stores the measure helper's `ST0` result there. Always finishes
/// through the teardown helper and returns its answer. The float arguments
/// travel on the stack bit-exact; the local's address is passed to every
/// helper but its contents are never read back.
export!(cdecl, rw_0091c740(a: f32, b: f32, q: u32, dst: u32) -> u32 {
    unsafe {
        if q == 0 {
            return 0;
        }
        let mut local = [0u32; 3];
        let _: u32 = callee_thiscall!(1, u32, local.as_mut_ptr() as u32);
        let _: u32 = callee_cdecl!(2, u32, a.to_bits(), b.to_bits(), q,
            local.as_mut_ptr() as u32);
        if dst != 0 {
            let r: f32 = callee_thiscall!(3, f32, local.as_mut_ptr() as u32);
            *(dst as *mut f32) = r;
        }
        callee_thiscall!(4, u32, local.as_mut_ptr() as u32)
    }
});
