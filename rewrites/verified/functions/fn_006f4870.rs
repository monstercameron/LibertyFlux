// original: 0x006f4870 timer_restart_stamp
/// Restarts the timer object and stamps the current tick into slot 0.
///
/// Runs the object's restart helper, samples the shared tick source (a fast
/// function-pointer path with a fallback path), stores the sample with its
/// low bit forced at slot 0, clears slot 1, and returns the stamped value.
export!(thiscall, rw_006f4870(this: u32) -> u32 {
    unsafe {
        callee_thiscall!(4, u32, this);
        let raw = *global::<u32>(0x17ACD20);
        let tick = if raw == 0 {
            let fallback: extern "cdecl" fn() -> u32 =
                core::mem::transmute(*global::<u32>(0xE73474));
            fallback()
        } else {
            let prime: extern "cdecl" fn() -> u32 = core::mem::transmute(raw);
            prime();
            let convert: extern "cdecl" fn(*mut u32, *mut u32) -> u32 =
                core::mem::transmute(*global::<u32>(0x17ACD00));
            let mut words = [0u32; 3];
            convert(words.as_mut_ptr().add(1), words.as_mut_ptr());
            words[0]
        };
        let stamped = tick | 1;
        let obj = this as *mut u32;
        *obj = stamped;
        *obj.add(1) = 0;
        stamped
    }
});
