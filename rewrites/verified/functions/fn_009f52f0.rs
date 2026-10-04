// original: 0x009f52f0 watched_value_notify
/// If the watched value changed since the last call, notify callee
/// (0x103, 1.0f) and record the new value. Returns the watched value.
export!(cdecl, rw_009f52f0() -> u32 {
    // SAFETY: worker maps the original image; both globals are .data.
    unsafe {
        let cur = global::<u32>(0x0117_3604).read();
        if cur != global::<u32>(0x012B_6258).read() {
            callee_cdecl!(1, u32, 0x103, 0x3F80_0000);
            let cur2 = global::<u32>(0x0117_3604).read();
            global::<u32>(0x012B_6258).write(cur2);
            cur2
        } else {
            cur
        }
    }
});
