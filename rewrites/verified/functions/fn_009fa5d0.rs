// original: 0x009fa5d0 playstat_report_emit
/// Build and emit a tagged report through the report pipeline.
///
/// Constructs a temporary report object on the stack, tags it, resolves
/// its payload, emits it, and tears the object down. The auxiliary stack
/// slot mirrors the original's frame layout so the call snapshots line
/// up. Returns nothing meaningful.
export!(cdecl, rw_009fa5d0() -> u32 {
    unsafe {
        // File VA of the report tag object; relocated like the original's
        // immediate, which carries a relocation entry.
        const TAG: u32 = 0x00e99878;
        let mut frame = [0u32; 24];
        let base = frame.as_mut_ptr();
        let temp = base.add(5);
        let aux = base.add(18);
        callee_thiscall!(1, u32, temp as u32, 1, 0x29);
        temp.write(relocated(TAG));
        let resolved = callee_cdecl!(2, u32, base as u32);
        callee_cdecl!(3, u32, resolved, aux as u32);
        callee_cdecl!(4, u32, temp as u32, 0x38);
        callee_thiscall!(5, u32, temp as u32);
        // Stack-cookie check: argument excluded from comparison (see 740).
        callee_thiscall!(6, u32, 0);
        0
    }
});
