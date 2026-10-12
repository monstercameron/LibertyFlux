// original: 0x00A5B020 timing_nodes_detach_both

/// Detaches each non-null node stored at object offsets +0x30 and +0x34 by
/// calling the unlink helper with the node and the address of its link field.
/// It then clears the low two bits of the word at +0x40 and zeros fields
/// +0x30, +0x34, +0x38 and +0x3c. The object pointer is in ECX; helper calls
/// are thiscall with one stack pointer argument. EAX is not a semantic result.
lf_checker_rt::export!(thiscall, rw_00A5B020(this: u32) -> u32 {
    const FIRST_LINK: u32 = 0x30;
    const SECOND_LINK: u32 = 0x34;
    const FLAGS: u32 = 0x40;
    const PRESERVED_FLAGS: u32 = !0x03;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    let first_node = unsafe { read_u32(this.wrapping_add(FIRST_LINK)) };
    if first_node != 0 {
        lf_checker_rt::callee_thiscall!(
            1, u32, first_node, this.wrapping_add(FIRST_LINK),
        );
    }
    let second_node = unsafe { read_u32(this.wrapping_add(SECOND_LINK)) };
    if second_node != 0 {
        lf_checker_rt::callee_thiscall!(
            2, u32, second_node, this.wrapping_add(SECOND_LINK),
        );
    }

    let old_flags = unsafe { read_u32(this.wrapping_add(FLAGS)) };
    unsafe {
        write_u32(this.wrapping_add(FLAGS), old_flags & PRESERVED_FLAGS);
        write_u32(this.wrapping_add(SECOND_LINK), 0);
        write_u32(this.wrapping_add(0x38), 0);
        write_u32(this.wrapping_add(0x3c), 0);
        write_u32(this.wrapping_add(FIRST_LINK), 0);
    }
    0
});
