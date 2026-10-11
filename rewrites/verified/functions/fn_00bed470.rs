// original: 0x00BED470 initialize_task_visual_descriptor

/// Initialize a visual task descriptor from three stack arguments. The first
/// argument points to an input record whose dword at `+0x64` is copied to
/// `this + 8`; the global descriptor word is copied to `this + 4`, and the
/// type byte `0x12` is stored at `this`. The third argument is forwarded first
/// to the three-channel converter and the second to the four-channel converter,
/// both as one-argument thiscalls on `this`. The second helper's EAX becomes
/// the result. The original thiscall pops three stack arguments.
lf_checker_rt::export!(thiscall, rw_00BED470(this: u32, input: u32, four_channel_source: u32, three_channel_source: u32) -> u32 {
    const TYPE_OFFSET: u32 = 0;
    const GLOBAL_FIELD: u32 = 4;
    const INPUT_FIELD: u32 = 8;
    const INPUT_VALUE_OFFSET: u32 = 0x64;
    const GLOBAL_WORD_VA: u32 = 0x011735A4;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    unsafe {
        write_u8(this + TYPE_OFFSET, 0x12);
        write_u32(this + INPUT_FIELD, read_u32(input + INPUT_VALUE_OFFSET));
        let descriptor = lf_checker_rt::global::<u32>(GLOBAL_WORD_VA).read_unaligned();
        write_u32(this + GLOBAL_FIELD, descriptor);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, three_channel_source);
        lf_checker_rt::callee_thiscall!(2, u32, this, four_channel_source)
    }
});
