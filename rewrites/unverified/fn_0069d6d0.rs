// original: 0x0069D6D0 rage::crAnimChannelRleInt::decode_at

/// Decodes the RLE sample covering a target position.
///
/// `this` is the channel object (sample base at `+8`, bit stream pointer
/// at `+0x10`, sample count as unsigned word at `+0xC`, shift byte at
/// `+0x18`) and the stack argument is the target position. An empty
/// channel (signed count `<= 0`) returns the word before the sample base.
/// Otherwise each sample's run length is scanned out of the bit stream
/// (skipping zero bits), extended by the bit decoder (callee 1, thiscall:
/// stream object, context slot, shift; the slot carries `this` and the
/// proof pins the decoder to preserve it), sign-flipped when the next bit
/// is set, and subtracted from the target (the original accumulates into
/// its own incoming stack slot, so the stack check is off; the value is
/// observed through the per-iteration sign exit and the call counts). A
/// negative target returns the current sample; surviving all samples
/// returns the last one.
///
/// Original: 0x0069D6D0 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_0069D6D0(this: u32, target: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x0C;
        const BASE_OFF: u32 = 8;
        const STREAM_OFF: u32 = 0x10;
        const SHIFT_OFF: u32 = 0x18;
        const DECODE: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        let count = rd16(this + COUNT_OFF);
        let base = rd32(this + BASE_OFF);
        if (count as i32) <= 0 {
            return rd32(base.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4));
        }
        let bits = rd32(this + STREAM_OFF);
        let shift = unsafe { ((this + SHIFT_OFF) as *const u8).read() as u32 };
        let mut target = target;
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
            let mut slot = this;
            let ans = lf_checker_rt::callee_thiscall!(DECODE, u32, this + STREAM_OFF,
                core::ptr::addr_of_mut!(slot) as u32, shift);
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
                let base2 = rd32(slot + BASE_OFF);
                return rd32(base2.wrapping_add(ebx.wrapping_mul(4)));
            }
            ebx = ebx.wrapping_add(1);
            if ebx >= count {
                break;
            }
        }
        rd32(base.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4))
    }
});
