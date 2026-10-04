// original: 0x00b93ed0 ATAN
/// Script native `ATAN` (hash 0x7FFE0A12).
///
/// Forwards one script argument (a float angle, copied as raw bits)
/// to the engine and stores its x87 floating-point answer into the
/// return slot.
export!(cdecl, rw_00b93ed0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer: f32 = callee_cdecl!(1, f32, *args);
        let slot = *(ctx as *const u32) as *mut f32;
        *slot = answer;
        slot as u32
    }
});
