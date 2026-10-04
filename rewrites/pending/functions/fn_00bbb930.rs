// original: 0x00bbb930 NativeImpl_DOES_SCENARIO_EXIST_IN_AREA
/// Report whether a scenario exists for a five-word area description.
///
/// Forwards the area words to the scenario probe with three zeroed scratch
/// slots: the first coordinate by address, the middle two coordinates dead on
/// the frame, the last two by value. Returns the probe answer's low byte as a
/// boolean, keeping the answer's upper bytes.
export!(cdecl, rw_00bbb930(f0: u32, _f1: u32, _f2: u32, f3: u32, f4: u32) -> u32 {
    unsafe {
        let mut c = f0;
        let mut b = 0u32;
        let mut a = 0u32;
        let r: u32 = callee_cdecl!(1, u32,
            &mut c as *mut u32 as u32, f3, 0, 0,
            &mut b as *mut u32 as u32, &mut a as *mut u32 as u32, f4, 0, 0);
        (r & 0xFFFFFF00) | (((r & 0xFF) != 0) as u32)
    }
});
