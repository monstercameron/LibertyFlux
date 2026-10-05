// original: 0x0069C930 rage::crAnimChannelCurveFloat::serialize_keys

/// Serializes one curve-float key segment through a stream.
///
/// `this` is the segment (count byte at `+2`, word base at `+4`) and the
/// stack argument the stream (`{flag+0, object+4}`). Each transfer goes to
/// the loader (callee 1, thiscall: stream object, pointer, size) when
/// `flag & 1` is set and to the storer (callee 2, same shape) otherwise.
/// The header (2 bytes at `+0`) and the count (1 byte at `+2`) transfer
/// first; on load the loader's words land through the declared out-write
/// (the proof pins the count to 0..=2 through them), then `(count + 1)`
/// words of storage are allocated through the thread allocator reached
/// as `tls[0] -> [+8] -> vtable[+8]` (callee 3, thiscall: allocator,
/// bytes, `0x10`, `0`; the multiply-overflow guard cannot trigger for a
/// byte count) and stored as the new word base. Finally `count + 1` words
/// transfer one by one (the loop's signed `jle` compares a non-negative
/// counter against a byte, so signedness is unobservable). Returns the
/// key count, reloaded into `eax` after the last word (as in copy_segment).
///
/// Original: 0x0069C930 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_0069C930(this: u32, src: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const ALLOC_OBJ_OFF: u32 = 8;
        const ALLOC_SLOT: u32 = 8;
        const ALLOC_HINT: u32 = 0x10;
        const STREAM_OBJ_OFF: u32 = 4;
        const COUNT_OFF: u32 = 2;
        const BASE_OFF: u32 = 4;
        const LOAD: u32 = 1;
        const STORE: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let stream = src;
        let flag = unsafe { (stream as *const u8).read() };
        let sobj = rd32(stream + STREAM_OBJ_OFF);
        let load = (flag & 1) != 0;
        if load {
            let _ = lf_checker_rt::callee_thiscall!(LOAD, u32, sobj, this, 2);
        } else {
            let _ = lf_checker_rt::callee_thiscall!(STORE, u32, sobj, this, 2);
        }
        if load {
            let _ = lf_checker_rt::callee_thiscall!(LOAD, u32, sobj, this + COUNT_OFF, 1);
        } else {
            let _ = lf_checker_rt::callee_thiscall!(STORE, u32, sobj, this + COUNT_OFF, 1);
        }
        if load {
        let tls_base = lf_checker_rt::tls_slot(TLS_SLOT);
        let heap_obj = rd32(tls_base + ALLOC_OBJ_OFF);
        let vtable = rd32(heap_obj);
            let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(rd32(vtable + ALLOC_SLOT) as usize) };
            let n = unsafe { ((this + COUNT_OFF) as *const u8).read() as u32 };
            let block = alloc(heap_obj, n.wrapping_add(1).wrapping_mul(4), ALLOC_HINT, 0);
            wr32(this + BASE_OFF, block);
        }
        let mut i: u32 = 0;
        loop {
            let base = rd32(this + BASE_OFF);
            let e = base.wrapping_add(i.wrapping_mul(4));
            if load {
                let _ = lf_checker_rt::callee_thiscall!(LOAD, u32, sobj, e, 4);
            } else {
                let _ = lf_checker_rt::callee_thiscall!(STORE, u32, sobj, e, 4);
            }
            let n = unsafe { ((this + COUNT_OFF) as *const u8).read() as u32 };
            i += 1;
            if i > n {
                break;
            }
        }
        unsafe { ((this + COUNT_OFF) as *const u8).read() as u32 }
    }
});
