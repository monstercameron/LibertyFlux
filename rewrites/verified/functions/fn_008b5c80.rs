// original: 0x008b5c80 handle_notify_triple
/// Triple notification for one handle-table slot.
///
/// Sends the handle stored at `index` in the shared handle table through three
/// notification calls: a two-argument update carrying `arg`, a three-argument
/// refresh carrying `arg` and the constant 1, and a one-argument commit.
/// Returns the commit call's answer.
export!(cdecl, rw_008b5c80(index: u32, arg: u32) -> u32 {
    unsafe {
        const HANDLES: u32 = 0x01160C0C;
        let addr = relocated(HANDLES).wrapping_add(index.wrapping_mul(4));
        let handle = (addr as *const u32).read();
        let _: u32 = callee_cdecl!(2, u32, handle, arg);
        let _: u32 = callee_cdecl!(3, u32, handle, arg, 1);
        callee_cdecl!(4, u32, handle)
    }
});
