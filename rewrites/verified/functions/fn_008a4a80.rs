// original: 0x008A4A80 rage::audVariableBlockSound::vf7
/// rage::audVariableBlockSound::vf7: validate a variable block of sound
/// records, stage the two kinds into callee-provided buffers, and commit.
///
/// `this` points to the sound object, `a0`/`a1`/`a2` are forwarded to the
/// validation callee and, later, `a1`/`a2` to the commit callee. Byte `+0x40`
/// is the table row; dword `+0x94` points to the record block whose byte
/// `+4` is the record count and whose records are 9 bytes from `+5`, each
/// with its kind byte at record `+8` (block `+0xD` for record 0).
///
/// The validation callee's answer is tested by its LOW BYTE only; 0 returns 0.
/// Otherwise the records are counted by kind into `+0xB0` (kind != 1) and
/// `+0xB4` (kind == 1). Each non-empty kind is staged: an allocator callee
/// (2 for kind != 1 with the stride global, row and 1; 3 for kind == 1 with
/// `count*8` and the row) provides a buffer, the buffer's index within its
/// table row is stored (`+0xB8` for the first, `+0xB9` for the second), and
/// the records of that kind are copied as 8-byte pairs into the buffer. The
/// first kind additionally requires its count to be at most `stride >> 3`.
/// A null buffer or a failed check returns 0.
///
/// Finally the commit callee runs with the block head, `this`, `a1` and `a2`.
/// A 0 answer selects index `0xFF`; otherwise the answer's table index (low
/// byte of the quotient) is stored at `+0x48`. Index `0xFF` or a zero
/// resolution returns 0, else 1. Only AL is meaningful on return.
///
/// All ordering comparisons are unsigned (`jb`/`jbe` loop bounds on the
/// count byte, `ja` on the staged count against `stride >> 3`); divisions
/// are 32-bit unsigned with a zero high word. Row cells live at
/// `table + row*0x6F40 + 0x6F10` (first kind and commit) and `+ 0x6F14`
/// (second kind).
///
/// Original: 0x008A4A80 (thiscall, three stack words).
export!(thiscall, rw_008a4a80(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6f40;
        const TABLE_BIAS: u32 = 0x6f10;
        const TABLE_BIAS2: u32 = 0x6f14;
        const ABSENT: u8 = 0xFF;

        let t = this as *const u8;
        let ans1: u32 = callee_thiscall!(1, u32, this, a0, a1, a2);
        if (ans1 & 0xFF) == 0 {
            return 0;
        }
        let ebx = (t.add(0x94) as *const u32).read_unaligned();
        let b = ebx as *const u8;
        let count = b.add(4).read();
        let mut b0 = 0u32;
        let mut b4 = 0u32;
        let mut i = 0u32;
        while i < count as u32 {
            if b.add(0x0D + (i * 9) as usize).read() == 1 {
                b4 += 1;
            } else {
                b0 += 1;
            }
            i += 1;
        }
        (this as *mut u32).add(0xB4 / 4).write_unaligned(b4);
        (this as *mut u32).add(0xB0 / 4).write_unaligned(b0);

        let stride = global::<u32>(0x115d964).read();
        let table = global::<u32>(0x115d988).read();
        let row = t.add(0x40).read();
        let row_base = table.wrapping_add((row as u32).wrapping_mul(ROW_STRIDE));
        let cell = ((row_base.wrapping_add(TABLE_BIAS)) as *const u32).read_unaligned();

        if b0 != 0 {
            let pool = global::<u8>(0x115d8a0) as u32;
            let edi: u32 = callee_thiscall!(2, u32, pool, stride, row as u32, 1);
            if edi == 0 {
                return 0;
            }
            let quot = edi.wrapping_sub(cell) / stride;
            (this as *mut u8).add(0xB8).write(quot as u8);
            if b0 > (stride >> 3) {
                return 0;
            }
            let mut dst = edi;
            let mut j = 0u32;
            while j < count as u32 {
                let rec = ebx.wrapping_add(5).wrapping_add(j * 9);
                if ((rec as *const u8).add(8).read()) == 0 {
                    let w0 = (rec as *const u32).read_unaligned();
                    let w1 = ((rec.wrapping_add(4)) as *const u32).read_unaligned();
                    (dst as *mut u32).write_unaligned(w0);
                    ((dst.wrapping_add(4)) as *mut u32).write_unaligned(w1);
                    dst = dst.wrapping_add(8);
                }
                j += 1;
            }
        }

        if b4 != 0 {
            let pool = global::<u8>(0x115d8a0) as u32;
            let edi: u32 =
                callee_thiscall!(3, u32, pool, b4.wrapping_mul(8), row as u32);
            if edi == 0 {
                return 0;
            }
            let cell2 =
                ((row_base.wrapping_add(TABLE_BIAS2)) as *const u32).read_unaligned();
            let stride2 = global::<u32>(0x115d968).read();
            let quot = edi.wrapping_sub(cell2) / stride2;
            (this as *mut u8).add(0xB9).write(quot as u8);
            let mut dst = edi;
            let mut j = 0u32;
            while j < count as u32 {
                let rec = ebx.wrapping_add(5).wrapping_add(j * 9);
                if ((rec as *const u8).add(8).read()) == 1 {
                    let w0 = (rec as *const u32).read_unaligned();
                    let w1 = ((rec.wrapping_add(4)) as *const u32).read_unaligned();
                    (dst as *mut u32).write_unaligned(w0);
                    ((dst.wrapping_add(4)) as *mut u32).write_unaligned(w1);
                    dst = dst.wrapping_add(8);
                }
                j += 1;
            }
        }

        let hub = global::<u8>(0x115dc18) as u32;
        let head = (ebx as *const u32).read_unaligned();
        let ans4: u32 = callee_thiscall!(4, u32, hub, head, this, a1, a2);
        let idx = if ans4 == 0 {
            ABSENT
        } else {
            (ans4.wrapping_sub(cell) / stride & 0xFF) as u8
        };
        (this as *mut u8).add(0x48).write(idx);
        if idx == ABSENT {
            return 0;
        }
        let obj = stride.wrapping_mul(idx as u32).wrapping_add(cell);
        if obj == 0 {
            return 0;
        }
        1
    }
});
