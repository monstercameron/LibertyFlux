// original: 0x00d8ded0 audio_entity_snapshot_copy
/// Copies an entity snapshot into a compact audio record.
///
/// Moves the id word, position triple, scaled direction bytes, flag bytes,
/// helper answer and misc dwords from the wide `src` entity into `dst`,
/// then assembles the status word at `dst+0x30` from scattered source
/// bits (setting bit 0 always). Returns the copied tag byte at 0x1072.
export!(thiscall, rw_00d8ded0(dst: *mut u8, src: *const u8) -> u32 {
    unsafe {
        *(dst.add(0x10) as *mut u16) = *(src.add(0x2E) as *const u16);
        let p = *(src.add(0x20) as *const u32) as *const u8;
        *(dst as *mut u32) = *(p.add(0x30) as *const u32);
        *(dst.add(4) as *mut u32) = *(p.add(0x34) as *const u32);
        *(dst.add(8) as *mut u32) = *(p.add(0x38) as *const u32);
        let s = *global::<f32>(0x00FE_8BB0);
        *dst.add(0x43) = cvtt_ss2si(*(p.add(0x10) as *const f32) * s) as u8;
        *dst.add(0x44) = cvtt_ss2si(*(p.add(0x14) as *const f32) * s) as u8;
        *dst.add(0x45) = cvtt_ss2si(*(p.add(0x18) as *const f32) * s) as u8;
        *dst.add(0x32) = *src.add(0xF94);
        *dst.add(0x33) = *src.add(0xF95);
        *dst.add(0x34) = *src.add(0xF96);
        *dst.add(0x35) = *src.add(0xF97);
        let answer = callee_thiscall!(1, u32, src as u32);
        *(dst.add(0x3C) as *mut u32) = answer;
        let dcc = *(src.add(0xDCC) as *const u32);
        *(dst.add(0xC) as *mut u32) = dcc;
        *(dst.add(0x38) as *mut u32) = *(src.add(0xDD0) as *const u32);
        let w24 = *(src.add(0x24) as *const u32);
        let w118 = *(src.add(0x118) as *const u32);
        let b_f1a = *src.add(0xF1A);
        let mut w = *(dst.add(0x30) as *const u16);
        w = (w & !0x2) | ((((w24 >> 0x1B) & 1) as u16) << 1);
        w = (w & !0x4) | ((((w118 >> 6) & 1) as u16) << 2);
        w = (w & !0x8) | ((((w118 >> 7) & 1) as u16) << 3);
        w = (w & !0x10) | ((((w118 >> 0xC) & 1) as u16) << 4);
        w = (w & !0x20) | ((((w118 >> 8) & 1) as u16) << 5);
        w = (w & !0x40) | ((((w118 >> 9) & 1) as u16) << 6);
        w = (w & !0x80) | ((((b_f1a as u32 >> 6) & 1) as u16) << 7);
        w = (w & !0x100) | ((((dcc >> 0x11) & 1) as u16) << 8);
        w = (w & !0x200) | ((((dcc >> 0x13) & 1) as u16) << 9);
        w |= 1;
        *(dst.add(0x30) as *mut u16) = w;
        let tag = *src.add(0x1072);
        *dst.add(0x42) = tag;
        tag as u32
    }
});
