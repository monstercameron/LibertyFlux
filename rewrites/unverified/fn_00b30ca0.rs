// original: 0x00B30CA0 read_blend_position

/// Copy three position words to the caller's output. When the resource link is
/// present and the byte flag is clear, use the child's position at +0x30;
/// otherwise use the receiver's inline position at +0x10. The four-byte words
/// are copied verbatim so NaN payloads and signed zero are preserved. This is
/// a thiscall method with an output pointer as its stack argument.
const POSITION_FALLBACK: u32 = 0x10;
const RESOURCE_FLAG: u32 = 0x27;
const RESOURCE_LINK: u32 = 0x28;
const RESOURCE_CHILD: u32 = 0x20;
const CHILD_POSITION: u32 = 0x30;

lf_checker_rt::export!(thiscall, rw_b30ca0(this: u32, output: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 { (address as *const u32).read_unaligned() }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) { (address as *mut u32).write_unaligned(value); }
        let resource = read_word(this.wrapping_add(RESOURCE_LINK));
        let flag = (this.wrapping_add(RESOURCE_FLAG) as *const u8).read_unaligned();
        let source = if resource != 0 && flag == 0 {
            let child = read_word(resource.wrapping_add(RESOURCE_CHILD));
            if child != 0 { child.wrapping_add(CHILD_POSITION) } else { resource.wrapping_add(POSITION_FALLBACK) }
        } else { this.wrapping_add(POSITION_FALLBACK) };
        write_word(output, read_word(source));
        write_word(output.wrapping_add(4), read_word(source.wrapping_add(4)));
        write_word(output.wrapping_add(8), read_word(source.wrapping_add(8)));
        output
    }
});
