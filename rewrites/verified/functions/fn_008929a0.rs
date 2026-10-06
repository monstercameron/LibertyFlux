// original: 0x008929A0 audsound_init_embedded_tables
/// Builds two embedded parameter tables inside the object from two scalars.
///
/// `obj` points at 72 writable bytes, `d` and `c` are scalar parameters. The
/// table at `obj+0x18` (32 bytes) is filled with (0, d, 0, c, d, d, 0, c) and
/// its address stored at `obj+0`; the table at `obj+0x38` (16 bytes) is filled
/// with (0, d, d, d) and its address stored at `obj+8`. Returns `obj+0x38`.
/// The incoming ECX is overwritten before any read, so this is a plain
/// three-argument function, not a method call.
/// Original: 0x008929A0 (stdcall, three stack words).
export!(stdcall, rw_008929A0(obj: *mut u8, d: u32, c: u32) -> u32 {
    unsafe {
        const T0: usize = 0x18;
        const T1: usize = 0x38;
        let t0 = obj.add(T0);
        *(obj as *mut u32) = t0 as u32;
        let w0 = t0 as *mut u32;
        *w0.add(0) = 0;
        *w0.add(1) = d;
        *w0.add(2) = 0;
        *w0.add(3) = c;
        *w0.add(4) = d;
        *w0.add(5) = d;
        *w0.add(6) = 0;
        *w0.add(7) = c;
        let t1 = obj.add(T1);
        *(obj.add(8) as *mut u32) = t1 as u32;
        let w1 = t1 as *mut u32;
        *w1.add(0) = 0;
        *w1.add(1) = d;
        *w1.add(2) = d;
        *w1.add(3) = d;
        t1 as u32
    }
});
