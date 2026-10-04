// original: 0x00ddb670 sized_container_build
/// Build the sized container and its two child items from nine measurements.
///
/// The first six measurements plus the lead value go to the layout helper,
/// whose answer and the first three inputs are stored at +0x1E0..+0x1F0. A
/// texture item is then allocated and constructed (token constant
/// 0xEFC52C), configured with a fixed colour, described by four blocks
/// (tags 4, 0x10, 8, 2 over the (0, 5), (0, -5), (0, 0), (0, 0) points) and
/// activated; a failed allocation still configures, then faults reading
/// the missing item, exactly like the original. A second, larger item is
/// allocated and constructed (token constant 0xEFC538); its colour comes
/// from the HUD-colour helper and it is described by two blocks (tags 4
/// and 2 over (0, 11) and (2, 0)), styled through a fixed sequence of
/// property slots, and the third measurement selects its text. The list
/// helper files the container and the slot-0xA0 answer for the stored
/// field at +0x1FC is returned.
export!(thiscall, rw_00ddb670(this_ptr: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32) -> u32 {
    unsafe {
        const QUERY_SLOT: u32 = 0x48;
        const TOKEN1: u32 = 0xEFC52C;
        const TOKEN2: u32 = 0xEFC538;
        const DESC_SLOT: u32 = 0x114;
        const ACTIVATE1: u32 = 0x120;
        const CONFIG_SLOT: u32 = 0x1CC;
        const STYLE_SLOT: u32 = 0xA0;
        const COLOR_SLOT: u32 = 0x208;
        const TEXT_SLOT: u32 = 0x1E0;
        const FLAG1_SLOT: u32 = 0x1FC;
        const FLAG2_SLOT: u32 = 0x200;
        const WIDTH_SLOT: u32 = 0x1D4;
        const FILE_SLOT: u32 = 0x13C;
        const EXTRA_SLOT: u32 = 0x1EC;

        ((this_ptr.wrapping_add(0x1E0)) as *mut u32).write(a1);
        ((this_ptr.wrapping_add(0x1E4)) as *mut u32).write(a2);
        ((this_ptr.wrapping_add(0x1EC)) as *mut u32).write(a0);
        let lay: u32 = callee_thiscall!(30, u32, a0, a1, a2, a3, a4, a5, a6, a7, a8);
        ((this_ptr.wrapping_add(0x1F0)) as *mut u32).write(lay);
        let buf1: u32 = callee_cdecl!(31, u32, 0x25C);
        let item1: u32;
        if buf1 == 0 {
            item1 = 0;
        } else {
            let this_vt = (this_ptr as *const u32).read();
            let query: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute((((this_vt.wrapping_add(QUERY_SLOT))) as *const u32).read() as usize);
            let q1 = query(this_ptr);
            let q2 = query(this_ptr);
            let tok: u32 = callee_cdecl!(32, u32, relocated(TOKEN1), q2);
            let built: u32 = callee_thiscall!(33, u32, buf1, tok, q1);
            item1 = built;
        }
        ((this_ptr.wrapping_add(0x1F4)) as *mut u32).write(item1);
        let mut tint = 0xff797979u32;
        let _: u32 = callee_thiscall!(34, u32, item1, 0, &mut tint as *mut u32 as u32, 0, 0xFFFFFFFF);
        let this_vt = (this_ptr as *const u32).read();
        let extra: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((this_vt.wrapping_add(EXTRA_SLOT))) as *const u32).read() as usize);
        let _: u32 = extra(this_ptr, 0);
        let item1_vt = (item1 as *const u32).read();
        let describe1: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute((((item1_vt.wrapping_add(DESC_SLOT))) as *const u32).read() as usize);
        let mut frame = [0u32; 8];
        let dp = frame.as_mut_ptr() as u32;
        let blocks1 = [(4u32, 0u32, 0x40A00000u32), (0x10, 0, 0xC0A00000), (8, 0, 0), (2, 0, 0)];
        for (tag, f0, f1) in blocks1 {
            let ans: u32 = callee_thiscall!(35, u32, dp, f0, f1);
            let w = |i: u32| ((ans.wrapping_add(i.wrapping_mul(4))) as *const u32).read();
            let _: u32 = describe1(item1, tag, w(0), w(1), w(2), w(3), w(4), w(5));
            let _: u32 = callee_thiscall!(36, u32, dp);
        }
        let activate: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((item1_vt.wrapping_add(ACTIVATE1))) as *const u32).read() as usize);
        let _: u32 = activate(item1, 1);
        let buf2: u32 = callee_cdecl!(39, u32, 0x610);
        let item2: u32;
        if buf2 == 0 {
            item2 = 0;
        } else {
            let this_vt2 = (this_ptr as *const u32).read();
            let query2: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute((((this_vt2.wrapping_add(QUERY_SLOT))) as *const u32).read() as usize);
            let q3 = query2(this_ptr);
            let q4 = query2(this_ptr);
            let tok2: u32 = callee_cdecl!(32, u32, relocated(TOKEN2), q4);
            let built2: u32 = callee_thiscall!(37, u32, buf2, tok2, q3);
            item2 = built2;
        }
        ((this_ptr.wrapping_add(0x1F8)) as *mut u32).write(item2);
        let item2_vt = (item2 as *const u32).read();
        let mut hud_arg = [0u32; 8];
        let hud: u32 = callee_cdecl!(38, u32, hud_arg.as_mut_ptr() as u32, 7);
        let config: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute((((item2_vt.wrapping_add(CONFIG_SLOT))) as *const u32).read() as usize);
        let _: u32 = config(item2, 0x41900000, hud, 0, 2);
        let describe2: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute((((item2_vt.wrapping_add(DESC_SLOT))) as *const u32).read() as usize);
        let blocks2 = [(4u32, 0u32, 0x41300000u32), (2, 0x40000000, 0)];
        for (tag, f0, f1) in blocks2 {
            let ans: u32 = callee_thiscall!(35, u32, dp, f0, f1);
            let w = |i: u32| ((ans.wrapping_add(i.wrapping_mul(4))) as *const u32).read();
            let _: u32 = describe2(item2, tag, w(0), w(1), w(2), w(3), w(4), w(5));
            let _: u32 = callee_thiscall!(36, u32, dp);
        }
        let style: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((item2_vt.wrapping_add(STYLE_SLOT))) as *const u32).read() as usize);
        let _: u32 = style(item2, 0x41700000);
        let mut ink = 0xff000000u32;
        let recolor: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((item2_vt.wrapping_add(COLOR_SLOT))) as *const u32).read() as usize);
        let _: u32 = recolor(item2, &mut ink as *mut u32 as u32);
        let set_text: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute((((item2_vt.wrapping_add(TEXT_SLOT))) as *const u32).read() as usize);
        let _: u32 = set_text(item2, a3, 0);
        let f1: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((item2_vt.wrapping_add(FLAG1_SLOT))) as *const u32).read() as usize);
        let _: u32 = f1(item2, 1);
        let f2: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((item2_vt.wrapping_add(FLAG2_SLOT))) as *const u32).read() as usize);
        let _: u32 = f2(item2, 1);
        let wd: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((item2_vt.wrapping_add(WIDTH_SLOT))) as *const u32).read() as usize);
        let _: u32 = wd(item2, 0x2C);
        let this_vt3 = (this_ptr as *const u32).read();
        let file: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((this_vt3.wrapping_add(FILE_SLOT))) as *const u32).read() as usize);
        let _: u32 = file(this_ptr, 1);
        let fval = ((this_ptr.wrapping_add(0x1FC)) as *const u32).read();
        let finish: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((this_vt3.wrapping_add(STYLE_SLOT))) as *const u32).read() as usize);
        finish(this_ptr, fval)
    }
});
