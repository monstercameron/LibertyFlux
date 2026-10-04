// original: 0x009CC110 PLAY_SOUND_FROM_PED
//
// Script native handler: forwards the sound id, sound name and ped handle
// to one engine function (cdecl/3). No return value stored.
export!(cdecl, rw_009cc110(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
