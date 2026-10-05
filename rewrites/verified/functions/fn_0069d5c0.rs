// original: 0x0069D5C0 rage::crAnimChannelRleInt::sample_indexed

/// Samples an RLE-compressed int channel at a fractional index into an output.
///
/// `this` is the channel object (sample base at `+8`, sample count as an
/// unsigned word at `+0xC`, bit stream pointer at `+0x10`, shift byte at
/// `+0x18`); the stack arguments are the integer `index`, the blend `t` as
/// `f32` bits and the output pointer. When `t` is at least the 0.5 threshold
/// (ordered float compare; NaN takes the below-threshold path), `index` is
/// incremented first. An empty channel (signed count `<= 0`) stores the word
/// before the sample base. Otherwise each sample's run length is scanned out
/// of the bit stream (skipping zero bits), extended by the bit decoder
/// (callee 1, thiscall: stream object, cursor slot, shift; the slot is the
/// dead `t` argument slot reused as scratch and is snapshotted at each call),
/// sign-flipped when the next bit is set, and subtracted from the target
/// (the original accumulates into its own incoming `index` slot, so the stack
/// check is off; the value is observed through the per-iteration sign exit
/// and the call counts). A negative target (signed) stores the current
/// sample; surviving all samples stores the last one. The sample-count
/// comparisons are signed (`jle`/`jl`) but both sides stay non-negative, so
/// signedness is unobservable there; the target exit is a genuine signed
/// `js`. No return value.
///
/// Original: 0x0069D5C0 (thiscall, three stack words, callee pops 12).
lf_checker_rt::export!(thiscall, rw_0069D5C0(this: u32, index: u32, tbits: u32, out: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x0C;
        const BASE_OFF: u32 = 8;
        const STREAM_OFF: u32 = 0x10;
        const SHIFT_OFF: u32 = 0x18;
        const THRESH_VA: u32 = 0xFE8830;
        const DECODE: u32 = 1;
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
        let thresh = f32::from_bits(rd32(lf_checker_rt::relocated(THRESH_VA)));
        let t = f32::from_bits(tbits);
        let mut target = index;
        if t >= thresh {
            target = target.wrapping_add(1);
        }
        let count = rd16(this + COUNT_OFF);
        let base = rd32(this + BASE_OFF);
        if (count as i32) <= 0 {
            wr32(out, rd32(base.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4)));
            return 0;
        }
        let bits = rd32(this + STREAM_OFF);
        let shift = unsafe { ((this + SHIFT_OFF) as *const u8).read() as u32 };
        let mut edx: u32 = 0;
        let mut ebx: u32 = 0;
        loop {
            let mut edi: u32 = 0;
            let w = rd32(bits.wrapping_add((edx >> 5).wrapping_mul(4)));
            let b = (w >> (edx & 31)) & 1;
            edx = edx.wrapping_add(1);
            if b == 0 {
                loop {
                    let w2 = rd32(bits.wrapping_add((edx >> 5).wrapping_mul(4)));
                    let b2 = (w2 >> (edx & 31)) & 1;
                    edi = edi.wrapping_add(1);
                    edx = edx.wrapping_add(1);
                    if b2 != 0 {
                        break;
                    }
                }
            }
            let mut slot = edx;
            let ans = lf_checker_rt::callee_thiscall!(DECODE, u32, this + STREAM_OFF,
                core::ptr::addr_of_mut!(slot) as u32, shift);
            edx = slot;
            let mut esi = ans | edi.wrapping_shl(shift & 31);
            if esi != 0 {
                let w3 = rd32(bits.wrapping_add((edx >> 5).wrapping_mul(4)));
                let b3 = (w3 >> (edx & 31)) & 1;
                edx = edx.wrapping_add(1);
                if b3 != 0 {
                    esi = 0u32.wrapping_sub(esi);
                }
            }
            target = target.wrapping_sub(esi);
            if (target as i32) < 0 {
                let base2 = rd32(this + BASE_OFF);
                wr32(out, rd32(base2.wrapping_add(ebx.wrapping_mul(4))));
                return 0;
            }
            ebx = ebx.wrapping_add(1);
            if !((ebx as i32) < (count as i32)) {
                break;
            }
        }
        wr32(out, rd32(base.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4)));
        0
    }
});
