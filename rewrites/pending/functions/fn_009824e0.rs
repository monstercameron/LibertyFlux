// original: 0x009824e0 audio_keyed_slot_detach
/// Original 0x009824e0 (unnamed): keyed detach over the slot table.
///
/// When the global audio gate is set, scans the 245 slots at +0x5bfc and
/// releases each live slot whose key at +0xc equals `arg`. Void; the checker
/// compares the release calls and heap.
export!(thiscall, rw_009824e0(this_: u32, arg: u32) -> u32 {
    let gate = unsafe { (relocated(0x01038A20) as *const u8).read() };
    if gate == 0 {
        return 0;
    }
    let guard = [0u32; 2];
    callee_thiscall!(1, u32, guard.as_ptr() as u32, this_.wrapping_add(0x6f3c));
    let mut e = this_.wrapping_add(0x5bfc);
    for _ in 0..0xf5u32 {
        let live = unsafe { ((e + 0x10) as *const u8).read() };
        if live != 0 {
            let key = unsafe { ((e + 0xc) as *const u32).read() };
            if key == arg {
                let h = unsafe { (e as *const u32).read() };
                if h != 0 {
                    callee_thiscall!(3, u32, h, 0);
                }
            }
        }
        e = e.wrapping_add(0x14);
    }
    callee_thiscall!(2, u32, guard.as_ptr() as u32);
    0
});
