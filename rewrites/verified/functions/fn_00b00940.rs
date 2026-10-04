// original: 0x00B00940 viewport_scene_construct
/// Viewport-scene constructor: installs the vtable, builds the embedded
/// camera, effect-list, light and overlay members, assigns a fresh scene id
/// from the global counter and sets the default fog/clear constants.
///
/// Original: 0x00B00940 (thiscall/0, returns the object pointer).
const SCENE_VTABLE: u32 = 0x00EA_9EA0;
const SCENE_ID_COUNTER: u32 = 0x0104_0008;
const EFFECT_TEMPLATE: u32 = 0x1110_090;
const DEFAULT_PRIORITY: u32 = 10_000;
const ONE_F32: u32 = 0x3F80_0000;

export!(thiscall, rw_00b00940(obj: u32) -> u32 {
    unsafe {
        let base = obj as *mut u8;
        let w = |off: usize| base.add(off) as *mut u32;
        let b = |off: usize| base.add(off);

        // Vtable + embedded camera block at +0x10.
        w(0x000).write(relocated(SCENE_VTABLE));
        callee_thiscall!(1, u32, obj.wrapping_add(0x010));

        // Effect-list header at +0x400.
        let list = obj.wrapping_add(0x400);
        (list as *mut u32).write(0);
        ((list + 4) as *mut u32).write(0);
        callee_thiscall!(2, u32, list.wrapping_add(0x00C));
        callee_thiscall!(2, u32, list.wrapping_add(0x018));
        *b(0x408) = 1;
        callee_thiscall!(3, u32, list);

        // Two light slots at +0x424, two overlay slots at +0x46C.
        for slot in 0..2u32 {
            let light = obj.wrapping_add(0x424).wrapping_add(slot.wrapping_mul(0x24));
            ((light + 0x18) as *mut u32).write(0xFFFF_FFFF);
            callee_thiscall!(4, u32, light);
        }
        for slot in 0..2u32 {
            let overlay = obj.wrapping_add(0x46C).wrapping_add(slot.wrapping_mul(0x20));
            callee_thiscall!(5, u32, overlay);
        }

        // Trailing member at +0x4B0, then clear the "unbuilt" flag bit.
        callee_thiscall!(6, u32, obj.wrapping_add(0x4B0));
        *b(0x558) &= 0xFD;

        // Camera projection defaults (six words: 0,0,1,1,0,1 as floats).
        callee_thiscall!(7, u32, obj.wrapping_add(0x010), 0, 0, ONE_F32, ONE_F32, 0, ONE_F32);

        // Scene registration + effect template installs.
        callee_thiscall!(8, u32, obj);
        callee_thiscall!(9, u32, obj.wrapping_add(0x010), relocated(EFFECT_TEMPLATE));
        callee_thiscall!(10, u32, obj.wrapping_add(0x010), relocated(EFFECT_TEMPLATE));
        *b(0x558) &= 0xFE;

        // Priority default, then flag the priority pass done.
        ((obj + 0x544) as *mut u32).write(DEFAULT_PRIORITY);
        callee_thiscall!(11, u32, obj);
        *b(0x558) &= 0xFB;

        // Fog/clear colour defaults.
        ((obj + 0x530) as *mut u32).write(ONE_F32);
        ((obj + 0x554) as *mut u32).write(0);
        ((obj + 0x534) as *mut u32).write(ONE_F32);

        // Fresh scene id from the global counter.
        let counter = global::<u32>(SCENE_ID_COUNTER);
        let id = counter.read();
        ((obj + 0x53C) as *mut u32).write(id);
        counter.write(id.wrapping_add(1));
        *b(0x558) &= 0xF7;

        // Finalise the effect list, then publish the remaining defaults.
        callee_thiscall!(12, u32, list, 0);
        ((obj + 0x54C) as *mut u32).write(ONE_F32);
        ((obj + 0x550) as *mut u32).write(0);
        ((obj + 0x548) as *mut u32).write(0);
        ((obj + 0x540) as *mut u32).write(0xFFFF_FFFF);
        let flags = (obj + 0x40C) as *mut u32;
        flags.write(flags.read() | 0x1000_0000);
        ((obj + 0x538) as *mut u32).write(0);
    }
    obj
});
