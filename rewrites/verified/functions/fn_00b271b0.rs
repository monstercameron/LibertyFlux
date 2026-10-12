// original: 0x00B271B0 clone_ped_blend_component

/// Clone a blend component only when the source passes the readiness check and
/// allocation returns a destination. Copy the source's top flag bit, type
/// halfword and state word while preserving other destination bits. Clear the
/// source's 0x20 flag bit and preserve the seven outgoing helper calls in order.
/// The source is one cdecl pointer argument; zero is returned when either
/// readiness or allocation fails.
const BLEND_FLAGS_OFFSET: u32 = 0x24;
const BLEND_TYPE_OFFSET: u32 = 0x42;
const BLEND_STATE_OFFSET: u32 = 0x50;
const PRESERVE_SOURCE_MASK: u32 = !0x20;

lf_checker_rt::export!(cdecl, rw_b271b0(source: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value); } }
        #[inline(always)]
        unsafe fn read_halfword(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)]
        unsafe fn write_halfword(address: u32, value: u16) { unsafe { (address as *mut u16).write_unaligned(value); } }
        let initial_source_flags = read_word(source.wrapping_add(BLEND_FLAGS_OFFSET));
        if lf_checker_rt::callee_cdecl!(1, u32, source) as u8 == 0 { return 0; }
        let destination = lf_checker_rt::callee_thiscall!(2, u32, source);
        if destination == 0 { return 0; }
        let destination_flags = read_word(destination.wrapping_add(BLEND_FLAGS_OFFSET));
        let source_top_bit = initial_source_flags >> 31;
        write_word(destination.wrapping_add(BLEND_FLAGS_OFFSET),
                   (destination_flags & 0x7fff_ffff) | (source_top_bit << 31));
        write_halfword(destination.wrapping_add(BLEND_TYPE_OFFSET),
                       read_halfword(source.wrapping_add(BLEND_TYPE_OFFSET)));
        let _ = lf_checker_rt::callee_thiscall!(3, u32, destination, source, 1);
        let _ = lf_checker_rt::callee_cdecl!(4, u32, source, 0);
        let current_source_flags = read_word(source.wrapping_add(BLEND_FLAGS_OFFSET));
        write_word(source.wrapping_add(BLEND_FLAGS_OFFSET), current_source_flags & PRESERVE_SOURCE_MASK);
        let _ = lf_checker_rt::callee_thiscall!(5, u32, source);
        let _ = lf_checker_rt::callee_thiscall!(6, u32, destination, source);
        write_word(destination.wrapping_add(BLEND_STATE_OFFSET),
                   read_word(source.wrapping_add(BLEND_STATE_OFFSET)));
        let _ = lf_checker_rt::callee_cdecl!(7, u32, destination, 0);
        destination
    }
});
