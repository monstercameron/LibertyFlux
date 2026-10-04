// original: 0x006f48e0 timer_elapsed_update
/// Refreshes the elapsed field while the run flag is set, and returns it.
///
/// When bit 0 of the byte at offset 8 is clear the object is untouched.
/// Otherwise samples the shared tick source (fast function-pointer path with
/// a fallback), stores the sample-minus-base difference at slot 1, and in all
/// cases returns the current slot 1.
export!(thiscall, rw_006f48e0(this: u32) -> u32 {
    unsafe {
        let obj = this as *mut u32;
        if *((this + 8) as *const u8) & 1 != 0 {
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
            *obj.add(1) = (tick | 1).wrapping_sub(*obj);
        }
        *obj.add(1)
    }
});
