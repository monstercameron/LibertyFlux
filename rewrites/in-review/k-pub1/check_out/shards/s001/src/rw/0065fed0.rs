// original: 0x0065fed0 rage::rlFireAndForgetTask<rage::snJoinCompleteTask>::vf3
/// Count down the join-complete barrier, then complete with `(1, 0)`.
///
/// Subtracts the argument from the pending count at `+0x14` when it is
/// positive; a count that reaches exactly zero becomes -1 (all done).
/// Always completes through virtual slot 7 with `(1, 0)` and returns
/// that call's answer.
export!(thiscall, rw_0065fed0(this: u32, amount: u32) -> u32 {
    unsafe {
        let count = ((this + 0x14) as *const u32).read() as i32;
        if count > 0 {
            let rest = (count as u32).wrapping_sub(amount);
            ((this + 0x14) as *mut u32).write(if rest == 0 { 0xffff_ffff } else { rest });
        }
        task_complete(this, 1, 0)
    }
});

/// Call the task's virtual slot 7 (`[vtable+0x1c]`) with two words.
///
/// Every task object in this family carries its vtable pointer at offset 0 and
/// reports completion through slot 7, a thiscall taking the task in ECX. Both
/// sides of the check land on the same planted recorder stub.
#[inline(always)]
unsafe fn task_complete(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let vtable = *(this as *const u32);
        let slot = ((vtable + 0x1c) as *const u32).read();
        let target: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        target(this, arg0, arg1)
    }
}
