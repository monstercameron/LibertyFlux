// original: 0x00879340 rage::crmtFrameBufferDynamic::initialize

/// Initialize a dynamic frame buffer and its embedded critical section.
///
/// Install the dynamic frame-buffer vtable, clear the first two state words,
/// initialize the critical section at `this + 0x0c`, then forward all three
/// caller arguments to the child-state initializer with this object in ECX.
/// Return the original object pointer.
///
/// Original calling convention: thiscall with three stack words.
lf_checker_rt::export!(thiscall, rw_00879340(this: u32, first: u32, second: u32, third: u32) -> u32 {
    const DYNAMIC_BUFFER_VTABLE: u32 = 0x00fe82f0;
    const CRITICAL_SECTION: u32 = 0x0c;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(DYNAMIC_BUFFER_VTABLE));
        (this.wrapping_add(4) as *mut u32).write_unaligned(0);
        (this.wrapping_add(8) as *mut u32).write_unaligned(0);
        let critical_section = this.wrapping_add(CRITICAL_SECTION);
        let _critical_section_result = lf_checker_rt::callee_stdcall!(2, u32, critical_section);
        let _child_state_result = lf_checker_rt::callee_thiscall!(1, u32, this, first, second, third);
    }
    this
});
