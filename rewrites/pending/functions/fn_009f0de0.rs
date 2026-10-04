// original: 0x009f0de0 ped_release_slot
/// Release the handle at field `0xd68` and clear the neighbouring slots.
///
/// When the handle is null there is nothing to do and the function returns
/// 0 (its `eax` on that path is whatever the caller left there; the
/// contract fixes entry `eax` to 0 since it is not an argument).
/// Otherwise it calls the handle's release entry with this object, zeroes
/// fields `0xd68`, `0xd78`, `0xd74` and `0xd70`, and returns the call's answer.
export!(thiscall, rw_009f0de0(this_ptr: u32) -> u32 {
    unsafe {
        let handle = *((this_ptr + 0xd68) as *const u32);
        if handle == 0 {
            return 0;
        }
        let answer: u32 = callee_thiscall!(2, u32, handle, this_ptr);
        *((this_ptr + 0xd68) as *mut u32) = 0;
        *((this_ptr + 0xd78) as *mut u32) = 0;
        *((this_ptr + 0xd74) as *mut u32) = 0;
        *((this_ptr + 0xd70) as *mut u32) = 0;
        answer
    }
});
