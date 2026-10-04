// original: 0x0097B8E0 FEET_LEATHER
// 0x0097B8E0 FEET_LEATHER (merged symbol; placeholder name): cdecl/0.
//
// Audio bank registration: resolves sample-name strings to handles, registers
// the event-handler table (code pointer plus name per entry), pushes mixer
// names through the string helper, and fills two 16-slot handle tables from
// global name tables with a fallback name past the end of each table.
// Straight-line registrar; the body below lists every registration in order.
// Returns the last table handle (the original's exit EAX on its only path).
export!(cdecl, rw_97b8e0() -> u32 {
    unsafe {
        // Generated from the disassembly shape (straight-line registrar);
        // reviewed against the original call by call.
        let mut r: u32 = 0;
        r = callee_cdecl!(1, u32, relocated(0x00E8CF18), 0x0);
        r = callee_thiscall!(2, u32, relocated(0x0115D9A0), r);
        *global::<u32>(0x012312E8) = r;
        r = callee_cdecl!(1, u32, relocated(0x00E8CF28), 0x0);
        r = callee_thiscall!(2, u32, relocated(0x0115D9A0), r);
        *global::<u32>(0x012312EC) = r;
        r = callee_cdecl!(1, u32, relocated(0x00E8CF3C), 0x0);
        r = callee_thiscall!(2, u32, relocated(0x0115D9A0), r);
        *global::<u32>(0x012312F0) = r;
        r = callee_cdecl!(1, u32, relocated(0x00E8CF50), 0x0);
        r = callee_thiscall!(2, u32, relocated(0x0115D9A0), r);
        *global::<u32>(0x012312F4) = r;
        r = callee_cdecl!(1, u32, relocated(0x00E8CF60), 0x0);
        r = callee_thiscall!(2, u32, relocated(0x0115D9A0), r);
        *global::<u32>(0x012312F8) = r;
        r = callee_cdecl!(1, u32, relocated(0x00E8CF70), 0x0);
        r = callee_thiscall!(2, u32, relocated(0x0115D9A0), r);
        *global::<u32>(0x012312FC) = r;
        r = callee_cdecl!(1, u32, relocated(0x00E8CF80), 0x0);
        r = callee_thiscall!(2, u32, relocated(0x0115D9A0), r);
        *global::<u32>(0x01231300) = r;
        r = callee_thiscall!(3, u32, relocated(0x012315D0), relocated(0x0097B130));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8CF94), relocated(0x0097C0E0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8CFA0), relocated(0x0097C1C0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8CFAC), relocated(0x0097C1A0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8CFB8), relocated(0x0097E550));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8CFC4), relocated(0x0097E630));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8CFD0), relocated(0x0097E610));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8CFDC), relocated(0x0097C180));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8CFEC), relocated(0x0097E5F0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8CFFC), relocated(0x0097E530));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D008), relocated(0x0097C0C0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D014), relocated(0x0097C120));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D030), relocated(0x0097C140));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D050), relocated(0x0097E590));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D06C), relocated(0x0097E5B0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D08C), relocated(0x0097C160));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D0A8), relocated(0x0097E5D0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D0C4), relocated(0x0097C100));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D0E4), relocated(0x0097E570));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D104), relocated(0x00981150));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D124), relocated(0x00981150));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D144), relocated(0x00981150));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D164), relocated(0x00981150));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D184), relocated(0x00981150));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D1A4), relocated(0x0097B110));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D1B8), relocated(0x0097B110));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D1D4), relocated(0x0097C280));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D1EC), relocated(0x0097B620));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D204), relocated(0x0097E4F0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D220), relocated(0x0097E680));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D230), relocated(0x0097E7F0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D24C), relocated(0x0097E8A0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D260), relocated(0x0097E820));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D270), relocated(0x0097E650));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D27C), relocated(0x0097E6B0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D294), relocated(0x0097E700));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D2AC), relocated(0x0097E750));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D2C4), relocated(0x0097E7A0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D2D0), relocated(0x0097E1D0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D2EC), relocated(0x0097C230));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D2F8), relocated(0x0097C1E0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D304), relocated(0x0097AF80));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D318), relocated(0x0097B6C0));
        r = callee_thiscall!(4, u32, relocated(0x012315D0), relocated(0x00E8D338), relocated(0x0097AFB0));
        r = callee_cdecl!(1, u32, *global::<u32>(0x01038978), 0x0);
        *global::<u32>(0x0123130C) = r;
        r = callee_cdecl!(1, u32, *global::<u32>(0x0103897C), 0x0);
        *global::<u32>(0x01231310) = r;
        r = callee_thiscall!(5, u32, relocated(0x012313F0), relocated(0x00E8D34C));
        r = callee_thiscall!(5, u32, relocated(0x012313C8), relocated(0x00E8D36C));
        r = callee_thiscall!(5, u32, relocated(0x012314A4), relocated(0x00E8D38C));
        r = callee_thiscall!(5, u32, relocated(0x01231424), relocated(0x00E8D3B4));
        r = callee_cdecl!(6, u32, relocated(0x00E8D3DC));
        *global::<u32>(0x01231314) = r;
        for i in 0..16u32 {
            let s = if i <= 6 { *global::<u32>(0x01038998 + i * 4) } else { relocated(0x00E8D474) };
            r = callee_thiscall!(7, u32, relocated(0x0115D9A0), s);
            *global::<u32>(0x01231320 + i * 4) = r;
        }
        for i in 0..16u32 {
            let s = if i <= 12 { *global::<u32>(0x010389B8 + i * 4) } else { relocated(0x00E8D56C) };
            r = callee_thiscall!(7, u32, relocated(0x0115D9A0), s);
            *global::<u32>(0x01231360 + i * 4) = r;
        }
        r = callee_thiscall!(5, u32, relocated(0x01231478), relocated(0x00E8D57C));
        r = callee_thiscall!(5, u32, relocated(0x012315EC), relocated(0x00E8D594));
        r = callee_thiscall!(7, u32, relocated(0x0115D9A0), relocated(0x00E8D5AC));
        *global::<u32>(0x01231318) = r;
        r = callee_thiscall!(7, u32, relocated(0x0115D9A0), relocated(0x00E8D5BC));
        *global::<u32>(0x012312E0) = r;
        r = callee_thiscall!(5, u32, relocated(0x0123151C), relocated(0x00E8D5D4));
        r = callee_thiscall!(5, u32, relocated(0x012315A8), relocated(0x00E8D5F4));
        r = callee_thiscall!(5, u32, relocated(0x0123157C), relocated(0x00E8D618));
        r = callee_thiscall!(5, u32, relocated(0x012313A0), relocated(0x00E8D634));
        r = callee_thiscall!(5, u32, relocated(0x012314F4), relocated(0x00E8D650));
        r = callee_thiscall!(5, u32, relocated(0x01231548), relocated(0x00E8D66C));
        r = callee_thiscall!(5, u32, relocated(0x012314CC), relocated(0x00E8D688));
        r = callee_thiscall!(5, u32, relocated(0x0123144C), relocated(0x00E8D6A4));
        r = callee_thiscall!(7, u32, relocated(0x0115D9A0), relocated(0x00E8D6C8));
        *global::<u32>(0x01231304) = r;
        r = callee_thiscall!(7, u32, relocated(0x0115D9A0), relocated(0x00E8D6D8));
        *global::<u32>(0x01231308) = r;
        r
    }
});
