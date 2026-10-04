// original: 0x0088F4A0 rage::audVoicePcAdpcm::vf2
// 0088F4A0 rage::audVoicePcAdpcm::vf2: release the child and the aux buffer,
// then forward to the base teardown (tail call) and return its result.
export!(thiscall, rw_0088f4a0(this: *mut u8) -> u32 {
    unsafe {
        let child = *(this.add(0x140) as *const u32);
        if child != 0 {
            callee_thiscall!(1, u32, child, 0);
            *(this.add(0x140) as *mut u32) = 0;
        }
        callee_cdecl!(2, u32, *(this.add(0x148) as *const u32));
        *(this.add(0x148) as *mut u32) = 0;
        callee_thiscall!(3, u32, this as u32)
    }
});
