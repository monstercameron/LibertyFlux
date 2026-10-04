// original: 0x00bc6400 GET_TOTAL_DURATION_OF_CAR_RECORDING
/// Call the recording-duration engine function (float result) and store it.
export!(cdecl, rw_00bc6400(ctx: u32) -> u32 {
    const RET_SLOT: usize = 0;
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let retp = unsafe { *c.add(RET_SLOT) } as *mut f32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let rec = unsafe { *args.add(0) };
    let r: f32 = callee_cdecl!(1, f32, rec);
    unsafe { *retp = r; }
    retp as u32
});
