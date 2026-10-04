// original: 0x008b5720 handle_release_and_invalidate
/// Handle release with invalidation.
///
/// Invokes the release handler for the handle stored at `index` in the shared
/// handle table, then marks that slot invalid (0xffffffa6). Returns the
/// release handler's answer.
export!(cdecl, rw_008b5720(index: u32) -> u32 {
    unsafe {
        const HANDLES: u32 = 0x01160C0C;
        const INVALID: u32 = 0xFFFF_FFA6;
        let addr = relocated(HANDLES).wrapping_add(index.wrapping_mul(4));
        let handle = (addr as *const u32).read();
        let answer: u32 = callee_cdecl!(2, u32, handle);
        (addr as *mut u32).write(INVALID);
        answer
    }
});
