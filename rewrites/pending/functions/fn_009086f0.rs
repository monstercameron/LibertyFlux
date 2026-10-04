// original: 0x009086f0 NativeImpl_SET_BLIP_COORDINATES
/// Apply a coordinates vector to a handle-selected blip (SET_BLIP_COORDINATES).
///
/// Resolves `handle` to a blip id and continues only when the record exists
/// and `op` is exactly 2 (the original decrements its own incoming op slot;
/// that caller-frame write is not compared). Hands the id and the vector to
/// the engine position step. Returns the position step's answer on the full
/// path, else the lookup answer.
export!(cdecl, rw_009086F0(op: u32, handle: u32, coord: u32) -> u32 {
    unsafe {
        let a = callee_cdecl!(1, u32, handle) as i32;
        if a < 0 {
            return a as u32;
        }
        let entry = blip(a as u32);
        if entry.is_null() {
            return a as u32;
        }
        if op.wrapping_sub(2) != 0 {
            return a as u32;
        }
        callee_cdecl!(2, u32, a as u32, coord)
    }
});
