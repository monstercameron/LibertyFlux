// original: 0x009a3d20 audRadioAudioEntity::vf1
// 0x009A3D20: audRadioAudioEntity::vf1 (merged symbol name).
//
// Resets a radio audio entity to its freshly-tuned state: clears the live
// tuning fields, parks every station slot on the "none" sentinel 0xFE, drops
// two global mute flags, re-seeds the two embedded filter blocks with their
// default coefficients, and finishes through the shared entity reset plus
// the base-class initializer.
//
// Convention: thiscall/0, returns whatever the base initializer answers.
// ---------------------------------------------------------------------------
export!(thiscall, rw_009a3d20(this: *mut u8) -> u32 {
    unsafe {
        const NONE: u32 = 0xFE;
        const UNIT: u32 = 0x3F800000; // 1.0f
        const FLOOR: u32 = 0x3A83126F; // default filter floor coefficient
        *this.add(0x6C).cast::<u32>() = 0;
        *this.add(0x70).cast::<u32>() = 0;
        *this.add(0x84).cast::<u16>() = 0;
        *global::<u8>(0x012845C8) = 0;
        *this.add(0x86) = 0;
        *this.add(0x88).cast::<u16>() = 0;
        *this.add(0x28).cast::<u32>() = 0;
        *this.add(0x2C).cast::<u32>() = 0;
        *this.add(0x1C).cast::<u32>() = 0;
        *this.add(0x74).cast::<u32>() = NONE;
        *this.add(0x78).cast::<u32>() = NONE;
        *this.add(0x7C).cast::<u32>() = NONE;
        *this.add(0x80).cast::<u32>() = NONE;
        *this.add(0x68).cast::<u32>() = NONE;
        *this.add(0x20).cast::<u32>() = 0;
        *this.add(0x10).cast::<u32>() = 0;
        *this.add(0x14).cast::<u32>() = 0;
        *this.add(0x98).cast::<u16>() = 0;
        *this.add(0x18).cast::<u32>() = 0;
        *this.add(0x8B) = 0;
        *global::<u8>(0x012845C9) = 0;
        *this.add(0x8C).cast::<u16>() = 0;
        *this.add(0x8E) = 0;
        callee_thiscall!(1, u32, this.add(0x30) as u32, FLOOR, FLOOR, 0, UNIT);
        let one = *global::<u32>(0x01038E48);
        callee_thiscall!(2, u32, this.add(0x4C) as u32, one, one);
        *this.add(0x8A) = 0;
        *this.add(0x87) = 0;
        *this.add(0x90).cast::<u32>() = 0;
        *this.add(0x94).cast::<u16>() = 0x100;
        callee_cdecl!(3, u32,);
        let r = callee_thiscall!(4, u32, this as u32);
        *this.add(0x96).cast::<u16>() = 0;
        r
    }
});
