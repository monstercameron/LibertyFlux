// original: 0x008d8cc0 file_state_sync_8cc0
/// Sync the global state byte with this object's field unless equal.
///
/// When the object's byte at 0xFC8 matches the caller's byte, the global
/// keeps its value; otherwise the global is set to 1. Returns nothing
/// meaningful (EAX keeps stale high bytes), hence `ret: none`.
export!(thiscall, rw_008d8cc0(this: u32, arg: u32) -> u32 {
    unsafe {
        /// Object field offset compared against the argument.
        const FIELD: usize = 0xFC8;
        /// Global state byte (file VA).
        const FLAG: u32 = 0x011737AD;
        let cur = (this as *const u8).add(FIELD).read();
        let flag = global::<u8>(FLAG);
        let old = flag.read();
        flag.write(if cur == (arg & 0xFF) as u8 { old } else { 1 });
        0
    }
});
