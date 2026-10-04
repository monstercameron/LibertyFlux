// original: 0x00b01d70 push_timer_rate
/// Measure a rate through the global timer (ST0 float), use the measured
/// value unless the override flag is set (then a stored constant), and push
/// the sub-object plus fixed parameters to the applier. Both manager
/// immediates are relocated (HIGHLOW entries verified).
export!(thiscall, rw_00b01d70(this_: *mut u8) -> u32 {
    let mgr = relocated(0x118d7f0);
    let measured: f32 = callee_thiscall!(1, f32, mgr, 1);
    let overridden = unsafe { *global::<u8>(0x118dc44) } != 0;
    let rate = if overridden {
        unsafe { *global::<f32>(0xfe89b8) }
    } else {
        measured
    };
    let obj = unsafe { this_.add(0x10) };
    callee_thiscall!(
        2,
        u32,
        mgr,
        obj as u32,
        0x42520000,
        rate.to_bits(),
        0x3f000000,
        0x44480000
    )
});
