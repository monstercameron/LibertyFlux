// original: 0x008A4140 rage::audSwitchSound::vf7
/// rage::audSwitchSound::vf7: validate the switch parameters, snapshot the
/// source voice, resolve the switch target and stage its records.
///
/// `this` points to the switch sound, `a0`/`a1` are forwarded to the
/// validation callee with `a2`, which also points to the 24-byte source
/// voice block. The validation answer is tested by its LOW BYTE only; 0
/// returns 0. Otherwise the source block is copied to `+0xB0` in three
/// 8-byte moves, and the virtual slot at `+0x10` of `this` resolves the
/// switch target from the block head at `[this+0x94]`; a null target returns
/// 0. Byte `+0xF3` records whether bits `0xC0000` of `+0x70` equal `0x40000`.
///
/// Dword `+0xCC` takes the record count (byte `+4` of the block). At most 8
/// records are copied inline from block `+5` to `+0xD0`. More records clamp
/// the count to `0x40`, allocate through callee 2 (pool, `0x100`, row, 1),
/// confirm through callee 3 (pool, row, buffer, low byte stored at `+0xF0`),
/// and copy the records into the buffer. A null buffer returns 0, else 1.
/// Only AL is meaningful on return.
///
/// All ordering comparisons are unsigned (`ja` against 8, `cmova` clamp to
/// `0x40`, `jb` loop bounds); callee answers are only tested for zero.
///
/// Original: 0x008A4140 (thiscall, three stack words).
export!(thiscall, rw_008a4140(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        let t = this as *const u8;
        let ans1: u32 = callee_thiscall!(1, u32, this, a0, a1, a2);
        if (ans1 & 0xFF) == 0 {
            return 0;
        }
        let mut i = 0u32;
        while i < 3 {
            let w = ((a2.wrapping_add(i * 8)) as *const u64).read_unaligned();
            ((this.wrapping_add(0xB0).wrapping_add(i * 8)) as *mut u64)
                .write_unaligned(w);
            i += 1;
        }
        let vtable = (this as *const u32).read_unaligned();
        let ebx = (t.add(0x94) as *const u32).read_unaligned();
        let head = (ebx as *const u32).read_unaligned();
        let target =
            ((vtable.wrapping_add(0x10)) as *const u32).read_unaligned();
        let slot: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let vp = slot(this, head);
        ((this.wrapping_add(0xC8)) as *mut u32).write_unaligned(vp);
        if vp == 0 {
            return 0;
        }
        let masked =
            (t.add(0x70) as *const u32).read_unaligned() & 0xC0000;
        (this as *mut u8).add(0xF3).write((masked == 0x40000) as u8);
        let count = (ebx as *const u8).add(4).read() as u32;
        ((this.wrapping_add(0xCC)) as *mut u32).write_unaligned(count);
        if count > 8 {
            let n = if count > 0x40 { 0x40 } else { count };
            ((this.wrapping_add(0xCC)) as *mut u32).write_unaligned(n);
            let row = t.add(0x40).read();
            let pool = global::<u8>(0x115d8a0) as u32;
            let edi: u32 = callee_thiscall!(2, u32, pool, 0x100, row as u32, 1);
            if edi == 0 {
                return 0;
            }
            let ans: u32 = callee_thiscall!(3, u32, pool, row as u32, edi);
            (this as *mut u8).add(0xF0).write(ans as u8);
            let mut k = 0u32;
            while k < n {
                let w = ((ebx.wrapping_add(5).wrapping_add(k * 4)) as *const u32)
                    .read_unaligned();
                ((edi.wrapping_add(k * 4)) as *mut u32).write_unaligned(w);
                k += 1;
            }
            return 1;
        }
        if count == 0 {
            return 1;
        }
        let mut k = 0u32;
        while k < count {
            let w = ((ebx.wrapping_add(5).wrapping_add(k * 4)) as *const u32)
                .read_unaligned();
            ((this
                .wrapping_add(0xD0)
                .wrapping_add(k * 4)) as *mut u32)
                .write_unaligned(w);
            k += 1;
        }
        1
    }
});
