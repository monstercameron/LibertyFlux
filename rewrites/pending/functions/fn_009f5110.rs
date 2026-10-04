// original: 0x009f5110 flag_toggle_notify_31
/// Toggle a state flag (0 -> 1, nonzero -> 0), stamp the shared tick, notify.
export!(cdecl, rw_009f5110() -> u32 {
    // SAFETY: worker maps the original image; all globals are .data.
    unsafe {
        let flag = global::<u8>(0x012B_61CF);
        let was_zero = flag.read() == 0;
        let stamp = global::<u32>(0x0117_35B4).read();
        flag.write(u8::from(was_zero));
        global::<u32>(0x012B_624C).write(stamp);
        callee_cdecl!(1, u32, 0x1f)
    }
});
