// original: 0x00CF09A0 CBike::vf85
/// Returns one when any enabled bike condition is active.
///
/// The method reads the flag byte at `0x1310`, the current speed at
/// `0x1078`, and the speed limit at `0x1404`. A nonzero marker at `0x0f50`
/// enables a second ordered comparison through the pointer at `0x20`, using
/// the nested float at offset `0x28`. NaN values follow the original ordered
/// comparison rules: they do not satisfy either greater-than test, while a
/// NaN current speed is different from zero. The result is returned in AL.
lf_checker_rt::export!(thiscall, rw_00cf09a0(this: *mut u8) -> u8 {
    const FLAG_BITS: usize = 0x1310;
    const SPEED_VALUE: usize = 0x1078;
    const SPEED_LIMIT: usize = 0x1404;
    const ACTIVE_MARKER: usize = 0x0F50;
    const POINTER_SLOT: usize = 0x20;
    const NESTED_LIMIT: usize = 0x28;
    const LOWER_LIMIT: u32 = 0x00FE8D7C;
    const ZERO_VALUE: u32 = 0x00FE8628;
    const UPPER_LIMIT: u32 = 0x00EDDB38;

    let flags = unsafe { this.add(FLAG_BITS).read() };
    let mut active = flags & 0x08 != 0;
    if !active {
        let current_speed = unsafe {
            core::ptr::read_unaligned(this.add(SPEED_VALUE).cast::<f32>())
        };
        let zero_value = unsafe {
            core::ptr::read_unaligned(lf_checker_rt::global::<f32>(ZERO_VALUE))
        };
        active = current_speed != zero_value;
    }

    let marker = unsafe {
        core::ptr::read_unaligned(this.add(ACTIVE_MARKER).cast::<u32>())
    };
    if marker != 0 {
        let nested = unsafe {
            core::ptr::read_unaligned(this.add(POINTER_SLOT).cast::<*const u8>())
        };
        let nested_speed = unsafe {
            core::ptr::read_unaligned(nested.add(NESTED_LIMIT).cast::<f32>())
        };
        let lower_limit = unsafe {
            core::ptr::read_unaligned(lf_checker_rt::global::<f32>(LOWER_LIMIT))
        };
        active |= lower_limit > nested_speed;
    }

    if flags & 0x10 != 0 {
        let speed_limit = unsafe {
            core::ptr::read_unaligned(this.add(SPEED_LIMIT).cast::<f32>())
        };
        let upper_limit = unsafe {
            core::ptr::read_unaligned(lf_checker_rt::global::<f32>(UPPER_LIMIT))
        };
        active |= speed_limit > upper_limit;
    }

    u8::from(active)
});
