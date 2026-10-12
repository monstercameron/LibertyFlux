// original: 0x00884770 audio_stream_buffer_release

/// Release every child object owned by the streaming container, destroy its
/// detached payload when present, clear that payload pointer, and finally
/// forward destruction to the embedded base subobject. Child objects occupy
/// the eight pointer slots beginning at byte offset 0x420; their count is at
/// 0x44c. The detached payload is at offset zero and the base subobject is at
/// 0x45c. Each outgoing method is routed through the checker runtime.
lf_checker_rt::export!(thiscall, rw_00884770(this: u32) -> u32 {
    unsafe {
        const CHILD_POINTERS_OFFSET: u32 = 0x420;
        const CHILD_COUNT_OFFSET: u32 = 0x44c;
        const EMBEDDED_BASE_OFFSET: u32 = 0x45c;
        const POINTER_SIZE: u32 = 4;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let child_count = read_u32(this.wrapping_add(CHILD_COUNT_OFFSET));
        for child_index in 0..child_count {
            let child_pointer_address = this
                .wrapping_add(CHILD_POINTERS_OFFSET)
                .wrapping_add(child_index.wrapping_mul(POINTER_SIZE));
            let child = read_u32(child_pointer_address);
            let _ = lf_checker_rt::callee_thiscall!(1, u32, child);
        }

        let detached_payload = read_u32(this);
        if detached_payload != 0 {
            let _ = lf_checker_rt::callee_cdecl!(2, u32, detached_payload);
            write_u32(this, 0);
        }

        let embedded_base = this.wrapping_add(EMBEDDED_BASE_OFFSET);
        lf_checker_rt::callee_thiscall!(3, u32, embedded_base)
    }
});