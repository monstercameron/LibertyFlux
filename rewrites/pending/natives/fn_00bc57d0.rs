// original: 0x00BC57D0 FIND_TIME_POSITION_IN_RECORDING
//
// Forwards the vehicle handle; the engine returns the recording time as an
// x87 float. The original spills it through its own incoming context slot
// before copying it to the return slot; both writes are reproduced (the
// slot offset is measured per build, see rw_00b9e340).
export!(cdecl, rw_00bc57d0(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        let result = callee_cdecl!(1, u32, *args);
        *retslot_of(ctx) = result;
        result
    }
});

