// original: 0x00897360 aud_environment_sound_refresh_cached_indices

/// Fill missing cached byte indices with two thiscall helpers when their
/// sentinel conditions apply, then call the object's update helper with the
/// second cdecl argument. Each helper's low byte is stored at `+0xEB` or
/// `+0xEC`; the final helper's full EAX result is returned.
export!(cdecl, rw_00897360(audio: u32, update_value: u32) -> u32 {
    unsafe {
        const FIRST_INDEX: u32 = 0xEB;
        const SECOND_INDEX: u32 = 0xEC;
        const FLAGS: u32 = 0xEE;
        const INDEX_NONE: u8 = 0xFF;
        const SECOND_INDEX_FLAG: u8 = 0x10;

        let first = (audio.wrapping_add(FIRST_INDEX) as *const u8).read();
        if first == INDEX_NONE {
            let value = lf_checker_rt::callee_thiscall!(1, u32, audio);
            (audio.wrapping_add(FIRST_INDEX) as *mut u8).write(value as u8);
        }

        let flags = (audio.wrapping_add(FLAGS) as *const u8).read();
        let second = (audio.wrapping_add(SECOND_INDEX) as *const u8).read();
        if flags & SECOND_INDEX_FLAG != 0 && second == INDEX_NONE {
            let value = lf_checker_rt::callee_thiscall!(2, u32, audio);
            (audio.wrapping_add(SECOND_INDEX) as *mut u8).write(value as u8);
        }

        lf_checker_rt::callee_thiscall!(3, u32, audio, update_value)
    }
});
