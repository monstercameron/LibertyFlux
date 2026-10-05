// original: 0x0069D9A0 rage::crAnimChannelRleInt::serialize

/// Serializes an RLE-int channel's samples through a stream.
///
/// `this` is the channel (sample base at `+8`, count as unsigned word at
/// `+0xC`, stream tail at `+0x10`) and the stack argument the stream
/// (`{flag+0, object+4}`). Each transfer goes to the loader (callee 1,
/// thiscall: stream object, pointer, size) when `flag & 1` is set and to
/// the storer (callee 2, same shape) otherwise. The count transfers
/// first, through the original's own frame slot (the pointer argument is
/// skipped and the slot snapshotted at each call; on load the loader's
/// word lands there and bounds the loop, pinned to 0..=2); on load the
/// count is then offered to the sample store (callee 3, thiscall:
/// `[this+8]`, count). Each sample transfers through the incoming
/// stream-argument slot, reused as scratch (so the stack check is off;
/// the slot's contents are snapshotted at each call instead): on store
/// the slot is primed with the sample first, on load the transferred
/// word is stored back to the samples. The loop's signed `jl` compares
/// a non-negative index against an unsigned word, so signedness is
/// unobservable. A trailing call (callee 4, thiscall: `[this+0x10]`,
/// stream) finishes, and its answer is returned.
///
/// Original: 0x0069D9A0 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_0069D9A0(this: u32, src: u32) -> u32 {
    unsafe {
        const STREAM_OBJ_OFF: u32 = 4;
        const BASE_OFF: u32 = 8;
        const COUNT_OFF: u32 = 0x0C;
        const SYNC_OFF: u32 = 8;
        const TAIL_OFF: u32 = 0x10;
        const LOAD: u32 = 1;
        const STORE: u32 = 2;
        const SYNC: u32 = 3;
        const TAIL: u32 = 4;
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
        let mut cnt_slot = rd16(this + COUNT_OFF);
        if load {
            let _ = lf_checker_rt::callee_thiscall!(LOAD, u32, sobj,
                core::ptr::addr_of_mut!(cnt_slot) as u32, 2);
        } else {
            let _ = lf_checker_rt::callee_thiscall!(STORE, u32, sobj,
                core::ptr::addr_of_mut!(cnt_slot) as u32, 2);
        }
        let mut count = cnt_slot & 0xFFFF;
        if load {
            let _ = lf_checker_rt::callee_thiscall!(SYNC, u32, this + SYNC_OFF, count);
        }
        if count != 0 {
            let mut i: u32 = 0;
            loop {
                let mut slot: u32 = 0;
                if !load {
                    let base = rd32(this + BASE_OFF);
                    slot = rd32(base.wrapping_add(i.wrapping_mul(4)));
                }
                if load {
                    let _ = lf_checker_rt::callee_thiscall!(LOAD, u32, sobj,
                        core::ptr::addr_of_mut!(slot) as u32, 4);
                } else {
                    let _ = lf_checker_rt::callee_thiscall!(STORE, u32, sobj,
                        core::ptr::addr_of_mut!(slot) as u32, 4);
                }
                if load {
                    let base = rd32(this + BASE_OFF);
                    wr32(base.wrapping_add(i.wrapping_mul(4)), slot);
                }
                i += 1;
                count = cnt_slot & 0xFFFF;
                if !((i as i32) < (count as i32)) {
                    break;
                }
            }
        }
        lf_checker_rt::callee_thiscall!(TAIL, u32, this + TAIL_OFF, stream)
    }
});
