// original: 0x00A757E0 update_task_pool_marker

/// Updates a global marker and, when the helper approves, marks the supplied
/// object at byte offset `0xe61`.
///
/// A null object clears the global byte and returns. Otherwise the task
/// helper is called with the object as its single cdecl argument. If its low
/// return byte is nonzero, the object byte is clear, and the global byte is
/// zero, the global byte is set to one and the object byte is set to one.
/// EAX has no semantic return value.
lf_checker_rt::export!(cdecl, rw_00a757e0(object: u32) -> u32 {
    unsafe {
        const CHECK_OBJECT: u32 = 1;
        const MARKER_VA: u32 = 0x12FA_6E8;
        const OBJECT_MARKER_OFFSET: u32 = 0xE61;

        let marker = lf_checker_rt::global::<u8>(MARKER_VA);
        if object == 0 {
            marker.write(0);
            return 0;
        }
        let allowed = lf_checker_rt::callee_cdecl!(CHECK_OBJECT, u32, object) as u8;
        if allowed != 0
            && ((object + OBJECT_MARKER_OFFSET) as *const u8).read() == 0
            && marker.read() == 0
        {
            marker.write(1);
            ((object + OBJECT_MARKER_OFFSET) as *mut u8).write(1);
        }
        0
    }
});
