// original: 0x009f6210 Stat_RegisterBest
/// Register a stat sample (kind, id, value). When the armed flag is set,
/// clear it and skip the notification; otherwise forward the stat id.
/// The kind and value arguments are not read. Callers ignore the return.
export!(cdecl, rw_009f6210(_kind: u32, stat: u32, _value: u32) -> u32 {
    // SAFETY: worker maps the original image; the flag is .data.
    unsafe {
        let flag = global::<u8>(0x012B_6260);
        if flag.read() != 0 {
            flag.write(0);
            0
        } else {
            callee_cdecl!(1, u32, stat)
        }
    }
});
