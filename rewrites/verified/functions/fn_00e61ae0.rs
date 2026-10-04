// original: 0x00e61ae0 init_pool512_and_register_00
use lf_checker_rt::{callee_cdecl, export, relocated};

// Rewrite of the original at 0x00e61ae0 (`init_pool512_and_register_00`): Clear one 512-record object pool, submit that subsystem teardown routine, return the status.
//
// Takes no arguments; returns whatever the registry helper returns.
export!(cdecl, rw_00e61ae0() -> u32 {
    const RECORDS: usize = 512;
    const STRIDE: usize = 0x2C;
    // First record starts one header below the anchor the original loads.
    const FIRST: u32 = 0x1A05880;
    unsafe {
        let pool = lf_checker_rt::global::<u8>(FIRST);
        for i in 0..RECORDS {
            let rec = pool.add(i.wrapping_mul(STRIDE));
            rec.cast::<u32>().write(0);
            rec.add(0x04).cast::<u32>().write(0);
            rec.add(0x08).cast::<u32>().write(0);
            rec.add(0x0C).cast::<u32>().write(0xFFFF_FFFF);
            rec.add(0x10).cast::<u16>().write(0);
            rec.add(0x14).cast::<u32>().write(0xFFFF_FFFF);
            rec.add(0x18).cast::<u16>().write(0);
            rec.add(0x1C).cast::<u64>().write_unaligned(0);
            rec.add(0x24).cast::<u64>().write_unaligned(0);
        }
    }
    callee_cdecl!(0, u32, relocated(0x00E70500))
});
