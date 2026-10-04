// original: 0x0065fc20 session_id_format
/// Format the session id for the task log, or zeros when there is no session.
///
/// Reads the session object at `this+0x60`. When it exists and its state field
/// (`+0x50`) is 2 or 3, the two id words at `+0xd8`/`+0xdc` are forwarded;
/// otherwise two zero words are used. The words go to the shared formatter
/// with a ` 0x%016I64x` format string. Returns the formatter's answer.
export!(thiscall, rw_0065fc20(this: u32) -> u32 {
    unsafe {
        let session = ((this + 0x60) as *const u32).read();
        let mut words = [0u32; 2];
        if session != 0 {
            let state = ((session + 0x50) as *const u32).read();
            if state == 2 || state == 3 {
                words[0] = ((session + 0xd8) as *const u32).read();
                words[1] = ((session + 0xdc) as *const u32).read();
            }
        }
        callee_cdecl!(1, u32, this + 0x6d, relocated(0xf9c370), words[0], words[1])
    }
});

