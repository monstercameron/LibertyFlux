// original: 0x00b05850 refresh_and_clear_flag

/// Runs the refresh worker on this object, then clears the flag byte.
export!(thiscall, rw_00b05850(obj: *mut u8) -> u32 {
    unsafe {
        let answer = callee_thiscall!(1, u32, obj as u32);
        *obj.add(0x0c) = 0;
        answer
    }
});
