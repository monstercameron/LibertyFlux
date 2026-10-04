// original: 0x00b55100 fetch_scalar_to_field_f0
/// Fetch a scalar through two helpers and store it at +0xf0 of the output.
///
/// When the +0x1a18 slot holds an object, resolves a source id through the
/// first helper and asks the second helper (mode 6) whether data is ready;
/// on success a third helper fills a seven-word scratch block whose last
/// word is the result float. Any failure along the way stores +0.0 instead. Returns
/// the output pointer. The scratch addresses passed to the helpers are frame
/// pointers and differ legitimately between sides; their contents and the
/// stored result are what the contract compares.
export!(thiscall, rw_00b55100(this: u32, out: u32) -> u32 {
    unsafe {
        if ((this + 0x1a18) as *const u32).read() == 0 {
            ((out + 0xf0) as *mut f32).write(0.0);
            return out;
        }
        let source: u32 = callee_cdecl!(1, u32, 4);
        if source == 0 {
            ((out + 0xf0) as *mut f32).write(0.0);
            return out;
        }
        // Scratch block offered to the readiness helper. Its first word reads
        // back the caller's leftover slot (the value 4 left below the frame
        // by the helper-1 call, which never cleans its argument); the rest
        // is zero fill, matching the checker's defined stack fill.
        let mut probe = [0u32; 8];
        probe[0] = 4;
        let ready: u32 = callee_thiscall!(
            2,
            u32,
            ((source + 0x10) as *const u32).read(),
            6,
            0,
            probe.as_mut_ptr() as u32
        );
        if ready & 0xff == 0 {
            ((out + 0xf0) as *mut f32).write(0.0);
            return out;
        }
        let mut block = [0u32; 8];
        let base = block.as_mut_ptr() as u32;
        callee_thiscall!(3, u32, base, base.wrapping_add(0x10));
        ((out + 0xf0) as *mut u32).write(((base + 0x18) as *const u32).read());
        out
    }
});
