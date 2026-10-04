// original: 0x00a7e990 CComplexGangDrivebyTaskInfo::vf6
/// Serialize `CComplexGangDrivebyTaskInfo` (kept for a later re-run).
///
/// Reads one flag byte through a helper, then either writes a count
/// prefix plus a derived bit count or a fixed 19-bit blob, followed by
/// two per-object bytes. Returns the running changed flag. The stock
/// checker cannot drive the flag byte, so this stays deferred.
lf_checker_rt::export!(thiscall, rw_00a7e990(this: u32, stream: u32) -> u32 {
    unsafe {
        let tbase = this as *const u8;
        let mut changed = lf_checker_rt::callee_thiscall!(1, u32, this, stream) as u8;
        // Initial value is dead once the helper writes the flag; without
        // byte-granularity stub writes the original keeps the top byte
        // of its pushed ECX (`this`), reproduced here for faithfulness.
        let mut flag: u8 = (this >> 24) as u8;
        lf_checker_rt::callee_thiscall!(2, u32, stream, &mut flag as *mut u8 as u32);
        let present = (tbase.add(0x18) as *const u16).read_unaligned() != 0;
        if flag != present as u8 {
            changed = 1;
        }
        let out = &mut changed as *mut u8 as u32;
        if flag != 0 {
            let count = (tbase.add(0x18) as *const u16).read_unaligned();
            lf_checker_rt::callee_thiscall!(3, u32, stream, count as u32, out);
            lf_checker_rt::callee_thiscall!(4, u32, stream, 0x20, 1);
            let n = lf_checker_rt::callee_cdecl!(5, u32,).wrapping_sub(0x2e);
            lf_checker_rt::callee_thiscall!(4, u32, stream, n, 1);
        } else {
            let blob = tbase.add(0x20) as u32;
            lf_checker_rt::callee_stdcall!(6, u32, blob, out, 0x13);
        }
        let b0 = tbase.add(0x31).read();
        lf_checker_rt::callee_thiscall!(7, u32, stream, b0 as u32, out);
        let b1 = tbase.add(0x32).read();
        let last = lf_checker_rt::callee_thiscall!(7, u32, stream, b1 as u32, out);
        (last & 0xffffff00) | changed as u32
    }
});
