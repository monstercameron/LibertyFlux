// original: 0x00981e90 audio_attach_voice_bank
/// Original 0x00981e90 (unnamed): attach a voice bank to the ambient entity.
///
/// Opens the bank named by the global handle; on success resolves the entry
/// count through a descriptor call that reports back through a local slot,
/// initialises the header at +0x6f30 on first use, visits each of the
/// reported records, then closes the bank. Void; calls and header are the
/// behaviour.
export!(thiscall, rw_00981e90(this_: u32) -> u32 {
    let g = unsafe { (relocated(0x01038A24) as *const u32).read() };
    let h = callee_cdecl!(1, u32, g, relocated(0x00E8D9CC));
    if h == 0 {
        return 0;
    }
    let tok = callee_cdecl!(2, u32, h, 1);
    let mut slot = 0u32;
    callee_cdecl!(
        3,
        u32,
        tok,
        relocated(0x00E8D9D0),
        &mut slot as *mut u32 as u32
    );
    let v = slot;
    let b = this_.wrapping_add(0x6f30);
    let init = unsafe { ((b + 6) as *const u16).read() };
    if init == 0 {
        unsafe { ((b + 6) as *mut u16).write(v as u16); }
        let arr = if v != 0 {
            callee_thiscall!(4, u32, b, v)
        } else {
            0
        };
        unsafe { (b as *mut u32).write(arr); }
    }
    unsafe { ((b + 4) as *mut u16).write(v as u16); }
    if v != 0 {
        let arr = unsafe { (b as *const u32).read() };
        let mut i = 0u32;
        while i < v {
            callee_thiscall!(5, u32, arr.wrapping_add(i.wrapping_mul(0x160)), h);
            i += 1;
        }
    }
    callee_cdecl!(6, u32, h);
    0
});
