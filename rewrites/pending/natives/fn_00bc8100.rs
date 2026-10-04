// original: 0x00bc8100 START_PLAYBACK_RECORDED_CAR_WITH_OFFSET
//
// Script native handler: forwards the car handle and path number to one
// engine function as two stack words. The original also copies the three
// float offset arguments into a frame temporary and passes its address in
// ECX; ECX is not part of the cdecl call record, so that channel (and the
// temporary's contents, which sit below the observed stack window) cannot be
// compared under interception. Only the two stack arguments are verified.
export!(cdecl, rw_00bc8100(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        let _offset = [*args.add(2), *args.add(3), *args.add(4)];
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
