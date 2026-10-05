// original: 0x008845a0 stream_clamp_limit (proposed)
/// Clamp a requested limit into a streaming object at `+0x10`.
///
/// The ceiling is eight times the table size found through the manager
/// (`manager` at `this+0x20`, size at `manager+0x80`). A request above the
/// ceiling stores the ceiling; anything at or below it stores the request
/// raised to a floor of 1,000 (`MIN_LIMIT`).
///
/// No return value is produced.
///
/// Original: thiscall, one stack argument, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_008845a0(this: u32, request: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x20;
        const TABLE_SIZE: u32 = 0x80;
        const LIMIT: u32 = 0x10;
        const SCALE_SHIFT: u32 = 3;
        const MIN_LIMIT: u32 = 1000;
        let manager = ((this + MANAGER) as *const u32).read_unaligned();
        let ceiling = ((manager + TABLE_SIZE) as *const u32).read_unaligned() << SCALE_SHIFT;
        let clamped = if request > ceiling {
            ceiling
        } else if request < MIN_LIMIT {
            MIN_LIMIT
        } else {
            request
        };
        ((this + LIMIT) as *mut u32).write_unaligned(clamped);
        0
    }
});
