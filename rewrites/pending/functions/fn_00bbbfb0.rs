// original: 0x00bbbfb0 NativeImpl_GET_SOUND_LEVEL_AT_COORDS_3
/// Measure the sound level at a coordinate triple into an out-slot.
///
/// Resolves the optional listener handle (a zero handle measures against
/// nothing), builds the coordinate record with a zero fourth word, runs it
/// through the two sound stages and stores the resulting level, delivered on
/// the x87 stack, into the out-slot. Returns the out-slot pointer.
export!(cdecl, rw_00bbbfb0(handle: u32, f1: u32, f2: u32, f3: u32, outp: u32) -> u32 {
    unsafe {
        let v: u32 = if handle == 0 {
            0
        } else {
            callee_thiscall!(1, u32, *global::<u32>(0x18B6F1C), handle)
        };
        // The original reads the fourth record word from an uninitialized
        // stack slot; the checker's fill makes it zero, as here.
        let mut buf = [f1, f2, f3, 0u32];
        let bp = buf.as_mut_ptr() as u32;
        let m: u32 = callee_thiscall!(2, u32, bp, v, bp);
        let bits: u32 = callee_thiscall!(3, u32, m);
        *(outp as *mut u32) = bits;
        outp
    }
});
