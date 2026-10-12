// One entry per instance in this crate: its rewrite and its three globals.
pub const INSTANCES: &[Instance] = &[
    Instance { name: "vehicle_tuning_ratio_init_19", address: "0x00E6A7C0", call: RefreshFn::Unit(rewrites::fn_00E6A7C0::rw_00e6a7c0), num: 0x01048274, den: 0x01048278, dst: 0x016B9D74 },
    Instance { name: "vehicle_tuning_ratio_init_20", address: "0x00E6A860", call: RefreshFn::Unit(rewrites::fn_00E6A860::rw_00e6a860), num: 0x01048340, den: 0x01048344, dst: 0x016C8E04 },
    Instance { name: "vehicle_tuning_ratio_init_21", address: "0x00E6AA50", call: RefreshFn::Unit(rewrites::fn_00E6AA50::rw_00e6aa50), num: 0x01048400, den: 0x01048404, dst: 0x016C8F90 },
    Instance { name: "vehicle_tuning_ratio_init_22", address: "0x00E6AAB0", call: RefreshFn::Unit(rewrites::fn_00E6AAB0::rw_00e6aab0), num: 0x0104842C, den: 0x01048430, dst: 0x016CF3C0 },
    Instance { name: "vehicle_tuning_ratio_init_23", address: "0x00E6AAF0", call: RefreshFn::Unit(rewrites::fn_00E6AAF0::rw_00e6aaf0), num: 0x01048448, den: 0x0104844C, dst: 0x016CF3C4 },
    Instance { name: "vehicle_tuning_ratio_init_24", address: "0x00E6AB10", call: RefreshFn::Unit(rewrites::fn_00E6AB10::rw_00e6ab10), num: 0x01048464, den: 0x01048468, dst: 0x016CFD74 },
    Instance { name: "vehicle_tuning_ratio_init_25", address: "0x00E6AB50", call: RefreshFn::Unit(rewrites::fn_00E6AB50::rw_00e6ab50), num: 0x010484D0, den: 0x010484D4, dst: 0x016CFD78 },
    Instance { name: "vehicle_tuning_ratio_init_26", address: "0x00E6AB70", call: RefreshFn::Unit(rewrites::fn_00E6AB70::rw_00e6ab70), num: 0x01048500, den: 0x01048504, dst: 0x016D00E8 },
    Instance { name: "vehicle_tuning_ratio_init_27", address: "0x00E6AB90", call: RefreshFn::Unit(rewrites::fn_00E6AB90::rw_00e6ab90), num: 0x01048A70, den: 0x01048A74, dst: 0x016D11F8 },
    Instance { name: "vehicle_tuning_ratio_init_28", address: "0x00E6ABB0", call: RefreshFn::Unit(rewrites::fn_00E6ABB0::rw_00e6abb0), num: 0x01048AAC, den: 0x01048AB0, dst: 0x016D2110 },
];
