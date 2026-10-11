// original: 0x00CD7800 task_notify_first_eligible_child

/// Scan three child slots under the supplied owner object. A null child or a
/// child whose state word at `+0x0c` has bit zero set is skipped. The first
/// remaining child receives the owner pointer, mode one, and a zero flag
/// through its virtual slot at vtable byte offset `0x14`. The slot's 32-bit
/// answer is returned; when its low byte is nonzero, bit one is set in the
/// child's state word. If no child qualifies, the function returns three.
///
/// `this` is unused by this implementation. The single stack argument is the
/// owner object; its child-list pointer is at `+0x224`, and the three child
/// pointers begin at `+0x70` in that list. The indirect virtual call is made
/// through the fabricated table entry used by the proof contract.
lf_checker_rt::export!(thiscall, rw_00cd7800(_this: u32, owner: u32) -> u32 {
    const OWNER_CHILD_LIST: u32 = 0x224;
    const CHILD_POINTERS: u32 = 0x70;
    const CHILD_COUNT: u32 = 3;
    const CHILD_STATE: u32 = 0x0c;
    const CHILD_ELIGIBLE: u8 = 1;
    const CHILD_NOTIFIED: u32 = 2;
    const NOTIFY_SLOT: u32 = 0x14;

    unsafe {
        let list = ((owner + OWNER_CHILD_LIST) as *const u32).read_unaligned();
        let mut index = 0;
        while index < CHILD_COUNT {
            let child = ((list + CHILD_POINTERS + index * 4) as *const u32).read_unaligned();
            if child != 0 {
                let state = ((child + CHILD_STATE) as *const u8).read();
                if state & CHILD_ELIGIBLE == 0 {
                    let vtable = (child as *const u32).read_unaligned();
                    let notify = ((vtable + NOTIFY_SLOT) as *const u32).read_unaligned();
                    let notify_fn: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(notify as usize);
                    let answer = notify_fn(child, owner, 1, 0);
                    if answer as u8 != 0 {
                        let state_address = child + CHILD_STATE;
                        let state_word = (state_address as *const u32).read_unaligned();
                        (state_address as *mut u32).write_unaligned(state_word | CHILD_NOTIFIED);
                    }
                    return answer;
                }
            }
            index += 1;
        }
    }
    CHILD_COUNT
});
