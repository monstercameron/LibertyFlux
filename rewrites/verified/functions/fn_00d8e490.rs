// original: 0x00d8e490 audio_probe_init
//! Initialize an audio probe object.
//!
//! Runs the sub-object hook, zeroes the state words, sets the identity rows
//! and defaults, then allocates the 0x50-byte child object (null when the
//! allocation fails). Returns the object pointer.
//!
//! Note: the original loads the initial value of the word at +0x4C from its
//! own uninitialized stack scratch and stores whatever it finds there. Under
//! the checker's defined zero stack fill that value is 0, which is what this
//! rewrite stores.
export!(thiscall, rw_00d8e490(this: u32) -> u32 {
    unsafe {
        let t = this as *mut u32;
        let _: u32 = callee_cdecl!(1, u32, this.wrapping_add(0x70));
        *t.add(0x84 / 4) = 0xA0;
        *t.add(0x50 / 4) = 0;
        *t.add(0x54 / 4) = 0;
        *t.add(0x78 / 4) = 0;
        *t.add(0x7C / 4) = 0;
        *t.add(0x58 / 4) = 0;
        *t.add(0x5C / 4) = 0;
        *t.add(0x6C / 4) = 0;
        *t.add(0x80 / 4) = 0xFFF;
        *t.add(0x60 / 4) = 0;
        *t.add(0x64 / 4) = 0;
        *t.add(0x68 / 4) = 0;
        *t.add(0x70 / 4) = 0;
        *t.add(0x8C / 4) = 0;
        *t.add(0x74 / 4) = 0;
        // Identity rows.
        *t.add(0x00 / 4) = 0x3F800000;
        *t.add(0x04 / 4) = 0;
        *t.add(0x08 / 4) = 0;
        *t.add(0x10 / 4) = 0;
        *t.add(0x14 / 4) = 0x3F800000;
        *t.add(0x18 / 4) = 0;
        *t.add(0x20 / 4) = 0;
        *t.add(0x24 / 4) = 0;
        *t.add(0x28 / 4) = 0x3F800000;
        *t.add(0x38 / 4) = 0;
        *t.add(0x34 / 4) = 0;
        *t.add(0x30 / 4) = 0;
        *t.add(0x40 / 4) = 0;
        *t.add(0x44 / 4) = 0;
        *t.add(0x48 / 4) = 0;
        // Uninitialized stack scratch; 0 under the defined fill.
        *(this as *mut f32).add(0x4C / 4) = 0.0;
        let child: u32 = callee_cdecl!(2, u32, 0x50);
        if child != 0 {
            let built: u32 = callee_thiscall!(3, u32, child);
            *t.add(0x90 / 4) = built;
        } else {
            *t.add(0x90 / 4) = 0;
        }
        this
    }
});
