// original: 0x00A3C8E0 vehicle_replace_timer_reference

/// When the receiver's elapsed time at `+0x08` is not below the shared
/// threshold, reset it to negative one, release the existing object stored
/// at `+0x0c`, replace that pointer with `replacement`, and acquire the new
/// object when it is non-null. A COMISS unordered result follows the early
/// return path, so NaN elapsed values leave both fields untouched.
///
/// `replacement` is the one stack argument. Release and acquire are distinct
/// thiscall callees, each receiving the field address as its stack argument;
/// their effects are represented by scripted call stand-ins.
lf_checker_rt::export!(thiscall, rw_00a3c8e0(this: u32, replacement: u32) -> () {
    const ELAPSED_TIME: u32 = 0x08;
    const OBJECT_LINK: u32 = 0x0c;
    const SHARED_THRESHOLD_VA: u32 = 0x00fe8628;
    const RELEASE_OBJECT: u32 = 1;
    const ACQUIRE_OBJECT: u32 = 2;
    const RESET_TIME_BITS: u32 = 0xbf800000;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let elapsed = f32::from_bits(read_u32(this.wrapping_add(ELAPSED_TIME)));
        let threshold_bits = unsafe { lf_checker_rt::global::<u32>(SHARED_THRESHOLD_VA).read_unaligned() };
        let threshold = f32::from_bits(threshold_bits);
        if elapsed.is_nan() || elapsed < threshold {
            return;
        }

        write_u32(this.wrapping_add(ELAPSED_TIME), RESET_TIME_BITS);
        let link_address = this.wrapping_add(OBJECT_LINK);
        let old_object = read_u32(link_address);
        if old_object != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_OBJECT, u32, old_object, link_address);
        }

        write_u32(link_address, replacement);
        if replacement != 0 {
            let _ = lf_checker_rt::callee_thiscall!(ACQUIRE_OBJECT, u32, replacement, link_address);
        }
    }
});
