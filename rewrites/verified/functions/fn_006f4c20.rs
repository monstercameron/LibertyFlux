// original: 0x006f4c20 timer_sync_clamp
/// Syncs the timer through the shared sink and decays the backlog.
///
/// Samples the shared tick source (fast function-pointer path with a
/// fallback), reports the stamped tick for this object, and when the sink
/// answers, reports the elapsed time clamped to 5000 for the slot-1 channel.
/// Then, unless the hold flag is set, subtracts slot 1 from the backlog at
/// slot 6 while it stays positive.
export!(thiscall, rw_006f4c20(this: u32) -> () {
    unsafe {
        let obj = this as *mut u32;
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
        let sink: extern "stdcall" fn(u32, u32) -> u32 =
            core::mem::transmute(*global::<u32>(0xE731F0));
        let answered = sink(this, stamped);
        if answered != 0 {
            let elapsed = stamped.wrapping_sub(answered).min(5000);
            sink(this.wrapping_add(4), elapsed);
        }
        if *((this + 0x20) as *const u8) & 1 == 0
            && (*obj.add(6) as i32) > 0
        {
            let drain = *obj.add(1);
            *obj.add(6) = (*obj.add(6)).wrapping_sub(drain);
        }
    }
});
