// original: 0x00bc69f0 IS_CAR_SIREN_ON
//
// Script native handler: passes the script vehicle handle to one engine
// function and stores the low byte of its answer (movzx from AL) into the
// return slot addressed by ctx+0. Leaves the return-slot pointer in EAX.
export!(cdecl, rw_00bc69f0(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        let answer: u32 = callee_cdecl!(1, u32, *args);
        let ret_slot = *ctx as *mut u32;
        *ret_slot = answer & 0xFF;
        *ctx
    }
});
