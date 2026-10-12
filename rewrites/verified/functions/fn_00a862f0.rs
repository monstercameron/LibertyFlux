// original: 0x00A862F0 CComplexSlideIntoCoverInfo::vf4

/// Write the slide-into-cover task's descriptor when the callback's low byte is nonzero, using wrapping word addition.
///
/// `this` is the task-info object and `stream` is the serializer context.
/// The branch follows the original field or callback result, and each
/// delegated value keeps its observed width and argument order.
lf_checker_rt::export!(thiscall, rw_00a862f0(this: u32, stream: u32) -> u32 {
    unsafe {
        let _builder_setup = lf_checker_rt::callee_thiscall!(1, u32, this, stream);
        let state = lf_checker_rt::callee_thiscall!(2, u32, this);
        let descriptor = stream.wrapping_add(4);
        let descriptor_value = unsafe { (descriptor as *const u32).read_unaligned() };
        let status = lf_checker_rt::callee_thiscall!(3, u32, descriptor, 5u32);
        if (status & 0xff) != 0 {
            let second_word = unsafe { (stream.wrapping_add(8) as *const u32).read_unaligned() };
            let fourth_word = unsafe { (stream.wrapping_add(0x10) as *const u32).read_unaligned() };
            let sum = fourth_word.wrapping_add(second_word);
            let _descriptor_write = lf_checker_rt::callee_cdecl!(4, u32, descriptor_value,
                state, 5u32, sum);
            lf_checker_rt::callee_thiscall!(5, u32, descriptor, 5u32)
        } else {
            status
        }
    }
});
