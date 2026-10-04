// original: 0x00adcfd0 CRenderPhaseInitC
use lf_checker_rt::{callee_thiscall, export, relocated};

// Callee ids in the Fn3 contract: 1 = base init (thiscall/1, passed 0),
// 2 = sub-object init at obj+0x960 (thiscall/0), 3 = indexed store
// (thiscall/2, passed 0 then the first argument), 4/5 = rect setups on
// obj+0xb0 with constant float pairs (thiscall/6 each).
//
// Default initializer of the sibling phase: base init, vtable install,
// sub-object init, one indexed store of the first argument, two constant
// rect setups, then a block of constant field defaults (1.0/0.5 floats,
// zeroed words, two flag bytes). Records the second argument at +0x940.
// Returns the object itself.
export!(thiscall, rw_00adcfd0(obj: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, obj, 0);
        let o = obj as *mut u8;
        (o as *mut u32).write_unaligned(relocated(0x00EA6FB4));
        callee_thiscall!(2, u32, obj.wrapping_add(0x960));
        callee_thiscall!(3, u32, obj, 0, a0);
        (o.add(0x894)).write_unaligned(0u8);
        (o.add(0x8f4) as *mut u32).write_unaligned(4);
        (o.add(0x940) as *mut u32).write_unaligned(a1);
        callee_thiscall!(
            4, u32, obj.wrapping_add(0xb0),
            0, 0, 0x3F800000, 0x3F800000, 0, 0x3F800000
        );
        callee_thiscall!(
            5, u32, obj.wrapping_add(0xb0),
            0, 0x3F800000, 0x3F800000, 0, 0, 0x3F800000
        );
        (o.add(0x944) as *mut u32).write_unaligned(0x3F000000);
        o.add(0x1c).write_unaligned(1u8);
        (o.add(0x94c) as *mut u32).write_unaligned(0x3F800000);
        (o.add(0x948) as *mut u32).write_unaligned(0x3F000000);
        (o.add(0xd60) as *mut u32).write_unaligned(0x3F800000);
        (o.add(0xd64) as *mut u32).write_unaligned(0x3F800000);
        (o.add(0xd68) as *mut u32).write_unaligned(0x3F800000);
        (o.add(0xd6c) as *mut u32).write_unaligned(0x3F800000);
        (o.add(0x950) as *mut u32).write_unaligned(0);
        (o.add(0xd50) as *mut u16).write_unaligned(0);
        o.add(0x20).write_unaligned(1u8);
        obj
    }
});
