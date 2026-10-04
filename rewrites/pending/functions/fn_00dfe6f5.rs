// original: 0x00dfe6f5 drive_ready
/// Whether drive index `arg` (1-based, A=1) holds a ready volume.
///
/// Drive 0 answers true without touching the disk. Other indexes build a
/// root path from the drive letter and ask the system for its type: missing
/// (`0`) and no-root (`1`) answers mean not ready, anything else means
/// ready. An index above 26 reports an invalid-drive error and answers 0.
/// The trailing cookie-check call is reproduced so the outgoing calls match.
export!(cdecl, rw_00dfe6f5(arg: u32) -> u32 {
    unsafe {
        if arg > 0x1A {
            let s1 = callee_cdecl!(1, u32,);
            (s1 as *mut u32).write(0xF);
            let s2 = callee_cdecl!(2, u32,);
            (s2 as *mut u32).write(0xD);
            callee_cdecl!(3, u32,);
            callee_cdecl!(5, u32,);
            return 0;
        }
        if arg == 0 {
            callee_cdecl!(5, u32,);
            return 1;
        }
        let mut path = [0u16; 4];
        path[0] = (arg.wrapping_add(0x40) & 0xFFFF) as u16;
        path[1] = 0x3A;
        path[2] = 0x5C;
        let t = callee_stdcall!(4, u32, path.as_ptr() as u32);
        callee_cdecl!(5, u32,);
        if t == 0 || t == 1 { 0 } else { 1 }
    }
});
