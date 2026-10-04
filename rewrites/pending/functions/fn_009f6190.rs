// original: 0x009f6190 cooldown_gate
/// Cooldown gate: if enabled and the value is positive, look up the cap for
/// the code (float table below 0xfd, int table above), clamp the overage at
/// zero, report it, and always run the follow-up. Returns the last answer.
export!(cdecl, rw_009f6190(code: u32, val: f32) -> u32 {
    unsafe {
        let ok = callee_cdecl!(1, u32, code);
        if ok as u8 == 0 {
            return ok;
        }
        if !(val > 0.0) {
            return ok;
        }
        if code > 0xfc && code.wrapping_sub(0xfd) > 0x18b {
            return callee_cdecl!(3, u32, code);
        }
        let cap: f32 = if code <= 0xfc {
            global::<f32>(0x012B_75B0).add(code as usize).read()
        } else {
            global::<i32>(0x012B_75D4).add(code as usize).read() as f32
        };
        let over = cap - val;
        let clamped = if over > 0.0 { over } else { 0.0 };
        callee_cdecl!(2, u32, code, clamped.to_bits());
        callee_cdecl!(3, u32, code)
    }
});
