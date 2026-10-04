// original: 0x008ABD30 audio_effect_create
/// Create an audio effect object by kind (1..=7).
///
/// A factory: allocate the kind's object, run the shared base initialiser,
/// install the kind's function table, fill the kind's default parameters,
/// then allocate and construct the kind's inner processor object (stored at
/// a kind-specific slot, or zero when that allocation fails). Returns the
/// new object, or null for an unknown kind or a failed allocation.
export!(cdecl, rw_008abd30(kind: u32) -> u32 {
    unsafe {
        // Per-kind function tables in read-only data.
        const VT1: u32 = 0x00e7cbc4;
        const VT2: u32 = 0x00e7c59c;
        const VT3: u32 = 0x00e7c548;
        const VT4: u32 = 0x00e7c644;
        const VT5: u32 = 0x00e7c5f0;
        const VT6: u32 = 0x00e7c6ec;
        const VT7: u32 = 0x00e7c698;
        // Callee ids: 1 = outer allocator, 2 = base initialiser,
        // 3 = inner allocator, 4..=9 = per-kind inner constructors.
        const BASE_INIT: u32 = 2;

        /// Write a 32-bit word at `obj + off`.
        #[inline(always)]
        unsafe fn w(obj: u32, off: u32, val: u32) {
            *(obj.wrapping_add(off) as *mut u32) = val;
        }

        // The original dispatches on kind-1 with an unsigned compare, so
        // kind 0 and anything above 7 take the same null path.
        if kind.wrapping_sub(1) > 6 {
            return 0;
        }

        /// Allocate the outer object; null when the allocator fails.
        macro_rules! make {
            ($size:expr) => {{
                let obj: u32 = callee_cdecl!(1, u32, $size);
                if obj == 0 {
                    return 0;
                }
                let _: u32 = callee_thiscall!(BASE_INIT, u32, obj);
                obj
            }};
        }
        /// Allocate and construct the inner processor into `obj + slot`.
        macro_rules! inner {
            ($obj:expr, $id:expr, $size:expr, $slot:expr) => {{
                let sub: u32 = callee_cdecl!(3, u32, $size);
                if sub == 0 {
                    w($obj, $slot, 0);
                } else {
                    let done: u32 = callee_thiscall!($id, u32, sub);
                    w($obj, $slot, done);
                }
            }};
        }

        match kind {
            1 => {
                let obj = make!(0x74);
                w(obj, 0, relocated(VT1));
                obj
            }
            2 => {
                let obj = make!(0xcc);
                w(obj, 0, relocated(VT2));
                // Four identical parameter quads plus a zero half-word.
                for b in [0x74u32, 0x88, 0x9c, 0xb0] {
                    w(obj, b, 0x3f800000);
                    w(obj, b.wrapping_add(4), 0x3f000000);
                    w(obj, b.wrapping_add(8), 0x3f000000);
                    w(obj, b.wrapping_add(0xc), 0);
                    *(obj.wrapping_add(b).wrapping_add(0x10) as *mut u16) = 0;
                }
                // Kind 2 keeps its inner object at a different slot.
                inner!(obj, 4, 0xc0, 0xc8);
                obj
            }
            3 => {
                let obj = make!(0xc0);
                w(obj, 0, relocated(VT3));
                // Three filter bands with identical defaults.
                for b in [0x78u32, 0x90, 0xa8] {
                    w(obj, b, 0x46bab800);
                    w(obj, b.wrapping_add(4), 0);
                    w(obj, b.wrapping_add(8), 0x43fa0000);
                    w(obj, b.wrapping_add(0xc), 0);
                    w(obj, b.wrapping_add(0x10), 1);
                    *(obj.wrapping_add(b).wrapping_add(0x14) as *mut u8) = 0;
                }
                inner!(obj, 5, 0xc4, 0x74);
                obj
            }
            4 => {
                let obj = make!(0x78);
                w(obj, 0, relocated(VT4));
                inner!(obj, 6, 4, 0x74);
                obj
            }
            5 => {
                let obj = make!(0xe4);
                w(obj, 0, relocated(VT5));
                // Three shaper stages with identical defaults.
                for b in [0x78u32, 0x9c, 0xc0] {
                    w(obj, b, 0);
                    w(obj, b.wrapping_add(4), 0xc0c00000);
                    w(obj, b.wrapping_add(8), 0x40000000);
                    w(obj, b.wrapping_add(0xc), 0x3ba3d70a);
                    w(obj, b.wrapping_add(0x10), 0x3df5c28f);
                    w(obj, b.wrapping_add(0x14), 0x3d75c28f);
                    w(obj, b.wrapping_add(0x18), 0x3e4ccccd);
                    w(obj, b.wrapping_add(0x1c), 0x3c23d70a);
                    *(obj.wrapping_add(b).wrapping_add(0x20) as *mut u8) = 1;
                }
                inner!(obj, 7, 0x2a0, 0x74);
                obj
            }
            6 => {
                let obj = make!(0xb4);
                w(obj, 0, relocated(VT6));
                inner!(obj, 8, 0x348, 0x74);
                obj
            }
            _ => {
                // Kind 7: three rows of six cells across three columns.
                let obj = make!(0x154);
                w(obj, 0, relocated(VT7));
                let mut row = obj.wrapping_add(0xa8);
                for _ in 0..3 {
                    let mut cell = row;
                    for _ in 0..6 {
                        w(cell.wrapping_sub(0x30), 0, 0x64);
                        w(cell.wrapping_sub(0x18), 0, 0xc0c00000);
                        w(cell, 0, 0xc2c80000);
                        cell = cell.wrapping_add(4);
                    }
                    row = row.wrapping_add(0x48);
                }
                inner!(obj, 9, 0x5c, 0x74);
                obj
            }
        }
    }
});
