// original: 0x006f6480 timer_stamp_slot_24
/// Refreshes the slot at offset 0x24 with the current tick when slot 0x38 is set.
///
/// Does nothing when the dword at offset 0x38 is zero. Otherwise samples the
/// shared tick source (a fast function-pointer path with a fallback path) and
/// stores the raw sample at offset 0x24.
export!(thiscall, rw_006f6480(this: u32) -> () {
    unsafe {
        let obj = this as *mut u32;
        if *obj.add(0x38 / 4) == 0 {
            return;
        }
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
        *obj.add(0x24 / 4) = tick;
    }
});
