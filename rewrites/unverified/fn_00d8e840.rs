// original: 0x00D8E840 audio_voice_release_all (proposed)

/// Release every live voice handle held by this audio object, then free its
/// mixer buffer.
///
/// `this` holds eight nullable handle words (at +0x58, +0x5c, +0x6c, +0x60,
/// +0x64, +0x70, +0x74, and the buffer at +0x90). Each nonzero handle is
/// released through the voice-table slot: the table base comes from thread
/// local storage (slot selected by a global index), the object at
/// `table + 0x10` provides the callee, and after the call the handle word is
/// cleared. The +0x70 handle additionally runs a pre-release call first
/// (callee 2). The +0x90 word, when nonzero, is passed to the heap-free call
/// (callee 3, cdecl) and cleared. The return value is the heap-free answer
/// when that call fires, else the (zero) buffer word the trailing load
/// fetches; earlier answers never survive.
///
/// Original: 0x00D8E840 (thiscall, no stack arguments; seven indirect
/// release sites sharing one vtable slot, two direct callees).
lf_checker_rt::export!(thiscall, rw_00d8e840(this: u32) -> u32 {
    unsafe {
        const TLS_INDEX_ADDR: u32 = 0x017aba14;
        const INNER_OFF: u32 = 0x10;
        const RELEASE_SLOT: u32 = 0x0c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn release(obj: u32, handle: u32) -> u32 {
            unsafe {
                let inner = rd32(obj.wrapping_add(INNER_OFF));
                let target = rd32(rd32(inner).wrapping_add(RELEASE_SLOT));
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                f(inner, handle)
            }
        }

        // Entry accumulator; the contract fixes it to 0 (see doc comment).
        let index = rd32(lf_checker_rt::relocated(TLS_INDEX_ADDR));
        let obj = lf_checker_rt::tls_slot(index as usize);

        let h = rd32(this + 0x58);
        if h != 0 {
            release(obj, h);
            wr32(this + 0x58, 0);
        }
        let h = rd32(this + 0x5c);
        if h != 0 {
            release(obj, h);
            wr32(this + 0x5c, 0);
        }
        let h = rd32(this + 0x6c);
        if h != 0 {
            release(obj, h);
            wr32(this + 0x6c, 0);
        }
        let h = rd32(this + 0x60);
        if h != 0 {
            release(obj, h);
            wr32(this + 0x60, 0);
        }
        let h = rd32(this + 0x64);
        if h != 0 {
            release(obj, h);
            wr32(this + 0x64, 0);
        }
        let h = rd32(this + 0x70);
        if h != 0 {
            ans = lf_checker_rt::callee_thiscall!(2, u32, h);
            release(obj, h);
            wr32(this + 0x70, 0);
        }
        let h = rd32(this + 0x74);
        if h != 0 {
            release(obj, h);
            wr32(this + 0x74, 0);
        }
        let h = rd32(this + 0x90);
        // The trailing load sets the return from this word on every path;
        // the entry accumulator never survives (any entry value works).
        let mut ans = h;
        if h != 0 {
            ans = lf_checker_rt::callee_cdecl!(3, u32, h);
            wr32(this + 0x90, 0);
        }
        ans
    }
});
