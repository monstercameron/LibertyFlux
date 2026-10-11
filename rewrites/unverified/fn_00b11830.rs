// original: 0x00B11830 CHeli::vf24

/// Ask the virtual resource lookup for a pointer, then fall back to the
/// object's field at offset 0x38. When both are null, forward `this` to the
/// original tail-called helper. Otherwise follow the resource links at +4 and
/// +0xc and return the final pointer plus 0x20. This is a no-argument thiscall.
lf_checker_rt::export!(thiscall, rw_00b11830(this: u32) -> u32 {
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let lookup_result = lf_checker_rt::callee_thiscall!(1, u32, this);
        let resource = if lookup_result != 0 {
            lookup_result
        } else {
            read_u32(this.wrapping_add(0x38))
        };
        if resource == 0 {
            return lf_checker_rt::callee_thiscall!(2, u32, this);
        }
        let first_link = read_u32(resource.wrapping_add(4));
        let second_link = read_u32(first_link.wrapping_add(0xC));
        second_link.wrapping_add(0x20)
    }
});
