// original: 0x008A5660 rage::audIfSound::vf7
/// rage::audIfSound::vf7: validate the branch parameters, resolve both arms
/// and either take the live arm or evaluate the condition.
///
/// `this` points to the if-sound, `a0`/`a1`/`a2` are forwarded to the
/// validation callee. The validation answer is tested by its LOW BYTE only;
/// 0 returns 0. Byte `+0xC4` records whether bits `0xC0000` of `+0x70` equal
/// `0x40000`. Dword `+0xB4` takes block `+0xD`, and the virtual slot at
/// `+0x10` resolves both arms from block `+0x11` (stored at `+0xB8`) and
/// block `+8` (stored at `+0xB0`); dword `+0xBC` takes block byte `+0xC`.
/// The block pointer is dword `+0x94` of `this`. Only AL is meaningful.
///
/// When the flag is set, the refresh callee runs and dword `+0xC0` selects:
/// 0 commits the head block through the hub callee and folds the answer
/// with 0 through the accumulate callee, 1 commits the second block and
/// folds with 1, anything else returns 1. The folded value goes through the
/// tail resolver: a null answer returns 0, else 1.
///
/// When the flag is clear, the 24 bytes at `a2` are staged into a frame
/// buffer, the head block commits through the hub callee and its table index
/// (low byte of the quotient, `0xFF` on a null answer) is stored at `+0x48`,
/// then the second block commits with the staged buffer and its index is
/// stored at `+0x49`. Selector `0xFF` or a zero resolution falls through to
/// the tail resolver with 1, else 1 is returned.
///
/// All ordering comparisons are unsigned (`jb` after the flag test is really
/// an equality test on the flag byte); divisions are 32-bit unsigned with a
/// zero high word; row cells live at `table + row*0x6F40 + 0x6F10`.
///
/// Original: 0x008A5660 (thiscall, three stack words).
export!(thiscall, rw_008a5660(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6f40;
        const TABLE_BIAS: u32 = 0x6f10;
        const ABSENT: u8 = 0xFF;

        let t = this as *const u8;
        let ans1: u32 = callee_thiscall!(1, u32, this, a0, a1, a2);
        if (ans1 & 0xFF) == 0 {
            return 0;
        }
        let masked = (t.add(0x70) as *const u32).read_unaligned() & 0xC0000;
        let flag = (masked == 0x40000) as u8;
        (this as *mut u8).add(0xC4).write(flag);
        let ebp = (t.add(0x94) as *const u32).read_unaligned();
        let b = ebp as *const u8;
        ((this.wrapping_add(0xB4)) as *mut u32)
            .write_unaligned((b.add(0x0D) as *const u32).read_unaligned());
        let vtable = (this as *const u32).read_unaligned();
        let target = ((vtable.wrapping_add(0x10)) as *const u32).read_unaligned();
        let slot: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let r_arm1 = slot(this, (b.add(0x11) as *const u32).read_unaligned());
        ((this.wrapping_add(0xB8)) as *mut u32).write_unaligned(r_arm1);
        let r_arm0 = slot(this, (b.add(0x08) as *const u32).read_unaligned());
        ((this.wrapping_add(0xB0)) as *mut u32).write_unaligned(r_arm0);
        ((this.wrapping_add(0xBC)) as *mut u32)
            .write_unaligned(b.add(0x0C).read() as u32);

        let stride = global::<u32>(0x115d964).read();
        let table = global::<u32>(0x115d988).read();
        let row = t.add(0x40).read();
        let cell = ((table
            .wrapping_add((row as u32).wrapping_mul(ROW_STRIDE))
            .wrapping_add(TABLE_BIAS)) as *const u32)
            .read_unaligned();
        let hub = global::<u8>(0x115dc18) as u32;

        if flag == 0 {
            // Stage the 24 source bytes at the buffer start: the original's
            // three 8-byte stores land contiguously right at the frame
            // pointer it passes on, so the contract snapshots 6 words at
            // offset 0 on both sides.
            let mut buf = [0u8; 24];
            let mut i = 0usize;
            while i < 24 {
                buf[i] = ((a2.wrapping_add(i as u32)) as *const u8).read();
                i += 1;
            }
            let head = (ebp as *const u32).read_unaligned();
            let r1: u32 = callee_thiscall!(3, u32, hub, head, this, a1, a2);
            let idx48 = if r1 == 0 {
                ABSENT
            } else {
                (r1.wrapping_sub(cell) / stride & 0xFF) as u8
            };
            (this as *mut u8).add(0x48).write(idx48);
            let head2 = ((ebp.wrapping_add(4)) as *const u32).read_unaligned();
            let r2: u32 = callee_thiscall!(
                4,
                u32,
                hub,
                head2,
                this,
                a1,
                buf.as_ptr() as u32
            );
            let idx49 = if r2 == 0 {
                ABSENT
            } else {
                (r2.wrapping_sub(cell) / stride & 0xFF) as u8
            };
            (this as *mut u8).add(0x49).write(idx49);
            let sel48 = (this as *const u8).add(0x48).read();
            if sel48 == ABSENT {
                let f: u32 = callee_thiscall!(6, u32, this, 1);
                return (f != 0) as u32;
            }
            let obj = stride
                .wrapping_mul(sel48 as u32)
                .wrapping_add(cell);
            if obj == 0 {
                let f: u32 = callee_thiscall!(6, u32, this, 1);
                return (f != 0) as u32;
            }
            return 1;
        }

        let _: u32 = callee_thiscall!(2, u32, this);
        let m = (t.add(0xC0) as *const u32).read_unaligned();
        if m == 0 {
            let head = (ebp as *const u32).read_unaligned();
            let r: u32 = callee_thiscall!(3, u32, hub, head, this, a1, a2);
            let _: u32 = callee_thiscall!(5, u32, this, 0, r);
            let f: u32 = callee_thiscall!(6, u32, this, 0);
            return (f != 0) as u32;
        }
        if m == 1 {
            let head2 = ((ebp.wrapping_add(4)) as *const u32).read_unaligned();
            let r: u32 = callee_thiscall!(3, u32, hub, head2, this, a1, a2);
            let _: u32 = callee_thiscall!(5, u32, this, 1, r);
            let f: u32 = callee_thiscall!(6, u32, this, 1);
            return (f != 0) as u32;
        }
        1
    }
});
