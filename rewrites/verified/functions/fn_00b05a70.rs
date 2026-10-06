// original: 0x00b05a70 build_obj_and_submit
/// Build a parameter object and submit it through the sink.
///
/// thiscall `(this, a0..a6)`: when `a6` is not -1 and the flag byte at
/// `[this+4]+a6` has bit 0x80 set, the object comes from helper 1
/// (thiscall `(this, a6)`), else from the default maker (helper 2,
/// thiscall, no stack args). A 48-byte frame parameter block is prepared
/// by helper 3 (thiscall `(block, 0)`), then fixed up in place (byte 0
/// and byte 18 set, words at +4 and +20 cleared, word at +44 set to 2,
/// `a5` stored at +12); the block address is skipped in both call
/// comparisons and its twelve words are snapshotted instead. The sink
/// object (from a global) consumes it through its vtable slot at `+0x38`
/// as thiscall `(sink, a0, 3, a1, a2, 0x20, block)`; the answer lands at
/// object `+0`, with `+4`/`+8` cleared and the low bytes of `a4`/`a3` at
/// `+0xc`/`+0xd`. Finally the object is registered (helper 5, thiscall
/// `(this, obj)`), whose answer is returned.
export!(thiscall, rw_00b05a70(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    const FLAGS_OFF: u32 = 4;
    const SINK: u32 = 0x017F_5630;
    const VTABLE_SLOT: u32 = 0x38;
    const FLAG_BIT: u8 = 0x80;
    let obj: u32 = if a6 == 0xFFFF_FFFF {
        callee_thiscall!(2, u32, this)
    } else {
        let take = unsafe {
            let fbase = ((this + FLAGS_OFF) as *const u32).read_unaligned();
            ((fbase.wrapping_add(a6)) as *const u8).read() & FLAG_BIT != 0
        };
        if take {
            callee_thiscall!(1, u32, this, a6)
        } else {
            callee_thiscall!(2, u32, this)
        }
    };
    let mut frame = [0u32; 12];
    let _: u32 = callee_thiscall!(3, u32, frame.as_mut_ptr() as u32, 0);
    unsafe {
        let fb = frame.as_mut_ptr() as *mut u8;
        ((fb.add(4)) as *mut u32).write_unaligned(0);
        fb.add(18).write(1);
        ((fb.add(20)) as *mut u32).write_unaligned(0);
        fb.add(0).write(1);
        ((fb.add(44)) as *mut u32).write_unaligned(2);
        ((fb.add(12)) as *mut u32).write_unaligned(a5);
        ((obj + 4) as *mut u32).write_unaligned(0);
        let sink = *global::<u32>(SINK);
        let vtbl = (sink as *const u32).read_unaligned();
        let tgt = ((vtbl + VTABLE_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let ans = f(sink, a0, 3, a1, a2, 0x20, frame.as_ptr() as u32);
        (obj as *mut u32).write_unaligned(ans);
        ((obj + 0xC) as *mut u8).write(a4 as u8);
        ((obj + 8) as *mut u32).write_unaligned(0);
        ((obj + 0xD) as *mut u8).write(a3 as u8);
    }
    let r: u32 = callee_thiscall!(5, u32, this, obj);
    r
});
