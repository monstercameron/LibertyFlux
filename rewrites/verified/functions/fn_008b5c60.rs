// original: 0x008b5c60 handle_forward
/// Handle-table call forwarder.
///
/// Pushes the handle stored at `index` in the shared handle table and tail
/// forwarder, returning the callee's answer.
export!(cdecl, rw_008b5c60(index: u32) -> u32 {
    unsafe {
        const HANDLES: u32 = 0x01160C0C;
        let addr = relocated(HANDLES).wrapping_add(index.wrapping_mul(4));
        let handle = (addr as *const u32).read();
        callee_cdecl!(2, u32, handle)
    }
});
