// original: 0x00d391d0 task_state_init_triple_copy
/// Triple-copy task-state initializer: copy three words from the source
/// triple, then stamp this variant's table and generation slots.
export!(thiscall, rw_00d391d0(this: u32, src: u32, gen: u32) -> u32 {
    callee_thiscall!(1, u32, this);
    unsafe {
        *(this as *mut u32) = relocated(0xee3824);
        *((this + 0x10) as *mut u32) = *(src as *const u32);
        *((this + 0x14) as *mut u32) = *((src + 4) as *const u32);
        *((this + 0x18) as *mut u32) = *((src + 8) as *const u32);
        *((this + 0x24) as *mut u32) = gen;
        *((this + 0x20) as *mut u32) = 0xffff_ffff;
        *((this + 0x28) as *mut u32) = 0;
    }
    this
});

/// Convert a file VA (as seen in disassembly, image base 0x400000) to the
/// relocated address in the worker's mapping.
#[inline(always)]
pub fn relocated(file_va: u32) -> u32 {
    file_va.wrapping_sub(0x400000).wrapping_add(xbase())
}

/// Relocated base of the original image.
#[inline(always)]
pub fn xbase() -> u32 {
    unsafe { CHECKER_XBASE }
}
