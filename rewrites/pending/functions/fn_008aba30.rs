// original: 0x008ABA30 audio_follower_setup
/// Configure the follower: store the nine rate and state inputs, mark the
/// object active, refresh both mode bytes through the probe routine, and
/// stamp the header. Returns the probe's second answer.
export!(thiscall, rw_008ABA30(
    obj: *mut u8,
    r0: f32, r1: f32, r2: f32, r3: f32, r4: f32,
    r5: f32, r6: f32, r7: f32, r8: f32,
) -> u32 {
    unsafe {
        *(obj.add(0x24) as *mut f32) = r0;
        *(obj.add(0x28) as *mut f32) = r1;
        let flags = obj.add(0x3C);
        *flags = (*flags & 0xFB) | 3;
        *(obj.add(4) as *mut f32) = r2;
        *(obj.add(8) as *mut f32) = r3;
        *(obj.add(0x0C) as *mut f32) = r4;
        *(obj.add(0x10) as *mut f32) = r5;
        *(obj.add(0x14) as *mut f32) = r6;
        *(obj.add(0x18) as *mut f32) = r7;
        *(obj.add(0x1C) as *mut f32) = r8;
        let first: u32 = callee_cdecl!(1, u32, 0x3F000000);
        *obj.add(0x20) = first as u8;
        let second: u32 = callee_cdecl!(1, u32, 0x3F000000);
        *obj.add(0x21) = second as u8;
        *(obj as *mut u16) = 0x101;
        second
    }
});
