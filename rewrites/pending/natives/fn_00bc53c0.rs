// original: 0x00bc53c0 CREATE_CARS_ON_GENERATORS_IN_AREA
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `CREATE_CARS_ON_GENERATORS_IN_AREA`: forwards six float bounds and one integer flag to the engine.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
export!(cdecl, rn24_create_cars_on_generators_in_area(ctx: u32) -> () {
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        callee_cdecl!(1, (), unsafe { (args as *const f32).read().to_bits() }, unsafe { (args.add(1) as *const f32).read().to_bits() }, unsafe { (args.add(2) as *const f32).read().to_bits() }, unsafe { (args.add(3) as *const f32).read().to_bits() }, unsafe { (args.add(4) as *const f32).read().to_bits() }, unsafe { (args.add(5) as *const f32).read().to_bits() }, unsafe { args.add(6).read() });
});
