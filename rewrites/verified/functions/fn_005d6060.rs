// original: 0x005d6060 html_data_node_construct (proposed)

/// Construct an HTML data node copying a caller string.
///
/// Runs the base constructor callee on the object with the caller's string
/// id, stamps the data vtable, folds the string-table adjustment callee's
/// answer into the text pointer at `+0xd8` when it is non-null, then
/// measures the text length, allocates `(length + 1) * 2` bytes through the
/// thread allocator (TLS slot 0: allocator at `+8`, vtable at `+0`, allocate
/// at vtable `+8`, called with the allocator in ECX and size, `0x10`, `0`
/// on the stack), stores the buffer at `+0xdc`, and copies the text into it
/// with the copy callee when the allocation succeeded. Returns the object.
/// (The original's second length loop runs only for a negative length,
/// which an in-bounds string never yields.)
///
/// Original: 0x005d6060 (thiscall, one stack word: the string id).
lf_checker_rt::export!(thiscall, rw_005d6060(this: u32, arg: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00FE0B60;
        const TEXT_OFF: u32 = 0xd8;
        const BUFFER_OFF: u32 = 0xdc;
        const BASE_CALLEE: u32 = 1;
        const ADJUST_CALLEE: u32 = 2;
        const COPY_CALLEE: u32 = 6;
        const ALLOC_ALIGN: u32 = 0x10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, this, arg);
        wr32(this, lf_checker_rt::relocated(VTABLE));
        let text = rd32(this + TEXT_OFF);
        if text != 0 {
            let delta: u32 = lf_checker_rt::callee_thiscall!(ADJUST_CALLEE, u32, arg, text);
            wr32(this + TEXT_OFF, text.wrapping_add(delta));
        }
        let text = rd32(this + TEXT_OFF);
        wr32(this + BUFFER_OFF, 0);
        let mut end = text;
        while rd8(end) != 0 {
            end = end.wrapping_add(1);
        }
        let length = end.wrapping_sub(text);
        let size = length.wrapping_add(1).wrapping_mul(2);
        let thread = lf_checker_rt::tls_slot(0);
        let allocator = rd32(thread + 8);
        let vtable = rd32(allocator);
        let alloc_fn: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + 8) as usize);
        let buffer = alloc_fn(allocator, size, ALLOC_ALIGN, 0);
        wr32(this + BUFFER_OFF, buffer);
        if buffer != 0 {
            lf_checker_rt::callee_cdecl!(COPY_CALLEE, u32, text, buffer);
        }
        this
    }
});
