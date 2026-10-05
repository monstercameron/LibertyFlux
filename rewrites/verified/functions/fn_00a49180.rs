// original: 0x00a49180 vehicle_info_thunk
/// Tail-thunk: forward this model's info row to the shared routine.
///
/// Loads the vehicle-info row for `[this+0x2E]` into ECX and jumps to the
/// shared routine (thiscall, no stack arguments); the callee's answer is the
/// return value. The forwarded ECX is compared as a call register.
export!(thiscall, rw_00a49180(this: u32) -> u32 {
    unsafe {
        const MODEL_INDEX_OFF: u32 = 0x2e;
        const MODEL_TABLE: u32 = 0x01295cd8;
        let model =
            ((this.wrapping_add(MODEL_INDEX_OFF)) as *const i16).read_unaligned() as i32 as u32;
        let row = (relocated(MODEL_TABLE).wrapping_add(model.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        callee_fastcall!(1, u32, row, 0)
    }
});
