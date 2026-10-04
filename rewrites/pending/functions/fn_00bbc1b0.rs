// original: 0x00bbc1b0 NativeImpl_IS_SITTING_OBJECT_NEAR
/// Report whether a sitting object is near a coordinate triple.
///
/// Arms the seat selector (a null selector skips the arming call), publishes
/// the coordinates with a zero fourth word to the shared probe slots, clears
/// the probe result pair and runs the sitting scan over the coordinate
/// record. Returns whether the scan raised the result flag, keeping the
/// scan answer's upper bytes.
export!(cdecl, rw_00bbc1b0(f0: u32, f1: u32, f2: u32, sel: u32) -> u32 {
    unsafe {
        *global::<u32>(0x1047380) = 0xFFFFFFFF;
        if sel != 0 {
            let _: u32 = callee_cdecl!(1, u32, sel, relocated(0x1047380));
        }
        // The original reads the fourth coordinate from an uninitialized
        // stack slot; the checker's fill makes it zero, as here.
        let f3 = 0u32;
        *global::<u32>(0x167F120) = f0;
        *global::<u32>(0x167F124) = f1;
        *global::<u32>(0x167F128) = f2;
        *global::<u32>(0x167F12C) = f3;
        *global::<u32>(0x167F114) = 0;
        *global::<u32>(0x167F118) = 0;
        let mut buf = [f0, f1, f2, 0u32, 0x40A00000u32];
        let r: u32 = callee_cdecl!(2, u32, buf.as_mut_ptr() as u32,
            relocated(0xBC3890), 0, 0x10, 0xD);
        (r & 0xFFFFFF00) | (((*global::<u32>(0x167F114)) != 0) as u32)
    }
});
