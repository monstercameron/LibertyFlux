// original: 0x0069b680 rage::crAnimChannelStaticQuaternion::vf20
/// Static-quaternion serializer: forwards the buffer at +0x08 and the
/// resource to the record helper. Returns the helper answer.
lf_k2_rt::export!(thiscall, rw_0069b680(this: *mut u8, res: u32) -> u32 {
    unsafe {
        let buf = *((this.add(8)) as *const u32);
        lf_k2_rt::callee_thiscall!(1, u32, res, buf)
    }
});
