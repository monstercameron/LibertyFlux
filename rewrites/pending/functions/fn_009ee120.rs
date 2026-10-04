// original: 0x009ee120 ped_flag_state_set
/// Update the flag word at `0x270` and either return or chain onward.
///
/// The low byte of `flag` selects the mode. In nonzero mode bit `0x100` is
/// set; when bits `0x300` were already present the word is stored and
/// returned, otherwise the value is stored at `0x14c` too and control
/// tail-jumps onward with (0, 0). In zero mode the mirror image holds with
/// bit `0x400`, the already-set test on `0xc00`, and a (1, 0) onward call.
/// Both onward jumps run with the object pointer advanced by `0x3c0`.
/// The rewrite expresses both tail jumps as forwarding calls.
export!(thiscall, rw_009ee120(this_ptr: u32, flag: u32, value: u32) -> u32 {
    unsafe {
        let bits = *((this_ptr + 0x270) as *const u32);
        if (flag & 0xff) != 0 {
            if bits & 0x300 == 0 {
                let next = bits | 0x100;
                *((this_ptr + 0x270) as *mut u32) = next;
                *((this_ptr + 0x14c) as *mut u32) = value;
                callee_thiscall!(2, u32, this_ptr.wrapping_add(0x3c0), 0, 0)
            } else {
                let next = bits | 0x100;
                *((this_ptr + 0x270) as *mut u32) = next;
                next
            }
        } else if bits & 0xc00 != 0 {
            let next = bits | 0x400;
            *((this_ptr + 0x270) as *mut u32) = next;
            next
        } else {
            let next = bits | 0x400;
            *((this_ptr + 0x270) as *mut u32) = next;
            *((this_ptr + 0x14c) as *mut u32) = value;
            callee_thiscall!(2, u32, this_ptr.wrapping_add(0x3c0), 1, 0)
        }
    }
});
