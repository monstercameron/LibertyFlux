// original: 0x00a7d1f0 CComplexClimbLadderTaskInfo::CComplexClimbLadderTaskInfo
/// Runs the base initialiser, stores the three option bytes and stamps the
/// method table for this task kind.
export!(thiscall, rw_00a7d1f0(obj: *mut u8, b0: u32, b1: u32, b2: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, obj as u32);
        *((obj as *mut u8).add(0x18) as *mut u8) = b0 as u8;
        *((obj as *mut u8).add(0x19) as *mut u8) = b1 as u8;
        *((obj as *mut u8).add(0x1a) as *mut u8) = b2 as u8;
        *((obj as *mut u8) as *mut u32) = relocated(0xEA1754);
        obj as u32
    }
});
