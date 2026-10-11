// original: 0x009C2560 stream_state_storage_reset

/// Reset the streaming state fields on `this`, clear the request counter at
/// `+0x10`, zero the three byte flags, and fill the 570-word scratch table at
/// `+0xFA98` with `u32::MAX`. The object layout is fixed by the original's
/// 32-bit offsets. Returns the same object pointer in EAX.
lf_checker_rt::export!(thiscall, rw_009c2560(this: u32) -> u32 {
    unsafe {
        const REQUEST_COUNTER: usize = 0x10;
        const SCRATCH_TABLE: usize = 0xfa98;
        const SCRATCH_WORDS: usize = 0x23a;
        const STATE_DWORDS: [usize; 10] = [
            0x10404, 0x10408, 0x103fc, 0x10400, 0x103e0,
            0x103e4, 0x103e8, 0x103f0, 0x103f4, 0x103f8,
        ];
        const STATE_BYTES: [usize; 4] = [0x1040c, 0x10380, 0x103a0, 0x103c0];
        let base = this as usize as *mut u8;
        for offset in STATE_DWORDS {
            base.add(offset).cast::<u32>().write_unaligned(0);
        }
        base.add(REQUEST_COUNTER).cast::<u32>().write_unaligned(0);
        for offset in STATE_BYTES {
            base.add(offset).write(0);
        }
        for index in 0..SCRATCH_WORDS {
            base.add(SCRATCH_TABLE + index * core::mem::size_of::<u32>())
                .cast::<u32>()
                .write_unaligned(u32::MAX);
        }
        this
    }
});
