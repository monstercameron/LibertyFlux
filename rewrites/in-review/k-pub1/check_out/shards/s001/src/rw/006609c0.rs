// original: 0x006609c0 rage::snAddRemoteGamerTask::vf11
/// Look up a gamer id in the candidate table; tail-complete when settled.
///
/// Clears the matched flag (`+0x90`), then scans `count` 8-byte entries at
/// `table` for the id in `+0xe0`/`+0xe4`. On a miss returns the count (0
/// when the table is empty). On a hit sets the flag and, unless the mode
/// word at `+0xc` is 0, returns it; in mode 0 the completion is a tail
/// jump to virtual slot 7 with `(0, 0)`, which the rewrite performs as
/// the same call. Returns the completion answer on the tail path.
export!(thiscall, rw_006609c0(this: u32, table: u32, count: u32) -> u32 {
    unsafe {
        ((this + 0x90) as *mut u8).write(0);
        if count == 0 {
            return 0;
        }
        let want_lo = ((this + 0xe0) as *const u32).read();
        let want_hi = ((this + 0xe4) as *const u32).read();
        let mut index = 0u32;
        loop {
            let entry = table + index * 8;
            if (entry as *const u32).read() == want_lo
                && ((entry + 4) as *const u32).read() == want_hi
            {
                break;
            }
            index += 1;
            if index >= count {
                return count;
            }
        }
        let mode = ((this + 0xc) as *const u32).read();
        ((this + 0x90) as *mut u8).write(1);
        if mode == 0 {
            task_complete(this, 0, 0)
        } else {
            mode
        }
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
