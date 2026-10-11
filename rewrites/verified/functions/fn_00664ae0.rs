// original: 0x00664AE0 rage::snChangeAttributesTask::vf3

/// Updates a pending count and dispatches a state callback through the object's
/// function table.
///
/// The signed count at `this + 0x14` is decremented only when it is positive.
/// If the subtraction reaches zero, the stored value becomes `u32::MAX`. The
/// state word at `this + 0x49c` selects the callback at vtable byte offset
/// `0x1c`: state 3 calls it with `(1, 0)`, state 1 skips it and returns 1, and
/// every other state calls it with `(0, 0)`. The callback's return value is
/// returned for the two calling states. This is a thiscall method with one
/// stack argument; the callback is also thiscall with two stack arguments.
lf_checker_rt::export!(thiscall, rw_00664ae0(this: u32, decrement: u32) -> u32 {
    unsafe {
        const PENDING_COUNT: u32 = 0x14;
        const CALLBACK_STATE: u32 = 0x49c;
        const CALLBACK_SLOT: u32 = 0x1c;
        const FINISHED_SENTINEL: u32 = u32::MAX;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn invoke_callback(this: u32, first: u32, second: u32) -> u32 {
            let vtable = unsafe { read_word(this) };
            let target = unsafe { read_word(vtable.wrapping_add(CALLBACK_SLOT)) };
            let callback: extern "thiscall" fn(u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            callback(this, first, second)
        }

        let pending = unsafe { read_word(this.wrapping_add(PENDING_COUNT)) };
        if (pending as i32) > 0 {
            let remaining = pending.wrapping_sub(decrement);
            let stored = if remaining == 0 { FINISHED_SENTINEL } else { remaining };
            unsafe { write_word(this.wrapping_add(PENDING_COUNT), stored) };
        }

        let state = unsafe { read_word(this.wrapping_add(CALLBACK_STATE)) };
        if state == 3 {
            return unsafe { invoke_callback(this, 1, 0) };
        }
        if state == 1 {
            return state;
        }
        unsafe { invoke_callback(this, 0, 0) }
    }
});
