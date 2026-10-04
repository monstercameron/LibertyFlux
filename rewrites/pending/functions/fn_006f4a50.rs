// original: 0x006f4a50 timer_elapsed_query
/// Returns the elapsed ticks since the base stamp, or zero when idle.
///
/// When slot 0 is zero or slot 3 is not positive the timer is idle and the
/// result is zero. Otherwise samples the shared tick source (fast
/// function-pointer path with a fallback) and returns sample-minus-base.
export!(thiscall, rw_006f4a50(this: u32) -> u32 {
    unsafe {
        let obj = this as *const u32;
        if *obj != 0 && (*obj.add(3) as i32) > 0 {
            let raw = *global::<u32>(0x17ACD20);
            let tick = if raw == 0 {
                let fallback: extern "cdecl" fn() -> u32 =
                    core::mem::transmute(*global::<u32>(0xE73474));
                fallback()
            } else {
                let prime: extern "cdecl" fn() -> u32 =
                    core::mem::transmute(raw);
                prime();
                let convert: extern "cdecl" fn(*mut u32, *mut u32) -> u32 =
                    core::mem::transmute(*global::<u32>(0x17ACD00));
                let mut words = [0u32; 3];
                convert(words.as_mut_ptr().add(1), words.as_mut_ptr());
                words[0]
            };
            (tick | 1).wrapping_sub(*obj)
        } else {
            0
        }
    }
});
