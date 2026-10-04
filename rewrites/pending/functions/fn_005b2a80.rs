// original: 0x005B2A80 loader_status_poll
/// Polls the loader object: refreshes it, and while it reports busy keeps the
/// latched mode byte non-zero, then mirrors the loader's done flag into the
/// status byte. Does nothing when no loader is attached.
export!(cdecl, rw_005B2A80() -> u32 {
    unsafe {
        let obj = *global::<u32>(0x018B6E84);
        if obj == 0 {
            return 0;
        }
        callee_thiscall!(1, u32, obj);
        let r = callee_thiscall!(2, u32, *global::<u32>(0x018B6E84));
        if r as u8 != 0 {
            let cur = *global::<u8>(0x018E51E1);
            *global::<u8>(0x018E51E1) = if cur == 0 { 1 } else { cur };
        }
        let obj = *global::<u32>(0x018B6E84);
        let done = *((obj.wrapping_add(0x3CF)) as *const u8) != 0;
        *global::<u8>(0x018E51E2) = done as u8;
        0
    }
});
