// original: 0x005E7040 CELL_CAM_IS_CHAR_VISIBLE_NO_FACE_CHECK
//
// Script native handler: passes the script character handle to one engine
// function in ECX (thiscall/0, no stack arguments) and stores the low byte
// of its answer (movzx from AL) into the return slot addressed by ctx+0.
// Ahead of the call the original zeroes the low byte of EDX (`(an instruction of the original)`);
// EDX is not part of the thiscall call record, so that write is
// unobservable under interception. Leaves the return-slot pointer in EAX.
export!(cdecl, rw_005e7040(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        let engine: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let answer: u32 = engine(*args);
        let ret_slot = *ctx as *mut u32;
        *ret_slot = answer & 0xFF;
        *ctx
    }
});
