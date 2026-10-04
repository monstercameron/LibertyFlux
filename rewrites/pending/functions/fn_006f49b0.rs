// original: 0x006f49b0 timer_accumulate
/// Folds elapsed time into the accumulator at slot 3 and re-stamps slot 0.
///
/// When the limit in slot 1 is positive and the measured elapsed time reaches
/// it, grows slot 3 by the elapsed amount with a signed-overflow guard. Then,
/// while slot 0 is set and slot 3 is positive, samples the shared tick source
/// (fast function-pointer path with a fallback), adds the base-minus-sample
/// drift to slot 3, and re-stamps slot 0.
export!(thiscall, rw_006f49b0(this: u32) -> () {
    unsafe {
        let obj = this as *mut u32;
        if *obj.add(1) > 0 {
            let elapsed = callee_thiscall!(4, u32, this);
            if elapsed >= *obj.add(1) && *obj != 0 {
                let extra = *obj.add(3);
                if (extra as i32) > 0 && (elapsed as i32) > 0 {
                    let grown = elapsed.wrapping_add(extra);
                    if (grown as i32) > (extra as i32) {
                        *obj.add(3) = grown;
                    }
                }
            }
        }
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
            let stamped = tick | 1;
            let prev = *obj;
            *obj.add(3) = (*obj.add(3)).wrapping_add(prev.wrapping_sub(stamped));
            *obj = stamped;
        }
    }
});
