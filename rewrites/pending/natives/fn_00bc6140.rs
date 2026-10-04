// original: 0x00bc6140 GET_POSITION_OF_CAR_RECORDING_AT_TIME
//
// Script native handler: samples a car-recording slot. Reads the recording
// handle, the time float and the script out-pointer from the argument array,
// stashes the out-pointer in the context temp area at ctx+0x10+count*4,
// copies the three pointed-to words into this call's vector slot at
// ctx+(count+2)*16, bumps the temp count at ctx+0xC, and calls one engine
// function (cdecl/3) with the handle, the time bits and the vector-slot
// pointer. Returns the engine answer in EAX.
export!(cdecl, rw_00bc6140(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        let rec = *args;
        let time = *args.add(1);
        let out = *args.add(2) as *mut u32;
        let count = *ctx.add(3);
        *ctx.add(4).add(count as usize) = out as u32;
        let vec = (ctx as *mut u8)
            .add((count.wrapping_add(2) as usize).wrapping_mul(16))
            as *mut u32;
        *vec = *out;
        *vec.add(1) = *out.add(1);
        *vec.add(2) = *out.add(2);
        *ctx.add(3) = count.wrapping_add(1);
        callee_cdecl!(1, u32, rec, time, vec as u32)
    }
});
