// original: 0x008a5960 aud_math_op_sample_and_threshold
/// Sample three values through a helper call and threshold the second.
///
/// Reads a per-object index byte (+0x40) and count byte (+0x48). A count of
/// 0xff returns early. Otherwise resolves a table slot from the audio entity
/// table (stride and base globals) and returns 0 when slot + count*stride is
/// 0, or that sum when the output pointer at +0xcc is null. The helper fills
/// three floats; when its low byte is 0 its full return value is passed
/// through. The first float is stored to the output pointer, the second is
/// grown by a small constant and must exceed the stored value, else the
/// output pointer is returned. On success forwards to a two-stage follow-up
/// pair and returns the final result.
export!(thiscall, rw_008a5960(this: *mut u8, arg0: u32) -> u32 {
    unsafe {
        let count = *this.add(0x48);
        if count == 0xff {
            return 0xff;
        }
        let index = *this.add(0x40) as u32;
        let stride = *global::<u32>(0x115d964);
        let table = *global::<u32>(0x115d988);
        let slot = table
            .wrapping_add(index.wrapping_mul(0x6f40))
            .wrapping_add(0x6f10) as *const u32;
        let total = (*slot).wrapping_add(stride.wrapping_mul(count as u32));
        if total == 0 {
            return 0;
        }
        let sink = *(this.add(0xcc) as *const u32);
        if sink == 0 {
            return total;
        }
        let mut first: f32 = 0.0;
        let mut second: f32 = 0.0;
        let mut _third: f32 = 0.0;
        let ok: u32 = callee_thiscall!(
            1,
            u32,
            this as u32,
            &mut first as *mut f32 as u32,
            &mut second as *mut f32 as u32,
            &mut _third as *mut f32 as u32
        );
        if (ok & 0xff) == 0 {
            return ok;
        }
        *(sink as *mut f32) = first;
        let limit = *global::<f32>(0xfe868c);
        let grown = second + limit;
        let stored = *(sink as *const f32);
        if !(grown > stored) {
            return sink;
        }
        let next: u32 = callee_thiscall!(2, u32, this as u32, 0, arg0);
        callee_thiscall!(3, u32, next)
    }
});
