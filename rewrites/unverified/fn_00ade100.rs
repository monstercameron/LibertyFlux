// original: 0x00ade100 render_target_selector
/// Render-target selector: publish fallback-or-override pointers.
///
/// When bit 0x10 of the flag byte at `this + 0x8e8` is set, publishes three
/// pointers: the per-slot overrides at `this + 0x94c/0x950`, each falling back
/// to the global default, and a derived pointer chosen from the object at
/// `this + 0x948` (or the default when null), then notifies a helper
/// (cdecl/0, stubbed) and publishes its answer as the fourth pointer. When
/// the bit is clear, publishes the default four times. Finally selects the
/// frame counter from one of two globals according to bit 0x40 and stores it
/// back, returning the selected value.

export!(thiscall, rw_00ade100(this: u32) -> u32 {
    unsafe {
        let flags = *((this + 0x8e8) as *const u8);
        let tail: u32;
        if flags & 0x10 != 0 {
            let fallback = *global::<u32>(0x17ED954);
            let first = *((this + 0x94c) as *const u32);
            *global::<u32>(0x1550E90) = if first != 0 { first } else { fallback };
            let second = *((this + 0x950) as *const u32);
            *global::<u32>(0x1550E94) = if second != 0 { second } else { fallback };
            let third = *((this + 0x948) as *const u32);
            if third != 0 {
                if *global::<u32>(0x1550DF4) != 0 {
                    *global::<u32>(0x1550E98) = *((third + 0x8a0) as *const u32);
                } else {
                    *global::<u32>(0x1550E98) = *((third + 0x890) as *const u32);
                }
            } else {
                *global::<u32>(0x1550E98) = fallback;
            }
            tail = callee_cdecl!(1, u32,);
        } else {
            let fallback = *global::<u32>(0x17ED954);
            *global::<u32>(0x1550E90) = fallback;
            *global::<u32>(0x1550E94) = fallback;
            *global::<u32>(0x1550E98) = fallback;
            tail = fallback;
        }
        *global::<u32>(0x1550E9C) = tail;
        let counter = if flags & 0x40 != 0 {
            *global::<u32>(0x154E174)
        } else {
            *global::<u32>(0x166DA1C)
        };
        *global::<u32>(0x166DA1C) = counter;
        counter
    }
});
