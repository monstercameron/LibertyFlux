// original: 0x00C1D200 CPlane::vf29

/// Prepare the plane's streaming request state. The incoming ECX object is
/// passed to the setup collaborator, then the request collaborator receives
/// the object's resource slot at `+0x20` advanced by `0x30`, the object, two
/// zero options, and a trailing zero word. A nonzero status word
/// at `+0x1ec8` is cleared. The method returns one in AL and takes no stack
/// arguments.
lf_checker_rt::export!(thiscall, rw_00c1d200(this: u32) -> u32 {
    unsafe {
        const RESOURCE_SLOT: u32 = 0x20;
        const RESOURCE_OFFSET: u32 = 0x30;
        const STATUS_WORD: u32 = 0x1ec8;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);

        let resource_base = read_u32(this.wrapping_add(RESOURCE_SLOT));
        let _ = lf_checker_rt::callee_cdecl!(
            2,
            u32,
            0x0d,
            resource_base.wrapping_add(RESOURCE_OFFSET),
            this,
            0,
            0,
            0,
        );

        let status_address = this.wrapping_add(STATUS_WORD);
        if read_u32(status_address) != 0 {
            write_u32(status_address, 0);
        }
        1
    }
});
