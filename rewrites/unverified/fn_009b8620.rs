// original: 0x009B8620 veh_refresh_cell_triple (proposed)

/// Refreshes one cell of the receiver then rebuilds its triple. Runs the cell updater on the cell at +0x10 with the argument, then calls the triple builder with pointers to the receiver's words at +0x20, +0x150 and +0x154. Returns the builder's answer.
///
/// Original: 0x009B8620 (thiscall).
lf_checker_rt::export!(thiscall, rw_009B8620(this: u32, arg: u32) -> u32 {

    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn write_byte(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_byte(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    unsafe {
        lf_checker_rt::callee_thiscall!(1, u32, this + 0x10, arg);
        lf_checker_rt::callee_cdecl!(2, u32, this + 0x20, this + 0x150, this + 0x154)
    }
});
