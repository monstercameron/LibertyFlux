// original: 0x006f4950 timer_stamp_and_latch
/// Stamps the current tick into slot 0 and latches slot 2 into slot 3.
///
/// Samples the shared tick source (fast function-pointer path with a
/// fallback), stores the sample with its low bit forced at slot 0, copies
/// slot 2 to slot 3, and returns the latched value.
export!(thiscall, rw_006f4950(this: u32) -> u32 {
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
        *obj = tick | 1;
        let latched = *obj.add(2);
        *obj.add(3) = latched;
        latched
    }
});
