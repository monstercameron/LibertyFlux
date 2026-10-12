// original: 0x00694E10 rage::crFrameFilterMover::construct (proposed)

/// Construct a frame-filter mover in caller-owned storage. The original
/// writes the relocated image address of its vtable at `+0`, clears the four
/// dwords at `+4..+0x14`, clears the 16-bit field at `+0x14`, sets byte
/// `+0x16` to one, and returns the original object pointer in EAX. It is a
/// thiscall constructor with no stack arguments and does not call helpers.
lf_checker_rt::export!(thiscall, rw_00694E10(this: u32) -> u32 {
    unsafe {
        const VTABLE_FILE_VA: u32 = 0x00FE3964;

        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        #[inline(always)]
        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        unsafe { write_u32(this, lf_checker_rt::relocated(VTABLE_FILE_VA)) };
        for offset in [0x04u32, 0x08, 0x0C, 0x10] {
            unsafe { write_u32(this + offset, 0) };
        }
        unsafe {
            write_u16(this + 0x14, 0);
            write_u8(this + 0x16, 1);
        }
        this
    }
});
