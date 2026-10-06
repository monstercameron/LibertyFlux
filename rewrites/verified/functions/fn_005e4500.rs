// original: 0x005e4500 thread_object_allocator_init
/// Initialize a thread-owned record from the thread allocator.
///
/// Stores the size mark, allocates the header block and the 16-byte tag
/// buffer through the current thread's allocator (reached via TLS slot 0),
/// stamps the remaining header fields, then reduces every tag byte to 0x81
/// in two steps (set bit 7, then keep bits 7 and 0 and set bit 0). The three
/// stack arguments are popped but never read. Returns the object pointer.
export!(thiscall, rw_005E4500(obj: *mut u8, _a: u32, _b: u32, _c: u32) -> u32 {
    unsafe {
        *(obj.add(0x0C) as *mut u32) = THREAD_OBJ_MARK;
        let tls0 = tls_slot(0);
        let inner = *((tls0 + 8) as *const u32);
        let vt = *(inner as *const u32);
        let tgt = *((vt as *const u8).add(8) as *const u32);
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let head = alloc(inner, THREAD_OBJ_HEAD_SIZE, THREAD_OBJ_ALIGN, 0);
        *(obj as *mut u32) = head;
        let buf = alloc(inner, THREAD_OBJ_TAG_LEN, THREAD_OBJ_ALIGN, 0);
        *(obj.add(4) as *mut u32) = buf;
        *obj.add(0x18) = 1;
        *(obj.add(8) as *mut u32) = THREAD_OBJ_TAG_LEN;
        *(obj.add(0x10) as *mut u32) = 0xFFFF_FFFF;
        *(obj.add(0x14) as *mut u32) = 0;
        let mut i = 0u32;
        while i < THREAD_OBJ_TAG_LEN {
            let p = (buf + i) as *mut u8;
            *p |= 0x80;
            *p = (*p & 0x81) | 1;
            i += 1;
        }
        obj as u32
    }
});
