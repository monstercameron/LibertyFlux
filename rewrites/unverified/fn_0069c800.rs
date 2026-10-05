// original: 0x0069C800 rage::crAnimChannelCurveFloat::serialize

/// Serializes a curve-float channel's header and keys through a stream.
///
/// `this` is the channel (key base at `+8`, count as unsigned word at
/// `+0xC`, capacity word at `+0xE`, header words at `+0x10`/`+0x14`) and
/// the stack argument the stream (`{flag+0, object+4}`). Each transfer
/// goes to the loader when `flag & 1` is set and to the storer otherwise
/// (both thiscall: stream object, pointer, size; separate stub ids for
/// the heap transfers and the frame-slot count transfer). The two header
/// words transfer first (heap pointers, compared), then the count through
/// the incoming stream-argument slot, reused as scratch: the pointer
/// argument is skipped and the slot snapshotted at the call, the stack
/// check is off, and on load the loader's word lands there and bounds
/// everything after (pinned to 0..=2). On load with a zero capacity word,
/// the capacity is set from the count and a fresh key array is allocated
/// (callee 6, stdcall: count, pinned live); a zero count stores null
/// instead. The count word is then stored, and each key is serialized in
/// turn (callee 5, thiscall: `base + i * 8`, stream; the loop's signed
/// `jl` compares a non-negative index against an unsigned word, so
/// signedness is unobservable). Returns the key count, reloaded into
/// `eax` after the last key (as in copy_segment and serialize_keys).
///
/// Original: 0x0069C800 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_0069C800(this: u32, src: u32) -> u32 {
    unsafe {
        const STREAM_OBJ_OFF: u32 = 4;
        const BASE_OFF: u32 = 8;
        const COUNT_OFF: u32 = 0x0C;
        const CAP_OFF: u32 = 0x0E;
        const HDR0_OFF: u32 = 0x10;
        const HDR1_OFF: u32 = 0x14;
        const KEY_SIZE: u32 = 8;
        const LOADF: u32 = 1;
        const STOREF: u32 = 2;
        const LOADH: u32 = 3;
        const STOREH: u32 = 4;
        const KEYS: u32 = 5;
        const AKEYS: u32 = 6;
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
        let load = (flag & 1) != 0;
        let sobj = rd32(stream + STREAM_OBJ_OFF);
        if load {
            let _ = lf_checker_rt::callee_thiscall!(LOADH, u32, sobj, this + HDR0_OFF, 4);
        } else {
            let _ = lf_checker_rt::callee_thiscall!(STOREH, u32, sobj, this + HDR0_OFF, 4);
        }
        if load {
            let _ = lf_checker_rt::callee_thiscall!(LOADH, u32, sobj, this + HDR1_OFF, 4);
        } else {
            let _ = lf_checker_rt::callee_thiscall!(STOREH, u32, sobj, this + HDR1_OFF, 4);
        }
        let mut cnt_slot = rd16(this + COUNT_OFF);
        if load {
            let _ = lf_checker_rt::callee_thiscall!(LOADF, u32, sobj,
                core::ptr::addr_of_mut!(cnt_slot) as u32, 2);
        } else {
            let _ = lf_checker_rt::callee_thiscall!(STOREF, u32, sobj,
                core::ptr::addr_of_mut!(cnt_slot) as u32, 2);
        }
        let count = cnt_slot & 0xFFFF;
        if load {
            if rd16(this + CAP_OFF) == 0 {
                unsafe { ((this + CAP_OFF) as *mut u16).write_unaligned(count as u16) };
                if count != 0 {
                    let block = lf_checker_rt::callee_stdcall!(AKEYS, u32, count);
                    wr32(this + BASE_OFF, block);
                } else {
                    wr32(this + BASE_OFF, 0);
                }
            }
            unsafe { ((this + COUNT_OFF) as *mut u16).write_unaligned(count as u16) };
        }
        if count != 0 {
            let mut i: u32 = 0;
            loop {
                let base = rd32(this + BASE_OFF);
                let _ = lf_checker_rt::callee_thiscall!(KEYS, u32,
                    base.wrapping_add(i.wrapping_mul(KEY_SIZE)), stream);
                i += 1;
                if !((i as i32) < ((cnt_slot & 0xFFFF) as i32)) {
                    break;
                }
            }
        }
        cnt_slot & 0xFFFF
    }
});
