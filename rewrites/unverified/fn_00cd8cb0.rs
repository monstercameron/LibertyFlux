// original: 0x00CD8CB0 CTaskComplexFollowLeaderAnyMeans::vf18

/// Query the task kind through the child object's virtual method and choose
/// a follow-task request for recognized kinds. The task-kind comparison is
/// signed: values above 0x2D4 select the high-kind cases, while 0x2D4, 0xCA
/// and 0xCB share the common request. Kind 0x39A checks the target's flag at
/// byte offset 0x26C and chooses between request codes 0x2D4 and 0xCB. Other
/// kinds return zero without making the request.
///
/// The child pointer is stored at `this + 8`; its virtual table is at offset
/// zero and the task-kind query slot is at byte offset 0xC. The target pointer
/// is the single incoming stack argument. The request helper receives the
/// selected code followed by that target pointer.
///
/// Calling convention: thiscall with one 32-bit stack pointer argument. The
/// kind query is thiscall with no stack arguments; the request helper is
/// thiscall with two stack words.
lf_checker_rt::export!(thiscall, rw_00cd8cb0(this: u32, target: u32) -> u32 {
    const CHILD_POINTER: u32 = 0x08;
    const VTABLE_POINTER: u32 = 0;
    const TASK_KIND_SLOT: u32 = 0x0C;
    const TARGET_FLAGS: u32 = 0x26C;
    const HAS_TASK_FLAG: u8 = 0x04;
    const KIND_THRESHOLD: i32 = 0x2D4;
    const KIND_THRESHOLD_EQUAL: u32 = 0x2D4;
    const KIND_CLEANUP: u32 = 0x391;
    const KIND_NEAR: u32 = 0xCA;
    const KIND_FAR: u32 = 0xCB;
    const KIND_OVERRIDE: u32 = 0x39A;
    const REQUEST_COMMON: u32 = 0x391;
    const REQUEST_OVERRIDE: u32 = 0x516;
    const REQUEST_FLAGGED: u32 = 0x2D4;
    const REQUEST_UNFLAGGED: u32 = 0xCB;
    const REQUEST_TASK: u32 = 2;

    unsafe {
        let child = ((this + CHILD_POINTER) as *const u32).read_unaligned();
        let vtable = ((child + VTABLE_POINTER) as *const u32).read_unaligned();
        let query_address = ((vtable + TASK_KIND_SLOT) as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(query_address as usize);
        let kind = query(child);

        if (kind as i32) > KIND_THRESHOLD {
            if kind == KIND_CLEANUP {
                return lf_checker_rt::callee_thiscall!(
                    REQUEST_TASK, u32, this, REQUEST_OVERRIDE, target
                );
            }
            if kind == KIND_OVERRIDE {
                let flags = ((target + TARGET_FLAGS) as *const u8).read();
                let request = if flags & HAS_TASK_FLAG != 0 {
                    REQUEST_FLAGGED
                } else {
                    REQUEST_UNFLAGGED
                };
                return lf_checker_rt::callee_thiscall!(REQUEST_TASK, u32, this, request, target);
            }
            return 0;
        }

        if kind == KIND_THRESHOLD_EQUAL || kind == KIND_NEAR || kind == KIND_FAR {
            lf_checker_rt::callee_thiscall!(REQUEST_TASK, u32, this, REQUEST_COMMON, target)
        } else {
            0
        }
    }
});
