// original: 0x0099cc60 audio_vehicle_voice_update
/// Update one vehicle audio voice for the current frame.
///
/// The object (passed in ECX) and a voice entity (first stack argument; the
/// second is ignored) select a mixer row from the runtime tables through the
/// entity's tag bytes, or zero when the tag is empty. The row's kind word
/// less one drives a switch: kinds 1-4 resolve the voice position through
/// one helper, kind 7 through another whose two outputs feed the mixer and
/// the tail computation, kinds 8-10 through a shared transform with three
/// parameter blocks, kind 11 through the transform plus a conditional voice
/// stop and a mixer reset, and kinds 5-6 and anything else skip straight to
/// the tail. The tail blends a per-object factor with the slot value,
/// re-selects the mixer row, truncates the blend to an integer and delivers
/// it to the mixer. The result is the mixer's answer.
lf_rb80_rt::export!(thiscall, rw_0099cc60(this: u32, ent: u32, _unused: u32) -> u32 {
    let rd32 = |addr: u32| unsafe { (addr as *const u32).read_unaligned() };
    let rd8 = |addr: u32| unsafe { (addr as *const u8).read_unaligned() };
    let obj = this;
    let esi = ent;
    let stride = rd32(lf_rb80_rt::relocated(0x115d968));
    let table = rd32(lf_rb80_rt::relocated(0x115d988));
    let select = |e: u32| {
        let tag = rd8(e.wrapping_add(4)) as u32;
        if tag == 0xff {
            0
        } else {
            let row = rd8(e.wrapping_add(0x40)) as u32;
            let cell = rd32(table.wrapping_add(row.wrapping_mul(0x6f40)).wrapping_add(0x6f14));
            stride.wrapping_mul(tag).wrapping_add(cell)
        }
    };
    let row = select(esi);
    let kind = rd32(row.wrapping_add(0xcc));
    let sw = kind.wrapping_sub(1);
    let mut tail_in = 23900.0f32;
    if sw <= 10 {
        match sw {
            0 | 1 | 2 | 3 => {
                let mut out = [0u32; 4];
                let r = lf_rb80_rt::callee_thiscall!(1, u32, obj, sw, out.as_mut_ptr() as u32);
                lf_rb80_rt::callee_thiscall!(5, u32, esi, r);
            }
            4 | 5 => {}
            6 => {
                let mut a0 = [0u32; 1];
                let mut a1 = [23900.0f32.to_bits()];
                lf_rb80_rt::callee_thiscall!(2, u32, obj, a0.as_mut_ptr() as u32,
                    a1.as_mut_ptr() as u32);
                lf_rb80_rt::callee_thiscall!(3, u32, esi, a0[0]);
                tail_in = f32::from_bits(a1[0]);
            }
            7 => {
                let mut out = [0u32; 4];
                let inner = rd32(obj.wrapping_add(0x820));
                let r = lf_rb80_rt::callee_thiscall!(4, u32, inner, out.as_mut_ptr() as u32,
                    obj.wrapping_add(0x20));
                lf_rb80_rt::callee_thiscall!(5, u32, esi, r);
            }
            8 => {
                let mut out = [0u32; 4];
                let inner = rd32(obj.wrapping_add(0x820));
                let r = lf_rb80_rt::callee_thiscall!(4, u32, inner, out.as_mut_ptr() as u32,
                    obj.wrapping_add(0x10));
                lf_rb80_rt::callee_thiscall!(5, u32, esi, r);
            }
            9 => {
                let mut out = [0u32; 4];
                let inner = rd32(obj.wrapping_add(0x820));
                let r = lf_rb80_rt::callee_thiscall!(4, u32, inner, out.as_mut_ptr() as u32,
                    obj.wrapping_add(0x30));
                lf_rb80_rt::callee_thiscall!(5, u32, esi, r);
            }
            _ => {
                let mut out = [0u32; 4];
                let inner = rd32(obj.wrapping_add(0x820));
                let r = lf_rb80_rt::callee_thiscall!(4, u32, inner, out.as_mut_ptr() as u32,
                    obj.wrapping_add(0x20));
                lf_rb80_rt::callee_thiscall!(5, u32, esi, r);
                let t: u32 = lf_rb80_rt::callee_thiscall!(6, u32, rd32(obj.wrapping_add(0x820)));
                if (t & 0xff) != 0 {
                    lf_rb80_rt::callee_thiscall!(7, u32, rd32(obj.wrapping_add(0x820)));
                }
                lf_rb80_rt::callee_thiscall!(3, u32, esi, 0);
            }
        }
    }
    let factor = f32::from_bits(rd32(obj.wrapping_add(0x9c8))) * 21000.0;
    let blend = 23900.0f32 - factor;
    let slot = if blend <= tail_in { blend } else { tail_in };
    let n = (slot as i64) as i32 as u32;
    lf_rb80_rt::callee_thiscall!(8, u32, select(esi), n);
    0
});
