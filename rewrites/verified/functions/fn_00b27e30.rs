// original: 0x00B27E30 write_blend_motion_values
/// Optionally update two float words at receiver offsets +0x0c and +0x10,
/// then call the receiver's virtual hook with no explicit stack arguments.
/// The update is skipped only when a nested resource exists, its enabled byte
/// at +0x0e is nonzero, and the request byte is zero. The receiver stays in
/// ECX; this wrapper cleans its three incoming stack words on return.
lf_checker_rt::export!(thiscall, rw_b27e30(this: u32, first_value: u32, second_value: u32, request: u32) -> u32 {
    unsafe {
        const NESTED_STATE: u32 = 0x24;
        const RESOURCE_LINK: u32 = 0x6c;
        const ENABLED_BYTE: u32 = 0x0e;
        const FIRST_DESTINATION: u32 = 0x0c;
        const SECOND_DESTINATION: u32 = 0x10;
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        let nested = read_word(this.wrapping_add(NESTED_STATE));
        let enabled = if nested == 0 {
            false
        } else {
            let resource = read_word(nested.wrapping_add(RESOURCE_LINK));
            resource != 0 && (resource.wrapping_add(ENABLED_BYTE) as *const u8).read() != 0
        };
        if !(enabled && request as u8 == 0) {
            (this.wrapping_add(FIRST_DESTINATION) as *mut u32).write_unaligned(second_value);
            (this.wrapping_add(SECOND_DESTINATION) as *mut u32).write_unaligned(first_value);
            let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        }
        0
    }
});
