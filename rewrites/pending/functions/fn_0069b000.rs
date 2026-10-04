// original: 0x0069b000 rage::crAnimChannelRawQuaternion::vf20
/// Raw-quaternion channel serializer: exchanges the element count through
/// the resource stream, grows the backing array when the flag bit requests
/// it, then serializes each 16-byte element via the record helper.
/// Returns zero for an empty channel, else the last helper answer.
lf_k2_rt::export!(thiscall, rw_0069b000(this: *mut u8, res: *const u8) -> u32 {
    unsafe {
        let mut count = *((this.add(0x0c)) as *const u16) as u32;
        {
            // The count-exchange answer is discarded (eax is cleared after).
            let flag_clear = (*res & 1) == 0;
            let stream = *((res.add(4)) as *const u32);
            let slot = (&mut count as *mut u32) as u32;
            if flag_clear {
                lf_k2_rt::callee_thiscall!(2, u32, stream, slot, 2);
            } else {
                lf_k2_rt::callee_thiscall!(1, u32, stream, slot, 2);
            }
        }
        if (*res & 1) != 0 {
            lf_k2_rt::callee_thiscall!(3, u32, (this as u32).wrapping_add(8), count);
        }
        let mut ans = 0u32;
        let base = *((this.add(8)) as *const u32);
        let mut i = 0u32;
        while i < count {
            let elem = base.wrapping_add(i.wrapping_mul(16));
            ans = lf_k2_rt::callee_thiscall!(4, u32, res as u32, elem);
            i += 1;
        }
        ans
    }
});
