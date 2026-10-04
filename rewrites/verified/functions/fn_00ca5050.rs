// original: 0x00ca5050 CEventHandler::~CEventHandler_2
/// Guarded builder: when the second argument is null it does nothing and the
/// original returns entry garbage (the rewrite returns 0 there, unchecked
/// by the primary `ret: none` contract; a second contract with a non-null
/// argument checks EAX). Otherwise builds an initialised object exactly
/// like rw_00ca4f10 and stores it at this+0xC.
lf_rs75_rt::export!(thiscall, rw_00ca5050(this: u32, _a: u32, b: u32, _c: u32) -> u32 {
    unsafe {
        if b == 0 {
            return 0;
        }
        let mgr = *lf_rs75_rt::global::<u32>(0x0167E2A0);
        let obj: u32 = lf_rs75_rt::callee_thiscall!(1, u32, mgr);
        if obj == 0 {
            *((this + 0xC) as *mut u32) = 0;
            return 0;
        }
        const KIND: u32 = 0x19B;
        const SPEED: u32 = 0x3F800000; // 1.0f bits
        const RANGE: u32 = 0x40800000; // 4.0f bits
        let ans: u32 = lf_rs75_rt::callee_thiscall!(
            2,
            u32,
            obj,
            0,
            1,
            RANGE,
            KIND,
            lf_rs75_rt::relocated(0x00ED7B48),
            0,
            SPEED,
            0
        );
        *(obj as *mut u32) = lf_rs75_rt::relocated(0x00ED7AF4);
        *((this + 0xC) as *mut u32) = obj;
        ans
    }
});
