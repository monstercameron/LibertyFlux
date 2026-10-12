// original: 0x00A7D9D0 CScriptAnimTaskInfo::CScriptAnimTaskInfo_2
/// Construct CScriptAnimTaskInfo from an optional source record. After the base constructor, clear its copied state and install the vtable. A no-argument query controls an optional weight call when `source` is non-null and the source's +0x58 float is below the image's threshold. A non-null source then supplies the words at +0x14 and +0x18, a scripted float result at +0x1c, a byte at +0x24, and a word at +0x20. The float comparison is ordered, so a NaN source value skips the optional weight call. Returns `this`.
lf_checker_rt::export!(thiscall, rw_00a7d9d0(this: u32, source: u32) -> u32 {
    const BASE_CTOR: u32 = 6;
    const QUERY_ANIMATION: u32 = 11;
    const BLEND_WEIGHT: u32 = 12;
    const VTABLE: u32 = 0x00EA19DC;
    const BLEND_LIMIT: u32 = 0x00FE88CC;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(0x14) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x18) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x1C) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x20) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x24) as *mut u8).write(0);
    }
    let query_animation: extern "cdecl" fn() -> u8 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(QUERY_ANIMATION) as usize)
    };
    let has_animation = query_animation() != 0;
    if has_animation && source != 0 {
        let limit = f32::from_bits(unsafe { lf_checker_rt::global::<u32>(BLEND_LIMIT).read_unaligned() });
        let source_weight = f32::from_bits(unsafe { (source.wrapping_add(0x58) as *const u32).read_unaligned() });
        if limit > source_weight {
            let _ = lf_checker_rt::callee_thiscall!(BLEND_WEIGHT, f32, source);
        }
    }
    if source != 0 {
        unsafe {
            let source_flags = (source.wrapping_add(0x14) as *const u32).read_unaligned();
            let source_state = (source.wrapping_add(0x18) as *const u32).read_unaligned();
            (this.wrapping_add(0x14) as *mut u32).write_unaligned(source_flags);
            (this.wrapping_add(0x18) as *mut u32).write_unaligned(source_state);
        }
        let weight = lf_checker_rt::callee_thiscall!(BLEND_WEIGHT, f32, source);
        unsafe {
            (this.wrapping_add(0x1C) as *mut u32).write_unaligned(weight.to_bits());
            (this.wrapping_add(0x24) as *mut u8).write((source.wrapping_add(8) as *const u8).read());
            let source_mode = (source.wrapping_add(4) as *const u32).read_unaligned();
            (this.wrapping_add(0x20) as *mut u32).write_unaligned(source_mode);
        }
    }
    this
});
