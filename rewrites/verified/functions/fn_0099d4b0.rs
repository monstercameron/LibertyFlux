// original: 0x0099d4b0 audio_gun_rotate_tick
/// Drive one tick of a rotating-gun audio voice.
///
/// The owning object (passed in ECX) keeps its voice handle in the slot at
/// +0xaa4. When the slot is empty the voice is created first: a parameter
/// frame is cleared, filled with the owner's tuning words, and handed to the
/// voice allocator together with the slot address and a name key; a still
/// empty slot afterwards ends the tick. Otherwise two sampled values are
/// read from the owner's driver block at +0x820 (a level and a scaled step
/// count, the scale being the constant 100.0), the handle's tag bytes select
/// a mixer row from the runtime tables, and the level and the step count are
/// delivered to the mixer through two calls. The function returns no value.
lf_rb80_rt::export!(thiscall, rw_0099d4b0(this: u32) -> u32 {
    let rd32 = |addr: u32| unsafe { (addr as *const u32).read_unaligned() };
    let obj = this;
    let slot = obj.wrapping_add(0xaa4);
    if rd32(slot) == 0 {
        let mut frame = [0u32; 12];
        let base = frame.as_mut_ptr() as u32;
        lf_rb80_rt::callee_thiscall!(1, u32, base);
        frame[11] = rd32(obj.wrapping_add(0xa20));
        frame[6] = rd32(obj.wrapping_add(0x820)).wrapping_add(0xdb8);
        lf_rb80_rt::callee_thiscall!(2, u32, obj, lf_rb80_rt::relocated(0xe9064c), slot,
            base.wrapping_add(12), 0xffffffffu32, 0, 0);
        if rd32(slot) == 0 {
            return 0;
        }
    }
    let inner = rd32(obj.wrapping_add(0x820));
    let speed_bits = unsafe { (inner.wrapping_add(0x14a0) as *const u32).read_unaligned() };
    let level: f32 = lf_rb80_rt::callee_cdecl!(3, f32, speed_bits);
    let scaled: f32 = lf_rb80_rt::callee_cdecl!(4, f32, 0xc0400000u32, 0x3f000000u32, 0u32,
        0x3f800000u32, speed_bits);
    let steps = (scaled * 100.0) as i32 as u32;
    let stride = rd32(lf_rb80_rt::relocated(0x115d968));
    let table = rd32(lf_rb80_rt::relocated(0x115d988));
    let select = |ent: u32| {
        let tag = unsafe { (ent.wrapping_add(4) as *const u8).read_unaligned() } as u32;
        if tag == 0xff {
            0
        } else {
            let row = unsafe { (ent.wrapping_add(0x40) as *const u8).read_unaligned() } as u32;
            let cell = rd32(table.wrapping_add(row.wrapping_mul(0x6f40)).wrapping_add(0x6f14));
            stride.wrapping_mul(tag).wrapping_add(cell)
        }
    };
    lf_rb80_rt::callee_thiscall!(5, u32, select(rd32(slot)), level.to_bits());
    lf_rb80_rt::callee_thiscall!(6, u32, select(rd32(slot)), steps);
    0
});
