#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Shadow registers unlock register"]
    pub unlk: UNLK,
    #[doc = "0x04 - Counter start register"]
    pub sta: STA,
    #[doc = "0x08 - Counter reload register"]
    pub rld: RLD,
    #[doc = "0x0c - Comparator register"]
    pub cmp_0: CMP_0,
    #[doc = "0x10 - Comparator register"]
    pub cmp_1: CMP_1,
    #[doc = "0x14 - Comparator register"]
    pub cmp_2: CMP_2,
    #[doc = "0x18 - Comparator register"]
    pub cmp_3: CMP_3,
    #[doc = "0x1c - Comparator register"]
    pub cmp_4: CMP_4,
    #[doc = "0x20 - Comparator register"]
    pub cmp_5: CMP_5,
    #[doc = "0x24 - Comparator register"]
    pub cmp_6: CMP_6,
    #[doc = "0x28 - Comparator register"]
    pub cmp_7: CMP_7,
    #[doc = "0x2c - Comparator register"]
    pub cmp_8: CMP_8,
    #[doc = "0x30 - Comparator register"]
    pub cmp_9: CMP_9,
    #[doc = "0x34 - Comparator register"]
    pub cmp_10: CMP_10,
    #[doc = "0x38 - Comparator register"]
    pub cmp_11: CMP_11,
    #[doc = "0x3c - Comparator register"]
    pub cmp_12: CMP_12,
    #[doc = "0x40 - Comparator register"]
    pub cmp_13: CMP_13,
    #[doc = "0x44 - Comparator register"]
    pub cmp_14: CMP_14,
    #[doc = "0x48 - Comparator register"]
    pub cmp_15: CMP_15,
    #[doc = "0x4c - Comparator register"]
    pub cmp_16: CMP_16,
    #[doc = "0x50 - Comparator register"]
    pub cmp_17: CMP_17,
    #[doc = "0x54 - Comparator register"]
    pub cmp_18: CMP_18,
    #[doc = "0x58 - Comparator register"]
    pub cmp_19: CMP_19,
    #[doc = "0x5c - Comparator register"]
    pub cmp_20: CMP_20,
    #[doc = "0x60 - Comparator register"]
    pub cmp_21: CMP_21,
    #[doc = "0x64 - Comparator register"]
    pub cmp_22: CMP_22,
    #[doc = "0x68 - Comparator register"]
    pub cmp_23: CMP_23,
    _reserved27: [u8; 0x0c],
    #[doc = "0x78 - Force output mode register"]
    pub frcmd: FRCMD,
    #[doc = "0x7c - Shadow registers lock register"]
    pub shlk: SHLK,
    #[doc = "0x80 - Output channel configure register"]
    pub chcfg_0: CHCFG_0,
    #[doc = "0x84 - Output channel configure register"]
    pub chcfg_1: CHCFG_1,
    #[doc = "0x88 - Output channel configure register"]
    pub chcfg_2: CHCFG_2,
    #[doc = "0x8c - Output channel configure register"]
    pub chcfg_3: CHCFG_3,
    #[doc = "0x90 - Output channel configure register"]
    pub chcfg_4: CHCFG_4,
    #[doc = "0x94 - Output channel configure register"]
    pub chcfg_5: CHCFG_5,
    #[doc = "0x98 - Output channel configure register"]
    pub chcfg_6: CHCFG_6,
    #[doc = "0x9c - Output channel configure register"]
    pub chcfg_7: CHCFG_7,
    #[doc = "0xa0 - Output channel configure register"]
    pub chcfg_8: CHCFG_8,
    #[doc = "0xa4 - Output channel configure register"]
    pub chcfg_9: CHCFG_9,
    #[doc = "0xa8 - Output channel configure register"]
    pub chcfg_10: CHCFG_10,
    #[doc = "0xac - Output channel configure register"]
    pub chcfg_11: CHCFG_11,
    #[doc = "0xb0 - Output channel configure register"]
    pub chcfg_12: CHCFG_12,
    #[doc = "0xb4 - Output channel configure register"]
    pub chcfg_13: CHCFG_13,
    #[doc = "0xb8 - Output channel configure register"]
    pub chcfg_14: CHCFG_14,
    #[doc = "0xbc - Output channel configure register"]
    pub chcfg_15: CHCFG_15,
    #[doc = "0xc0 - Output channel configure register"]
    pub chcfg_16: CHCFG_16,
    #[doc = "0xc4 - Output channel configure register"]
    pub chcfg_17: CHCFG_17,
    #[doc = "0xc8 - Output channel configure register"]
    pub chcfg_18: CHCFG_18,
    #[doc = "0xcc - Output channel configure register"]
    pub chcfg_19: CHCFG_19,
    #[doc = "0xd0 - Output channel configure register"]
    pub chcfg_20: CHCFG_20,
    #[doc = "0xd4 - Output channel configure register"]
    pub chcfg_21: CHCFG_21,
    #[doc = "0xd8 - Output channel configure register"]
    pub chcfg_22: CHCFG_22,
    #[doc = "0xdc - Output channel configure register"]
    pub chcfg_23: CHCFG_23,
    _reserved53: [u8; 0x10],
    #[doc = "0xf0 - Global control register"]
    pub gcr: GCR,
    #[doc = "0xf4 - Shadow register control register"]
    pub shcr: SHCR,
    _reserved55: [u8; 0x08],
    #[doc = "0x100 - Capture rising edge register"]
    pub cappos_0: CAPPOS_0,
    #[doc = "0x104 - Capture rising edge register"]
    pub cappos_1: CAPPOS_1,
    #[doc = "0x108 - Capture rising edge register"]
    pub cappos_2: CAPPOS_2,
    #[doc = "0x10c - Capture rising edge register"]
    pub cappos_3: CAPPOS_3,
    #[doc = "0x110 - Capture rising edge register"]
    pub cappos_4: CAPPOS_4,
    #[doc = "0x114 - Capture rising edge register"]
    pub cappos_5: CAPPOS_5,
    #[doc = "0x118 - Capture rising edge register"]
    pub cappos_6: CAPPOS_6,
    #[doc = "0x11c - Capture rising edge register"]
    pub cappos_7: CAPPOS_7,
    #[doc = "0x120 - Capture rising edge register"]
    pub cappos_8: CAPPOS_8,
    #[doc = "0x124 - Capture rising edge register"]
    pub cappos_9: CAPPOS_9,
    #[doc = "0x128 - Capture rising edge register"]
    pub cappos_10: CAPPOS_10,
    #[doc = "0x12c - Capture rising edge register"]
    pub cappos_11: CAPPOS_11,
    #[doc = "0x130 - Capture rising edge register"]
    pub cappos_12: CAPPOS_12,
    #[doc = "0x134 - Capture rising edge register"]
    pub cappos_13: CAPPOS_13,
    #[doc = "0x138 - Capture rising edge register"]
    pub cappos_14: CAPPOS_14,
    #[doc = "0x13c - Capture rising edge register"]
    pub cappos_15: CAPPOS_15,
    #[doc = "0x140 - Capture rising edge register"]
    pub cappos_16: CAPPOS_16,
    #[doc = "0x144 - Capture rising edge register"]
    pub cappos_17: CAPPOS_17,
    #[doc = "0x148 - Capture rising edge register"]
    pub cappos_18: CAPPOS_18,
    #[doc = "0x14c - Capture rising edge register"]
    pub cappos_19: CAPPOS_19,
    #[doc = "0x150 - Capture rising edge register"]
    pub cappos_20: CAPPOS_20,
    #[doc = "0x154 - Capture rising edge register"]
    pub cappos_21: CAPPOS_21,
    #[doc = "0x158 - Capture rising edge register"]
    pub cappos_22: CAPPOS_22,
    #[doc = "0x15c - Capture rising edge register"]
    pub cappos_23: CAPPOS_23,
    _reserved79: [u8; 0x10],
    #[doc = "0x170 - Counter"]
    pub cnt: CNT,
    _reserved80: [u8; 0x0c],
    #[doc = "0x180 - Capture falling edge register"]
    pub capneg_0: CAPNEG_0,
    #[doc = "0x184 - Capture falling edge register"]
    pub capneg_1: CAPNEG_1,
    #[doc = "0x188 - Capture falling edge register"]
    pub capneg_2: CAPNEG_2,
    #[doc = "0x18c - Capture falling edge register"]
    pub capneg_3: CAPNEG_3,
    #[doc = "0x190 - Capture falling edge register"]
    pub capneg_4: CAPNEG_4,
    #[doc = "0x194 - Capture falling edge register"]
    pub capneg_5: CAPNEG_5,
    #[doc = "0x198 - Capture falling edge register"]
    pub capneg_6: CAPNEG_6,
    #[doc = "0x19c - Capture falling edge register"]
    pub capneg_7: CAPNEG_7,
    #[doc = "0x1a0 - Capture falling edge register"]
    pub capneg_8: CAPNEG_8,
    #[doc = "0x1a4 - Capture falling edge register"]
    pub capneg_9: CAPNEG_9,
    #[doc = "0x1a8 - Capture falling edge register"]
    pub capneg_10: CAPNEG_10,
    #[doc = "0x1ac - Capture falling edge register"]
    pub capneg_11: CAPNEG_11,
    #[doc = "0x1b0 - Capture falling edge register"]
    pub capneg_12: CAPNEG_12,
    #[doc = "0x1b4 - Capture falling edge register"]
    pub capneg_13: CAPNEG_13,
    #[doc = "0x1b8 - Capture falling edge register"]
    pub capneg_14: CAPNEG_14,
    #[doc = "0x1bc - Capture falling edge register"]
    pub capneg_15: CAPNEG_15,
    #[doc = "0x1c0 - Capture falling edge register"]
    pub capneg_16: CAPNEG_16,
    #[doc = "0x1c4 - Capture falling edge register"]
    pub capneg_17: CAPNEG_17,
    #[doc = "0x1c8 - Capture falling edge register"]
    pub capneg_18: CAPNEG_18,
    #[doc = "0x1cc - Capture falling edge register"]
    pub capneg_19: CAPNEG_19,
    #[doc = "0x1d0 - Capture falling edge register"]
    pub capneg_20: CAPNEG_20,
    #[doc = "0x1d4 - Capture falling edge register"]
    pub capneg_21: CAPNEG_21,
    #[doc = "0x1d8 - Capture falling edge register"]
    pub capneg_22: CAPNEG_22,
    #[doc = "0x1dc - Capture falling edge register"]
    pub capneg_23: CAPNEG_23,
    _reserved104: [u8; 0x10],
    #[doc = "0x1f0 - Counter copy"]
    pub cntcopy: CNTCOPY,
    _reserved105: [u8; 0x0c],
    #[doc = "0x200 - PWM channel configure register"]
    pub pwmcfg_0: PWMCFG_0,
    #[doc = "0x204 - PWM channel configure register"]
    pub pwmcfg_1: PWMCFG_1,
    #[doc = "0x208 - PWM channel configure register"]
    pub pwmcfg_2: PWMCFG_2,
    #[doc = "0x20c - PWM channel configure register"]
    pub pwmcfg_3: PWMCFG_3,
    #[doc = "0x210 - PWM channel configure register"]
    pub pwmcfg_4: PWMCFG_4,
    #[doc = "0x214 - PWM channel configure register"]
    pub pwmcfg_5: PWMCFG_5,
    #[doc = "0x218 - PWM channel configure register"]
    pub pwmcfg_6: PWMCFG_6,
    #[doc = "0x21c - PWM channel configure register"]
    pub pwmcfg_7: PWMCFG_7,
    #[doc = "0x220 - Status register"]
    pub sr: SR,
    #[doc = "0x224 - Interrupt request enable register"]
    pub irqen: IRQEN,
    _reserved115: [u8; 0x04],
    #[doc = "0x22c - DMA request enable register"]
    pub dmaen: DMAEN,
    #[doc = "0x230 - Comparator configure register"]
    pub cmpcfg_cmpcfg0: CMPCFG_CMPCFG0,
    #[doc = "0x234 - Comparator configure register"]
    pub cmpcfg_1: CMPCFG_1,
    #[doc = "0x238 - Comparator configure register"]
    pub cmpcfg_2: CMPCFG_2,
    #[doc = "0x23c - Comparator configure register"]
    pub cmpcfg_3: CMPCFG_3,
    #[doc = "0x240 - Comparator configure register"]
    pub cmpcfg_4: CMPCFG_4,
    #[doc = "0x244 - Comparator configure register"]
    pub cmpcfg_5: CMPCFG_5,
    #[doc = "0x248 - Comparator configure register"]
    pub cmpcfg_6: CMPCFG_6,
    #[doc = "0x24c - Comparator configure register"]
    pub cmpcfg_7: CMPCFG_7,
    #[doc = "0x250 - Comparator configure register"]
    pub cmpcfg_8: CMPCFG_8,
    #[doc = "0x254 - Comparator configure register"]
    pub cmpcfg_9: CMPCFG_9,
    #[doc = "0x258 - Comparator configure register"]
    pub cmpcfg_10: CMPCFG_10,
    #[doc = "0x25c - Comparator configure register"]
    pub cmpcfg_11: CMPCFG_11,
    #[doc = "0x260 - Comparator configure register"]
    pub cmpcfg_12: CMPCFG_12,
    #[doc = "0x264 - Comparator configure register"]
    pub cmpcfg_13: CMPCFG_13,
    #[doc = "0x268 - Comparator configure register"]
    pub cmpcfg_14: CMPCFG_14,
    #[doc = "0x26c - Comparator configure register"]
    pub cmpcfg_15: CMPCFG_15,
    #[doc = "0x270 - Comparator configure register"]
    pub cmpcfg_16: CMPCFG_16,
    #[doc = "0x274 - Comparator configure register"]
    pub cmpcfg_17: CMPCFG_17,
    #[doc = "0x278 - Comparator configure register"]
    pub cmpcfg_18: CMPCFG_18,
    #[doc = "0x27c - Comparator configure register"]
    pub cmpcfg_19: CMPCFG_19,
    #[doc = "0x280 - Comparator configure register"]
    pub cmpcfg_20: CMPCFG_20,
    #[doc = "0x284 - Comparator configure register"]
    pub cmpcfg_21: CMPCFG_21,
    #[doc = "0x288 - Comparator configure register"]
    pub cmpcfg_22: CMPCFG_22,
    #[doc = "0x28c - Comparator configure register"]
    pub cmpcfg_23: CMPCFG_23,
}
#[doc = "UNLK (rw) register accessor: an alias for `Reg<UNLK_SPEC>`"]
pub type UNLK = crate::Reg<unlk::UNLK_SPEC>;
#[doc = "Shadow registers unlock register"]
pub mod unlk;
#[doc = "STA (rw) register accessor: an alias for `Reg<STA_SPEC>`"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "Counter start register"]
pub mod sta;
#[doc = "RLD (rw) register accessor: an alias for `Reg<RLD_SPEC>`"]
pub type RLD = crate::Reg<rld::RLD_SPEC>;
#[doc = "Counter reload register"]
pub mod rld;
#[doc = "CMP_0 (rw) register accessor: an alias for `Reg<CMP_0_SPEC>`"]
pub type CMP_0 = crate::Reg<cmp_0::CMP_0_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_0;
#[doc = "CMP_1 (rw) register accessor: an alias for `Reg<CMP_1_SPEC>`"]
pub type CMP_1 = crate::Reg<cmp_1::CMP_1_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_1;
#[doc = "CMP_2 (rw) register accessor: an alias for `Reg<CMP_2_SPEC>`"]
pub type CMP_2 = crate::Reg<cmp_2::CMP_2_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_2;
#[doc = "CMP_3 (rw) register accessor: an alias for `Reg<CMP_3_SPEC>`"]
pub type CMP_3 = crate::Reg<cmp_3::CMP_3_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_3;
#[doc = "CMP_4 (rw) register accessor: an alias for `Reg<CMP_4_SPEC>`"]
pub type CMP_4 = crate::Reg<cmp_4::CMP_4_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_4;
#[doc = "CMP_5 (rw) register accessor: an alias for `Reg<CMP_5_SPEC>`"]
pub type CMP_5 = crate::Reg<cmp_5::CMP_5_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_5;
#[doc = "CMP_6 (rw) register accessor: an alias for `Reg<CMP_6_SPEC>`"]
pub type CMP_6 = crate::Reg<cmp_6::CMP_6_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_6;
#[doc = "CMP_7 (rw) register accessor: an alias for `Reg<CMP_7_SPEC>`"]
pub type CMP_7 = crate::Reg<cmp_7::CMP_7_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_7;
#[doc = "CMP_8 (rw) register accessor: an alias for `Reg<CMP_8_SPEC>`"]
pub type CMP_8 = crate::Reg<cmp_8::CMP_8_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_8;
#[doc = "CMP_9 (rw) register accessor: an alias for `Reg<CMP_9_SPEC>`"]
pub type CMP_9 = crate::Reg<cmp_9::CMP_9_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_9;
#[doc = "CMP_10 (rw) register accessor: an alias for `Reg<CMP_10_SPEC>`"]
pub type CMP_10 = crate::Reg<cmp_10::CMP_10_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_10;
#[doc = "CMP_11 (rw) register accessor: an alias for `Reg<CMP_11_SPEC>`"]
pub type CMP_11 = crate::Reg<cmp_11::CMP_11_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_11;
#[doc = "CMP_12 (rw) register accessor: an alias for `Reg<CMP_12_SPEC>`"]
pub type CMP_12 = crate::Reg<cmp_12::CMP_12_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_12;
#[doc = "CMP_13 (rw) register accessor: an alias for `Reg<CMP_13_SPEC>`"]
pub type CMP_13 = crate::Reg<cmp_13::CMP_13_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_13;
#[doc = "CMP_14 (rw) register accessor: an alias for `Reg<CMP_14_SPEC>`"]
pub type CMP_14 = crate::Reg<cmp_14::CMP_14_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_14;
#[doc = "CMP_15 (rw) register accessor: an alias for `Reg<CMP_15_SPEC>`"]
pub type CMP_15 = crate::Reg<cmp_15::CMP_15_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_15;
#[doc = "CMP_16 (rw) register accessor: an alias for `Reg<CMP_16_SPEC>`"]
pub type CMP_16 = crate::Reg<cmp_16::CMP_16_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_16;
#[doc = "CMP_17 (rw) register accessor: an alias for `Reg<CMP_17_SPEC>`"]
pub type CMP_17 = crate::Reg<cmp_17::CMP_17_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_17;
#[doc = "CMP_18 (rw) register accessor: an alias for `Reg<CMP_18_SPEC>`"]
pub type CMP_18 = crate::Reg<cmp_18::CMP_18_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_18;
#[doc = "CMP_19 (rw) register accessor: an alias for `Reg<CMP_19_SPEC>`"]
pub type CMP_19 = crate::Reg<cmp_19::CMP_19_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_19;
#[doc = "CMP_20 (rw) register accessor: an alias for `Reg<CMP_20_SPEC>`"]
pub type CMP_20 = crate::Reg<cmp_20::CMP_20_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_20;
#[doc = "CMP_21 (rw) register accessor: an alias for `Reg<CMP_21_SPEC>`"]
pub type CMP_21 = crate::Reg<cmp_21::CMP_21_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_21;
#[doc = "CMP_22 (rw) register accessor: an alias for `Reg<CMP_22_SPEC>`"]
pub type CMP_22 = crate::Reg<cmp_22::CMP_22_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_22;
#[doc = "CMP_23 (rw) register accessor: an alias for `Reg<CMP_23_SPEC>`"]
pub type CMP_23 = crate::Reg<cmp_23::CMP_23_SPEC>;
#[doc = "Comparator register"]
pub mod cmp_23;
#[doc = "FRCMD (rw) register accessor: an alias for `Reg<FRCMD_SPEC>`"]
pub type FRCMD = crate::Reg<frcmd::FRCMD_SPEC>;
#[doc = "Force output mode register"]
pub mod frcmd;
#[doc = "SHLK (rw) register accessor: an alias for `Reg<SHLK_SPEC>`"]
pub type SHLK = crate::Reg<shlk::SHLK_SPEC>;
#[doc = "Shadow registers lock register"]
pub mod shlk;
#[doc = "CHCFG_0 (rw) register accessor: an alias for `Reg<CHCFG_0_SPEC>`"]
pub type CHCFG_0 = crate::Reg<chcfg_0::CHCFG_0_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_0;
#[doc = "CHCFG_1 (rw) register accessor: an alias for `Reg<CHCFG_1_SPEC>`"]
pub type CHCFG_1 = crate::Reg<chcfg_1::CHCFG_1_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_1;
#[doc = "CHCFG_2 (rw) register accessor: an alias for `Reg<CHCFG_2_SPEC>`"]
pub type CHCFG_2 = crate::Reg<chcfg_2::CHCFG_2_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_2;
#[doc = "CHCFG_3 (rw) register accessor: an alias for `Reg<CHCFG_3_SPEC>`"]
pub type CHCFG_3 = crate::Reg<chcfg_3::CHCFG_3_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_3;
#[doc = "CHCFG_4 (rw) register accessor: an alias for `Reg<CHCFG_4_SPEC>`"]
pub type CHCFG_4 = crate::Reg<chcfg_4::CHCFG_4_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_4;
#[doc = "CHCFG_5 (rw) register accessor: an alias for `Reg<CHCFG_5_SPEC>`"]
pub type CHCFG_5 = crate::Reg<chcfg_5::CHCFG_5_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_5;
#[doc = "CHCFG_6 (rw) register accessor: an alias for `Reg<CHCFG_6_SPEC>`"]
pub type CHCFG_6 = crate::Reg<chcfg_6::CHCFG_6_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_6;
#[doc = "CHCFG_7 (rw) register accessor: an alias for `Reg<CHCFG_7_SPEC>`"]
pub type CHCFG_7 = crate::Reg<chcfg_7::CHCFG_7_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_7;
#[doc = "CHCFG_8 (rw) register accessor: an alias for `Reg<CHCFG_8_SPEC>`"]
pub type CHCFG_8 = crate::Reg<chcfg_8::CHCFG_8_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_8;
#[doc = "CHCFG_9 (rw) register accessor: an alias for `Reg<CHCFG_9_SPEC>`"]
pub type CHCFG_9 = crate::Reg<chcfg_9::CHCFG_9_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_9;
#[doc = "CHCFG_10 (rw) register accessor: an alias for `Reg<CHCFG_10_SPEC>`"]
pub type CHCFG_10 = crate::Reg<chcfg_10::CHCFG_10_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_10;
#[doc = "CHCFG_11 (rw) register accessor: an alias for `Reg<CHCFG_11_SPEC>`"]
pub type CHCFG_11 = crate::Reg<chcfg_11::CHCFG_11_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_11;
#[doc = "CHCFG_12 (rw) register accessor: an alias for `Reg<CHCFG_12_SPEC>`"]
pub type CHCFG_12 = crate::Reg<chcfg_12::CHCFG_12_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_12;
#[doc = "CHCFG_13 (rw) register accessor: an alias for `Reg<CHCFG_13_SPEC>`"]
pub type CHCFG_13 = crate::Reg<chcfg_13::CHCFG_13_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_13;
#[doc = "CHCFG_14 (rw) register accessor: an alias for `Reg<CHCFG_14_SPEC>`"]
pub type CHCFG_14 = crate::Reg<chcfg_14::CHCFG_14_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_14;
#[doc = "CHCFG_15 (rw) register accessor: an alias for `Reg<CHCFG_15_SPEC>`"]
pub type CHCFG_15 = crate::Reg<chcfg_15::CHCFG_15_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_15;
#[doc = "CHCFG_16 (rw) register accessor: an alias for `Reg<CHCFG_16_SPEC>`"]
pub type CHCFG_16 = crate::Reg<chcfg_16::CHCFG_16_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_16;
#[doc = "CHCFG_17 (rw) register accessor: an alias for `Reg<CHCFG_17_SPEC>`"]
pub type CHCFG_17 = crate::Reg<chcfg_17::CHCFG_17_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_17;
#[doc = "CHCFG_18 (rw) register accessor: an alias for `Reg<CHCFG_18_SPEC>`"]
pub type CHCFG_18 = crate::Reg<chcfg_18::CHCFG_18_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_18;
#[doc = "CHCFG_19 (rw) register accessor: an alias for `Reg<CHCFG_19_SPEC>`"]
pub type CHCFG_19 = crate::Reg<chcfg_19::CHCFG_19_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_19;
#[doc = "CHCFG_20 (rw) register accessor: an alias for `Reg<CHCFG_20_SPEC>`"]
pub type CHCFG_20 = crate::Reg<chcfg_20::CHCFG_20_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_20;
#[doc = "CHCFG_21 (rw) register accessor: an alias for `Reg<CHCFG_21_SPEC>`"]
pub type CHCFG_21 = crate::Reg<chcfg_21::CHCFG_21_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_21;
#[doc = "CHCFG_22 (rw) register accessor: an alias for `Reg<CHCFG_22_SPEC>`"]
pub type CHCFG_22 = crate::Reg<chcfg_22::CHCFG_22_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_22;
#[doc = "CHCFG_23 (rw) register accessor: an alias for `Reg<CHCFG_23_SPEC>`"]
pub type CHCFG_23 = crate::Reg<chcfg_23::CHCFG_23_SPEC>;
#[doc = "Output channel configure register"]
pub mod chcfg_23;
#[doc = "GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Global control register"]
pub mod gcr;
#[doc = "SHCR (rw) register accessor: an alias for `Reg<SHCR_SPEC>`"]
pub type SHCR = crate::Reg<shcr::SHCR_SPEC>;
#[doc = "Shadow register control register"]
pub mod shcr;
#[doc = "CAPPOS_0 (r) register accessor: an alias for `Reg<CAPPOS_0_SPEC>`"]
pub type CAPPOS_0 = crate::Reg<cappos_0::CAPPOS_0_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_0;
#[doc = "CAPPOS_1 (r) register accessor: an alias for `Reg<CAPPOS_1_SPEC>`"]
pub type CAPPOS_1 = crate::Reg<cappos_1::CAPPOS_1_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_1;
#[doc = "CAPPOS_2 (r) register accessor: an alias for `Reg<CAPPOS_2_SPEC>`"]
pub type CAPPOS_2 = crate::Reg<cappos_2::CAPPOS_2_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_2;
#[doc = "CAPPOS_3 (r) register accessor: an alias for `Reg<CAPPOS_3_SPEC>`"]
pub type CAPPOS_3 = crate::Reg<cappos_3::CAPPOS_3_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_3;
#[doc = "CAPPOS_4 (r) register accessor: an alias for `Reg<CAPPOS_4_SPEC>`"]
pub type CAPPOS_4 = crate::Reg<cappos_4::CAPPOS_4_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_4;
#[doc = "CAPPOS_5 (r) register accessor: an alias for `Reg<CAPPOS_5_SPEC>`"]
pub type CAPPOS_5 = crate::Reg<cappos_5::CAPPOS_5_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_5;
#[doc = "CAPPOS_6 (r) register accessor: an alias for `Reg<CAPPOS_6_SPEC>`"]
pub type CAPPOS_6 = crate::Reg<cappos_6::CAPPOS_6_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_6;
#[doc = "CAPPOS_7 (r) register accessor: an alias for `Reg<CAPPOS_7_SPEC>`"]
pub type CAPPOS_7 = crate::Reg<cappos_7::CAPPOS_7_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_7;
#[doc = "CAPPOS_8 (r) register accessor: an alias for `Reg<CAPPOS_8_SPEC>`"]
pub type CAPPOS_8 = crate::Reg<cappos_8::CAPPOS_8_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_8;
#[doc = "CAPPOS_9 (r) register accessor: an alias for `Reg<CAPPOS_9_SPEC>`"]
pub type CAPPOS_9 = crate::Reg<cappos_9::CAPPOS_9_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_9;
#[doc = "CAPPOS_10 (r) register accessor: an alias for `Reg<CAPPOS_10_SPEC>`"]
pub type CAPPOS_10 = crate::Reg<cappos_10::CAPPOS_10_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_10;
#[doc = "CAPPOS_11 (r) register accessor: an alias for `Reg<CAPPOS_11_SPEC>`"]
pub type CAPPOS_11 = crate::Reg<cappos_11::CAPPOS_11_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_11;
#[doc = "CAPPOS_12 (r) register accessor: an alias for `Reg<CAPPOS_12_SPEC>`"]
pub type CAPPOS_12 = crate::Reg<cappos_12::CAPPOS_12_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_12;
#[doc = "CAPPOS_13 (r) register accessor: an alias for `Reg<CAPPOS_13_SPEC>`"]
pub type CAPPOS_13 = crate::Reg<cappos_13::CAPPOS_13_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_13;
#[doc = "CAPPOS_14 (r) register accessor: an alias for `Reg<CAPPOS_14_SPEC>`"]
pub type CAPPOS_14 = crate::Reg<cappos_14::CAPPOS_14_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_14;
#[doc = "CAPPOS_15 (r) register accessor: an alias for `Reg<CAPPOS_15_SPEC>`"]
pub type CAPPOS_15 = crate::Reg<cappos_15::CAPPOS_15_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_15;
#[doc = "CAPPOS_16 (r) register accessor: an alias for `Reg<CAPPOS_16_SPEC>`"]
pub type CAPPOS_16 = crate::Reg<cappos_16::CAPPOS_16_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_16;
#[doc = "CAPPOS_17 (r) register accessor: an alias for `Reg<CAPPOS_17_SPEC>`"]
pub type CAPPOS_17 = crate::Reg<cappos_17::CAPPOS_17_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_17;
#[doc = "CAPPOS_18 (r) register accessor: an alias for `Reg<CAPPOS_18_SPEC>`"]
pub type CAPPOS_18 = crate::Reg<cappos_18::CAPPOS_18_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_18;
#[doc = "CAPPOS_19 (r) register accessor: an alias for `Reg<CAPPOS_19_SPEC>`"]
pub type CAPPOS_19 = crate::Reg<cappos_19::CAPPOS_19_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_19;
#[doc = "CAPPOS_20 (r) register accessor: an alias for `Reg<CAPPOS_20_SPEC>`"]
pub type CAPPOS_20 = crate::Reg<cappos_20::CAPPOS_20_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_20;
#[doc = "CAPPOS_21 (r) register accessor: an alias for `Reg<CAPPOS_21_SPEC>`"]
pub type CAPPOS_21 = crate::Reg<cappos_21::CAPPOS_21_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_21;
#[doc = "CAPPOS_22 (r) register accessor: an alias for `Reg<CAPPOS_22_SPEC>`"]
pub type CAPPOS_22 = crate::Reg<cappos_22::CAPPOS_22_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_22;
#[doc = "CAPPOS_23 (r) register accessor: an alias for `Reg<CAPPOS_23_SPEC>`"]
pub type CAPPOS_23 = crate::Reg<cappos_23::CAPPOS_23_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos_23;
#[doc = "CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter"]
pub mod cnt;
#[doc = "CAPNEG_0 (r) register accessor: an alias for `Reg<CAPNEG_0_SPEC>`"]
pub type CAPNEG_0 = crate::Reg<capneg_0::CAPNEG_0_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_0;
#[doc = "CAPNEG_1 (r) register accessor: an alias for `Reg<CAPNEG_1_SPEC>`"]
pub type CAPNEG_1 = crate::Reg<capneg_1::CAPNEG_1_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_1;
#[doc = "CAPNEG_2 (r) register accessor: an alias for `Reg<CAPNEG_2_SPEC>`"]
pub type CAPNEG_2 = crate::Reg<capneg_2::CAPNEG_2_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_2;
#[doc = "CAPNEG_3 (r) register accessor: an alias for `Reg<CAPNEG_3_SPEC>`"]
pub type CAPNEG_3 = crate::Reg<capneg_3::CAPNEG_3_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_3;
#[doc = "CAPNEG_4 (r) register accessor: an alias for `Reg<CAPNEG_4_SPEC>`"]
pub type CAPNEG_4 = crate::Reg<capneg_4::CAPNEG_4_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_4;
#[doc = "CAPNEG_5 (r) register accessor: an alias for `Reg<CAPNEG_5_SPEC>`"]
pub type CAPNEG_5 = crate::Reg<capneg_5::CAPNEG_5_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_5;
#[doc = "CAPNEG_6 (r) register accessor: an alias for `Reg<CAPNEG_6_SPEC>`"]
pub type CAPNEG_6 = crate::Reg<capneg_6::CAPNEG_6_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_6;
#[doc = "CAPNEG_7 (r) register accessor: an alias for `Reg<CAPNEG_7_SPEC>`"]
pub type CAPNEG_7 = crate::Reg<capneg_7::CAPNEG_7_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_7;
#[doc = "CAPNEG_8 (r) register accessor: an alias for `Reg<CAPNEG_8_SPEC>`"]
pub type CAPNEG_8 = crate::Reg<capneg_8::CAPNEG_8_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_8;
#[doc = "CAPNEG_9 (r) register accessor: an alias for `Reg<CAPNEG_9_SPEC>`"]
pub type CAPNEG_9 = crate::Reg<capneg_9::CAPNEG_9_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_9;
#[doc = "CAPNEG_10 (r) register accessor: an alias for `Reg<CAPNEG_10_SPEC>`"]
pub type CAPNEG_10 = crate::Reg<capneg_10::CAPNEG_10_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_10;
#[doc = "CAPNEG_11 (r) register accessor: an alias for `Reg<CAPNEG_11_SPEC>`"]
pub type CAPNEG_11 = crate::Reg<capneg_11::CAPNEG_11_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_11;
#[doc = "CAPNEG_12 (r) register accessor: an alias for `Reg<CAPNEG_12_SPEC>`"]
pub type CAPNEG_12 = crate::Reg<capneg_12::CAPNEG_12_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_12;
#[doc = "CAPNEG_13 (r) register accessor: an alias for `Reg<CAPNEG_13_SPEC>`"]
pub type CAPNEG_13 = crate::Reg<capneg_13::CAPNEG_13_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_13;
#[doc = "CAPNEG_14 (r) register accessor: an alias for `Reg<CAPNEG_14_SPEC>`"]
pub type CAPNEG_14 = crate::Reg<capneg_14::CAPNEG_14_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_14;
#[doc = "CAPNEG_15 (r) register accessor: an alias for `Reg<CAPNEG_15_SPEC>`"]
pub type CAPNEG_15 = crate::Reg<capneg_15::CAPNEG_15_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_15;
#[doc = "CAPNEG_16 (r) register accessor: an alias for `Reg<CAPNEG_16_SPEC>`"]
pub type CAPNEG_16 = crate::Reg<capneg_16::CAPNEG_16_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_16;
#[doc = "CAPNEG_17 (r) register accessor: an alias for `Reg<CAPNEG_17_SPEC>`"]
pub type CAPNEG_17 = crate::Reg<capneg_17::CAPNEG_17_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_17;
#[doc = "CAPNEG_18 (r) register accessor: an alias for `Reg<CAPNEG_18_SPEC>`"]
pub type CAPNEG_18 = crate::Reg<capneg_18::CAPNEG_18_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_18;
#[doc = "CAPNEG_19 (r) register accessor: an alias for `Reg<CAPNEG_19_SPEC>`"]
pub type CAPNEG_19 = crate::Reg<capneg_19::CAPNEG_19_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_19;
#[doc = "CAPNEG_20 (r) register accessor: an alias for `Reg<CAPNEG_20_SPEC>`"]
pub type CAPNEG_20 = crate::Reg<capneg_20::CAPNEG_20_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_20;
#[doc = "CAPNEG_21 (r) register accessor: an alias for `Reg<CAPNEG_21_SPEC>`"]
pub type CAPNEG_21 = crate::Reg<capneg_21::CAPNEG_21_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_21;
#[doc = "CAPNEG_22 (r) register accessor: an alias for `Reg<CAPNEG_22_SPEC>`"]
pub type CAPNEG_22 = crate::Reg<capneg_22::CAPNEG_22_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_22;
#[doc = "CAPNEG_23 (r) register accessor: an alias for `Reg<CAPNEG_23_SPEC>`"]
pub type CAPNEG_23 = crate::Reg<capneg_23::CAPNEG_23_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg_23;
#[doc = "CNTCOPY (r) register accessor: an alias for `Reg<CNTCOPY_SPEC>`"]
pub type CNTCOPY = crate::Reg<cntcopy::CNTCOPY_SPEC>;
#[doc = "Counter copy"]
pub mod cntcopy;
#[doc = "PWMCFG_0 (rw) register accessor: an alias for `Reg<PWMCFG_0_SPEC>`"]
pub type PWMCFG_0 = crate::Reg<pwmcfg_0::PWMCFG_0_SPEC>;
#[doc = "PWM channel configure register"]
pub mod pwmcfg_0;
#[doc = "PWMCFG_1 (rw) register accessor: an alias for `Reg<PWMCFG_1_SPEC>`"]
pub type PWMCFG_1 = crate::Reg<pwmcfg_1::PWMCFG_1_SPEC>;
#[doc = "PWM channel configure register"]
pub mod pwmcfg_1;
#[doc = "PWMCFG_2 (rw) register accessor: an alias for `Reg<PWMCFG_2_SPEC>`"]
pub type PWMCFG_2 = crate::Reg<pwmcfg_2::PWMCFG_2_SPEC>;
#[doc = "PWM channel configure register"]
pub mod pwmcfg_2;
#[doc = "PWMCFG_3 (rw) register accessor: an alias for `Reg<PWMCFG_3_SPEC>`"]
pub type PWMCFG_3 = crate::Reg<pwmcfg_3::PWMCFG_3_SPEC>;
#[doc = "PWM channel configure register"]
pub mod pwmcfg_3;
#[doc = "PWMCFG_4 (rw) register accessor: an alias for `Reg<PWMCFG_4_SPEC>`"]
pub type PWMCFG_4 = crate::Reg<pwmcfg_4::PWMCFG_4_SPEC>;
#[doc = "PWM channel configure register"]
pub mod pwmcfg_4;
#[doc = "PWMCFG_5 (rw) register accessor: an alias for `Reg<PWMCFG_5_SPEC>`"]
pub type PWMCFG_5 = crate::Reg<pwmcfg_5::PWMCFG_5_SPEC>;
#[doc = "PWM channel configure register"]
pub mod pwmcfg_5;
#[doc = "PWMCFG_6 (rw) register accessor: an alias for `Reg<PWMCFG_6_SPEC>`"]
pub type PWMCFG_6 = crate::Reg<pwmcfg_6::PWMCFG_6_SPEC>;
#[doc = "PWM channel configure register"]
pub mod pwmcfg_6;
#[doc = "PWMCFG_7 (rw) register accessor: an alias for `Reg<PWMCFG_7_SPEC>`"]
pub type PWMCFG_7 = crate::Reg<pwmcfg_7::PWMCFG_7_SPEC>;
#[doc = "PWM channel configure register"]
pub mod pwmcfg_7;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IRQEN (rw) register accessor: an alias for `Reg<IRQEN_SPEC>`"]
pub type IRQEN = crate::Reg<irqen::IRQEN_SPEC>;
#[doc = "Interrupt request enable register"]
pub mod irqen;
#[doc = "DMAEN (rw) register accessor: an alias for `Reg<DMAEN_SPEC>`"]
pub type DMAEN = crate::Reg<dmaen::DMAEN_SPEC>;
#[doc = "DMA request enable register"]
pub mod dmaen;
#[doc = "CMPCFG_CMPCFG0 (rw) register accessor: an alias for `Reg<CMPCFG_CMPCFG0_SPEC>`"]
pub type CMPCFG_CMPCFG0 = crate::Reg<cmpcfg_cmpcfg0::CMPCFG_CMPCFG0_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_cmpcfg0;
#[doc = "CMPCFG_1 (rw) register accessor: an alias for `Reg<CMPCFG_1_SPEC>`"]
pub type CMPCFG_1 = crate::Reg<cmpcfg_1::CMPCFG_1_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_1;
#[doc = "CMPCFG_2 (rw) register accessor: an alias for `Reg<CMPCFG_2_SPEC>`"]
pub type CMPCFG_2 = crate::Reg<cmpcfg_2::CMPCFG_2_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_2;
#[doc = "CMPCFG_3 (rw) register accessor: an alias for `Reg<CMPCFG_3_SPEC>`"]
pub type CMPCFG_3 = crate::Reg<cmpcfg_3::CMPCFG_3_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_3;
#[doc = "CMPCFG_4 (rw) register accessor: an alias for `Reg<CMPCFG_4_SPEC>`"]
pub type CMPCFG_4 = crate::Reg<cmpcfg_4::CMPCFG_4_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_4;
#[doc = "CMPCFG_5 (rw) register accessor: an alias for `Reg<CMPCFG_5_SPEC>`"]
pub type CMPCFG_5 = crate::Reg<cmpcfg_5::CMPCFG_5_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_5;
#[doc = "CMPCFG_6 (rw) register accessor: an alias for `Reg<CMPCFG_6_SPEC>`"]
pub type CMPCFG_6 = crate::Reg<cmpcfg_6::CMPCFG_6_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_6;
#[doc = "CMPCFG_7 (rw) register accessor: an alias for `Reg<CMPCFG_7_SPEC>`"]
pub type CMPCFG_7 = crate::Reg<cmpcfg_7::CMPCFG_7_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_7;
#[doc = "CMPCFG_8 (rw) register accessor: an alias for `Reg<CMPCFG_8_SPEC>`"]
pub type CMPCFG_8 = crate::Reg<cmpcfg_8::CMPCFG_8_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_8;
#[doc = "CMPCFG_9 (rw) register accessor: an alias for `Reg<CMPCFG_9_SPEC>`"]
pub type CMPCFG_9 = crate::Reg<cmpcfg_9::CMPCFG_9_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_9;
#[doc = "CMPCFG_10 (rw) register accessor: an alias for `Reg<CMPCFG_10_SPEC>`"]
pub type CMPCFG_10 = crate::Reg<cmpcfg_10::CMPCFG_10_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_10;
#[doc = "CMPCFG_11 (rw) register accessor: an alias for `Reg<CMPCFG_11_SPEC>`"]
pub type CMPCFG_11 = crate::Reg<cmpcfg_11::CMPCFG_11_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_11;
#[doc = "CMPCFG_12 (rw) register accessor: an alias for `Reg<CMPCFG_12_SPEC>`"]
pub type CMPCFG_12 = crate::Reg<cmpcfg_12::CMPCFG_12_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_12;
#[doc = "CMPCFG_13 (rw) register accessor: an alias for `Reg<CMPCFG_13_SPEC>`"]
pub type CMPCFG_13 = crate::Reg<cmpcfg_13::CMPCFG_13_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_13;
#[doc = "CMPCFG_14 (rw) register accessor: an alias for `Reg<CMPCFG_14_SPEC>`"]
pub type CMPCFG_14 = crate::Reg<cmpcfg_14::CMPCFG_14_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_14;
#[doc = "CMPCFG_15 (rw) register accessor: an alias for `Reg<CMPCFG_15_SPEC>`"]
pub type CMPCFG_15 = crate::Reg<cmpcfg_15::CMPCFG_15_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_15;
#[doc = "CMPCFG_16 (rw) register accessor: an alias for `Reg<CMPCFG_16_SPEC>`"]
pub type CMPCFG_16 = crate::Reg<cmpcfg_16::CMPCFG_16_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_16;
#[doc = "CMPCFG_17 (rw) register accessor: an alias for `Reg<CMPCFG_17_SPEC>`"]
pub type CMPCFG_17 = crate::Reg<cmpcfg_17::CMPCFG_17_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_17;
#[doc = "CMPCFG_18 (rw) register accessor: an alias for `Reg<CMPCFG_18_SPEC>`"]
pub type CMPCFG_18 = crate::Reg<cmpcfg_18::CMPCFG_18_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_18;
#[doc = "CMPCFG_19 (rw) register accessor: an alias for `Reg<CMPCFG_19_SPEC>`"]
pub type CMPCFG_19 = crate::Reg<cmpcfg_19::CMPCFG_19_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_19;
#[doc = "CMPCFG_20 (rw) register accessor: an alias for `Reg<CMPCFG_20_SPEC>`"]
pub type CMPCFG_20 = crate::Reg<cmpcfg_20::CMPCFG_20_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_20;
#[doc = "CMPCFG_21 (rw) register accessor: an alias for `Reg<CMPCFG_21_SPEC>`"]
pub type CMPCFG_21 = crate::Reg<cmpcfg_21::CMPCFG_21_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_21;
#[doc = "CMPCFG_22 (rw) register accessor: an alias for `Reg<CMPCFG_22_SPEC>`"]
pub type CMPCFG_22 = crate::Reg<cmpcfg_22::CMPCFG_22_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_22;
#[doc = "CMPCFG_23 (rw) register accessor: an alias for `Reg<CMPCFG_23_SPEC>`"]
pub type CMPCFG_23 = crate::Reg<cmpcfg_23::CMPCFG_23_SPEC>;
#[doc = "Comparator configure register"]
pub mod cmpcfg_23;
