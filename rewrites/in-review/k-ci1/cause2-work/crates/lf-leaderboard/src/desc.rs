//! Per-leaderboard descriptors.
//!
//! GENERATED FILE - do not edit by hand. The generator reads every verified
//! family member's board constant from its rewrite text and checks that all
//! slots of one leaderboard agree (they do; any disagreement is reported,
//! not generated). The generator is not in the repository yet.

/// What varies between instantiations of the family: the board id the
/// slots fetch by, and the row-collector bound for the boards that have a
/// verified vf14 (`rows` is 0 where no verified vf14 pins it yet).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeaderboardDesc {
    /// Short leaderboard name (from the verified symbol, for debugging).
    pub board: &'static str,
    /// The board id every slot of this board fetches by.
    pub board_id: u32,
    /// The vf14 row count of this board (0 when unknown).
    pub rows: u32,
}

/// One descriptor per leaderboard, generated from the verified rewrites.
pub const DESCRIPTORS: &[LeaderboardDesc] = &[
    LeaderboardDesc {
        board: "Leaderboard_Race43NoHolds",
        board_id: 24,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Race44NoHolds",
        board_id: 25,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Race45NoHolds",
        board_id: 30,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Race46NoHolds",
        board_id: 31,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Race47NoHolds",
        board_id: 33,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Race48NoHolds",
        board_id: 34,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Race49NoHolds",
        board_id: 45,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Race50NoHolds",
        board_id: 35,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CompCarSteal",
        board_id: 9,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CompCarSteal_BG",
        board_id: 32,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CompDeathmatch",
        board_id: 7,
        rows: 6,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CompDeathmatch_BG",
        board_id: 23,
        rows: 4,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CompMafiaWork",
        board_id: 8,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CompMafiaWork_BG",
        board_id: 29,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CoopBombBase",
        board_id: 3,
        rows: 8,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CoopBombBase_BG",
        board_id: 40,
        rows: 6,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CoopBombBase_BG_TIME",
        board_id: 49,
        rows: 6,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CoopDrugFactory",
        board_id: 2,
        rows: 8,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CoopDrugFactory_BG",
        board_id: 41,
        rows: 6,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CoopDrugFactory_BG_TIME",
        board_id: 22,
        rows: 6,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CoopSwatAssault",
        board_id: 17,
        rows: 8,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CoopSwatAssault_BG",
        board_id: 39,
        rows: 6,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_CoopSwatAssault_BG_TIME",
        board_id: 21,
        rows: 6,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_0",
        board_id: 158,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_1",
        board_id: 161,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_10",
        board_id: 170,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_11",
        board_id: 171,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_12",
        board_id: 172,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_13",
        board_id: 173,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_14",
        board_id: 174,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_15",
        board_id: 210,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_16",
        board_id: 211,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_17",
        board_id: 212,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_2",
        board_id: 162,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_3",
        board_id: 163,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_4",
        board_id: 164,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_5",
        board_id: 165,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_6",
        board_id: 166,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_7",
        board_id: 168,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_8",
        board_id: 167,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_9",
        board_id: 169,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_0",
        board_id: 251,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_1",
        board_id: 159,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_10",
        board_id: 188,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_11",
        board_id: 189,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_12",
        board_id: 190,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_13",
        board_id: 191,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_14",
        board_id: 192,
        rows: 25,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_15",
        board_id: 193,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_16",
        board_id: 199,
        rows: 23,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_17",
        board_id: 200,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_18",
        board_id: 201,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_19",
        board_id: 202,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_2",
        board_id: 160,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_20",
        board_id: 203,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_21",
        board_id: 204,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_22",
        board_id: 205,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_23",
        board_id: 206,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_24",
        board_id: 207,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_25",
        board_id: 208,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_26",
        board_id: 182,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_3",
        board_id: 180,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_4",
        board_id: 181,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_5",
        board_id: 183,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_6",
        board_id: 184,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_7",
        board_id: 185,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_8",
        board_id: 186,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_BG_9",
        board_id: 187,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_0",
        board_id: 175,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_1",
        board_id: 176,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_10",
        board_id: 213,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_100",
        board_id: 303,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_101",
        board_id: 304,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_102",
        board_id: 305,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_103",
        board_id: 306,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_104",
        board_id: 307,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_105",
        board_id: 308,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_106",
        board_id: 309,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_107",
        board_id: 310,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_108",
        board_id: 311,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_109",
        board_id: 312,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_11",
        board_id: 214,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_110",
        board_id: 313,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_111",
        board_id: 314,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_112",
        board_id: 315,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_113",
        board_id: 316,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_114",
        board_id: 317,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_115",
        board_id: 318,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_116",
        board_id: 319,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_117",
        board_id: 320,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_118",
        board_id: 321,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_119",
        board_id: 322,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_12",
        board_id: 215,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_120",
        board_id: 323,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_121",
        board_id: 324,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_122",
        board_id: 325,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_123",
        board_id: 326,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_124",
        board_id: 327,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_125",
        board_id: 328,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_126",
        board_id: 329,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_127",
        board_id: 330,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_128",
        board_id: 331,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_129",
        board_id: 332,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_13",
        board_id: 216,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_130",
        board_id: 333,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_131",
        board_id: 334,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_132",
        board_id: 335,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_133",
        board_id: 336,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_134",
        board_id: 337,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_135",
        board_id: 351,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_136",
        board_id: 352,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_137",
        board_id: 353,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_138",
        board_id: 354,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_139",
        board_id: 355,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_14",
        board_id: 217,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_140",
        board_id: 356,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_141",
        board_id: 357,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_142",
        board_id: 358,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_143",
        board_id: 359,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_144",
        board_id: 360,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_145",
        board_id: 361,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_146",
        board_id: 362,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_147",
        board_id: 363,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_148",
        board_id: 364,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_149",
        board_id: 365,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_15",
        board_id: 218,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_150",
        board_id: 366,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_151",
        board_id: 367,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_152",
        board_id: 368,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_153",
        board_id: 369,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_154",
        board_id: 370,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_155",
        board_id: 371,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_156",
        board_id: 372,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_157",
        board_id: 373,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_158",
        board_id: 374,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_159",
        board_id: 375,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_16",
        board_id: 219,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_160",
        board_id: 376,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_161",
        board_id: 377,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_162",
        board_id: 378,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_163",
        board_id: 379,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_164",
        board_id: 380,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_165",
        board_id: 381,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_166",
        board_id: 382,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_167",
        board_id: 383,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_168",
        board_id: 384,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_169",
        board_id: 385,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_17",
        board_id: 220,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_170",
        board_id: 386,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_171",
        board_id: 387,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_172",
        board_id: 388,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_173",
        board_id: 389,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_174",
        board_id: 390,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_175",
        board_id: 391,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_176",
        board_id: 392,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_177",
        board_id: 393,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_178",
        board_id: 394,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_179",
        board_id: 395,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_18",
        board_id: 221,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_180",
        board_id: 396,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_181",
        board_id: 397,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_182",
        board_id: 398,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_183",
        board_id: 399,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_184",
        board_id: 400,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_185",
        board_id: 401,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_186",
        board_id: 402,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_187",
        board_id: 403,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_188",
        board_id: 404,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_189",
        board_id: 405,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_19",
        board_id: 222,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_190",
        board_id: 406,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_191",
        board_id: 407,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_192",
        board_id: 408,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_193",
        board_id: 409,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_194",
        board_id: 410,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_195",
        board_id: 411,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_196",
        board_id: 412,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_197",
        board_id: 413,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_198",
        board_id: 414,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_199",
        board_id: 415,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_2",
        board_id: 177,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_20",
        board_id: 223,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_200",
        board_id: 416,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_201",
        board_id: 417,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_202",
        board_id: 418,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_203",
        board_id: 419,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_204",
        board_id: 420,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_205",
        board_id: 421,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_206",
        board_id: 422,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_207",
        board_id: 423,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_208",
        board_id: 424,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_209",
        board_id: 425,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_21",
        board_id: 224,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_210",
        board_id: 426,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_211",
        board_id: 427,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_212",
        board_id: 428,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_213",
        board_id: 429,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_214",
        board_id: 430,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_215",
        board_id: 431,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_216",
        board_id: 432,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_217",
        board_id: 433,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_218",
        board_id: 434,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_219",
        board_id: 435,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_22",
        board_id: 225,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_220",
        board_id: 436,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_221",
        board_id: 437,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_222",
        board_id: 438,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_223",
        board_id: 439,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_224",
        board_id: 440,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_225",
        board_id: 441,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_226",
        board_id: 442,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_227",
        board_id: 443,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_228",
        board_id: 444,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_229",
        board_id: 445,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_23",
        board_id: 226,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_230",
        board_id: 446,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_231",
        board_id: 447,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_232",
        board_id: 448,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_233",
        board_id: 449,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_234",
        board_id: 450,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_235",
        board_id: 451,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_236",
        board_id: 452,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_237",
        board_id: 453,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_238",
        board_id: 454,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_239",
        board_id: 455,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_24",
        board_id: 227,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_240",
        board_id: 456,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_241",
        board_id: 457,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_242",
        board_id: 458,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_243",
        board_id: 459,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_244",
        board_id: 460,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_245",
        board_id: 461,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_246",
        board_id: 462,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_247",
        board_id: 463,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_248",
        board_id: 464,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_249",
        board_id: 465,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_25",
        board_id: 229,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_250",
        board_id: 466,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_251",
        board_id: 467,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_252",
        board_id: 468,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_253",
        board_id: 469,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_254",
        board_id: 470,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_255",
        board_id: 346,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_256",
        board_id: 347,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_257",
        board_id: 348,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_258",
        board_id: 349,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_259",
        board_id: 471,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_26",
        board_id: 233,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_260",
        board_id: 472,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_261",
        board_id: 473,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_262",
        board_id: 474,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_263",
        board_id: 475,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_264",
        board_id: 476,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_265",
        board_id: 477,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_266",
        board_id: 478,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_267",
        board_id: 479,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_268",
        board_id: 480,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_269",
        board_id: 481,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_27",
        board_id: 237,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_28",
        board_id: 239,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_29",
        board_id: 243,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_3",
        board_id: 178,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_30",
        board_id: 232,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_31",
        board_id: 235,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_32",
        board_id: 242,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_33",
        board_id: 245,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_34",
        board_id: 247,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_35",
        board_id: 241,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_36",
        board_id: 240,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_37",
        board_id: 234,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_38",
        board_id: 244,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_39",
        board_id: 246,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_4",
        board_id: 179,
        rows: 24,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_40",
        board_id: 236,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_41",
        board_id: 231,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_42",
        board_id: 230,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_43",
        board_id: 248,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_44",
        board_id: 228,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_45",
        board_id: 249,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_46",
        board_id: 238,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_47",
        board_id: 250,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_48",
        board_id: 209,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_49",
        board_id: 252,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_5",
        board_id: 194,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_50",
        board_id: 253,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_51",
        board_id: 254,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_52",
        board_id: 255,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_53",
        board_id: 256,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_54",
        board_id: 257,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_55",
        board_id: 258,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_56",
        board_id: 259,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_57",
        board_id: 260,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_58",
        board_id: 261,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_59",
        board_id: 262,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_6",
        board_id: 195,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_60",
        board_id: 263,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_61",
        board_id: 264,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_62",
        board_id: 265,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_63",
        board_id: 266,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_64",
        board_id: 267,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_65",
        board_id: 268,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_66",
        board_id: 269,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_67",
        board_id: 270,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_68",
        board_id: 271,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_69",
        board_id: 272,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_7",
        board_id: 196,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_70",
        board_id: 273,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_71",
        board_id: 274,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_72",
        board_id: 275,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_73",
        board_id: 276,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_74",
        board_id: 277,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_75",
        board_id: 278,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_76",
        board_id: 279,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_77",
        board_id: 280,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_78",
        board_id: 281,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_79",
        board_id: 282,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_8",
        board_id: 197,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_80",
        board_id: 283,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_81",
        board_id: 284,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_82",
        board_id: 285,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_83",
        board_id: 286,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_84",
        board_id: 287,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_85",
        board_id: 288,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_86",
        board_id: 289,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_87",
        board_id: 290,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_88",
        board_id: 291,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_89",
        board_id: 292,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_9",
        board_id: 198,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_90",
        board_id: 293,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_91",
        board_id: 294,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_92",
        board_id: 295,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_93",
        board_id: 296,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_94",
        board_id: 297,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_95",
        board_id: 298,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_96",
        board_id: 299,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_97",
        board_id: 300,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_98",
        board_id: 301,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Episodic_Race_99",
        board_id: 302,
        rows: 19,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_GamerMoney",
        board_id: 109,
        rows: 4,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race10NoHolds",
        board_id: 103,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race10Standard",
        board_id: 88,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race11NoHolds",
        board_id: 104,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race11Standard",
        board_id: 89,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race12NoHolds",
        board_id: 105,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race12Standard",
        board_id: 90,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race13NoHolds",
        board_id: 106,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race13Standard",
        board_id: 91,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race14NoHolds",
        board_id: 107,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race14Standard",
        board_id: 92,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race15NoHolds",
        board_id: 108,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race15Standard",
        board_id: 93,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race16NoHolds",
        board_id: 1,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race16Standard",
        board_id: 15,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race17NoHolds",
        board_id: 110,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race17Standard",
        board_id: 16,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race18NoHolds",
        board_id: 111,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race18Standard",
        board_id: 18,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race19NoHolds",
        board_id: 112,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race19Standard",
        board_id: 19,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race1NoHolds",
        board_id: 94,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race1Standard",
        board_id: 79,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race20NoHolds",
        board_id: 113,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race20Standard",
        board_id: 20,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race21NoHolds",
        board_id: 114,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race21Standard",
        board_id: 26,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race22NoHolds",
        board_id: 115,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race22Standard",
        board_id: 27,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race23NoHolds",
        board_id: 116,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race23Standard",
        board_id: 28,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race24NoHolds",
        board_id: 13,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race24Standard",
        board_id: 37,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race25NoHolds",
        board_id: 14,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race25Standard",
        board_id: 42,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race26NoHolds",
        board_id: 60,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race26Standard",
        board_id: 47,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race27NoHolds",
        board_id: 61,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race27Standard",
        board_id: 48,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race28NoHolds",
        board_id: 62,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race28Standard",
        board_id: 5,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race29NoHolds",
        board_id: 69,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race29Standard",
        board_id: 59,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race2NoHolds",
        board_id: 95,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race2Standard",
        board_id: 80,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race30NoHolds",
        board_id: 70,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race30Standard",
        board_id: 6,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race31NoHolds",
        board_id: 10,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race31Standard",
        board_id: 128,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race32NoHolds",
        board_id: 117,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race32Standard",
        board_id: 129,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race33NoHolds",
        board_id: 118,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race33Standard",
        board_id: 143,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race34NoHolds",
        board_id: 119,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race34Standard",
        board_id: 144,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race35NoHolds",
        board_id: 120,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race35Standard",
        board_id: 148,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race36NoHolds",
        board_id: 121,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race36Standard",
        board_id: 136,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race37NoHolds",
        board_id: 122,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race37Standard",
        board_id: 132,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race38NoHolds",
        board_id: 123,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race38Standard",
        board_id: 154,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race39NoHolds",
        board_id: 124,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race39Standard",
        board_id: 151,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race3NoHolds",
        board_id: 96,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race3Standard",
        board_id: 81,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race40NoHolds",
        board_id: 125,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race40Standard",
        board_id: 152,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race41NoHolds",
        board_id: 126,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race41Standard",
        board_id: 156,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race42NoHolds",
        board_id: 127,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race42Standard",
        board_id: 134,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race43NoHolds",
        board_id: 24,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race43Standard",
        board_id: 137,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race44NoHolds",
        board_id: 25,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race44Standard",
        board_id: 139,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race45NoHolds",
        board_id: 30,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race45Standard",
        board_id: 135,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race46NoHolds",
        board_id: 31,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race46Standard",
        board_id: 140,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race47NoHolds",
        board_id: 33,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race47Standard",
        board_id: 141,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race48NoHolds",
        board_id: 34,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race48Standard",
        board_id: 146,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race49NoHolds",
        board_id: 45,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race49Standard",
        board_id: 153,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race4NoHolds",
        board_id: 97,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race4Standard",
        board_id: 82,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race50NoHolds",
        board_id: 35,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race50Standard",
        board_id: 142,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race51NoHolds",
        board_id: 53,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race51Standard",
        board_id: 145,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race52NoHolds",
        board_id: 65,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race52Standard",
        board_id: 131,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race53NoHolds",
        board_id: 57,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race53Standard",
        board_id: 138,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race54NoHolds",
        board_id: 66,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race54Standard",
        board_id: 149,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race55NoHolds",
        board_id: 67,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race55Standard",
        board_id: 147,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race56NoHolds",
        board_id: 68,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race56Standard",
        board_id: 130,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race57NoHolds",
        board_id: 72,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race57Standard",
        board_id: 133,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race58NoHolds",
        board_id: 71,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race58Standard",
        board_id: 155,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race59NoHolds",
        board_id: 74,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race59Standard",
        board_id: 150,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race5NoHolds",
        board_id: 98,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race5Standard",
        board_id: 83,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race60NoHolds",
        board_id: 76,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race60Standard",
        board_id: 157,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race6NoHolds",
        board_id: 99,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race6Standard",
        board_id: 84,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race7NoHolds",
        board_id: 100,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race7Standard",
        board_id: 85,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race8NoHolds",
        board_id: 101,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race8Standard",
        board_id: 86,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race9NoHolds",
        board_id: 102,
        rows: 0,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_Race9Standard",
        board_id: 87,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_TeamCarSteal",
        board_id: 46,
        rows: 9,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_TeamCarSteal_BG",
        board_id: 56,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_TeamDeathmatch",
        board_id: 4,
        rows: 8,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_TeamDeathmatch_BG",
        board_id: 52,
        rows: 6,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_TeamGangBase",
        board_id: 12,
        rows: 9,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_TeamGangBase_BG",
        board_id: 38,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_TeamMafya",
        board_id: 44,
        rows: 9,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_TeamMafya_BG",
        board_id: 54,
        rows: 7,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_TeamVip",
        board_id: 11,
        rows: 8,
    },
    LeaderboardDesc {
        board: "Leaderboard_Ranked_TeamVip_BG",
        board_id: 36,
        rows: 6,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_CompCarSteal",
        board_id: 43,
        rows: 4,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_CompDeathmatch",
        board_id: 51,
        rows: 4,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_CompMafiaWork",
        board_id: 55,
        rows: 4,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_CoopBombBase_BG_TIME",
        board_id: 50,
        rows: 6,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_CoopDrugFactory_BG_TIME",
        board_id: 77,
        rows: 6,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_CoopSwatAssault_BG_TIME",
        board_id: 78,
        rows: 6,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_Episodic_0",
        board_id: 338,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_Episodic_1",
        board_id: 339,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_Episodic_2",
        board_id: 341,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_Episodic_3",
        board_id: 340,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_Episodic_4",
        board_id: 342,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_Episodic_5",
        board_id: 343,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_Episodic_6",
        board_id: 344,
        rows: 26,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_TeamCarSteal",
        board_id: 64,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_TeamDeathmatch",
        board_id: 58,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_TeamGangBase",
        board_id: 73,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_TeamMafya",
        board_id: 63,
        rows: 5,
    },
    LeaderboardDesc {
        board: "Leaderboard_Standard_TeamVip",
        board_id: 75,
        rows: 5,
    },
];

/// Finds a board's descriptor by its id.
#[must_use]
pub fn describe(board_id: u32) -> Option<&'static LeaderboardDesc> {
    DESCRIPTORS.iter().find(|d| d.board_id == board_id)
}

/// Boards with verified members but no known board id (only constructor
/// and/or probe rewrites verified, which carry no board constant).
pub const BOARDS_WITHOUT_IDS: &[&str] = &["rlConcreteLeaderboardInfo_Race111_ctor"];
