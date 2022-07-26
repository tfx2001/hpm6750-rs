#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ALT SELECT"]
    pub pad_pa00_func_ctl: crate::Reg<pad_pa00_func_ctl::PAD_PA00_FUNC_CTL_SPEC>,
    #[doc = "0x04 - PAD SETTINGS"]
    pub pad_pa00_pad_ctl: crate::Reg<pad_pa00_pad_ctl::PAD_PA00_PAD_CTL_SPEC>,
    #[doc = "0x08 - ALT SELECT"]
    pub pad_pa01_func_ctl: crate::Reg<pad_pa01_func_ctl::PAD_PA01_FUNC_CTL_SPEC>,
    #[doc = "0x0c - PAD SETTINGS"]
    pub pad_pa01_pad_ctl: crate::Reg<pad_pa01_pad_ctl::PAD_PA01_PAD_CTL_SPEC>,
    #[doc = "0x10 - ALT SELECT"]
    pub pad_pa02_func_ctl: crate::Reg<pad_pa02_func_ctl::PAD_PA02_FUNC_CTL_SPEC>,
    #[doc = "0x14 - PAD SETTINGS"]
    pub pad_pa02_pad_ctl: crate::Reg<pad_pa02_pad_ctl::PAD_PA02_PAD_CTL_SPEC>,
    #[doc = "0x18 - ALT SELECT"]
    pub pad_pa03_func_ctl: crate::Reg<pad_pa03_func_ctl::PAD_PA03_FUNC_CTL_SPEC>,
    #[doc = "0x1c - PAD SETTINGS"]
    pub pad_pa03_pad_ctl: crate::Reg<pad_pa03_pad_ctl::PAD_PA03_PAD_CTL_SPEC>,
    #[doc = "0x20 - ALT SELECT"]
    pub pad_pa04_func_ctl: crate::Reg<pad_pa04_func_ctl::PAD_PA04_FUNC_CTL_SPEC>,
    #[doc = "0x24 - PAD SETTINGS"]
    pub pad_pa04_pad_ctl: crate::Reg<pad_pa04_pad_ctl::PAD_PA04_PAD_CTL_SPEC>,
    #[doc = "0x28 - ALT SELECT"]
    pub pad_pa05_func_ctl: crate::Reg<pad_pa05_func_ctl::PAD_PA05_FUNC_CTL_SPEC>,
    #[doc = "0x2c - PAD SETTINGS"]
    pub pad_pa05_pad_ctl: crate::Reg<pad_pa05_pad_ctl::PAD_PA05_PAD_CTL_SPEC>,
    #[doc = "0x30 - ALT SELECT"]
    pub pad_pa06_func_ctl: crate::Reg<pad_pa06_func_ctl::PAD_PA06_FUNC_CTL_SPEC>,
    #[doc = "0x34 - PAD SETTINGS"]
    pub pad_pa06_pad_ctl: crate::Reg<pad_pa06_pad_ctl::PAD_PA06_PAD_CTL_SPEC>,
    #[doc = "0x38 - ALT SELECT"]
    pub pad_pa07_func_ctl: crate::Reg<pad_pa07_func_ctl::PAD_PA07_FUNC_CTL_SPEC>,
    #[doc = "0x3c - PAD SETTINGS"]
    pub pad_pa07_pad_ctl: crate::Reg<pad_pa07_pad_ctl::PAD_PA07_PAD_CTL_SPEC>,
    #[doc = "0x40 - ALT SELECT"]
    pub pad_pa08_func_ctl: crate::Reg<pad_pa08_func_ctl::PAD_PA08_FUNC_CTL_SPEC>,
    #[doc = "0x44 - PAD SETTINGS"]
    pub pad_pa08_pad_ctl: crate::Reg<pad_pa08_pad_ctl::PAD_PA08_PAD_CTL_SPEC>,
    #[doc = "0x48 - ALT SELECT"]
    pub pad_pa09_func_ctl: crate::Reg<pad_pa09_func_ctl::PAD_PA09_FUNC_CTL_SPEC>,
    #[doc = "0x4c - PAD SETTINGS"]
    pub pad_pa09_pad_ctl: crate::Reg<pad_pa09_pad_ctl::PAD_PA09_PAD_CTL_SPEC>,
    #[doc = "0x50 - ALT SELECT"]
    pub pad_pa10_func_ctl: crate::Reg<pad_pa10_func_ctl::PAD_PA10_FUNC_CTL_SPEC>,
    #[doc = "0x54 - PAD SETTINGS"]
    pub pad_pa10_pad_ctl: crate::Reg<pad_pa10_pad_ctl::PAD_PA10_PAD_CTL_SPEC>,
    #[doc = "0x58 - ALT SELECT"]
    pub pad_pa11_func_ctl: crate::Reg<pad_pa11_func_ctl::PAD_PA11_FUNC_CTL_SPEC>,
    #[doc = "0x5c - PAD SETTINGS"]
    pub pad_pa11_pad_ctl: crate::Reg<pad_pa11_pad_ctl::PAD_PA11_PAD_CTL_SPEC>,
    #[doc = "0x60 - ALT SELECT"]
    pub pad_pa12_func_ctl: crate::Reg<pad_pa12_func_ctl::PAD_PA12_FUNC_CTL_SPEC>,
    #[doc = "0x64 - PAD SETTINGS"]
    pub pad_pa12_pad_ctl: crate::Reg<pad_pa12_pad_ctl::PAD_PA12_PAD_CTL_SPEC>,
    #[doc = "0x68 - ALT SELECT"]
    pub pad_pa13_func_ctl: crate::Reg<pad_pa13_func_ctl::PAD_PA13_FUNC_CTL_SPEC>,
    #[doc = "0x6c - PAD SETTINGS"]
    pub pad_pa13_pad_ctl: crate::Reg<pad_pa13_pad_ctl::PAD_PA13_PAD_CTL_SPEC>,
    #[doc = "0x70 - ALT SELECT"]
    pub pad_pa14_func_ctl: crate::Reg<pad_pa14_func_ctl::PAD_PA14_FUNC_CTL_SPEC>,
    #[doc = "0x74 - PAD SETTINGS"]
    pub pad_pa14_pad_ctl: crate::Reg<pad_pa14_pad_ctl::PAD_PA14_PAD_CTL_SPEC>,
    #[doc = "0x78 - ALT SELECT"]
    pub pad_pa15_func_ctl: crate::Reg<pad_pa15_func_ctl::PAD_PA15_FUNC_CTL_SPEC>,
    #[doc = "0x7c - PAD SETTINGS"]
    pub pad_pa15_pad_ctl: crate::Reg<pad_pa15_pad_ctl::PAD_PA15_PAD_CTL_SPEC>,
    #[doc = "0x80 - ALT SELECT"]
    pub pad_pa16_func_ctl: crate::Reg<pad_pa16_func_ctl::PAD_PA16_FUNC_CTL_SPEC>,
    #[doc = "0x84 - PAD SETTINGS"]
    pub pad_pa16_pad_ctl: crate::Reg<pad_pa16_pad_ctl::PAD_PA16_PAD_CTL_SPEC>,
    #[doc = "0x88 - ALT SELECT"]
    pub pad_pa17_func_ctl: crate::Reg<pad_pa17_func_ctl::PAD_PA17_FUNC_CTL_SPEC>,
    #[doc = "0x8c - PAD SETTINGS"]
    pub pad_pa17_pad_ctl: crate::Reg<pad_pa17_pad_ctl::PAD_PA17_PAD_CTL_SPEC>,
    #[doc = "0x90 - ALT SELECT"]
    pub pad_pa18_func_ctl: crate::Reg<pad_pa18_func_ctl::PAD_PA18_FUNC_CTL_SPEC>,
    #[doc = "0x94 - PAD SETTINGS"]
    pub pad_pa18_pad_ctl: crate::Reg<pad_pa18_pad_ctl::PAD_PA18_PAD_CTL_SPEC>,
    #[doc = "0x98 - ALT SELECT"]
    pub pad_pa19_func_ctl: crate::Reg<pad_pa19_func_ctl::PAD_PA19_FUNC_CTL_SPEC>,
    #[doc = "0x9c - PAD SETTINGS"]
    pub pad_pa19_pad_ctl: crate::Reg<pad_pa19_pad_ctl::PAD_PA19_PAD_CTL_SPEC>,
    #[doc = "0xa0 - ALT SELECT"]
    pub pad_pa20_func_ctl: crate::Reg<pad_pa20_func_ctl::PAD_PA20_FUNC_CTL_SPEC>,
    #[doc = "0xa4 - PAD SETTINGS"]
    pub pad_pa20_pad_ctl: crate::Reg<pad_pa20_pad_ctl::PAD_PA20_PAD_CTL_SPEC>,
    #[doc = "0xa8 - ALT SELECT"]
    pub pad_pa21_func_ctl: crate::Reg<pad_pa21_func_ctl::PAD_PA21_FUNC_CTL_SPEC>,
    #[doc = "0xac - PAD SETTINGS"]
    pub pad_pa21_pad_ctl: crate::Reg<pad_pa21_pad_ctl::PAD_PA21_PAD_CTL_SPEC>,
    #[doc = "0xb0 - ALT SELECT"]
    pub pad_pa22_func_ctl: crate::Reg<pad_pa22_func_ctl::PAD_PA22_FUNC_CTL_SPEC>,
    #[doc = "0xb4 - PAD SETTINGS"]
    pub pad_pa22_pad_ctl: crate::Reg<pad_pa22_pad_ctl::PAD_PA22_PAD_CTL_SPEC>,
    #[doc = "0xb8 - ALT SELECT"]
    pub pad_pa23_func_ctl: crate::Reg<pad_pa23_func_ctl::PAD_PA23_FUNC_CTL_SPEC>,
    #[doc = "0xbc - PAD SETTINGS"]
    pub pad_pa23_pad_ctl: crate::Reg<pad_pa23_pad_ctl::PAD_PA23_PAD_CTL_SPEC>,
    #[doc = "0xc0 - ALT SELECT"]
    pub pad_pa24_func_ctl: crate::Reg<pad_pa24_func_ctl::PAD_PA24_FUNC_CTL_SPEC>,
    #[doc = "0xc4 - PAD SETTINGS"]
    pub pad_pa24_pad_ctl: crate::Reg<pad_pa24_pad_ctl::PAD_PA24_PAD_CTL_SPEC>,
    #[doc = "0xc8 - ALT SELECT"]
    pub pad_pa25_func_ctl: crate::Reg<pad_pa25_func_ctl::PAD_PA25_FUNC_CTL_SPEC>,
    #[doc = "0xcc - PAD SETTINGS"]
    pub pad_pa25_pad_ctl: crate::Reg<pad_pa25_pad_ctl::PAD_PA25_PAD_CTL_SPEC>,
    #[doc = "0xd0 - ALT SELECT"]
    pub pad_pa26_func_ctl: crate::Reg<pad_pa26_func_ctl::PAD_PA26_FUNC_CTL_SPEC>,
    #[doc = "0xd4 - PAD SETTINGS"]
    pub pad_pa26_pad_ctl: crate::Reg<pad_pa26_pad_ctl::PAD_PA26_PAD_CTL_SPEC>,
    #[doc = "0xd8 - ALT SELECT"]
    pub pad_pa27_func_ctl: crate::Reg<pad_pa27_func_ctl::PAD_PA27_FUNC_CTL_SPEC>,
    #[doc = "0xdc - PAD SETTINGS"]
    pub pad_pa27_pad_ctl: crate::Reg<pad_pa27_pad_ctl::PAD_PA27_PAD_CTL_SPEC>,
    #[doc = "0xe0 - ALT SELECT"]
    pub pad_pa28_func_ctl: crate::Reg<pad_pa28_func_ctl::PAD_PA28_FUNC_CTL_SPEC>,
    #[doc = "0xe4 - PAD SETTINGS"]
    pub pad_pa28_pad_ctl: crate::Reg<pad_pa28_pad_ctl::PAD_PA28_PAD_CTL_SPEC>,
    #[doc = "0xe8 - ALT SELECT"]
    pub pad_pa29_func_ctl: crate::Reg<pad_pa29_func_ctl::PAD_PA29_FUNC_CTL_SPEC>,
    #[doc = "0xec - PAD SETTINGS"]
    pub pad_pa29_pad_ctl: crate::Reg<pad_pa29_pad_ctl::PAD_PA29_PAD_CTL_SPEC>,
    #[doc = "0xf0 - ALT SELECT"]
    pub pad_pa30_func_ctl: crate::Reg<pad_pa30_func_ctl::PAD_PA30_FUNC_CTL_SPEC>,
    #[doc = "0xf4 - PAD SETTINGS"]
    pub pad_pa30_pad_ctl: crate::Reg<pad_pa30_pad_ctl::PAD_PA30_PAD_CTL_SPEC>,
    #[doc = "0xf8 - ALT SELECT"]
    pub pad_pa31_func_ctl: crate::Reg<pad_pa31_func_ctl::PAD_PA31_FUNC_CTL_SPEC>,
    #[doc = "0xfc - PAD SETTINGS"]
    pub pad_pa31_pad_ctl: crate::Reg<pad_pa31_pad_ctl::PAD_PA31_PAD_CTL_SPEC>,
    #[doc = "0x100 - ALT SELECT"]
    pub pad_pb00_func_ctl: crate::Reg<pad_pb00_func_ctl::PAD_PB00_FUNC_CTL_SPEC>,
    #[doc = "0x104 - PAD SETTINGS"]
    pub pad_pb00_pad_ctl: crate::Reg<pad_pb00_pad_ctl::PAD_PB00_PAD_CTL_SPEC>,
    #[doc = "0x108 - ALT SELECT"]
    pub pad_pb01_func_ctl: crate::Reg<pad_pb01_func_ctl::PAD_PB01_FUNC_CTL_SPEC>,
    #[doc = "0x10c - PAD SETTINGS"]
    pub pad_pb01_pad_ctl: crate::Reg<pad_pb01_pad_ctl::PAD_PB01_PAD_CTL_SPEC>,
    #[doc = "0x110 - ALT SELECT"]
    pub pad_pb02_func_ctl: crate::Reg<pad_pb02_func_ctl::PAD_PB02_FUNC_CTL_SPEC>,
    #[doc = "0x114 - PAD SETTINGS"]
    pub pad_pb02_pad_ctl: crate::Reg<pad_pb02_pad_ctl::PAD_PB02_PAD_CTL_SPEC>,
    #[doc = "0x118 - ALT SELECT"]
    pub pad_pb03_func_ctl: crate::Reg<pad_pb03_func_ctl::PAD_PB03_FUNC_CTL_SPEC>,
    #[doc = "0x11c - PAD SETTINGS"]
    pub pad_pb03_pad_ctl: crate::Reg<pad_pb03_pad_ctl::PAD_PB03_PAD_CTL_SPEC>,
    #[doc = "0x120 - ALT SELECT"]
    pub pad_pb04_func_ctl: crate::Reg<pad_pb04_func_ctl::PAD_PB04_FUNC_CTL_SPEC>,
    #[doc = "0x124 - PAD SETTINGS"]
    pub pad_pb04_pad_ctl: crate::Reg<pad_pb04_pad_ctl::PAD_PB04_PAD_CTL_SPEC>,
    #[doc = "0x128 - ALT SELECT"]
    pub pad_pb05_func_ctl: crate::Reg<pad_pb05_func_ctl::PAD_PB05_FUNC_CTL_SPEC>,
    #[doc = "0x12c - PAD SETTINGS"]
    pub pad_pb05_pad_ctl: crate::Reg<pad_pb05_pad_ctl::PAD_PB05_PAD_CTL_SPEC>,
    #[doc = "0x130 - ALT SELECT"]
    pub pad_pb06_func_ctl: crate::Reg<pad_pb06_func_ctl::PAD_PB06_FUNC_CTL_SPEC>,
    #[doc = "0x134 - PAD SETTINGS"]
    pub pad_pb06_pad_ctl: crate::Reg<pad_pb06_pad_ctl::PAD_PB06_PAD_CTL_SPEC>,
    #[doc = "0x138 - ALT SELECT"]
    pub pad_pb07_func_ctl: crate::Reg<pad_pb07_func_ctl::PAD_PB07_FUNC_CTL_SPEC>,
    #[doc = "0x13c - PAD SETTINGS"]
    pub pad_pb07_pad_ctl: crate::Reg<pad_pb07_pad_ctl::PAD_PB07_PAD_CTL_SPEC>,
    #[doc = "0x140 - ALT SELECT"]
    pub pad_pb08_func_ctl: crate::Reg<pad_pb08_func_ctl::PAD_PB08_FUNC_CTL_SPEC>,
    #[doc = "0x144 - PAD SETTINGS"]
    pub pad_pb08_pad_ctl: crate::Reg<pad_pb08_pad_ctl::PAD_PB08_PAD_CTL_SPEC>,
    #[doc = "0x148 - ALT SELECT"]
    pub pad_pb09_func_ctl: crate::Reg<pad_pb09_func_ctl::PAD_PB09_FUNC_CTL_SPEC>,
    #[doc = "0x14c - PAD SETTINGS"]
    pub pad_pb09_pad_ctl: crate::Reg<pad_pb09_pad_ctl::PAD_PB09_PAD_CTL_SPEC>,
    #[doc = "0x150 - ALT SELECT"]
    pub pad_pb10_func_ctl: crate::Reg<pad_pb10_func_ctl::PAD_PB10_FUNC_CTL_SPEC>,
    #[doc = "0x154 - PAD SETTINGS"]
    pub pad_pb10_pad_ctl: crate::Reg<pad_pb10_pad_ctl::PAD_PB10_PAD_CTL_SPEC>,
    #[doc = "0x158 - ALT SELECT"]
    pub pad_pb11_func_ctl: crate::Reg<pad_pb11_func_ctl::PAD_PB11_FUNC_CTL_SPEC>,
    #[doc = "0x15c - PAD SETTINGS"]
    pub pad_pb11_pad_ctl: crate::Reg<pad_pb11_pad_ctl::PAD_PB11_PAD_CTL_SPEC>,
    #[doc = "0x160 - ALT SELECT"]
    pub pad_pb12_func_ctl: crate::Reg<pad_pb12_func_ctl::PAD_PB12_FUNC_CTL_SPEC>,
    #[doc = "0x164 - PAD SETTINGS"]
    pub pad_pb12_pad_ctl: crate::Reg<pad_pb12_pad_ctl::PAD_PB12_PAD_CTL_SPEC>,
    #[doc = "0x168 - ALT SELECT"]
    pub pad_pb13_func_ctl: crate::Reg<pad_pb13_func_ctl::PAD_PB13_FUNC_CTL_SPEC>,
    #[doc = "0x16c - PAD SETTINGS"]
    pub pad_pb13_pad_ctl: crate::Reg<pad_pb13_pad_ctl::PAD_PB13_PAD_CTL_SPEC>,
    #[doc = "0x170 - ALT SELECT"]
    pub pad_pb14_func_ctl: crate::Reg<pad_pb14_func_ctl::PAD_PB14_FUNC_CTL_SPEC>,
    #[doc = "0x174 - PAD SETTINGS"]
    pub pad_pb14_pad_ctl: crate::Reg<pad_pb14_pad_ctl::PAD_PB14_PAD_CTL_SPEC>,
    #[doc = "0x178 - ALT SELECT"]
    pub pad_pb15_func_ctl: crate::Reg<pad_pb15_func_ctl::PAD_PB15_FUNC_CTL_SPEC>,
    #[doc = "0x17c - PAD SETTINGS"]
    pub pad_pb15_pad_ctl: crate::Reg<pad_pb15_pad_ctl::PAD_PB15_PAD_CTL_SPEC>,
    #[doc = "0x180 - ALT SELECT"]
    pub pad_pb16_func_ctl: crate::Reg<pad_pb16_func_ctl::PAD_PB16_FUNC_CTL_SPEC>,
    #[doc = "0x184 - PAD SETTINGS"]
    pub pad_pb16_pad_ctl: crate::Reg<pad_pb16_pad_ctl::PAD_PB16_PAD_CTL_SPEC>,
    #[doc = "0x188 - ALT SELECT"]
    pub pad_pb17_func_ctl: crate::Reg<pad_pb17_func_ctl::PAD_PB17_FUNC_CTL_SPEC>,
    #[doc = "0x18c - PAD SETTINGS"]
    pub pad_pb17_pad_ctl: crate::Reg<pad_pb17_pad_ctl::PAD_PB17_PAD_CTL_SPEC>,
    #[doc = "0x190 - ALT SELECT"]
    pub pad_pb18_func_ctl: crate::Reg<pad_pb18_func_ctl::PAD_PB18_FUNC_CTL_SPEC>,
    #[doc = "0x194 - PAD SETTINGS"]
    pub pad_pb18_pad_ctl: crate::Reg<pad_pb18_pad_ctl::PAD_PB18_PAD_CTL_SPEC>,
    #[doc = "0x198 - ALT SELECT"]
    pub pad_pb19_func_ctl: crate::Reg<pad_pb19_func_ctl::PAD_PB19_FUNC_CTL_SPEC>,
    #[doc = "0x19c - PAD SETTINGS"]
    pub pad_pb19_pad_ctl: crate::Reg<pad_pb19_pad_ctl::PAD_PB19_PAD_CTL_SPEC>,
    #[doc = "0x1a0 - ALT SELECT"]
    pub pad_pb20_func_ctl: crate::Reg<pad_pb20_func_ctl::PAD_PB20_FUNC_CTL_SPEC>,
    #[doc = "0x1a4 - PAD SETTINGS"]
    pub pad_pb20_pad_ctl: crate::Reg<pad_pb20_pad_ctl::PAD_PB20_PAD_CTL_SPEC>,
    #[doc = "0x1a8 - ALT SELECT"]
    pub pad_pb21_func_ctl: crate::Reg<pad_pb21_func_ctl::PAD_PB21_FUNC_CTL_SPEC>,
    #[doc = "0x1ac - PAD SETTINGS"]
    pub pad_pb21_pad_ctl: crate::Reg<pad_pb21_pad_ctl::PAD_PB21_PAD_CTL_SPEC>,
    #[doc = "0x1b0 - ALT SELECT"]
    pub pad_pb22_func_ctl: crate::Reg<pad_pb22_func_ctl::PAD_PB22_FUNC_CTL_SPEC>,
    #[doc = "0x1b4 - PAD SETTINGS"]
    pub pad_pb22_pad_ctl: crate::Reg<pad_pb22_pad_ctl::PAD_PB22_PAD_CTL_SPEC>,
    #[doc = "0x1b8 - ALT SELECT"]
    pub pad_pb23_func_ctl: crate::Reg<pad_pb23_func_ctl::PAD_PB23_FUNC_CTL_SPEC>,
    #[doc = "0x1bc - PAD SETTINGS"]
    pub pad_pb23_pad_ctl: crate::Reg<pad_pb23_pad_ctl::PAD_PB23_PAD_CTL_SPEC>,
    #[doc = "0x1c0 - ALT SELECT"]
    pub pad_pb24_func_ctl: crate::Reg<pad_pb24_func_ctl::PAD_PB24_FUNC_CTL_SPEC>,
    #[doc = "0x1c4 - PAD SETTINGS"]
    pub pad_pb24_pad_ctl: crate::Reg<pad_pb24_pad_ctl::PAD_PB24_PAD_CTL_SPEC>,
    #[doc = "0x1c8 - ALT SELECT"]
    pub pad_pb25_func_ctl: crate::Reg<pad_pb25_func_ctl::PAD_PB25_FUNC_CTL_SPEC>,
    #[doc = "0x1cc - PAD SETTINGS"]
    pub pad_pb25_pad_ctl: crate::Reg<pad_pb25_pad_ctl::PAD_PB25_PAD_CTL_SPEC>,
    #[doc = "0x1d0 - ALT SELECT"]
    pub pad_pb26_func_ctl: crate::Reg<pad_pb26_func_ctl::PAD_PB26_FUNC_CTL_SPEC>,
    #[doc = "0x1d4 - PAD SETTINGS"]
    pub pad_pb26_pad_ctl: crate::Reg<pad_pb26_pad_ctl::PAD_PB26_PAD_CTL_SPEC>,
    #[doc = "0x1d8 - ALT SELECT"]
    pub pad_pb27_func_ctl: crate::Reg<pad_pb27_func_ctl::PAD_PB27_FUNC_CTL_SPEC>,
    #[doc = "0x1dc - PAD SETTINGS"]
    pub pad_pb27_pad_ctl: crate::Reg<pad_pb27_pad_ctl::PAD_PB27_PAD_CTL_SPEC>,
    #[doc = "0x1e0 - ALT SELECT"]
    pub pad_pb28_func_ctl: crate::Reg<pad_pb28_func_ctl::PAD_PB28_FUNC_CTL_SPEC>,
    #[doc = "0x1e4 - PAD SETTINGS"]
    pub pad_pb28_pad_ctl: crate::Reg<pad_pb28_pad_ctl::PAD_PB28_PAD_CTL_SPEC>,
    #[doc = "0x1e8 - ALT SELECT"]
    pub pad_pb29_func_ctl: crate::Reg<pad_pb29_func_ctl::PAD_PB29_FUNC_CTL_SPEC>,
    #[doc = "0x1ec - PAD SETTINGS"]
    pub pad_pb29_pad_ctl: crate::Reg<pad_pb29_pad_ctl::PAD_PB29_PAD_CTL_SPEC>,
    #[doc = "0x1f0 - ALT SELECT"]
    pub pad_pb30_func_ctl: crate::Reg<pad_pb30_func_ctl::PAD_PB30_FUNC_CTL_SPEC>,
    #[doc = "0x1f4 - PAD SETTINGS"]
    pub pad_pb30_pad_ctl: crate::Reg<pad_pb30_pad_ctl::PAD_PB30_PAD_CTL_SPEC>,
    #[doc = "0x1f8 - ALT SELECT"]
    pub pad_pb31_func_ctl: crate::Reg<pad_pb31_func_ctl::PAD_PB31_FUNC_CTL_SPEC>,
    #[doc = "0x1fc - PAD SETTINGS"]
    pub pad_pb31_pad_ctl: crate::Reg<pad_pb31_pad_ctl::PAD_PB31_PAD_CTL_SPEC>,
    #[doc = "0x200 - ALT SELECT"]
    pub pad_pc00_func_ctl: crate::Reg<pad_pc00_func_ctl::PAD_PC00_FUNC_CTL_SPEC>,
    #[doc = "0x204 - PAD SETTINGS"]
    pub pad_pc00_pad_ctl: crate::Reg<pad_pc00_pad_ctl::PAD_PC00_PAD_CTL_SPEC>,
    #[doc = "0x208 - ALT SELECT"]
    pub pad_pc01_func_ctl: crate::Reg<pad_pc01_func_ctl::PAD_PC01_FUNC_CTL_SPEC>,
    #[doc = "0x20c - PAD SETTINGS"]
    pub pad_pc01_pad_ctl: crate::Reg<pad_pc01_pad_ctl::PAD_PC01_PAD_CTL_SPEC>,
    #[doc = "0x210 - ALT SELECT"]
    pub pad_pc02_func_ctl: crate::Reg<pad_pc02_func_ctl::PAD_PC02_FUNC_CTL_SPEC>,
    #[doc = "0x214 - PAD SETTINGS"]
    pub pad_pc02_pad_ctl: crate::Reg<pad_pc02_pad_ctl::PAD_PC02_PAD_CTL_SPEC>,
    #[doc = "0x218 - ALT SELECT"]
    pub pad_pc03_func_ctl: crate::Reg<pad_pc03_func_ctl::PAD_PC03_FUNC_CTL_SPEC>,
    #[doc = "0x21c - PAD SETTINGS"]
    pub pad_pc03_pad_ctl: crate::Reg<pad_pc03_pad_ctl::PAD_PC03_PAD_CTL_SPEC>,
    #[doc = "0x220 - ALT SELECT"]
    pub pad_pc04_func_ctl: crate::Reg<pad_pc04_func_ctl::PAD_PC04_FUNC_CTL_SPEC>,
    #[doc = "0x224 - PAD SETTINGS"]
    pub pad_pc04_pad_ctl: crate::Reg<pad_pc04_pad_ctl::PAD_PC04_PAD_CTL_SPEC>,
    #[doc = "0x228 - ALT SELECT"]
    pub pad_pc05_func_ctl: crate::Reg<pad_pc05_func_ctl::PAD_PC05_FUNC_CTL_SPEC>,
    #[doc = "0x22c - PAD SETTINGS"]
    pub pad_pc05_pad_ctl: crate::Reg<pad_pc05_pad_ctl::PAD_PC05_PAD_CTL_SPEC>,
    #[doc = "0x230 - ALT SELECT"]
    pub pad_pc06_func_ctl: crate::Reg<pad_pc06_func_ctl::PAD_PC06_FUNC_CTL_SPEC>,
    #[doc = "0x234 - PAD SETTINGS"]
    pub pad_pc06_pad_ctl: crate::Reg<pad_pc06_pad_ctl::PAD_PC06_PAD_CTL_SPEC>,
    #[doc = "0x238 - ALT SELECT"]
    pub pad_pc07_func_ctl: crate::Reg<pad_pc07_func_ctl::PAD_PC07_FUNC_CTL_SPEC>,
    #[doc = "0x23c - PAD SETTINGS"]
    pub pad_pc07_pad_ctl: crate::Reg<pad_pc07_pad_ctl::PAD_PC07_PAD_CTL_SPEC>,
    #[doc = "0x240 - ALT SELECT"]
    pub pad_pc08_func_ctl: crate::Reg<pad_pc08_func_ctl::PAD_PC08_FUNC_CTL_SPEC>,
    #[doc = "0x244 - PAD SETTINGS"]
    pub pad_pc08_pad_ctl: crate::Reg<pad_pc08_pad_ctl::PAD_PC08_PAD_CTL_SPEC>,
    #[doc = "0x248 - ALT SELECT"]
    pub pad_pc09_func_ctl: crate::Reg<pad_pc09_func_ctl::PAD_PC09_FUNC_CTL_SPEC>,
    #[doc = "0x24c - PAD SETTINGS"]
    pub pad_pc09_pad_ctl: crate::Reg<pad_pc09_pad_ctl::PAD_PC09_PAD_CTL_SPEC>,
    #[doc = "0x250 - ALT SELECT"]
    pub pad_pc10_func_ctl: crate::Reg<pad_pc10_func_ctl::PAD_PC10_FUNC_CTL_SPEC>,
    #[doc = "0x254 - PAD SETTINGS"]
    pub pad_pc10_pad_ctl: crate::Reg<pad_pc10_pad_ctl::PAD_PC10_PAD_CTL_SPEC>,
    #[doc = "0x258 - ALT SELECT"]
    pub pad_pc11_func_ctl: crate::Reg<pad_pc11_func_ctl::PAD_PC11_FUNC_CTL_SPEC>,
    #[doc = "0x25c - PAD SETTINGS"]
    pub pad_pc11_pad_ctl: crate::Reg<pad_pc11_pad_ctl::PAD_PC11_PAD_CTL_SPEC>,
    #[doc = "0x260 - ALT SELECT"]
    pub pad_pc12_func_ctl: crate::Reg<pad_pc12_func_ctl::PAD_PC12_FUNC_CTL_SPEC>,
    #[doc = "0x264 - PAD SETTINGS"]
    pub pad_pc12_pad_ctl: crate::Reg<pad_pc12_pad_ctl::PAD_PC12_PAD_CTL_SPEC>,
    #[doc = "0x268 - ALT SELECT"]
    pub pad_pc13_func_ctl: crate::Reg<pad_pc13_func_ctl::PAD_PC13_FUNC_CTL_SPEC>,
    #[doc = "0x26c - PAD SETTINGS"]
    pub pad_pc13_pad_ctl: crate::Reg<pad_pc13_pad_ctl::PAD_PC13_PAD_CTL_SPEC>,
    #[doc = "0x270 - ALT SELECT"]
    pub pad_pc14_func_ctl: crate::Reg<pad_pc14_func_ctl::PAD_PC14_FUNC_CTL_SPEC>,
    #[doc = "0x274 - PAD SETTINGS"]
    pub pad_pc14_pad_ctl: crate::Reg<pad_pc14_pad_ctl::PAD_PC14_PAD_CTL_SPEC>,
    #[doc = "0x278 - ALT SELECT"]
    pub pad_pc15_func_ctl: crate::Reg<pad_pc15_func_ctl::PAD_PC15_FUNC_CTL_SPEC>,
    #[doc = "0x27c - PAD SETTINGS"]
    pub pad_pc15_pad_ctl: crate::Reg<pad_pc15_pad_ctl::PAD_PC15_PAD_CTL_SPEC>,
    #[doc = "0x280 - ALT SELECT"]
    pub pad_pc16_func_ctl: crate::Reg<pad_pc16_func_ctl::PAD_PC16_FUNC_CTL_SPEC>,
    #[doc = "0x284 - PAD SETTINGS"]
    pub pad_pc16_pad_ctl: crate::Reg<pad_pc16_pad_ctl::PAD_PC16_PAD_CTL_SPEC>,
    #[doc = "0x288 - ALT SELECT"]
    pub pad_pc17_func_ctl: crate::Reg<pad_pc17_func_ctl::PAD_PC17_FUNC_CTL_SPEC>,
    #[doc = "0x28c - PAD SETTINGS"]
    pub pad_pc17_pad_ctl: crate::Reg<pad_pc17_pad_ctl::PAD_PC17_PAD_CTL_SPEC>,
    #[doc = "0x290 - ALT SELECT"]
    pub pad_pc18_func_ctl: crate::Reg<pad_pc18_func_ctl::PAD_PC18_FUNC_CTL_SPEC>,
    #[doc = "0x294 - PAD SETTINGS"]
    pub pad_pc18_pad_ctl: crate::Reg<pad_pc18_pad_ctl::PAD_PC18_PAD_CTL_SPEC>,
    #[doc = "0x298 - ALT SELECT"]
    pub pad_pc19_func_ctl: crate::Reg<pad_pc19_func_ctl::PAD_PC19_FUNC_CTL_SPEC>,
    #[doc = "0x29c - PAD SETTINGS"]
    pub pad_pc19_pad_ctl: crate::Reg<pad_pc19_pad_ctl::PAD_PC19_PAD_CTL_SPEC>,
    #[doc = "0x2a0 - ALT SELECT"]
    pub pad_pc20_func_ctl: crate::Reg<pad_pc20_func_ctl::PAD_PC20_FUNC_CTL_SPEC>,
    #[doc = "0x2a4 - PAD SETTINGS"]
    pub pad_pc20_pad_ctl: crate::Reg<pad_pc20_pad_ctl::PAD_PC20_PAD_CTL_SPEC>,
    #[doc = "0x2a8 - ALT SELECT"]
    pub pad_pc21_func_ctl: crate::Reg<pad_pc21_func_ctl::PAD_PC21_FUNC_CTL_SPEC>,
    #[doc = "0x2ac - PAD SETTINGS"]
    pub pad_pc21_pad_ctl: crate::Reg<pad_pc21_pad_ctl::PAD_PC21_PAD_CTL_SPEC>,
    #[doc = "0x2b0 - ALT SELECT"]
    pub pad_pc22_func_ctl: crate::Reg<pad_pc22_func_ctl::PAD_PC22_FUNC_CTL_SPEC>,
    #[doc = "0x2b4 - PAD SETTINGS"]
    pub pad_pc22_pad_ctl: crate::Reg<pad_pc22_pad_ctl::PAD_PC22_PAD_CTL_SPEC>,
    #[doc = "0x2b8 - ALT SELECT"]
    pub pad_pc23_func_ctl: crate::Reg<pad_pc23_func_ctl::PAD_PC23_FUNC_CTL_SPEC>,
    #[doc = "0x2bc - PAD SETTINGS"]
    pub pad_pc23_pad_ctl: crate::Reg<pad_pc23_pad_ctl::PAD_PC23_PAD_CTL_SPEC>,
    #[doc = "0x2c0 - ALT SELECT"]
    pub pad_pc24_func_ctl: crate::Reg<pad_pc24_func_ctl::PAD_PC24_FUNC_CTL_SPEC>,
    #[doc = "0x2c4 - PAD SETTINGS"]
    pub pad_pc24_pad_ctl: crate::Reg<pad_pc24_pad_ctl::PAD_PC24_PAD_CTL_SPEC>,
    #[doc = "0x2c8 - ALT SELECT"]
    pub pad_pc25_func_ctl: crate::Reg<pad_pc25_func_ctl::PAD_PC25_FUNC_CTL_SPEC>,
    #[doc = "0x2cc - PAD SETTINGS"]
    pub pad_pc25_pad_ctl: crate::Reg<pad_pc25_pad_ctl::PAD_PC25_PAD_CTL_SPEC>,
    #[doc = "0x2d0 - ALT SELECT"]
    pub pad_pc26_func_ctl: crate::Reg<pad_pc26_func_ctl::PAD_PC26_FUNC_CTL_SPEC>,
    #[doc = "0x2d4 - PAD SETTINGS"]
    pub pad_pc26_pad_ctl: crate::Reg<pad_pc26_pad_ctl::PAD_PC26_PAD_CTL_SPEC>,
    #[doc = "0x2d8 - ALT SELECT"]
    pub pad_pc27_func_ctl: crate::Reg<pad_pc27_func_ctl::PAD_PC27_FUNC_CTL_SPEC>,
    #[doc = "0x2dc - PAD SETTINGS"]
    pub pad_pc27_pad_ctl: crate::Reg<pad_pc27_pad_ctl::PAD_PC27_PAD_CTL_SPEC>,
    #[doc = "0x2e0 - ALT SELECT"]
    pub pad_pc28_func_ctl: crate::Reg<pad_pc28_func_ctl::PAD_PC28_FUNC_CTL_SPEC>,
    #[doc = "0x2e4 - PAD SETTINGS"]
    pub pad_pc28_pad_ctl: crate::Reg<pad_pc28_pad_ctl::PAD_PC28_PAD_CTL_SPEC>,
    #[doc = "0x2e8 - ALT SELECT"]
    pub pad_pc29_func_ctl: crate::Reg<pad_pc29_func_ctl::PAD_PC29_FUNC_CTL_SPEC>,
    #[doc = "0x2ec - PAD SETTINGS"]
    pub pad_pc29_pad_ctl: crate::Reg<pad_pc29_pad_ctl::PAD_PC29_PAD_CTL_SPEC>,
    #[doc = "0x2f0 - ALT SELECT"]
    pub pad_pc30_func_ctl: crate::Reg<pad_pc30_func_ctl::PAD_PC30_FUNC_CTL_SPEC>,
    #[doc = "0x2f4 - PAD SETTINGS"]
    pub pad_pc30_pad_ctl: crate::Reg<pad_pc30_pad_ctl::PAD_PC30_PAD_CTL_SPEC>,
    #[doc = "0x2f8 - ALT SELECT"]
    pub pad_pc31_func_ctl: crate::Reg<pad_pc31_func_ctl::PAD_PC31_FUNC_CTL_SPEC>,
    #[doc = "0x2fc - PAD SETTINGS"]
    pub pad_pc31_pad_ctl: crate::Reg<pad_pc31_pad_ctl::PAD_PC31_PAD_CTL_SPEC>,
    #[doc = "0x300 - ALT SELECT"]
    pub pad_pd00_func_ctl: crate::Reg<pad_pd00_func_ctl::PAD_PD00_FUNC_CTL_SPEC>,
    #[doc = "0x304 - PAD SETTINGS"]
    pub pad_pd00_pad_ctl: crate::Reg<pad_pd00_pad_ctl::PAD_PD00_PAD_CTL_SPEC>,
    #[doc = "0x308 - ALT SELECT"]
    pub pad_pd01_func_ctl: crate::Reg<pad_pd01_func_ctl::PAD_PD01_FUNC_CTL_SPEC>,
    #[doc = "0x30c - PAD SETTINGS"]
    pub pad_pd01_pad_ctl: crate::Reg<pad_pd01_pad_ctl::PAD_PD01_PAD_CTL_SPEC>,
    #[doc = "0x310 - ALT SELECT"]
    pub pad_pd02_func_ctl: crate::Reg<pad_pd02_func_ctl::PAD_PD02_FUNC_CTL_SPEC>,
    #[doc = "0x314 - PAD SETTINGS"]
    pub pad_pd02_pad_ctl: crate::Reg<pad_pd02_pad_ctl::PAD_PD02_PAD_CTL_SPEC>,
    #[doc = "0x318 - ALT SELECT"]
    pub pad_pd03_func_ctl: crate::Reg<pad_pd03_func_ctl::PAD_PD03_FUNC_CTL_SPEC>,
    #[doc = "0x31c - PAD SETTINGS"]
    pub pad_pd03_pad_ctl: crate::Reg<pad_pd03_pad_ctl::PAD_PD03_PAD_CTL_SPEC>,
    #[doc = "0x320 - ALT SELECT"]
    pub pad_pd04_func_ctl: crate::Reg<pad_pd04_func_ctl::PAD_PD04_FUNC_CTL_SPEC>,
    #[doc = "0x324 - PAD SETTINGS"]
    pub pad_pd04_pad_ctl: crate::Reg<pad_pd04_pad_ctl::PAD_PD04_PAD_CTL_SPEC>,
    #[doc = "0x328 - ALT SELECT"]
    pub pad_pd05_func_ctl: crate::Reg<pad_pd05_func_ctl::PAD_PD05_FUNC_CTL_SPEC>,
    #[doc = "0x32c - PAD SETTINGS"]
    pub pad_pd05_pad_ctl: crate::Reg<pad_pd05_pad_ctl::PAD_PD05_PAD_CTL_SPEC>,
    #[doc = "0x330 - ALT SELECT"]
    pub pad_pd06_func_ctl: crate::Reg<pad_pd06_func_ctl::PAD_PD06_FUNC_CTL_SPEC>,
    #[doc = "0x334 - PAD SETTINGS"]
    pub pad_pd06_pad_ctl: crate::Reg<pad_pd06_pad_ctl::PAD_PD06_PAD_CTL_SPEC>,
    #[doc = "0x338 - ALT SELECT"]
    pub pad_pd07_func_ctl: crate::Reg<pad_pd07_func_ctl::PAD_PD07_FUNC_CTL_SPEC>,
    #[doc = "0x33c - PAD SETTINGS"]
    pub pad_pd07_pad_ctl: crate::Reg<pad_pd07_pad_ctl::PAD_PD07_PAD_CTL_SPEC>,
    #[doc = "0x340 - ALT SELECT"]
    pub pad_pd08_func_ctl: crate::Reg<pad_pd08_func_ctl::PAD_PD08_FUNC_CTL_SPEC>,
    #[doc = "0x344 - PAD SETTINGS"]
    pub pad_pd08_pad_ctl: crate::Reg<pad_pd08_pad_ctl::PAD_PD08_PAD_CTL_SPEC>,
    #[doc = "0x348 - ALT SELECT"]
    pub pad_pd09_func_ctl: crate::Reg<pad_pd09_func_ctl::PAD_PD09_FUNC_CTL_SPEC>,
    #[doc = "0x34c - PAD SETTINGS"]
    pub pad_pd09_pad_ctl: crate::Reg<pad_pd09_pad_ctl::PAD_PD09_PAD_CTL_SPEC>,
    #[doc = "0x350 - ALT SELECT"]
    pub pad_pd10_func_ctl: crate::Reg<pad_pd10_func_ctl::PAD_PD10_FUNC_CTL_SPEC>,
    #[doc = "0x354 - PAD SETTINGS"]
    pub pad_pd10_pad_ctl: crate::Reg<pad_pd10_pad_ctl::PAD_PD10_PAD_CTL_SPEC>,
    #[doc = "0x358 - ALT SELECT"]
    pub pad_pd11_func_ctl: crate::Reg<pad_pd11_func_ctl::PAD_PD11_FUNC_CTL_SPEC>,
    #[doc = "0x35c - PAD SETTINGS"]
    pub pad_pd11_pad_ctl: crate::Reg<pad_pd11_pad_ctl::PAD_PD11_PAD_CTL_SPEC>,
    #[doc = "0x360 - ALT SELECT"]
    pub pad_pd12_func_ctl: crate::Reg<pad_pd12_func_ctl::PAD_PD12_FUNC_CTL_SPEC>,
    #[doc = "0x364 - PAD SETTINGS"]
    pub pad_pd12_pad_ctl: crate::Reg<pad_pd12_pad_ctl::PAD_PD12_PAD_CTL_SPEC>,
    #[doc = "0x368 - ALT SELECT"]
    pub pad_pd13_func_ctl: crate::Reg<pad_pd13_func_ctl::PAD_PD13_FUNC_CTL_SPEC>,
    #[doc = "0x36c - PAD SETTINGS"]
    pub pad_pd13_pad_ctl: crate::Reg<pad_pd13_pad_ctl::PAD_PD13_PAD_CTL_SPEC>,
    #[doc = "0x370 - ALT SELECT"]
    pub pad_pd14_func_ctl: crate::Reg<pad_pd14_func_ctl::PAD_PD14_FUNC_CTL_SPEC>,
    #[doc = "0x374 - PAD SETTINGS"]
    pub pad_pd14_pad_ctl: crate::Reg<pad_pd14_pad_ctl::PAD_PD14_PAD_CTL_SPEC>,
    #[doc = "0x378 - ALT SELECT"]
    pub pad_pd15_func_ctl: crate::Reg<pad_pd15_func_ctl::PAD_PD15_FUNC_CTL_SPEC>,
    #[doc = "0x37c - PAD SETTINGS"]
    pub pad_pd15_pad_ctl: crate::Reg<pad_pd15_pad_ctl::PAD_PD15_PAD_CTL_SPEC>,
    #[doc = "0x380 - ALT SELECT"]
    pub pad_pd16_func_ctl: crate::Reg<pad_pd16_func_ctl::PAD_PD16_FUNC_CTL_SPEC>,
    #[doc = "0x384 - PAD SETTINGS"]
    pub pad_pd16_pad_ctl: crate::Reg<pad_pd16_pad_ctl::PAD_PD16_PAD_CTL_SPEC>,
    #[doc = "0x388 - ALT SELECT"]
    pub pad_pd17_func_ctl: crate::Reg<pad_pd17_func_ctl::PAD_PD17_FUNC_CTL_SPEC>,
    #[doc = "0x38c - PAD SETTINGS"]
    pub pad_pd17_pad_ctl: crate::Reg<pad_pd17_pad_ctl::PAD_PD17_PAD_CTL_SPEC>,
    #[doc = "0x390 - ALT SELECT"]
    pub pad_pd18_func_ctl: crate::Reg<pad_pd18_func_ctl::PAD_PD18_FUNC_CTL_SPEC>,
    #[doc = "0x394 - PAD SETTINGS"]
    pub pad_pd18_pad_ctl: crate::Reg<pad_pd18_pad_ctl::PAD_PD18_PAD_CTL_SPEC>,
    #[doc = "0x398 - ALT SELECT"]
    pub pad_pd19_func_ctl: crate::Reg<pad_pd19_func_ctl::PAD_PD19_FUNC_CTL_SPEC>,
    #[doc = "0x39c - PAD SETTINGS"]
    pub pad_pd19_pad_ctl: crate::Reg<pad_pd19_pad_ctl::PAD_PD19_PAD_CTL_SPEC>,
    #[doc = "0x3a0 - ALT SELECT"]
    pub pad_pd20_func_ctl: crate::Reg<pad_pd20_func_ctl::PAD_PD20_FUNC_CTL_SPEC>,
    #[doc = "0x3a4 - PAD SETTINGS"]
    pub pad_pd20_pad_ctl: crate::Reg<pad_pd20_pad_ctl::PAD_PD20_PAD_CTL_SPEC>,
    #[doc = "0x3a8 - ALT SELECT"]
    pub pad_pd21_func_ctl: crate::Reg<pad_pd21_func_ctl::PAD_PD21_FUNC_CTL_SPEC>,
    #[doc = "0x3ac - PAD SETTINGS"]
    pub pad_pd21_pad_ctl: crate::Reg<pad_pd21_pad_ctl::PAD_PD21_PAD_CTL_SPEC>,
    #[doc = "0x3b0 - ALT SELECT"]
    pub pad_pd22_func_ctl: crate::Reg<pad_pd22_func_ctl::PAD_PD22_FUNC_CTL_SPEC>,
    #[doc = "0x3b4 - PAD SETTINGS"]
    pub pad_pd22_pad_ctl: crate::Reg<pad_pd22_pad_ctl::PAD_PD22_PAD_CTL_SPEC>,
    #[doc = "0x3b8 - ALT SELECT"]
    pub pad_pd23_func_ctl: crate::Reg<pad_pd23_func_ctl::PAD_PD23_FUNC_CTL_SPEC>,
    #[doc = "0x3bc - PAD SETTINGS"]
    pub pad_pd23_pad_ctl: crate::Reg<pad_pd23_pad_ctl::PAD_PD23_PAD_CTL_SPEC>,
    #[doc = "0x3c0 - ALT SELECT"]
    pub pad_pd24_func_ctl: crate::Reg<pad_pd24_func_ctl::PAD_PD24_FUNC_CTL_SPEC>,
    #[doc = "0x3c4 - PAD SETTINGS"]
    pub pad_pd24_pad_ctl: crate::Reg<pad_pd24_pad_ctl::PAD_PD24_PAD_CTL_SPEC>,
    #[doc = "0x3c8 - ALT SELECT"]
    pub pad_pd25_func_ctl: crate::Reg<pad_pd25_func_ctl::PAD_PD25_FUNC_CTL_SPEC>,
    #[doc = "0x3cc - PAD SETTINGS"]
    pub pad_pd25_pad_ctl: crate::Reg<pad_pd25_pad_ctl::PAD_PD25_PAD_CTL_SPEC>,
    #[doc = "0x3d0 - ALT SELECT"]
    pub pad_pd26_func_ctl: crate::Reg<pad_pd26_func_ctl::PAD_PD26_FUNC_CTL_SPEC>,
    #[doc = "0x3d4 - PAD SETTINGS"]
    pub pad_pd26_pad_ctl: crate::Reg<pad_pd26_pad_ctl::PAD_PD26_PAD_CTL_SPEC>,
    #[doc = "0x3d8 - ALT SELECT"]
    pub pad_pd27_func_ctl: crate::Reg<pad_pd27_func_ctl::PAD_PD27_FUNC_CTL_SPEC>,
    #[doc = "0x3dc - PAD SETTINGS"]
    pub pad_pd27_pad_ctl: crate::Reg<pad_pd27_pad_ctl::PAD_PD27_PAD_CTL_SPEC>,
    #[doc = "0x3e0 - ALT SELECT"]
    pub pad_pd28_func_ctl: crate::Reg<pad_pd28_func_ctl::PAD_PD28_FUNC_CTL_SPEC>,
    #[doc = "0x3e4 - PAD SETTINGS"]
    pub pad_pd28_pad_ctl: crate::Reg<pad_pd28_pad_ctl::PAD_PD28_PAD_CTL_SPEC>,
    #[doc = "0x3e8 - ALT SELECT"]
    pub pad_pd29_func_ctl: crate::Reg<pad_pd29_func_ctl::PAD_PD29_FUNC_CTL_SPEC>,
    #[doc = "0x3ec - PAD SETTINGS"]
    pub pad_pd29_pad_ctl: crate::Reg<pad_pd29_pad_ctl::PAD_PD29_PAD_CTL_SPEC>,
    #[doc = "0x3f0 - ALT SELECT"]
    pub pad_pd30_func_ctl: crate::Reg<pad_pd30_func_ctl::PAD_PD30_FUNC_CTL_SPEC>,
    #[doc = "0x3f4 - PAD SETTINGS"]
    pub pad_pd30_pad_ctl: crate::Reg<pad_pd30_pad_ctl::PAD_PD30_PAD_CTL_SPEC>,
    #[doc = "0x3f8 - ALT SELECT"]
    pub pad_pd31_func_ctl: crate::Reg<pad_pd31_func_ctl::PAD_PD31_FUNC_CTL_SPEC>,
    #[doc = "0x3fc - PAD SETTINGS"]
    pub pad_pd31_pad_ctl: crate::Reg<pad_pd31_pad_ctl::PAD_PD31_PAD_CTL_SPEC>,
    #[doc = "0x400 - ALT SELECT"]
    pub pad_pe00_func_ctl: crate::Reg<pad_pe00_func_ctl::PAD_PE00_FUNC_CTL_SPEC>,
    #[doc = "0x404 - PAD SETTINGS"]
    pub pad_pe00_pad_ctl: crate::Reg<pad_pe00_pad_ctl::PAD_PE00_PAD_CTL_SPEC>,
    #[doc = "0x408 - ALT SELECT"]
    pub pad_pe01_func_ctl: crate::Reg<pad_pe01_func_ctl::PAD_PE01_FUNC_CTL_SPEC>,
    #[doc = "0x40c - PAD SETTINGS"]
    pub pad_pe01_pad_ctl: crate::Reg<pad_pe01_pad_ctl::PAD_PE01_PAD_CTL_SPEC>,
    #[doc = "0x410 - ALT SELECT"]
    pub pad_pe02_func_ctl: crate::Reg<pad_pe02_func_ctl::PAD_PE02_FUNC_CTL_SPEC>,
    #[doc = "0x414 - PAD SETTINGS"]
    pub pad_pe02_pad_ctl: crate::Reg<pad_pe02_pad_ctl::PAD_PE02_PAD_CTL_SPEC>,
    #[doc = "0x418 - ALT SELECT"]
    pub pad_pe03_func_ctl: crate::Reg<pad_pe03_func_ctl::PAD_PE03_FUNC_CTL_SPEC>,
    #[doc = "0x41c - PAD SETTINGS"]
    pub pad_pe03_pad_ctl: crate::Reg<pad_pe03_pad_ctl::PAD_PE03_PAD_CTL_SPEC>,
    #[doc = "0x420 - ALT SELECT"]
    pub pad_pe04_func_ctl: crate::Reg<pad_pe04_func_ctl::PAD_PE04_FUNC_CTL_SPEC>,
    #[doc = "0x424 - PAD SETTINGS"]
    pub pad_pe04_pad_ctl: crate::Reg<pad_pe04_pad_ctl::PAD_PE04_PAD_CTL_SPEC>,
    #[doc = "0x428 - ALT SELECT"]
    pub pad_pe05_func_ctl: crate::Reg<pad_pe05_func_ctl::PAD_PE05_FUNC_CTL_SPEC>,
    #[doc = "0x42c - PAD SETTINGS"]
    pub pad_pe05_pad_ctl: crate::Reg<pad_pe05_pad_ctl::PAD_PE05_PAD_CTL_SPEC>,
    #[doc = "0x430 - ALT SELECT"]
    pub pad_pe06_func_ctl: crate::Reg<pad_pe06_func_ctl::PAD_PE06_FUNC_CTL_SPEC>,
    #[doc = "0x434 - PAD SETTINGS"]
    pub pad_pe06_pad_ctl: crate::Reg<pad_pe06_pad_ctl::PAD_PE06_PAD_CTL_SPEC>,
    #[doc = "0x438 - ALT SELECT"]
    pub pad_pe07_func_ctl: crate::Reg<pad_pe07_func_ctl::PAD_PE07_FUNC_CTL_SPEC>,
    #[doc = "0x43c - PAD SETTINGS"]
    pub pad_pe07_pad_ctl: crate::Reg<pad_pe07_pad_ctl::PAD_PE07_PAD_CTL_SPEC>,
    #[doc = "0x440 - ALT SELECT"]
    pub pad_pe08_func_ctl: crate::Reg<pad_pe08_func_ctl::PAD_PE08_FUNC_CTL_SPEC>,
    #[doc = "0x444 - PAD SETTINGS"]
    pub pad_pe08_pad_ctl: crate::Reg<pad_pe08_pad_ctl::PAD_PE08_PAD_CTL_SPEC>,
    #[doc = "0x448 - ALT SELECT"]
    pub pad_pe09_func_ctl: crate::Reg<pad_pe09_func_ctl::PAD_PE09_FUNC_CTL_SPEC>,
    #[doc = "0x44c - PAD SETTINGS"]
    pub pad_pe09_pad_ctl: crate::Reg<pad_pe09_pad_ctl::PAD_PE09_PAD_CTL_SPEC>,
    #[doc = "0x450 - ALT SELECT"]
    pub pad_pe10_func_ctl: crate::Reg<pad_pe10_func_ctl::PAD_PE10_FUNC_CTL_SPEC>,
    #[doc = "0x454 - PAD SETTINGS"]
    pub pad_pe10_pad_ctl: crate::Reg<pad_pe10_pad_ctl::PAD_PE10_PAD_CTL_SPEC>,
    #[doc = "0x458 - ALT SELECT"]
    pub pad_pe11_func_ctl: crate::Reg<pad_pe11_func_ctl::PAD_PE11_FUNC_CTL_SPEC>,
    #[doc = "0x45c - PAD SETTINGS"]
    pub pad_pe11_pad_ctl: crate::Reg<pad_pe11_pad_ctl::PAD_PE11_PAD_CTL_SPEC>,
    #[doc = "0x460 - ALT SELECT"]
    pub pad_pe12_func_ctl: crate::Reg<pad_pe12_func_ctl::PAD_PE12_FUNC_CTL_SPEC>,
    #[doc = "0x464 - PAD SETTINGS"]
    pub pad_pe12_pad_ctl: crate::Reg<pad_pe12_pad_ctl::PAD_PE12_PAD_CTL_SPEC>,
    #[doc = "0x468 - ALT SELECT"]
    pub pad_pe13_func_ctl: crate::Reg<pad_pe13_func_ctl::PAD_PE13_FUNC_CTL_SPEC>,
    #[doc = "0x46c - PAD SETTINGS"]
    pub pad_pe13_pad_ctl: crate::Reg<pad_pe13_pad_ctl::PAD_PE13_PAD_CTL_SPEC>,
    #[doc = "0x470 - ALT SELECT"]
    pub pad_pe14_func_ctl: crate::Reg<pad_pe14_func_ctl::PAD_PE14_FUNC_CTL_SPEC>,
    #[doc = "0x474 - PAD SETTINGS"]
    pub pad_pe14_pad_ctl: crate::Reg<pad_pe14_pad_ctl::PAD_PE14_PAD_CTL_SPEC>,
    #[doc = "0x478 - ALT SELECT"]
    pub pad_pe15_func_ctl: crate::Reg<pad_pe15_func_ctl::PAD_PE15_FUNC_CTL_SPEC>,
    #[doc = "0x47c - PAD SETTINGS"]
    pub pad_pe15_pad_ctl: crate::Reg<pad_pe15_pad_ctl::PAD_PE15_PAD_CTL_SPEC>,
    #[doc = "0x480 - ALT SELECT"]
    pub pad_pe16_func_ctl: crate::Reg<pad_pe16_func_ctl::PAD_PE16_FUNC_CTL_SPEC>,
    #[doc = "0x484 - PAD SETTINGS"]
    pub pad_pe16_pad_ctl: crate::Reg<pad_pe16_pad_ctl::PAD_PE16_PAD_CTL_SPEC>,
    #[doc = "0x488 - ALT SELECT"]
    pub pad_pe17_func_ctl: crate::Reg<pad_pe17_func_ctl::PAD_PE17_FUNC_CTL_SPEC>,
    #[doc = "0x48c - PAD SETTINGS"]
    pub pad_pe17_pad_ctl: crate::Reg<pad_pe17_pad_ctl::PAD_PE17_PAD_CTL_SPEC>,
    #[doc = "0x490 - ALT SELECT"]
    pub pad_pe18_func_ctl: crate::Reg<pad_pe18_func_ctl::PAD_PE18_FUNC_CTL_SPEC>,
    #[doc = "0x494 - PAD SETTINGS"]
    pub pad_pe18_pad_ctl: crate::Reg<pad_pe18_pad_ctl::PAD_PE18_PAD_CTL_SPEC>,
    #[doc = "0x498 - ALT SELECT"]
    pub pad_pe19_func_ctl: crate::Reg<pad_pe19_func_ctl::PAD_PE19_FUNC_CTL_SPEC>,
    #[doc = "0x49c - PAD SETTINGS"]
    pub pad_pe19_pad_ctl: crate::Reg<pad_pe19_pad_ctl::PAD_PE19_PAD_CTL_SPEC>,
    #[doc = "0x4a0 - ALT SELECT"]
    pub pad_pe20_func_ctl: crate::Reg<pad_pe20_func_ctl::PAD_PE20_FUNC_CTL_SPEC>,
    #[doc = "0x4a4 - PAD SETTINGS"]
    pub pad_pe20_pad_ctl: crate::Reg<pad_pe20_pad_ctl::PAD_PE20_PAD_CTL_SPEC>,
    #[doc = "0x4a8 - ALT SELECT"]
    pub pad_pe21_func_ctl: crate::Reg<pad_pe21_func_ctl::PAD_PE21_FUNC_CTL_SPEC>,
    #[doc = "0x4ac - PAD SETTINGS"]
    pub pad_pe21_pad_ctl: crate::Reg<pad_pe21_pad_ctl::PAD_PE21_PAD_CTL_SPEC>,
    #[doc = "0x4b0 - ALT SELECT"]
    pub pad_pe22_func_ctl: crate::Reg<pad_pe22_func_ctl::PAD_PE22_FUNC_CTL_SPEC>,
    #[doc = "0x4b4 - PAD SETTINGS"]
    pub pad_pe22_pad_ctl: crate::Reg<pad_pe22_pad_ctl::PAD_PE22_PAD_CTL_SPEC>,
    #[doc = "0x4b8 - ALT SELECT"]
    pub pad_pe23_func_ctl: crate::Reg<pad_pe23_func_ctl::PAD_PE23_FUNC_CTL_SPEC>,
    #[doc = "0x4bc - PAD SETTINGS"]
    pub pad_pe23_pad_ctl: crate::Reg<pad_pe23_pad_ctl::PAD_PE23_PAD_CTL_SPEC>,
    #[doc = "0x4c0 - ALT SELECT"]
    pub pad_pe24_func_ctl: crate::Reg<pad_pe24_func_ctl::PAD_PE24_FUNC_CTL_SPEC>,
    #[doc = "0x4c4 - PAD SETTINGS"]
    pub pad_pe24_pad_ctl: crate::Reg<pad_pe24_pad_ctl::PAD_PE24_PAD_CTL_SPEC>,
    #[doc = "0x4c8 - ALT SELECT"]
    pub pad_pe25_func_ctl: crate::Reg<pad_pe25_func_ctl::PAD_PE25_FUNC_CTL_SPEC>,
    #[doc = "0x4cc - PAD SETTINGS"]
    pub pad_pe25_pad_ctl: crate::Reg<pad_pe25_pad_ctl::PAD_PE25_PAD_CTL_SPEC>,
    #[doc = "0x4d0 - ALT SELECT"]
    pub pad_pe26_func_ctl: crate::Reg<pad_pe26_func_ctl::PAD_PE26_FUNC_CTL_SPEC>,
    #[doc = "0x4d4 - PAD SETTINGS"]
    pub pad_pe26_pad_ctl: crate::Reg<pad_pe26_pad_ctl::PAD_PE26_PAD_CTL_SPEC>,
    #[doc = "0x4d8 - ALT SELECT"]
    pub pad_pe27_func_ctl: crate::Reg<pad_pe27_func_ctl::PAD_PE27_FUNC_CTL_SPEC>,
    #[doc = "0x4dc - PAD SETTINGS"]
    pub pad_pe27_pad_ctl: crate::Reg<pad_pe27_pad_ctl::PAD_PE27_PAD_CTL_SPEC>,
    #[doc = "0x4e0 - ALT SELECT"]
    pub pad_pe28_func_ctl: crate::Reg<pad_pe28_func_ctl::PAD_PE28_FUNC_CTL_SPEC>,
    #[doc = "0x4e4 - PAD SETTINGS"]
    pub pad_pe28_pad_ctl: crate::Reg<pad_pe28_pad_ctl::PAD_PE28_PAD_CTL_SPEC>,
    #[doc = "0x4e8 - ALT SELECT"]
    pub pad_pe29_func_ctl: crate::Reg<pad_pe29_func_ctl::PAD_PE29_FUNC_CTL_SPEC>,
    #[doc = "0x4ec - PAD SETTINGS"]
    pub pad_pe29_pad_ctl: crate::Reg<pad_pe29_pad_ctl::PAD_PE29_PAD_CTL_SPEC>,
    #[doc = "0x4f0 - ALT SELECT"]
    pub pad_pe30_func_ctl: crate::Reg<pad_pe30_func_ctl::PAD_PE30_FUNC_CTL_SPEC>,
    #[doc = "0x4f4 - PAD SETTINGS"]
    pub pad_pe30_pad_ctl: crate::Reg<pad_pe30_pad_ctl::PAD_PE30_PAD_CTL_SPEC>,
    #[doc = "0x4f8 - ALT SELECT"]
    pub pad_pe31_func_ctl: crate::Reg<pad_pe31_func_ctl::PAD_PE31_FUNC_CTL_SPEC>,
    #[doc = "0x4fc - PAD SETTINGS"]
    pub pad_pe31_pad_ctl: crate::Reg<pad_pe31_pad_ctl::PAD_PE31_PAD_CTL_SPEC>,
    #[doc = "0x500 - ALT SELECT"]
    pub pad_pf00_func_ctl: crate::Reg<pad_pf00_func_ctl::PAD_PF00_FUNC_CTL_SPEC>,
    #[doc = "0x504 - PAD SETTINGS"]
    pub pad_pf00_pad_ctl: crate::Reg<pad_pf00_pad_ctl::PAD_PF00_PAD_CTL_SPEC>,
    #[doc = "0x508 - ALT SELECT"]
    pub pad_pf01_func_ctl: crate::Reg<pad_pf01_func_ctl::PAD_PF01_FUNC_CTL_SPEC>,
    #[doc = "0x50c - PAD SETTINGS"]
    pub pad_pf01_pad_ctl: crate::Reg<pad_pf01_pad_ctl::PAD_PF01_PAD_CTL_SPEC>,
    #[doc = "0x510 - ALT SELECT"]
    pub pad_pf02_func_ctl: crate::Reg<pad_pf02_func_ctl::PAD_PF02_FUNC_CTL_SPEC>,
    #[doc = "0x514 - PAD SETTINGS"]
    pub pad_pf02_pad_ctl: crate::Reg<pad_pf02_pad_ctl::PAD_PF02_PAD_CTL_SPEC>,
    #[doc = "0x518 - ALT SELECT"]
    pub pad_pf03_func_ctl: crate::Reg<pad_pf03_func_ctl::PAD_PF03_FUNC_CTL_SPEC>,
    #[doc = "0x51c - PAD SETTINGS"]
    pub pad_pf03_pad_ctl: crate::Reg<pad_pf03_pad_ctl::PAD_PF03_PAD_CTL_SPEC>,
    #[doc = "0x520 - ALT SELECT"]
    pub pad_pf04_func_ctl: crate::Reg<pad_pf04_func_ctl::PAD_PF04_FUNC_CTL_SPEC>,
    #[doc = "0x524 - PAD SETTINGS"]
    pub pad_pf04_pad_ctl: crate::Reg<pad_pf04_pad_ctl::PAD_PF04_PAD_CTL_SPEC>,
    #[doc = "0x528 - ALT SELECT"]
    pub pad_pf05_func_ctl: crate::Reg<pad_pf05_func_ctl::PAD_PF05_FUNC_CTL_SPEC>,
    #[doc = "0x52c - PAD SETTINGS"]
    pub pad_pf05_pad_ctl: crate::Reg<pad_pf05_pad_ctl::PAD_PF05_PAD_CTL_SPEC>,
    #[doc = "0x530 - ALT SELECT"]
    pub pad_pf06_func_ctl: crate::Reg<pad_pf06_func_ctl::PAD_PF06_FUNC_CTL_SPEC>,
    #[doc = "0x534 - PAD SETTINGS"]
    pub pad_pf06_pad_ctl: crate::Reg<pad_pf06_pad_ctl::PAD_PF06_PAD_CTL_SPEC>,
    #[doc = "0x538 - ALT SELECT"]
    pub pad_pf07_func_ctl: crate::Reg<pad_pf07_func_ctl::PAD_PF07_FUNC_CTL_SPEC>,
    #[doc = "0x53c - PAD SETTINGS"]
    pub pad_pf07_pad_ctl: crate::Reg<pad_pf07_pad_ctl::PAD_PF07_PAD_CTL_SPEC>,
    #[doc = "0x540 - ALT SELECT"]
    pub pad_pf08_func_ctl: crate::Reg<pad_pf08_func_ctl::PAD_PF08_FUNC_CTL_SPEC>,
    #[doc = "0x544 - PAD SETTINGS"]
    pub pad_pf08_pad_ctl: crate::Reg<pad_pf08_pad_ctl::PAD_PF08_PAD_CTL_SPEC>,
    #[doc = "0x548 - ALT SELECT"]
    pub pad_pf09_func_ctl: crate::Reg<pad_pf09_func_ctl::PAD_PF09_FUNC_CTL_SPEC>,
    #[doc = "0x54c - PAD SETTINGS"]
    pub pad_pf09_pad_ctl: crate::Reg<pad_pf09_pad_ctl::PAD_PF09_PAD_CTL_SPEC>,
    #[doc = "0x550 - ALT SELECT"]
    pub pad_pf10_func_ctl: crate::Reg<pad_pf10_func_ctl::PAD_PF10_FUNC_CTL_SPEC>,
    #[doc = "0x554 - PAD SETTINGS"]
    pub pad_pf10_pad_ctl: crate::Reg<pad_pf10_pad_ctl::PAD_PF10_PAD_CTL_SPEC>,
    _reserved342: [u8; 0x07a8],
    #[doc = "0xd00 - ALT SELECT"]
    pub pad_px00_func_ctl: crate::Reg<pad_px00_func_ctl::PAD_PX00_FUNC_CTL_SPEC>,
    #[doc = "0xd04 - PAD SETTINGS"]
    pub pad_px00_pad_ctl: crate::Reg<pad_px00_pad_ctl::PAD_PX00_PAD_CTL_SPEC>,
    #[doc = "0xd08 - ALT SELECT"]
    pub pad_px01_func_ctl: crate::Reg<pad_px01_func_ctl::PAD_PX01_FUNC_CTL_SPEC>,
    #[doc = "0xd0c - PAD SETTINGS"]
    pub pad_px01_pad_ctl: crate::Reg<pad_px01_pad_ctl::PAD_PX01_PAD_CTL_SPEC>,
    #[doc = "0xd10 - ALT SELECT"]
    pub pad_px02_func_ctl: crate::Reg<pad_px02_func_ctl::PAD_PX02_FUNC_CTL_SPEC>,
    #[doc = "0xd14 - PAD SETTINGS"]
    pub pad_px02_pad_ctl: crate::Reg<pad_px02_pad_ctl::PAD_PX02_PAD_CTL_SPEC>,
    #[doc = "0xd18 - ALT SELECT"]
    pub pad_px03_func_ctl: crate::Reg<pad_px03_func_ctl::PAD_PX03_FUNC_CTL_SPEC>,
    #[doc = "0xd1c - PAD SETTINGS"]
    pub pad_px03_pad_ctl: crate::Reg<pad_px03_pad_ctl::PAD_PX03_PAD_CTL_SPEC>,
    #[doc = "0xd20 - ALT SELECT"]
    pub pad_px04_func_ctl: crate::Reg<pad_px04_func_ctl::PAD_PX04_FUNC_CTL_SPEC>,
    #[doc = "0xd24 - PAD SETTINGS"]
    pub pad_px04_pad_ctl: crate::Reg<pad_px04_pad_ctl::PAD_PX04_PAD_CTL_SPEC>,
    #[doc = "0xd28 - ALT SELECT"]
    pub pad_px05_func_ctl: crate::Reg<pad_px05_func_ctl::PAD_PX05_FUNC_CTL_SPEC>,
    #[doc = "0xd2c - PAD SETTINGS"]
    pub pad_px05_pad_ctl: crate::Reg<pad_px05_pad_ctl::PAD_PX05_PAD_CTL_SPEC>,
    #[doc = "0xd30 - ALT SELECT"]
    pub pad_px06_func_ctl: crate::Reg<pad_px06_func_ctl::PAD_PX06_FUNC_CTL_SPEC>,
    #[doc = "0xd34 - PAD SETTINGS"]
    pub pad_px06_pad_ctl: crate::Reg<pad_px06_pad_ctl::PAD_PX06_PAD_CTL_SPEC>,
    #[doc = "0xd38 - ALT SELECT"]
    pub pad_px07_func_ctl: crate::Reg<pad_px07_func_ctl::PAD_PX07_FUNC_CTL_SPEC>,
    #[doc = "0xd3c - PAD SETTINGS"]
    pub pad_px07_pad_ctl: crate::Reg<pad_px07_pad_ctl::PAD_PX07_PAD_CTL_SPEC>,
    #[doc = "0xd40 - ALT SELECT"]
    pub pad_px08_func_ctl: crate::Reg<pad_px08_func_ctl::PAD_PX08_FUNC_CTL_SPEC>,
    #[doc = "0xd44 - PAD SETTINGS"]
    pub pad_px08_pad_ctl: crate::Reg<pad_px08_pad_ctl::PAD_PX08_PAD_CTL_SPEC>,
    #[doc = "0xd48 - ALT SELECT"]
    pub pad_px09_func_ctl: crate::Reg<pad_px09_func_ctl::PAD_PX09_FUNC_CTL_SPEC>,
    #[doc = "0xd4c - PAD SETTINGS"]
    pub pad_px09_pad_ctl: crate::Reg<pad_px09_pad_ctl::PAD_PX09_PAD_CTL_SPEC>,
    #[doc = "0xd50 - ALT SELECT"]
    pub pad_px10_func_ctl: crate::Reg<pad_px10_func_ctl::PAD_PX10_FUNC_CTL_SPEC>,
    #[doc = "0xd54 - PAD SETTINGS"]
    pub pad_px10_pad_ctl: crate::Reg<pad_px10_pad_ctl::PAD_PX10_PAD_CTL_SPEC>,
    #[doc = "0xd58 - ALT SELECT"]
    pub pad_px11_func_ctl: crate::Reg<pad_px11_func_ctl::PAD_PX11_FUNC_CTL_SPEC>,
    #[doc = "0xd5c - PAD SETTINGS"]
    pub pad_px11_pad_ctl: crate::Reg<pad_px11_pad_ctl::PAD_PX11_PAD_CTL_SPEC>,
    _reserved366: [u8; 0xa0],
    #[doc = "0xe00 - ALT SELECT"]
    pub pad_py00_func_ctl: crate::Reg<pad_py00_func_ctl::PAD_PY00_FUNC_CTL_SPEC>,
    #[doc = "0xe04 - PAD SETTINGS"]
    pub pad_py00_pad_ctl: crate::Reg<pad_py00_pad_ctl::PAD_PY00_PAD_CTL_SPEC>,
    #[doc = "0xe08 - ALT SELECT"]
    pub pad_py01_func_ctl: crate::Reg<pad_py01_func_ctl::PAD_PY01_FUNC_CTL_SPEC>,
    #[doc = "0xe0c - PAD SETTINGS"]
    pub pad_py01_pad_ctl: crate::Reg<pad_py01_pad_ctl::PAD_PY01_PAD_CTL_SPEC>,
    #[doc = "0xe10 - ALT SELECT"]
    pub pad_py02_func_ctl: crate::Reg<pad_py02_func_ctl::PAD_PY02_FUNC_CTL_SPEC>,
    #[doc = "0xe14 - PAD SETTINGS"]
    pub pad_py02_pad_ctl: crate::Reg<pad_py02_pad_ctl::PAD_PY02_PAD_CTL_SPEC>,
    #[doc = "0xe18 - ALT SELECT"]
    pub pad_py03_func_ctl: crate::Reg<pad_py03_func_ctl::PAD_PY03_FUNC_CTL_SPEC>,
    #[doc = "0xe1c - PAD SETTINGS"]
    pub pad_py03_pad_ctl: crate::Reg<pad_py03_pad_ctl::PAD_PY03_PAD_CTL_SPEC>,
    #[doc = "0xe20 - ALT SELECT"]
    pub pad_py04_func_ctl: crate::Reg<pad_py04_func_ctl::PAD_PY04_FUNC_CTL_SPEC>,
    #[doc = "0xe24 - PAD SETTINGS"]
    pub pad_py04_pad_ctl: crate::Reg<pad_py04_pad_ctl::PAD_PY04_PAD_CTL_SPEC>,
    #[doc = "0xe28 - ALT SELECT"]
    pub pad_py05_func_ctl: crate::Reg<pad_py05_func_ctl::PAD_PY05_FUNC_CTL_SPEC>,
    #[doc = "0xe2c - PAD SETTINGS"]
    pub pad_py05_pad_ctl: crate::Reg<pad_py05_pad_ctl::PAD_PY05_PAD_CTL_SPEC>,
    #[doc = "0xe30 - ALT SELECT"]
    pub pad_py06_func_ctl: crate::Reg<pad_py06_func_ctl::PAD_PY06_FUNC_CTL_SPEC>,
    #[doc = "0xe34 - PAD SETTINGS"]
    pub pad_py06_pad_ctl: crate::Reg<pad_py06_pad_ctl::PAD_PY06_PAD_CTL_SPEC>,
    #[doc = "0xe38 - ALT SELECT"]
    pub pad_py07_func_ctl: crate::Reg<pad_py07_func_ctl::PAD_PY07_FUNC_CTL_SPEC>,
    #[doc = "0xe3c - PAD SETTINGS"]
    pub pad_py07_pad_ctl: crate::Reg<pad_py07_pad_ctl::PAD_PY07_PAD_CTL_SPEC>,
    #[doc = "0xe40 - ALT SELECT"]
    pub pad_py08_func_ctl: crate::Reg<pad_py08_func_ctl::PAD_PY08_FUNC_CTL_SPEC>,
    #[doc = "0xe44 - PAD SETTINGS"]
    pub pad_py08_pad_ctl: crate::Reg<pad_py08_pad_ctl::PAD_PY08_PAD_CTL_SPEC>,
    #[doc = "0xe48 - ALT SELECT"]
    pub pad_py09_func_ctl: crate::Reg<pad_py09_func_ctl::PAD_PY09_FUNC_CTL_SPEC>,
    #[doc = "0xe4c - PAD SETTINGS"]
    pub pad_py09_pad_ctl: crate::Reg<pad_py09_pad_ctl::PAD_PY09_PAD_CTL_SPEC>,
    #[doc = "0xe50 - ALT SELECT"]
    pub pad_py10_func_ctl: crate::Reg<pad_py10_func_ctl::PAD_PY10_FUNC_CTL_SPEC>,
    #[doc = "0xe54 - PAD SETTINGS"]
    pub pad_py10_pad_ctl: crate::Reg<pad_py10_pad_ctl::PAD_PY10_PAD_CTL_SPEC>,
    #[doc = "0xe58 - ALT SELECT"]
    pub pad_py11_func_ctl: crate::Reg<pad_py11_func_ctl::PAD_PY11_FUNC_CTL_SPEC>,
    #[doc = "0xe5c - PAD SETTINGS"]
    pub pad_py11_pad_ctl: crate::Reg<pad_py11_pad_ctl::PAD_PY11_PAD_CTL_SPEC>,
    _reserved390: [u8; 0xa0],
    #[doc = "0xf00 - ALT SELECT"]
    pub pad_pz00_func_ctl: crate::Reg<pad_pz00_func_ctl::PAD_PZ00_FUNC_CTL_SPEC>,
    #[doc = "0xf04 - PAD SETTINGS"]
    pub pad_pz00_pad_ctl: crate::Reg<pad_pz00_pad_ctl::PAD_PZ00_PAD_CTL_SPEC>,
    #[doc = "0xf08 - ALT SELECT"]
    pub pad_pz01_func_ctl: crate::Reg<pad_pz01_func_ctl::PAD_PZ01_FUNC_CTL_SPEC>,
    #[doc = "0xf0c - PAD SETTINGS"]
    pub pad_pz01_pad_ctl: crate::Reg<pad_pz01_pad_ctl::PAD_PZ01_PAD_CTL_SPEC>,
    #[doc = "0xf10 - ALT SELECT"]
    pub pad_pz02_func_ctl: crate::Reg<pad_pz02_func_ctl::PAD_PZ02_FUNC_CTL_SPEC>,
    #[doc = "0xf14 - PAD SETTINGS"]
    pub pad_pz02_pad_ctl: crate::Reg<pad_pz02_pad_ctl::PAD_PZ02_PAD_CTL_SPEC>,
    #[doc = "0xf18 - ALT SELECT"]
    pub pad_pz03_func_ctl: crate::Reg<pad_pz03_func_ctl::PAD_PZ03_FUNC_CTL_SPEC>,
    #[doc = "0xf1c - PAD SETTINGS"]
    pub pad_pz03_pad_ctl: crate::Reg<pad_pz03_pad_ctl::PAD_PZ03_PAD_CTL_SPEC>,
    #[doc = "0xf20 - ALT SELECT"]
    pub pad_pz04_func_ctl: crate::Reg<pad_pz04_func_ctl::PAD_PZ04_FUNC_CTL_SPEC>,
    #[doc = "0xf24 - PAD SETTINGS"]
    pub pad_pz04_pad_ctl: crate::Reg<pad_pz04_pad_ctl::PAD_PZ04_PAD_CTL_SPEC>,
    #[doc = "0xf28 - ALT SELECT"]
    pub pad_pz05_func_ctl: crate::Reg<pad_pz05_func_ctl::PAD_PZ05_FUNC_CTL_SPEC>,
    #[doc = "0xf2c - PAD SETTINGS"]
    pub pad_pz05_pad_ctl: crate::Reg<pad_pz05_pad_ctl::PAD_PZ05_PAD_CTL_SPEC>,
    #[doc = "0xf30 - ALT SELECT"]
    pub pad_pz06_func_ctl: crate::Reg<pad_pz06_func_ctl::PAD_PZ06_FUNC_CTL_SPEC>,
    #[doc = "0xf34 - PAD SETTINGS"]
    pub pad_pz06_pad_ctl: crate::Reg<pad_pz06_pad_ctl::PAD_PZ06_PAD_CTL_SPEC>,
    #[doc = "0xf38 - ALT SELECT"]
    pub pad_pz07_func_ctl: crate::Reg<pad_pz07_func_ctl::PAD_PZ07_FUNC_CTL_SPEC>,
    #[doc = "0xf3c - PAD SETTINGS"]
    pub pad_pz07_pad_ctl: crate::Reg<pad_pz07_pad_ctl::PAD_PZ07_PAD_CTL_SPEC>,
    #[doc = "0xf40 - ALT SELECT"]
    pub pad_pz08_func_ctl: crate::Reg<pad_pz08_func_ctl::PAD_PZ08_FUNC_CTL_SPEC>,
    #[doc = "0xf44 - PAD SETTINGS"]
    pub pad_pz08_pad_ctl: crate::Reg<pad_pz08_pad_ctl::PAD_PZ08_PAD_CTL_SPEC>,
    #[doc = "0xf48 - ALT SELECT"]
    pub pad_pz09_func_ctl: crate::Reg<pad_pz09_func_ctl::PAD_PZ09_FUNC_CTL_SPEC>,
    #[doc = "0xf4c - PAD SETTINGS"]
    pub pad_pz09_pad_ctl: crate::Reg<pad_pz09_pad_ctl::PAD_PZ09_PAD_CTL_SPEC>,
    #[doc = "0xf50 - ALT SELECT"]
    pub pad_pz10_func_ctl: crate::Reg<pad_pz10_func_ctl::PAD_PZ10_FUNC_CTL_SPEC>,
    #[doc = "0xf54 - PAD SETTINGS"]
    pub pad_pz10_pad_ctl: crate::Reg<pad_pz10_pad_ctl::PAD_PZ10_PAD_CTL_SPEC>,
    #[doc = "0xf58 - ALT SELECT"]
    pub pad_pz11_func_ctl: crate::Reg<pad_pz11_func_ctl::PAD_PZ11_FUNC_CTL_SPEC>,
    #[doc = "0xf5c - PAD SETTINGS"]
    pub pad_pz11_pad_ctl: crate::Reg<pad_pz11_pad_ctl::PAD_PZ11_PAD_CTL_SPEC>,
}
#[doc = "PAD_PA00_FUNC_CTL register accessor: an alias for `Reg<PAD_PA00_FUNC_CTL_SPEC>`"]
pub type PAD_PA00_FUNC_CTL = crate::Reg<pad_pa00_func_ctl::PAD_PA00_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa00_func_ctl;
#[doc = "PAD_PA00_PAD_CTL register accessor: an alias for `Reg<PAD_PA00_PAD_CTL_SPEC>`"]
pub type PAD_PA00_PAD_CTL = crate::Reg<pad_pa00_pad_ctl::PAD_PA00_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa00_pad_ctl;
#[doc = "PAD_PA01_FUNC_CTL register accessor: an alias for `Reg<PAD_PA01_FUNC_CTL_SPEC>`"]
pub type PAD_PA01_FUNC_CTL = crate::Reg<pad_pa01_func_ctl::PAD_PA01_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa01_func_ctl;
#[doc = "PAD_PA01_PAD_CTL register accessor: an alias for `Reg<PAD_PA01_PAD_CTL_SPEC>`"]
pub type PAD_PA01_PAD_CTL = crate::Reg<pad_pa01_pad_ctl::PAD_PA01_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa01_pad_ctl;
#[doc = "PAD_PA02_FUNC_CTL register accessor: an alias for `Reg<PAD_PA02_FUNC_CTL_SPEC>`"]
pub type PAD_PA02_FUNC_CTL = crate::Reg<pad_pa02_func_ctl::PAD_PA02_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa02_func_ctl;
#[doc = "PAD_PA02_PAD_CTL register accessor: an alias for `Reg<PAD_PA02_PAD_CTL_SPEC>`"]
pub type PAD_PA02_PAD_CTL = crate::Reg<pad_pa02_pad_ctl::PAD_PA02_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa02_pad_ctl;
#[doc = "PAD_PA03_FUNC_CTL register accessor: an alias for `Reg<PAD_PA03_FUNC_CTL_SPEC>`"]
pub type PAD_PA03_FUNC_CTL = crate::Reg<pad_pa03_func_ctl::PAD_PA03_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa03_func_ctl;
#[doc = "PAD_PA03_PAD_CTL register accessor: an alias for `Reg<PAD_PA03_PAD_CTL_SPEC>`"]
pub type PAD_PA03_PAD_CTL = crate::Reg<pad_pa03_pad_ctl::PAD_PA03_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa03_pad_ctl;
#[doc = "PAD_PA04_FUNC_CTL register accessor: an alias for `Reg<PAD_PA04_FUNC_CTL_SPEC>`"]
pub type PAD_PA04_FUNC_CTL = crate::Reg<pad_pa04_func_ctl::PAD_PA04_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa04_func_ctl;
#[doc = "PAD_PA04_PAD_CTL register accessor: an alias for `Reg<PAD_PA04_PAD_CTL_SPEC>`"]
pub type PAD_PA04_PAD_CTL = crate::Reg<pad_pa04_pad_ctl::PAD_PA04_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa04_pad_ctl;
#[doc = "PAD_PA05_FUNC_CTL register accessor: an alias for `Reg<PAD_PA05_FUNC_CTL_SPEC>`"]
pub type PAD_PA05_FUNC_CTL = crate::Reg<pad_pa05_func_ctl::PAD_PA05_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa05_func_ctl;
#[doc = "PAD_PA05_PAD_CTL register accessor: an alias for `Reg<PAD_PA05_PAD_CTL_SPEC>`"]
pub type PAD_PA05_PAD_CTL = crate::Reg<pad_pa05_pad_ctl::PAD_PA05_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa05_pad_ctl;
#[doc = "PAD_PA06_FUNC_CTL register accessor: an alias for `Reg<PAD_PA06_FUNC_CTL_SPEC>`"]
pub type PAD_PA06_FUNC_CTL = crate::Reg<pad_pa06_func_ctl::PAD_PA06_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa06_func_ctl;
#[doc = "PAD_PA06_PAD_CTL register accessor: an alias for `Reg<PAD_PA06_PAD_CTL_SPEC>`"]
pub type PAD_PA06_PAD_CTL = crate::Reg<pad_pa06_pad_ctl::PAD_PA06_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa06_pad_ctl;
#[doc = "PAD_PA07_FUNC_CTL register accessor: an alias for `Reg<PAD_PA07_FUNC_CTL_SPEC>`"]
pub type PAD_PA07_FUNC_CTL = crate::Reg<pad_pa07_func_ctl::PAD_PA07_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa07_func_ctl;
#[doc = "PAD_PA07_PAD_CTL register accessor: an alias for `Reg<PAD_PA07_PAD_CTL_SPEC>`"]
pub type PAD_PA07_PAD_CTL = crate::Reg<pad_pa07_pad_ctl::PAD_PA07_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa07_pad_ctl;
#[doc = "PAD_PA08_FUNC_CTL register accessor: an alias for `Reg<PAD_PA08_FUNC_CTL_SPEC>`"]
pub type PAD_PA08_FUNC_CTL = crate::Reg<pad_pa08_func_ctl::PAD_PA08_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa08_func_ctl;
#[doc = "PAD_PA08_PAD_CTL register accessor: an alias for `Reg<PAD_PA08_PAD_CTL_SPEC>`"]
pub type PAD_PA08_PAD_CTL = crate::Reg<pad_pa08_pad_ctl::PAD_PA08_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa08_pad_ctl;
#[doc = "PAD_PA09_FUNC_CTL register accessor: an alias for `Reg<PAD_PA09_FUNC_CTL_SPEC>`"]
pub type PAD_PA09_FUNC_CTL = crate::Reg<pad_pa09_func_ctl::PAD_PA09_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa09_func_ctl;
#[doc = "PAD_PA09_PAD_CTL register accessor: an alias for `Reg<PAD_PA09_PAD_CTL_SPEC>`"]
pub type PAD_PA09_PAD_CTL = crate::Reg<pad_pa09_pad_ctl::PAD_PA09_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa09_pad_ctl;
#[doc = "PAD_PA10_FUNC_CTL register accessor: an alias for `Reg<PAD_PA10_FUNC_CTL_SPEC>`"]
pub type PAD_PA10_FUNC_CTL = crate::Reg<pad_pa10_func_ctl::PAD_PA10_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa10_func_ctl;
#[doc = "PAD_PA10_PAD_CTL register accessor: an alias for `Reg<PAD_PA10_PAD_CTL_SPEC>`"]
pub type PAD_PA10_PAD_CTL = crate::Reg<pad_pa10_pad_ctl::PAD_PA10_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa10_pad_ctl;
#[doc = "PAD_PA11_FUNC_CTL register accessor: an alias for `Reg<PAD_PA11_FUNC_CTL_SPEC>`"]
pub type PAD_PA11_FUNC_CTL = crate::Reg<pad_pa11_func_ctl::PAD_PA11_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa11_func_ctl;
#[doc = "PAD_PA11_PAD_CTL register accessor: an alias for `Reg<PAD_PA11_PAD_CTL_SPEC>`"]
pub type PAD_PA11_PAD_CTL = crate::Reg<pad_pa11_pad_ctl::PAD_PA11_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa11_pad_ctl;
#[doc = "PAD_PA12_FUNC_CTL register accessor: an alias for `Reg<PAD_PA12_FUNC_CTL_SPEC>`"]
pub type PAD_PA12_FUNC_CTL = crate::Reg<pad_pa12_func_ctl::PAD_PA12_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa12_func_ctl;
#[doc = "PAD_PA12_PAD_CTL register accessor: an alias for `Reg<PAD_PA12_PAD_CTL_SPEC>`"]
pub type PAD_PA12_PAD_CTL = crate::Reg<pad_pa12_pad_ctl::PAD_PA12_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa12_pad_ctl;
#[doc = "PAD_PA13_FUNC_CTL register accessor: an alias for `Reg<PAD_PA13_FUNC_CTL_SPEC>`"]
pub type PAD_PA13_FUNC_CTL = crate::Reg<pad_pa13_func_ctl::PAD_PA13_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa13_func_ctl;
#[doc = "PAD_PA13_PAD_CTL register accessor: an alias for `Reg<PAD_PA13_PAD_CTL_SPEC>`"]
pub type PAD_PA13_PAD_CTL = crate::Reg<pad_pa13_pad_ctl::PAD_PA13_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa13_pad_ctl;
#[doc = "PAD_PA14_FUNC_CTL register accessor: an alias for `Reg<PAD_PA14_FUNC_CTL_SPEC>`"]
pub type PAD_PA14_FUNC_CTL = crate::Reg<pad_pa14_func_ctl::PAD_PA14_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa14_func_ctl;
#[doc = "PAD_PA14_PAD_CTL register accessor: an alias for `Reg<PAD_PA14_PAD_CTL_SPEC>`"]
pub type PAD_PA14_PAD_CTL = crate::Reg<pad_pa14_pad_ctl::PAD_PA14_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa14_pad_ctl;
#[doc = "PAD_PA15_FUNC_CTL register accessor: an alias for `Reg<PAD_PA15_FUNC_CTL_SPEC>`"]
pub type PAD_PA15_FUNC_CTL = crate::Reg<pad_pa15_func_ctl::PAD_PA15_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa15_func_ctl;
#[doc = "PAD_PA15_PAD_CTL register accessor: an alias for `Reg<PAD_PA15_PAD_CTL_SPEC>`"]
pub type PAD_PA15_PAD_CTL = crate::Reg<pad_pa15_pad_ctl::PAD_PA15_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa15_pad_ctl;
#[doc = "PAD_PA16_FUNC_CTL register accessor: an alias for `Reg<PAD_PA16_FUNC_CTL_SPEC>`"]
pub type PAD_PA16_FUNC_CTL = crate::Reg<pad_pa16_func_ctl::PAD_PA16_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa16_func_ctl;
#[doc = "PAD_PA16_PAD_CTL register accessor: an alias for `Reg<PAD_PA16_PAD_CTL_SPEC>`"]
pub type PAD_PA16_PAD_CTL = crate::Reg<pad_pa16_pad_ctl::PAD_PA16_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa16_pad_ctl;
#[doc = "PAD_PA17_FUNC_CTL register accessor: an alias for `Reg<PAD_PA17_FUNC_CTL_SPEC>`"]
pub type PAD_PA17_FUNC_CTL = crate::Reg<pad_pa17_func_ctl::PAD_PA17_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa17_func_ctl;
#[doc = "PAD_PA17_PAD_CTL register accessor: an alias for `Reg<PAD_PA17_PAD_CTL_SPEC>`"]
pub type PAD_PA17_PAD_CTL = crate::Reg<pad_pa17_pad_ctl::PAD_PA17_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa17_pad_ctl;
#[doc = "PAD_PA18_FUNC_CTL register accessor: an alias for `Reg<PAD_PA18_FUNC_CTL_SPEC>`"]
pub type PAD_PA18_FUNC_CTL = crate::Reg<pad_pa18_func_ctl::PAD_PA18_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa18_func_ctl;
#[doc = "PAD_PA18_PAD_CTL register accessor: an alias for `Reg<PAD_PA18_PAD_CTL_SPEC>`"]
pub type PAD_PA18_PAD_CTL = crate::Reg<pad_pa18_pad_ctl::PAD_PA18_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa18_pad_ctl;
#[doc = "PAD_PA19_FUNC_CTL register accessor: an alias for `Reg<PAD_PA19_FUNC_CTL_SPEC>`"]
pub type PAD_PA19_FUNC_CTL = crate::Reg<pad_pa19_func_ctl::PAD_PA19_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa19_func_ctl;
#[doc = "PAD_PA19_PAD_CTL register accessor: an alias for `Reg<PAD_PA19_PAD_CTL_SPEC>`"]
pub type PAD_PA19_PAD_CTL = crate::Reg<pad_pa19_pad_ctl::PAD_PA19_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa19_pad_ctl;
#[doc = "PAD_PA20_FUNC_CTL register accessor: an alias for `Reg<PAD_PA20_FUNC_CTL_SPEC>`"]
pub type PAD_PA20_FUNC_CTL = crate::Reg<pad_pa20_func_ctl::PAD_PA20_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa20_func_ctl;
#[doc = "PAD_PA20_PAD_CTL register accessor: an alias for `Reg<PAD_PA20_PAD_CTL_SPEC>`"]
pub type PAD_PA20_PAD_CTL = crate::Reg<pad_pa20_pad_ctl::PAD_PA20_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa20_pad_ctl;
#[doc = "PAD_PA21_FUNC_CTL register accessor: an alias for `Reg<PAD_PA21_FUNC_CTL_SPEC>`"]
pub type PAD_PA21_FUNC_CTL = crate::Reg<pad_pa21_func_ctl::PAD_PA21_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa21_func_ctl;
#[doc = "PAD_PA21_PAD_CTL register accessor: an alias for `Reg<PAD_PA21_PAD_CTL_SPEC>`"]
pub type PAD_PA21_PAD_CTL = crate::Reg<pad_pa21_pad_ctl::PAD_PA21_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa21_pad_ctl;
#[doc = "PAD_PA22_FUNC_CTL register accessor: an alias for `Reg<PAD_PA22_FUNC_CTL_SPEC>`"]
pub type PAD_PA22_FUNC_CTL = crate::Reg<pad_pa22_func_ctl::PAD_PA22_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa22_func_ctl;
#[doc = "PAD_PA22_PAD_CTL register accessor: an alias for `Reg<PAD_PA22_PAD_CTL_SPEC>`"]
pub type PAD_PA22_PAD_CTL = crate::Reg<pad_pa22_pad_ctl::PAD_PA22_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa22_pad_ctl;
#[doc = "PAD_PA23_FUNC_CTL register accessor: an alias for `Reg<PAD_PA23_FUNC_CTL_SPEC>`"]
pub type PAD_PA23_FUNC_CTL = crate::Reg<pad_pa23_func_ctl::PAD_PA23_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa23_func_ctl;
#[doc = "PAD_PA23_PAD_CTL register accessor: an alias for `Reg<PAD_PA23_PAD_CTL_SPEC>`"]
pub type PAD_PA23_PAD_CTL = crate::Reg<pad_pa23_pad_ctl::PAD_PA23_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa23_pad_ctl;
#[doc = "PAD_PA24_FUNC_CTL register accessor: an alias for `Reg<PAD_PA24_FUNC_CTL_SPEC>`"]
pub type PAD_PA24_FUNC_CTL = crate::Reg<pad_pa24_func_ctl::PAD_PA24_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa24_func_ctl;
#[doc = "PAD_PA24_PAD_CTL register accessor: an alias for `Reg<PAD_PA24_PAD_CTL_SPEC>`"]
pub type PAD_PA24_PAD_CTL = crate::Reg<pad_pa24_pad_ctl::PAD_PA24_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa24_pad_ctl;
#[doc = "PAD_PA25_FUNC_CTL register accessor: an alias for `Reg<PAD_PA25_FUNC_CTL_SPEC>`"]
pub type PAD_PA25_FUNC_CTL = crate::Reg<pad_pa25_func_ctl::PAD_PA25_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa25_func_ctl;
#[doc = "PAD_PA25_PAD_CTL register accessor: an alias for `Reg<PAD_PA25_PAD_CTL_SPEC>`"]
pub type PAD_PA25_PAD_CTL = crate::Reg<pad_pa25_pad_ctl::PAD_PA25_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa25_pad_ctl;
#[doc = "PAD_PA26_FUNC_CTL register accessor: an alias for `Reg<PAD_PA26_FUNC_CTL_SPEC>`"]
pub type PAD_PA26_FUNC_CTL = crate::Reg<pad_pa26_func_ctl::PAD_PA26_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa26_func_ctl;
#[doc = "PAD_PA26_PAD_CTL register accessor: an alias for `Reg<PAD_PA26_PAD_CTL_SPEC>`"]
pub type PAD_PA26_PAD_CTL = crate::Reg<pad_pa26_pad_ctl::PAD_PA26_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa26_pad_ctl;
#[doc = "PAD_PA27_FUNC_CTL register accessor: an alias for `Reg<PAD_PA27_FUNC_CTL_SPEC>`"]
pub type PAD_PA27_FUNC_CTL = crate::Reg<pad_pa27_func_ctl::PAD_PA27_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa27_func_ctl;
#[doc = "PAD_PA27_PAD_CTL register accessor: an alias for `Reg<PAD_PA27_PAD_CTL_SPEC>`"]
pub type PAD_PA27_PAD_CTL = crate::Reg<pad_pa27_pad_ctl::PAD_PA27_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa27_pad_ctl;
#[doc = "PAD_PA28_FUNC_CTL register accessor: an alias for `Reg<PAD_PA28_FUNC_CTL_SPEC>`"]
pub type PAD_PA28_FUNC_CTL = crate::Reg<pad_pa28_func_ctl::PAD_PA28_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa28_func_ctl;
#[doc = "PAD_PA28_PAD_CTL register accessor: an alias for `Reg<PAD_PA28_PAD_CTL_SPEC>`"]
pub type PAD_PA28_PAD_CTL = crate::Reg<pad_pa28_pad_ctl::PAD_PA28_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa28_pad_ctl;
#[doc = "PAD_PA29_FUNC_CTL register accessor: an alias for `Reg<PAD_PA29_FUNC_CTL_SPEC>`"]
pub type PAD_PA29_FUNC_CTL = crate::Reg<pad_pa29_func_ctl::PAD_PA29_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa29_func_ctl;
#[doc = "PAD_PA29_PAD_CTL register accessor: an alias for `Reg<PAD_PA29_PAD_CTL_SPEC>`"]
pub type PAD_PA29_PAD_CTL = crate::Reg<pad_pa29_pad_ctl::PAD_PA29_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa29_pad_ctl;
#[doc = "PAD_PA30_FUNC_CTL register accessor: an alias for `Reg<PAD_PA30_FUNC_CTL_SPEC>`"]
pub type PAD_PA30_FUNC_CTL = crate::Reg<pad_pa30_func_ctl::PAD_PA30_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa30_func_ctl;
#[doc = "PAD_PA30_PAD_CTL register accessor: an alias for `Reg<PAD_PA30_PAD_CTL_SPEC>`"]
pub type PAD_PA30_PAD_CTL = crate::Reg<pad_pa30_pad_ctl::PAD_PA30_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa30_pad_ctl;
#[doc = "PAD_PA31_FUNC_CTL register accessor: an alias for `Reg<PAD_PA31_FUNC_CTL_SPEC>`"]
pub type PAD_PA31_FUNC_CTL = crate::Reg<pad_pa31_func_ctl::PAD_PA31_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pa31_func_ctl;
#[doc = "PAD_PA31_PAD_CTL register accessor: an alias for `Reg<PAD_PA31_PAD_CTL_SPEC>`"]
pub type PAD_PA31_PAD_CTL = crate::Reg<pad_pa31_pad_ctl::PAD_PA31_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pa31_pad_ctl;
#[doc = "PAD_PB00_FUNC_CTL register accessor: an alias for `Reg<PAD_PB00_FUNC_CTL_SPEC>`"]
pub type PAD_PB00_FUNC_CTL = crate::Reg<pad_pb00_func_ctl::PAD_PB00_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb00_func_ctl;
#[doc = "PAD_PB00_PAD_CTL register accessor: an alias for `Reg<PAD_PB00_PAD_CTL_SPEC>`"]
pub type PAD_PB00_PAD_CTL = crate::Reg<pad_pb00_pad_ctl::PAD_PB00_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb00_pad_ctl;
#[doc = "PAD_PB01_FUNC_CTL register accessor: an alias for `Reg<PAD_PB01_FUNC_CTL_SPEC>`"]
pub type PAD_PB01_FUNC_CTL = crate::Reg<pad_pb01_func_ctl::PAD_PB01_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb01_func_ctl;
#[doc = "PAD_PB01_PAD_CTL register accessor: an alias for `Reg<PAD_PB01_PAD_CTL_SPEC>`"]
pub type PAD_PB01_PAD_CTL = crate::Reg<pad_pb01_pad_ctl::PAD_PB01_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb01_pad_ctl;
#[doc = "PAD_PB02_FUNC_CTL register accessor: an alias for `Reg<PAD_PB02_FUNC_CTL_SPEC>`"]
pub type PAD_PB02_FUNC_CTL = crate::Reg<pad_pb02_func_ctl::PAD_PB02_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb02_func_ctl;
#[doc = "PAD_PB02_PAD_CTL register accessor: an alias for `Reg<PAD_PB02_PAD_CTL_SPEC>`"]
pub type PAD_PB02_PAD_CTL = crate::Reg<pad_pb02_pad_ctl::PAD_PB02_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb02_pad_ctl;
#[doc = "PAD_PB03_FUNC_CTL register accessor: an alias for `Reg<PAD_PB03_FUNC_CTL_SPEC>`"]
pub type PAD_PB03_FUNC_CTL = crate::Reg<pad_pb03_func_ctl::PAD_PB03_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb03_func_ctl;
#[doc = "PAD_PB03_PAD_CTL register accessor: an alias for `Reg<PAD_PB03_PAD_CTL_SPEC>`"]
pub type PAD_PB03_PAD_CTL = crate::Reg<pad_pb03_pad_ctl::PAD_PB03_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb03_pad_ctl;
#[doc = "PAD_PB04_FUNC_CTL register accessor: an alias for `Reg<PAD_PB04_FUNC_CTL_SPEC>`"]
pub type PAD_PB04_FUNC_CTL = crate::Reg<pad_pb04_func_ctl::PAD_PB04_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb04_func_ctl;
#[doc = "PAD_PB04_PAD_CTL register accessor: an alias for `Reg<PAD_PB04_PAD_CTL_SPEC>`"]
pub type PAD_PB04_PAD_CTL = crate::Reg<pad_pb04_pad_ctl::PAD_PB04_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb04_pad_ctl;
#[doc = "PAD_PB05_FUNC_CTL register accessor: an alias for `Reg<PAD_PB05_FUNC_CTL_SPEC>`"]
pub type PAD_PB05_FUNC_CTL = crate::Reg<pad_pb05_func_ctl::PAD_PB05_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb05_func_ctl;
#[doc = "PAD_PB05_PAD_CTL register accessor: an alias for `Reg<PAD_PB05_PAD_CTL_SPEC>`"]
pub type PAD_PB05_PAD_CTL = crate::Reg<pad_pb05_pad_ctl::PAD_PB05_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb05_pad_ctl;
#[doc = "PAD_PB06_FUNC_CTL register accessor: an alias for `Reg<PAD_PB06_FUNC_CTL_SPEC>`"]
pub type PAD_PB06_FUNC_CTL = crate::Reg<pad_pb06_func_ctl::PAD_PB06_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb06_func_ctl;
#[doc = "PAD_PB06_PAD_CTL register accessor: an alias for `Reg<PAD_PB06_PAD_CTL_SPEC>`"]
pub type PAD_PB06_PAD_CTL = crate::Reg<pad_pb06_pad_ctl::PAD_PB06_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb06_pad_ctl;
#[doc = "PAD_PB07_FUNC_CTL register accessor: an alias for `Reg<PAD_PB07_FUNC_CTL_SPEC>`"]
pub type PAD_PB07_FUNC_CTL = crate::Reg<pad_pb07_func_ctl::PAD_PB07_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb07_func_ctl;
#[doc = "PAD_PB07_PAD_CTL register accessor: an alias for `Reg<PAD_PB07_PAD_CTL_SPEC>`"]
pub type PAD_PB07_PAD_CTL = crate::Reg<pad_pb07_pad_ctl::PAD_PB07_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb07_pad_ctl;
#[doc = "PAD_PB08_FUNC_CTL register accessor: an alias for `Reg<PAD_PB08_FUNC_CTL_SPEC>`"]
pub type PAD_PB08_FUNC_CTL = crate::Reg<pad_pb08_func_ctl::PAD_PB08_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb08_func_ctl;
#[doc = "PAD_PB08_PAD_CTL register accessor: an alias for `Reg<PAD_PB08_PAD_CTL_SPEC>`"]
pub type PAD_PB08_PAD_CTL = crate::Reg<pad_pb08_pad_ctl::PAD_PB08_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb08_pad_ctl;
#[doc = "PAD_PB09_FUNC_CTL register accessor: an alias for `Reg<PAD_PB09_FUNC_CTL_SPEC>`"]
pub type PAD_PB09_FUNC_CTL = crate::Reg<pad_pb09_func_ctl::PAD_PB09_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb09_func_ctl;
#[doc = "PAD_PB09_PAD_CTL register accessor: an alias for `Reg<PAD_PB09_PAD_CTL_SPEC>`"]
pub type PAD_PB09_PAD_CTL = crate::Reg<pad_pb09_pad_ctl::PAD_PB09_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb09_pad_ctl;
#[doc = "PAD_PB10_FUNC_CTL register accessor: an alias for `Reg<PAD_PB10_FUNC_CTL_SPEC>`"]
pub type PAD_PB10_FUNC_CTL = crate::Reg<pad_pb10_func_ctl::PAD_PB10_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb10_func_ctl;
#[doc = "PAD_PB10_PAD_CTL register accessor: an alias for `Reg<PAD_PB10_PAD_CTL_SPEC>`"]
pub type PAD_PB10_PAD_CTL = crate::Reg<pad_pb10_pad_ctl::PAD_PB10_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb10_pad_ctl;
#[doc = "PAD_PB11_FUNC_CTL register accessor: an alias for `Reg<PAD_PB11_FUNC_CTL_SPEC>`"]
pub type PAD_PB11_FUNC_CTL = crate::Reg<pad_pb11_func_ctl::PAD_PB11_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb11_func_ctl;
#[doc = "PAD_PB11_PAD_CTL register accessor: an alias for `Reg<PAD_PB11_PAD_CTL_SPEC>`"]
pub type PAD_PB11_PAD_CTL = crate::Reg<pad_pb11_pad_ctl::PAD_PB11_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb11_pad_ctl;
#[doc = "PAD_PB12_FUNC_CTL register accessor: an alias for `Reg<PAD_PB12_FUNC_CTL_SPEC>`"]
pub type PAD_PB12_FUNC_CTL = crate::Reg<pad_pb12_func_ctl::PAD_PB12_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb12_func_ctl;
#[doc = "PAD_PB12_PAD_CTL register accessor: an alias for `Reg<PAD_PB12_PAD_CTL_SPEC>`"]
pub type PAD_PB12_PAD_CTL = crate::Reg<pad_pb12_pad_ctl::PAD_PB12_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb12_pad_ctl;
#[doc = "PAD_PB13_FUNC_CTL register accessor: an alias for `Reg<PAD_PB13_FUNC_CTL_SPEC>`"]
pub type PAD_PB13_FUNC_CTL = crate::Reg<pad_pb13_func_ctl::PAD_PB13_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb13_func_ctl;
#[doc = "PAD_PB13_PAD_CTL register accessor: an alias for `Reg<PAD_PB13_PAD_CTL_SPEC>`"]
pub type PAD_PB13_PAD_CTL = crate::Reg<pad_pb13_pad_ctl::PAD_PB13_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb13_pad_ctl;
#[doc = "PAD_PB14_FUNC_CTL register accessor: an alias for `Reg<PAD_PB14_FUNC_CTL_SPEC>`"]
pub type PAD_PB14_FUNC_CTL = crate::Reg<pad_pb14_func_ctl::PAD_PB14_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb14_func_ctl;
#[doc = "PAD_PB14_PAD_CTL register accessor: an alias for `Reg<PAD_PB14_PAD_CTL_SPEC>`"]
pub type PAD_PB14_PAD_CTL = crate::Reg<pad_pb14_pad_ctl::PAD_PB14_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb14_pad_ctl;
#[doc = "PAD_PB15_FUNC_CTL register accessor: an alias for `Reg<PAD_PB15_FUNC_CTL_SPEC>`"]
pub type PAD_PB15_FUNC_CTL = crate::Reg<pad_pb15_func_ctl::PAD_PB15_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb15_func_ctl;
#[doc = "PAD_PB15_PAD_CTL register accessor: an alias for `Reg<PAD_PB15_PAD_CTL_SPEC>`"]
pub type PAD_PB15_PAD_CTL = crate::Reg<pad_pb15_pad_ctl::PAD_PB15_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb15_pad_ctl;
#[doc = "PAD_PB16_FUNC_CTL register accessor: an alias for `Reg<PAD_PB16_FUNC_CTL_SPEC>`"]
pub type PAD_PB16_FUNC_CTL = crate::Reg<pad_pb16_func_ctl::PAD_PB16_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb16_func_ctl;
#[doc = "PAD_PB16_PAD_CTL register accessor: an alias for `Reg<PAD_PB16_PAD_CTL_SPEC>`"]
pub type PAD_PB16_PAD_CTL = crate::Reg<pad_pb16_pad_ctl::PAD_PB16_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb16_pad_ctl;
#[doc = "PAD_PB17_FUNC_CTL register accessor: an alias for `Reg<PAD_PB17_FUNC_CTL_SPEC>`"]
pub type PAD_PB17_FUNC_CTL = crate::Reg<pad_pb17_func_ctl::PAD_PB17_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb17_func_ctl;
#[doc = "PAD_PB17_PAD_CTL register accessor: an alias for `Reg<PAD_PB17_PAD_CTL_SPEC>`"]
pub type PAD_PB17_PAD_CTL = crate::Reg<pad_pb17_pad_ctl::PAD_PB17_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb17_pad_ctl;
#[doc = "PAD_PB18_FUNC_CTL register accessor: an alias for `Reg<PAD_PB18_FUNC_CTL_SPEC>`"]
pub type PAD_PB18_FUNC_CTL = crate::Reg<pad_pb18_func_ctl::PAD_PB18_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb18_func_ctl;
#[doc = "PAD_PB18_PAD_CTL register accessor: an alias for `Reg<PAD_PB18_PAD_CTL_SPEC>`"]
pub type PAD_PB18_PAD_CTL = crate::Reg<pad_pb18_pad_ctl::PAD_PB18_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb18_pad_ctl;
#[doc = "PAD_PB19_FUNC_CTL register accessor: an alias for `Reg<PAD_PB19_FUNC_CTL_SPEC>`"]
pub type PAD_PB19_FUNC_CTL = crate::Reg<pad_pb19_func_ctl::PAD_PB19_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb19_func_ctl;
#[doc = "PAD_PB19_PAD_CTL register accessor: an alias for `Reg<PAD_PB19_PAD_CTL_SPEC>`"]
pub type PAD_PB19_PAD_CTL = crate::Reg<pad_pb19_pad_ctl::PAD_PB19_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb19_pad_ctl;
#[doc = "PAD_PB20_FUNC_CTL register accessor: an alias for `Reg<PAD_PB20_FUNC_CTL_SPEC>`"]
pub type PAD_PB20_FUNC_CTL = crate::Reg<pad_pb20_func_ctl::PAD_PB20_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb20_func_ctl;
#[doc = "PAD_PB20_PAD_CTL register accessor: an alias for `Reg<PAD_PB20_PAD_CTL_SPEC>`"]
pub type PAD_PB20_PAD_CTL = crate::Reg<pad_pb20_pad_ctl::PAD_PB20_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb20_pad_ctl;
#[doc = "PAD_PB21_FUNC_CTL register accessor: an alias for `Reg<PAD_PB21_FUNC_CTL_SPEC>`"]
pub type PAD_PB21_FUNC_CTL = crate::Reg<pad_pb21_func_ctl::PAD_PB21_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb21_func_ctl;
#[doc = "PAD_PB21_PAD_CTL register accessor: an alias for `Reg<PAD_PB21_PAD_CTL_SPEC>`"]
pub type PAD_PB21_PAD_CTL = crate::Reg<pad_pb21_pad_ctl::PAD_PB21_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb21_pad_ctl;
#[doc = "PAD_PB22_FUNC_CTL register accessor: an alias for `Reg<PAD_PB22_FUNC_CTL_SPEC>`"]
pub type PAD_PB22_FUNC_CTL = crate::Reg<pad_pb22_func_ctl::PAD_PB22_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb22_func_ctl;
#[doc = "PAD_PB22_PAD_CTL register accessor: an alias for `Reg<PAD_PB22_PAD_CTL_SPEC>`"]
pub type PAD_PB22_PAD_CTL = crate::Reg<pad_pb22_pad_ctl::PAD_PB22_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb22_pad_ctl;
#[doc = "PAD_PB23_FUNC_CTL register accessor: an alias for `Reg<PAD_PB23_FUNC_CTL_SPEC>`"]
pub type PAD_PB23_FUNC_CTL = crate::Reg<pad_pb23_func_ctl::PAD_PB23_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb23_func_ctl;
#[doc = "PAD_PB23_PAD_CTL register accessor: an alias for `Reg<PAD_PB23_PAD_CTL_SPEC>`"]
pub type PAD_PB23_PAD_CTL = crate::Reg<pad_pb23_pad_ctl::PAD_PB23_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb23_pad_ctl;
#[doc = "PAD_PB24_FUNC_CTL register accessor: an alias for `Reg<PAD_PB24_FUNC_CTL_SPEC>`"]
pub type PAD_PB24_FUNC_CTL = crate::Reg<pad_pb24_func_ctl::PAD_PB24_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb24_func_ctl;
#[doc = "PAD_PB24_PAD_CTL register accessor: an alias for `Reg<PAD_PB24_PAD_CTL_SPEC>`"]
pub type PAD_PB24_PAD_CTL = crate::Reg<pad_pb24_pad_ctl::PAD_PB24_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb24_pad_ctl;
#[doc = "PAD_PB25_FUNC_CTL register accessor: an alias for `Reg<PAD_PB25_FUNC_CTL_SPEC>`"]
pub type PAD_PB25_FUNC_CTL = crate::Reg<pad_pb25_func_ctl::PAD_PB25_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb25_func_ctl;
#[doc = "PAD_PB25_PAD_CTL register accessor: an alias for `Reg<PAD_PB25_PAD_CTL_SPEC>`"]
pub type PAD_PB25_PAD_CTL = crate::Reg<pad_pb25_pad_ctl::PAD_PB25_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb25_pad_ctl;
#[doc = "PAD_PB26_FUNC_CTL register accessor: an alias for `Reg<PAD_PB26_FUNC_CTL_SPEC>`"]
pub type PAD_PB26_FUNC_CTL = crate::Reg<pad_pb26_func_ctl::PAD_PB26_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb26_func_ctl;
#[doc = "PAD_PB26_PAD_CTL register accessor: an alias for `Reg<PAD_PB26_PAD_CTL_SPEC>`"]
pub type PAD_PB26_PAD_CTL = crate::Reg<pad_pb26_pad_ctl::PAD_PB26_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb26_pad_ctl;
#[doc = "PAD_PB27_FUNC_CTL register accessor: an alias for `Reg<PAD_PB27_FUNC_CTL_SPEC>`"]
pub type PAD_PB27_FUNC_CTL = crate::Reg<pad_pb27_func_ctl::PAD_PB27_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb27_func_ctl;
#[doc = "PAD_PB27_PAD_CTL register accessor: an alias for `Reg<PAD_PB27_PAD_CTL_SPEC>`"]
pub type PAD_PB27_PAD_CTL = crate::Reg<pad_pb27_pad_ctl::PAD_PB27_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb27_pad_ctl;
#[doc = "PAD_PB28_FUNC_CTL register accessor: an alias for `Reg<PAD_PB28_FUNC_CTL_SPEC>`"]
pub type PAD_PB28_FUNC_CTL = crate::Reg<pad_pb28_func_ctl::PAD_PB28_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb28_func_ctl;
#[doc = "PAD_PB28_PAD_CTL register accessor: an alias for `Reg<PAD_PB28_PAD_CTL_SPEC>`"]
pub type PAD_PB28_PAD_CTL = crate::Reg<pad_pb28_pad_ctl::PAD_PB28_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb28_pad_ctl;
#[doc = "PAD_PB29_FUNC_CTL register accessor: an alias for `Reg<PAD_PB29_FUNC_CTL_SPEC>`"]
pub type PAD_PB29_FUNC_CTL = crate::Reg<pad_pb29_func_ctl::PAD_PB29_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb29_func_ctl;
#[doc = "PAD_PB29_PAD_CTL register accessor: an alias for `Reg<PAD_PB29_PAD_CTL_SPEC>`"]
pub type PAD_PB29_PAD_CTL = crate::Reg<pad_pb29_pad_ctl::PAD_PB29_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb29_pad_ctl;
#[doc = "PAD_PB30_FUNC_CTL register accessor: an alias for `Reg<PAD_PB30_FUNC_CTL_SPEC>`"]
pub type PAD_PB30_FUNC_CTL = crate::Reg<pad_pb30_func_ctl::PAD_PB30_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb30_func_ctl;
#[doc = "PAD_PB30_PAD_CTL register accessor: an alias for `Reg<PAD_PB30_PAD_CTL_SPEC>`"]
pub type PAD_PB30_PAD_CTL = crate::Reg<pad_pb30_pad_ctl::PAD_PB30_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb30_pad_ctl;
#[doc = "PAD_PB31_FUNC_CTL register accessor: an alias for `Reg<PAD_PB31_FUNC_CTL_SPEC>`"]
pub type PAD_PB31_FUNC_CTL = crate::Reg<pad_pb31_func_ctl::PAD_PB31_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pb31_func_ctl;
#[doc = "PAD_PB31_PAD_CTL register accessor: an alias for `Reg<PAD_PB31_PAD_CTL_SPEC>`"]
pub type PAD_PB31_PAD_CTL = crate::Reg<pad_pb31_pad_ctl::PAD_PB31_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pb31_pad_ctl;
#[doc = "PAD_PC00_FUNC_CTL register accessor: an alias for `Reg<PAD_PC00_FUNC_CTL_SPEC>`"]
pub type PAD_PC00_FUNC_CTL = crate::Reg<pad_pc00_func_ctl::PAD_PC00_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc00_func_ctl;
#[doc = "PAD_PC00_PAD_CTL register accessor: an alias for `Reg<PAD_PC00_PAD_CTL_SPEC>`"]
pub type PAD_PC00_PAD_CTL = crate::Reg<pad_pc00_pad_ctl::PAD_PC00_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc00_pad_ctl;
#[doc = "PAD_PC01_FUNC_CTL register accessor: an alias for `Reg<PAD_PC01_FUNC_CTL_SPEC>`"]
pub type PAD_PC01_FUNC_CTL = crate::Reg<pad_pc01_func_ctl::PAD_PC01_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc01_func_ctl;
#[doc = "PAD_PC01_PAD_CTL register accessor: an alias for `Reg<PAD_PC01_PAD_CTL_SPEC>`"]
pub type PAD_PC01_PAD_CTL = crate::Reg<pad_pc01_pad_ctl::PAD_PC01_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc01_pad_ctl;
#[doc = "PAD_PC02_FUNC_CTL register accessor: an alias for `Reg<PAD_PC02_FUNC_CTL_SPEC>`"]
pub type PAD_PC02_FUNC_CTL = crate::Reg<pad_pc02_func_ctl::PAD_PC02_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc02_func_ctl;
#[doc = "PAD_PC02_PAD_CTL register accessor: an alias for `Reg<PAD_PC02_PAD_CTL_SPEC>`"]
pub type PAD_PC02_PAD_CTL = crate::Reg<pad_pc02_pad_ctl::PAD_PC02_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc02_pad_ctl;
#[doc = "PAD_PC03_FUNC_CTL register accessor: an alias for `Reg<PAD_PC03_FUNC_CTL_SPEC>`"]
pub type PAD_PC03_FUNC_CTL = crate::Reg<pad_pc03_func_ctl::PAD_PC03_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc03_func_ctl;
#[doc = "PAD_PC03_PAD_CTL register accessor: an alias for `Reg<PAD_PC03_PAD_CTL_SPEC>`"]
pub type PAD_PC03_PAD_CTL = crate::Reg<pad_pc03_pad_ctl::PAD_PC03_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc03_pad_ctl;
#[doc = "PAD_PC04_FUNC_CTL register accessor: an alias for `Reg<PAD_PC04_FUNC_CTL_SPEC>`"]
pub type PAD_PC04_FUNC_CTL = crate::Reg<pad_pc04_func_ctl::PAD_PC04_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc04_func_ctl;
#[doc = "PAD_PC04_PAD_CTL register accessor: an alias for `Reg<PAD_PC04_PAD_CTL_SPEC>`"]
pub type PAD_PC04_PAD_CTL = crate::Reg<pad_pc04_pad_ctl::PAD_PC04_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc04_pad_ctl;
#[doc = "PAD_PC05_FUNC_CTL register accessor: an alias for `Reg<PAD_PC05_FUNC_CTL_SPEC>`"]
pub type PAD_PC05_FUNC_CTL = crate::Reg<pad_pc05_func_ctl::PAD_PC05_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc05_func_ctl;
#[doc = "PAD_PC05_PAD_CTL register accessor: an alias for `Reg<PAD_PC05_PAD_CTL_SPEC>`"]
pub type PAD_PC05_PAD_CTL = crate::Reg<pad_pc05_pad_ctl::PAD_PC05_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc05_pad_ctl;
#[doc = "PAD_PC06_FUNC_CTL register accessor: an alias for `Reg<PAD_PC06_FUNC_CTL_SPEC>`"]
pub type PAD_PC06_FUNC_CTL = crate::Reg<pad_pc06_func_ctl::PAD_PC06_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc06_func_ctl;
#[doc = "PAD_PC06_PAD_CTL register accessor: an alias for `Reg<PAD_PC06_PAD_CTL_SPEC>`"]
pub type PAD_PC06_PAD_CTL = crate::Reg<pad_pc06_pad_ctl::PAD_PC06_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc06_pad_ctl;
#[doc = "PAD_PC07_FUNC_CTL register accessor: an alias for `Reg<PAD_PC07_FUNC_CTL_SPEC>`"]
pub type PAD_PC07_FUNC_CTL = crate::Reg<pad_pc07_func_ctl::PAD_PC07_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc07_func_ctl;
#[doc = "PAD_PC07_PAD_CTL register accessor: an alias for `Reg<PAD_PC07_PAD_CTL_SPEC>`"]
pub type PAD_PC07_PAD_CTL = crate::Reg<pad_pc07_pad_ctl::PAD_PC07_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc07_pad_ctl;
#[doc = "PAD_PC08_FUNC_CTL register accessor: an alias for `Reg<PAD_PC08_FUNC_CTL_SPEC>`"]
pub type PAD_PC08_FUNC_CTL = crate::Reg<pad_pc08_func_ctl::PAD_PC08_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc08_func_ctl;
#[doc = "PAD_PC08_PAD_CTL register accessor: an alias for `Reg<PAD_PC08_PAD_CTL_SPEC>`"]
pub type PAD_PC08_PAD_CTL = crate::Reg<pad_pc08_pad_ctl::PAD_PC08_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc08_pad_ctl;
#[doc = "PAD_PC09_FUNC_CTL register accessor: an alias for `Reg<PAD_PC09_FUNC_CTL_SPEC>`"]
pub type PAD_PC09_FUNC_CTL = crate::Reg<pad_pc09_func_ctl::PAD_PC09_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc09_func_ctl;
#[doc = "PAD_PC09_PAD_CTL register accessor: an alias for `Reg<PAD_PC09_PAD_CTL_SPEC>`"]
pub type PAD_PC09_PAD_CTL = crate::Reg<pad_pc09_pad_ctl::PAD_PC09_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc09_pad_ctl;
#[doc = "PAD_PC10_FUNC_CTL register accessor: an alias for `Reg<PAD_PC10_FUNC_CTL_SPEC>`"]
pub type PAD_PC10_FUNC_CTL = crate::Reg<pad_pc10_func_ctl::PAD_PC10_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc10_func_ctl;
#[doc = "PAD_PC10_PAD_CTL register accessor: an alias for `Reg<PAD_PC10_PAD_CTL_SPEC>`"]
pub type PAD_PC10_PAD_CTL = crate::Reg<pad_pc10_pad_ctl::PAD_PC10_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc10_pad_ctl;
#[doc = "PAD_PC11_FUNC_CTL register accessor: an alias for `Reg<PAD_PC11_FUNC_CTL_SPEC>`"]
pub type PAD_PC11_FUNC_CTL = crate::Reg<pad_pc11_func_ctl::PAD_PC11_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc11_func_ctl;
#[doc = "PAD_PC11_PAD_CTL register accessor: an alias for `Reg<PAD_PC11_PAD_CTL_SPEC>`"]
pub type PAD_PC11_PAD_CTL = crate::Reg<pad_pc11_pad_ctl::PAD_PC11_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc11_pad_ctl;
#[doc = "PAD_PC12_FUNC_CTL register accessor: an alias for `Reg<PAD_PC12_FUNC_CTL_SPEC>`"]
pub type PAD_PC12_FUNC_CTL = crate::Reg<pad_pc12_func_ctl::PAD_PC12_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc12_func_ctl;
#[doc = "PAD_PC12_PAD_CTL register accessor: an alias for `Reg<PAD_PC12_PAD_CTL_SPEC>`"]
pub type PAD_PC12_PAD_CTL = crate::Reg<pad_pc12_pad_ctl::PAD_PC12_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc12_pad_ctl;
#[doc = "PAD_PC13_FUNC_CTL register accessor: an alias for `Reg<PAD_PC13_FUNC_CTL_SPEC>`"]
pub type PAD_PC13_FUNC_CTL = crate::Reg<pad_pc13_func_ctl::PAD_PC13_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc13_func_ctl;
#[doc = "PAD_PC13_PAD_CTL register accessor: an alias for `Reg<PAD_PC13_PAD_CTL_SPEC>`"]
pub type PAD_PC13_PAD_CTL = crate::Reg<pad_pc13_pad_ctl::PAD_PC13_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc13_pad_ctl;
#[doc = "PAD_PC14_FUNC_CTL register accessor: an alias for `Reg<PAD_PC14_FUNC_CTL_SPEC>`"]
pub type PAD_PC14_FUNC_CTL = crate::Reg<pad_pc14_func_ctl::PAD_PC14_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc14_func_ctl;
#[doc = "PAD_PC14_PAD_CTL register accessor: an alias for `Reg<PAD_PC14_PAD_CTL_SPEC>`"]
pub type PAD_PC14_PAD_CTL = crate::Reg<pad_pc14_pad_ctl::PAD_PC14_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc14_pad_ctl;
#[doc = "PAD_PC15_FUNC_CTL register accessor: an alias for `Reg<PAD_PC15_FUNC_CTL_SPEC>`"]
pub type PAD_PC15_FUNC_CTL = crate::Reg<pad_pc15_func_ctl::PAD_PC15_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc15_func_ctl;
#[doc = "PAD_PC15_PAD_CTL register accessor: an alias for `Reg<PAD_PC15_PAD_CTL_SPEC>`"]
pub type PAD_PC15_PAD_CTL = crate::Reg<pad_pc15_pad_ctl::PAD_PC15_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc15_pad_ctl;
#[doc = "PAD_PC16_FUNC_CTL register accessor: an alias for `Reg<PAD_PC16_FUNC_CTL_SPEC>`"]
pub type PAD_PC16_FUNC_CTL = crate::Reg<pad_pc16_func_ctl::PAD_PC16_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc16_func_ctl;
#[doc = "PAD_PC16_PAD_CTL register accessor: an alias for `Reg<PAD_PC16_PAD_CTL_SPEC>`"]
pub type PAD_PC16_PAD_CTL = crate::Reg<pad_pc16_pad_ctl::PAD_PC16_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc16_pad_ctl;
#[doc = "PAD_PC17_FUNC_CTL register accessor: an alias for `Reg<PAD_PC17_FUNC_CTL_SPEC>`"]
pub type PAD_PC17_FUNC_CTL = crate::Reg<pad_pc17_func_ctl::PAD_PC17_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc17_func_ctl;
#[doc = "PAD_PC17_PAD_CTL register accessor: an alias for `Reg<PAD_PC17_PAD_CTL_SPEC>`"]
pub type PAD_PC17_PAD_CTL = crate::Reg<pad_pc17_pad_ctl::PAD_PC17_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc17_pad_ctl;
#[doc = "PAD_PC18_FUNC_CTL register accessor: an alias for `Reg<PAD_PC18_FUNC_CTL_SPEC>`"]
pub type PAD_PC18_FUNC_CTL = crate::Reg<pad_pc18_func_ctl::PAD_PC18_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc18_func_ctl;
#[doc = "PAD_PC18_PAD_CTL register accessor: an alias for `Reg<PAD_PC18_PAD_CTL_SPEC>`"]
pub type PAD_PC18_PAD_CTL = crate::Reg<pad_pc18_pad_ctl::PAD_PC18_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc18_pad_ctl;
#[doc = "PAD_PC19_FUNC_CTL register accessor: an alias for `Reg<PAD_PC19_FUNC_CTL_SPEC>`"]
pub type PAD_PC19_FUNC_CTL = crate::Reg<pad_pc19_func_ctl::PAD_PC19_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc19_func_ctl;
#[doc = "PAD_PC19_PAD_CTL register accessor: an alias for `Reg<PAD_PC19_PAD_CTL_SPEC>`"]
pub type PAD_PC19_PAD_CTL = crate::Reg<pad_pc19_pad_ctl::PAD_PC19_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc19_pad_ctl;
#[doc = "PAD_PC20_FUNC_CTL register accessor: an alias for `Reg<PAD_PC20_FUNC_CTL_SPEC>`"]
pub type PAD_PC20_FUNC_CTL = crate::Reg<pad_pc20_func_ctl::PAD_PC20_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc20_func_ctl;
#[doc = "PAD_PC20_PAD_CTL register accessor: an alias for `Reg<PAD_PC20_PAD_CTL_SPEC>`"]
pub type PAD_PC20_PAD_CTL = crate::Reg<pad_pc20_pad_ctl::PAD_PC20_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc20_pad_ctl;
#[doc = "PAD_PC21_FUNC_CTL register accessor: an alias for `Reg<PAD_PC21_FUNC_CTL_SPEC>`"]
pub type PAD_PC21_FUNC_CTL = crate::Reg<pad_pc21_func_ctl::PAD_PC21_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc21_func_ctl;
#[doc = "PAD_PC21_PAD_CTL register accessor: an alias for `Reg<PAD_PC21_PAD_CTL_SPEC>`"]
pub type PAD_PC21_PAD_CTL = crate::Reg<pad_pc21_pad_ctl::PAD_PC21_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc21_pad_ctl;
#[doc = "PAD_PC22_FUNC_CTL register accessor: an alias for `Reg<PAD_PC22_FUNC_CTL_SPEC>`"]
pub type PAD_PC22_FUNC_CTL = crate::Reg<pad_pc22_func_ctl::PAD_PC22_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc22_func_ctl;
#[doc = "PAD_PC22_PAD_CTL register accessor: an alias for `Reg<PAD_PC22_PAD_CTL_SPEC>`"]
pub type PAD_PC22_PAD_CTL = crate::Reg<pad_pc22_pad_ctl::PAD_PC22_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc22_pad_ctl;
#[doc = "PAD_PC23_FUNC_CTL register accessor: an alias for `Reg<PAD_PC23_FUNC_CTL_SPEC>`"]
pub type PAD_PC23_FUNC_CTL = crate::Reg<pad_pc23_func_ctl::PAD_PC23_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc23_func_ctl;
#[doc = "PAD_PC23_PAD_CTL register accessor: an alias for `Reg<PAD_PC23_PAD_CTL_SPEC>`"]
pub type PAD_PC23_PAD_CTL = crate::Reg<pad_pc23_pad_ctl::PAD_PC23_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc23_pad_ctl;
#[doc = "PAD_PC24_FUNC_CTL register accessor: an alias for `Reg<PAD_PC24_FUNC_CTL_SPEC>`"]
pub type PAD_PC24_FUNC_CTL = crate::Reg<pad_pc24_func_ctl::PAD_PC24_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc24_func_ctl;
#[doc = "PAD_PC24_PAD_CTL register accessor: an alias for `Reg<PAD_PC24_PAD_CTL_SPEC>`"]
pub type PAD_PC24_PAD_CTL = crate::Reg<pad_pc24_pad_ctl::PAD_PC24_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc24_pad_ctl;
#[doc = "PAD_PC25_FUNC_CTL register accessor: an alias for `Reg<PAD_PC25_FUNC_CTL_SPEC>`"]
pub type PAD_PC25_FUNC_CTL = crate::Reg<pad_pc25_func_ctl::PAD_PC25_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc25_func_ctl;
#[doc = "PAD_PC25_PAD_CTL register accessor: an alias for `Reg<PAD_PC25_PAD_CTL_SPEC>`"]
pub type PAD_PC25_PAD_CTL = crate::Reg<pad_pc25_pad_ctl::PAD_PC25_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc25_pad_ctl;
#[doc = "PAD_PC26_FUNC_CTL register accessor: an alias for `Reg<PAD_PC26_FUNC_CTL_SPEC>`"]
pub type PAD_PC26_FUNC_CTL = crate::Reg<pad_pc26_func_ctl::PAD_PC26_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc26_func_ctl;
#[doc = "PAD_PC26_PAD_CTL register accessor: an alias for `Reg<PAD_PC26_PAD_CTL_SPEC>`"]
pub type PAD_PC26_PAD_CTL = crate::Reg<pad_pc26_pad_ctl::PAD_PC26_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc26_pad_ctl;
#[doc = "PAD_PC27_FUNC_CTL register accessor: an alias for `Reg<PAD_PC27_FUNC_CTL_SPEC>`"]
pub type PAD_PC27_FUNC_CTL = crate::Reg<pad_pc27_func_ctl::PAD_PC27_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc27_func_ctl;
#[doc = "PAD_PC27_PAD_CTL register accessor: an alias for `Reg<PAD_PC27_PAD_CTL_SPEC>`"]
pub type PAD_PC27_PAD_CTL = crate::Reg<pad_pc27_pad_ctl::PAD_PC27_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc27_pad_ctl;
#[doc = "PAD_PC28_FUNC_CTL register accessor: an alias for `Reg<PAD_PC28_FUNC_CTL_SPEC>`"]
pub type PAD_PC28_FUNC_CTL = crate::Reg<pad_pc28_func_ctl::PAD_PC28_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc28_func_ctl;
#[doc = "PAD_PC28_PAD_CTL register accessor: an alias for `Reg<PAD_PC28_PAD_CTL_SPEC>`"]
pub type PAD_PC28_PAD_CTL = crate::Reg<pad_pc28_pad_ctl::PAD_PC28_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc28_pad_ctl;
#[doc = "PAD_PC29_FUNC_CTL register accessor: an alias for `Reg<PAD_PC29_FUNC_CTL_SPEC>`"]
pub type PAD_PC29_FUNC_CTL = crate::Reg<pad_pc29_func_ctl::PAD_PC29_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc29_func_ctl;
#[doc = "PAD_PC29_PAD_CTL register accessor: an alias for `Reg<PAD_PC29_PAD_CTL_SPEC>`"]
pub type PAD_PC29_PAD_CTL = crate::Reg<pad_pc29_pad_ctl::PAD_PC29_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc29_pad_ctl;
#[doc = "PAD_PC30_FUNC_CTL register accessor: an alias for `Reg<PAD_PC30_FUNC_CTL_SPEC>`"]
pub type PAD_PC30_FUNC_CTL = crate::Reg<pad_pc30_func_ctl::PAD_PC30_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc30_func_ctl;
#[doc = "PAD_PC30_PAD_CTL register accessor: an alias for `Reg<PAD_PC30_PAD_CTL_SPEC>`"]
pub type PAD_PC30_PAD_CTL = crate::Reg<pad_pc30_pad_ctl::PAD_PC30_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc30_pad_ctl;
#[doc = "PAD_PC31_FUNC_CTL register accessor: an alias for `Reg<PAD_PC31_FUNC_CTL_SPEC>`"]
pub type PAD_PC31_FUNC_CTL = crate::Reg<pad_pc31_func_ctl::PAD_PC31_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pc31_func_ctl;
#[doc = "PAD_PC31_PAD_CTL register accessor: an alias for `Reg<PAD_PC31_PAD_CTL_SPEC>`"]
pub type PAD_PC31_PAD_CTL = crate::Reg<pad_pc31_pad_ctl::PAD_PC31_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pc31_pad_ctl;
#[doc = "PAD_PD00_FUNC_CTL register accessor: an alias for `Reg<PAD_PD00_FUNC_CTL_SPEC>`"]
pub type PAD_PD00_FUNC_CTL = crate::Reg<pad_pd00_func_ctl::PAD_PD00_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd00_func_ctl;
#[doc = "PAD_PD00_PAD_CTL register accessor: an alias for `Reg<PAD_PD00_PAD_CTL_SPEC>`"]
pub type PAD_PD00_PAD_CTL = crate::Reg<pad_pd00_pad_ctl::PAD_PD00_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd00_pad_ctl;
#[doc = "PAD_PD01_FUNC_CTL register accessor: an alias for `Reg<PAD_PD01_FUNC_CTL_SPEC>`"]
pub type PAD_PD01_FUNC_CTL = crate::Reg<pad_pd01_func_ctl::PAD_PD01_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd01_func_ctl;
#[doc = "PAD_PD01_PAD_CTL register accessor: an alias for `Reg<PAD_PD01_PAD_CTL_SPEC>`"]
pub type PAD_PD01_PAD_CTL = crate::Reg<pad_pd01_pad_ctl::PAD_PD01_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd01_pad_ctl;
#[doc = "PAD_PD02_FUNC_CTL register accessor: an alias for `Reg<PAD_PD02_FUNC_CTL_SPEC>`"]
pub type PAD_PD02_FUNC_CTL = crate::Reg<pad_pd02_func_ctl::PAD_PD02_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd02_func_ctl;
#[doc = "PAD_PD02_PAD_CTL register accessor: an alias for `Reg<PAD_PD02_PAD_CTL_SPEC>`"]
pub type PAD_PD02_PAD_CTL = crate::Reg<pad_pd02_pad_ctl::PAD_PD02_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd02_pad_ctl;
#[doc = "PAD_PD03_FUNC_CTL register accessor: an alias for `Reg<PAD_PD03_FUNC_CTL_SPEC>`"]
pub type PAD_PD03_FUNC_CTL = crate::Reg<pad_pd03_func_ctl::PAD_PD03_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd03_func_ctl;
#[doc = "PAD_PD03_PAD_CTL register accessor: an alias for `Reg<PAD_PD03_PAD_CTL_SPEC>`"]
pub type PAD_PD03_PAD_CTL = crate::Reg<pad_pd03_pad_ctl::PAD_PD03_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd03_pad_ctl;
#[doc = "PAD_PD04_FUNC_CTL register accessor: an alias for `Reg<PAD_PD04_FUNC_CTL_SPEC>`"]
pub type PAD_PD04_FUNC_CTL = crate::Reg<pad_pd04_func_ctl::PAD_PD04_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd04_func_ctl;
#[doc = "PAD_PD04_PAD_CTL register accessor: an alias for `Reg<PAD_PD04_PAD_CTL_SPEC>`"]
pub type PAD_PD04_PAD_CTL = crate::Reg<pad_pd04_pad_ctl::PAD_PD04_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd04_pad_ctl;
#[doc = "PAD_PD05_FUNC_CTL register accessor: an alias for `Reg<PAD_PD05_FUNC_CTL_SPEC>`"]
pub type PAD_PD05_FUNC_CTL = crate::Reg<pad_pd05_func_ctl::PAD_PD05_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd05_func_ctl;
#[doc = "PAD_PD05_PAD_CTL register accessor: an alias for `Reg<PAD_PD05_PAD_CTL_SPEC>`"]
pub type PAD_PD05_PAD_CTL = crate::Reg<pad_pd05_pad_ctl::PAD_PD05_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd05_pad_ctl;
#[doc = "PAD_PD06_FUNC_CTL register accessor: an alias for `Reg<PAD_PD06_FUNC_CTL_SPEC>`"]
pub type PAD_PD06_FUNC_CTL = crate::Reg<pad_pd06_func_ctl::PAD_PD06_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd06_func_ctl;
#[doc = "PAD_PD06_PAD_CTL register accessor: an alias for `Reg<PAD_PD06_PAD_CTL_SPEC>`"]
pub type PAD_PD06_PAD_CTL = crate::Reg<pad_pd06_pad_ctl::PAD_PD06_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd06_pad_ctl;
#[doc = "PAD_PD07_FUNC_CTL register accessor: an alias for `Reg<PAD_PD07_FUNC_CTL_SPEC>`"]
pub type PAD_PD07_FUNC_CTL = crate::Reg<pad_pd07_func_ctl::PAD_PD07_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd07_func_ctl;
#[doc = "PAD_PD07_PAD_CTL register accessor: an alias for `Reg<PAD_PD07_PAD_CTL_SPEC>`"]
pub type PAD_PD07_PAD_CTL = crate::Reg<pad_pd07_pad_ctl::PAD_PD07_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd07_pad_ctl;
#[doc = "PAD_PD08_FUNC_CTL register accessor: an alias for `Reg<PAD_PD08_FUNC_CTL_SPEC>`"]
pub type PAD_PD08_FUNC_CTL = crate::Reg<pad_pd08_func_ctl::PAD_PD08_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd08_func_ctl;
#[doc = "PAD_PD08_PAD_CTL register accessor: an alias for `Reg<PAD_PD08_PAD_CTL_SPEC>`"]
pub type PAD_PD08_PAD_CTL = crate::Reg<pad_pd08_pad_ctl::PAD_PD08_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd08_pad_ctl;
#[doc = "PAD_PD09_FUNC_CTL register accessor: an alias for `Reg<PAD_PD09_FUNC_CTL_SPEC>`"]
pub type PAD_PD09_FUNC_CTL = crate::Reg<pad_pd09_func_ctl::PAD_PD09_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd09_func_ctl;
#[doc = "PAD_PD09_PAD_CTL register accessor: an alias for `Reg<PAD_PD09_PAD_CTL_SPEC>`"]
pub type PAD_PD09_PAD_CTL = crate::Reg<pad_pd09_pad_ctl::PAD_PD09_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd09_pad_ctl;
#[doc = "PAD_PD10_FUNC_CTL register accessor: an alias for `Reg<PAD_PD10_FUNC_CTL_SPEC>`"]
pub type PAD_PD10_FUNC_CTL = crate::Reg<pad_pd10_func_ctl::PAD_PD10_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd10_func_ctl;
#[doc = "PAD_PD10_PAD_CTL register accessor: an alias for `Reg<PAD_PD10_PAD_CTL_SPEC>`"]
pub type PAD_PD10_PAD_CTL = crate::Reg<pad_pd10_pad_ctl::PAD_PD10_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd10_pad_ctl;
#[doc = "PAD_PD11_FUNC_CTL register accessor: an alias for `Reg<PAD_PD11_FUNC_CTL_SPEC>`"]
pub type PAD_PD11_FUNC_CTL = crate::Reg<pad_pd11_func_ctl::PAD_PD11_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd11_func_ctl;
#[doc = "PAD_PD11_PAD_CTL register accessor: an alias for `Reg<PAD_PD11_PAD_CTL_SPEC>`"]
pub type PAD_PD11_PAD_CTL = crate::Reg<pad_pd11_pad_ctl::PAD_PD11_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd11_pad_ctl;
#[doc = "PAD_PD12_FUNC_CTL register accessor: an alias for `Reg<PAD_PD12_FUNC_CTL_SPEC>`"]
pub type PAD_PD12_FUNC_CTL = crate::Reg<pad_pd12_func_ctl::PAD_PD12_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd12_func_ctl;
#[doc = "PAD_PD12_PAD_CTL register accessor: an alias for `Reg<PAD_PD12_PAD_CTL_SPEC>`"]
pub type PAD_PD12_PAD_CTL = crate::Reg<pad_pd12_pad_ctl::PAD_PD12_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd12_pad_ctl;
#[doc = "PAD_PD13_FUNC_CTL register accessor: an alias for `Reg<PAD_PD13_FUNC_CTL_SPEC>`"]
pub type PAD_PD13_FUNC_CTL = crate::Reg<pad_pd13_func_ctl::PAD_PD13_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd13_func_ctl;
#[doc = "PAD_PD13_PAD_CTL register accessor: an alias for `Reg<PAD_PD13_PAD_CTL_SPEC>`"]
pub type PAD_PD13_PAD_CTL = crate::Reg<pad_pd13_pad_ctl::PAD_PD13_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd13_pad_ctl;
#[doc = "PAD_PD14_FUNC_CTL register accessor: an alias for `Reg<PAD_PD14_FUNC_CTL_SPEC>`"]
pub type PAD_PD14_FUNC_CTL = crate::Reg<pad_pd14_func_ctl::PAD_PD14_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd14_func_ctl;
#[doc = "PAD_PD14_PAD_CTL register accessor: an alias for `Reg<PAD_PD14_PAD_CTL_SPEC>`"]
pub type PAD_PD14_PAD_CTL = crate::Reg<pad_pd14_pad_ctl::PAD_PD14_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd14_pad_ctl;
#[doc = "PAD_PD15_FUNC_CTL register accessor: an alias for `Reg<PAD_PD15_FUNC_CTL_SPEC>`"]
pub type PAD_PD15_FUNC_CTL = crate::Reg<pad_pd15_func_ctl::PAD_PD15_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd15_func_ctl;
#[doc = "PAD_PD15_PAD_CTL register accessor: an alias for `Reg<PAD_PD15_PAD_CTL_SPEC>`"]
pub type PAD_PD15_PAD_CTL = crate::Reg<pad_pd15_pad_ctl::PAD_PD15_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd15_pad_ctl;
#[doc = "PAD_PD16_FUNC_CTL register accessor: an alias for `Reg<PAD_PD16_FUNC_CTL_SPEC>`"]
pub type PAD_PD16_FUNC_CTL = crate::Reg<pad_pd16_func_ctl::PAD_PD16_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd16_func_ctl;
#[doc = "PAD_PD16_PAD_CTL register accessor: an alias for `Reg<PAD_PD16_PAD_CTL_SPEC>`"]
pub type PAD_PD16_PAD_CTL = crate::Reg<pad_pd16_pad_ctl::PAD_PD16_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd16_pad_ctl;
#[doc = "PAD_PD17_FUNC_CTL register accessor: an alias for `Reg<PAD_PD17_FUNC_CTL_SPEC>`"]
pub type PAD_PD17_FUNC_CTL = crate::Reg<pad_pd17_func_ctl::PAD_PD17_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd17_func_ctl;
#[doc = "PAD_PD17_PAD_CTL register accessor: an alias for `Reg<PAD_PD17_PAD_CTL_SPEC>`"]
pub type PAD_PD17_PAD_CTL = crate::Reg<pad_pd17_pad_ctl::PAD_PD17_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd17_pad_ctl;
#[doc = "PAD_PD18_FUNC_CTL register accessor: an alias for `Reg<PAD_PD18_FUNC_CTL_SPEC>`"]
pub type PAD_PD18_FUNC_CTL = crate::Reg<pad_pd18_func_ctl::PAD_PD18_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd18_func_ctl;
#[doc = "PAD_PD18_PAD_CTL register accessor: an alias for `Reg<PAD_PD18_PAD_CTL_SPEC>`"]
pub type PAD_PD18_PAD_CTL = crate::Reg<pad_pd18_pad_ctl::PAD_PD18_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd18_pad_ctl;
#[doc = "PAD_PD19_FUNC_CTL register accessor: an alias for `Reg<PAD_PD19_FUNC_CTL_SPEC>`"]
pub type PAD_PD19_FUNC_CTL = crate::Reg<pad_pd19_func_ctl::PAD_PD19_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd19_func_ctl;
#[doc = "PAD_PD19_PAD_CTL register accessor: an alias for `Reg<PAD_PD19_PAD_CTL_SPEC>`"]
pub type PAD_PD19_PAD_CTL = crate::Reg<pad_pd19_pad_ctl::PAD_PD19_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd19_pad_ctl;
#[doc = "PAD_PD20_FUNC_CTL register accessor: an alias for `Reg<PAD_PD20_FUNC_CTL_SPEC>`"]
pub type PAD_PD20_FUNC_CTL = crate::Reg<pad_pd20_func_ctl::PAD_PD20_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd20_func_ctl;
#[doc = "PAD_PD20_PAD_CTL register accessor: an alias for `Reg<PAD_PD20_PAD_CTL_SPEC>`"]
pub type PAD_PD20_PAD_CTL = crate::Reg<pad_pd20_pad_ctl::PAD_PD20_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd20_pad_ctl;
#[doc = "PAD_PD21_FUNC_CTL register accessor: an alias for `Reg<PAD_PD21_FUNC_CTL_SPEC>`"]
pub type PAD_PD21_FUNC_CTL = crate::Reg<pad_pd21_func_ctl::PAD_PD21_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd21_func_ctl;
#[doc = "PAD_PD21_PAD_CTL register accessor: an alias for `Reg<PAD_PD21_PAD_CTL_SPEC>`"]
pub type PAD_PD21_PAD_CTL = crate::Reg<pad_pd21_pad_ctl::PAD_PD21_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd21_pad_ctl;
#[doc = "PAD_PD22_FUNC_CTL register accessor: an alias for `Reg<PAD_PD22_FUNC_CTL_SPEC>`"]
pub type PAD_PD22_FUNC_CTL = crate::Reg<pad_pd22_func_ctl::PAD_PD22_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd22_func_ctl;
#[doc = "PAD_PD22_PAD_CTL register accessor: an alias for `Reg<PAD_PD22_PAD_CTL_SPEC>`"]
pub type PAD_PD22_PAD_CTL = crate::Reg<pad_pd22_pad_ctl::PAD_PD22_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd22_pad_ctl;
#[doc = "PAD_PD23_FUNC_CTL register accessor: an alias for `Reg<PAD_PD23_FUNC_CTL_SPEC>`"]
pub type PAD_PD23_FUNC_CTL = crate::Reg<pad_pd23_func_ctl::PAD_PD23_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd23_func_ctl;
#[doc = "PAD_PD23_PAD_CTL register accessor: an alias for `Reg<PAD_PD23_PAD_CTL_SPEC>`"]
pub type PAD_PD23_PAD_CTL = crate::Reg<pad_pd23_pad_ctl::PAD_PD23_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd23_pad_ctl;
#[doc = "PAD_PD24_FUNC_CTL register accessor: an alias for `Reg<PAD_PD24_FUNC_CTL_SPEC>`"]
pub type PAD_PD24_FUNC_CTL = crate::Reg<pad_pd24_func_ctl::PAD_PD24_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd24_func_ctl;
#[doc = "PAD_PD24_PAD_CTL register accessor: an alias for `Reg<PAD_PD24_PAD_CTL_SPEC>`"]
pub type PAD_PD24_PAD_CTL = crate::Reg<pad_pd24_pad_ctl::PAD_PD24_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd24_pad_ctl;
#[doc = "PAD_PD25_FUNC_CTL register accessor: an alias for `Reg<PAD_PD25_FUNC_CTL_SPEC>`"]
pub type PAD_PD25_FUNC_CTL = crate::Reg<pad_pd25_func_ctl::PAD_PD25_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd25_func_ctl;
#[doc = "PAD_PD25_PAD_CTL register accessor: an alias for `Reg<PAD_PD25_PAD_CTL_SPEC>`"]
pub type PAD_PD25_PAD_CTL = crate::Reg<pad_pd25_pad_ctl::PAD_PD25_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd25_pad_ctl;
#[doc = "PAD_PD26_FUNC_CTL register accessor: an alias for `Reg<PAD_PD26_FUNC_CTL_SPEC>`"]
pub type PAD_PD26_FUNC_CTL = crate::Reg<pad_pd26_func_ctl::PAD_PD26_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd26_func_ctl;
#[doc = "PAD_PD26_PAD_CTL register accessor: an alias for `Reg<PAD_PD26_PAD_CTL_SPEC>`"]
pub type PAD_PD26_PAD_CTL = crate::Reg<pad_pd26_pad_ctl::PAD_PD26_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd26_pad_ctl;
#[doc = "PAD_PD27_FUNC_CTL register accessor: an alias for `Reg<PAD_PD27_FUNC_CTL_SPEC>`"]
pub type PAD_PD27_FUNC_CTL = crate::Reg<pad_pd27_func_ctl::PAD_PD27_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd27_func_ctl;
#[doc = "PAD_PD27_PAD_CTL register accessor: an alias for `Reg<PAD_PD27_PAD_CTL_SPEC>`"]
pub type PAD_PD27_PAD_CTL = crate::Reg<pad_pd27_pad_ctl::PAD_PD27_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd27_pad_ctl;
#[doc = "PAD_PD28_FUNC_CTL register accessor: an alias for `Reg<PAD_PD28_FUNC_CTL_SPEC>`"]
pub type PAD_PD28_FUNC_CTL = crate::Reg<pad_pd28_func_ctl::PAD_PD28_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd28_func_ctl;
#[doc = "PAD_PD28_PAD_CTL register accessor: an alias for `Reg<PAD_PD28_PAD_CTL_SPEC>`"]
pub type PAD_PD28_PAD_CTL = crate::Reg<pad_pd28_pad_ctl::PAD_PD28_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd28_pad_ctl;
#[doc = "PAD_PD29_FUNC_CTL register accessor: an alias for `Reg<PAD_PD29_FUNC_CTL_SPEC>`"]
pub type PAD_PD29_FUNC_CTL = crate::Reg<pad_pd29_func_ctl::PAD_PD29_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd29_func_ctl;
#[doc = "PAD_PD29_PAD_CTL register accessor: an alias for `Reg<PAD_PD29_PAD_CTL_SPEC>`"]
pub type PAD_PD29_PAD_CTL = crate::Reg<pad_pd29_pad_ctl::PAD_PD29_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd29_pad_ctl;
#[doc = "PAD_PD30_FUNC_CTL register accessor: an alias for `Reg<PAD_PD30_FUNC_CTL_SPEC>`"]
pub type PAD_PD30_FUNC_CTL = crate::Reg<pad_pd30_func_ctl::PAD_PD30_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd30_func_ctl;
#[doc = "PAD_PD30_PAD_CTL register accessor: an alias for `Reg<PAD_PD30_PAD_CTL_SPEC>`"]
pub type PAD_PD30_PAD_CTL = crate::Reg<pad_pd30_pad_ctl::PAD_PD30_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd30_pad_ctl;
#[doc = "PAD_PD31_FUNC_CTL register accessor: an alias for `Reg<PAD_PD31_FUNC_CTL_SPEC>`"]
pub type PAD_PD31_FUNC_CTL = crate::Reg<pad_pd31_func_ctl::PAD_PD31_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pd31_func_ctl;
#[doc = "PAD_PD31_PAD_CTL register accessor: an alias for `Reg<PAD_PD31_PAD_CTL_SPEC>`"]
pub type PAD_PD31_PAD_CTL = crate::Reg<pad_pd31_pad_ctl::PAD_PD31_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pd31_pad_ctl;
#[doc = "PAD_PE00_FUNC_CTL register accessor: an alias for `Reg<PAD_PE00_FUNC_CTL_SPEC>`"]
pub type PAD_PE00_FUNC_CTL = crate::Reg<pad_pe00_func_ctl::PAD_PE00_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe00_func_ctl;
#[doc = "PAD_PE00_PAD_CTL register accessor: an alias for `Reg<PAD_PE00_PAD_CTL_SPEC>`"]
pub type PAD_PE00_PAD_CTL = crate::Reg<pad_pe00_pad_ctl::PAD_PE00_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe00_pad_ctl;
#[doc = "PAD_PE01_FUNC_CTL register accessor: an alias for `Reg<PAD_PE01_FUNC_CTL_SPEC>`"]
pub type PAD_PE01_FUNC_CTL = crate::Reg<pad_pe01_func_ctl::PAD_PE01_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe01_func_ctl;
#[doc = "PAD_PE01_PAD_CTL register accessor: an alias for `Reg<PAD_PE01_PAD_CTL_SPEC>`"]
pub type PAD_PE01_PAD_CTL = crate::Reg<pad_pe01_pad_ctl::PAD_PE01_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe01_pad_ctl;
#[doc = "PAD_PE02_FUNC_CTL register accessor: an alias for `Reg<PAD_PE02_FUNC_CTL_SPEC>`"]
pub type PAD_PE02_FUNC_CTL = crate::Reg<pad_pe02_func_ctl::PAD_PE02_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe02_func_ctl;
#[doc = "PAD_PE02_PAD_CTL register accessor: an alias for `Reg<PAD_PE02_PAD_CTL_SPEC>`"]
pub type PAD_PE02_PAD_CTL = crate::Reg<pad_pe02_pad_ctl::PAD_PE02_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe02_pad_ctl;
#[doc = "PAD_PE03_FUNC_CTL register accessor: an alias for `Reg<PAD_PE03_FUNC_CTL_SPEC>`"]
pub type PAD_PE03_FUNC_CTL = crate::Reg<pad_pe03_func_ctl::PAD_PE03_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe03_func_ctl;
#[doc = "PAD_PE03_PAD_CTL register accessor: an alias for `Reg<PAD_PE03_PAD_CTL_SPEC>`"]
pub type PAD_PE03_PAD_CTL = crate::Reg<pad_pe03_pad_ctl::PAD_PE03_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe03_pad_ctl;
#[doc = "PAD_PE04_FUNC_CTL register accessor: an alias for `Reg<PAD_PE04_FUNC_CTL_SPEC>`"]
pub type PAD_PE04_FUNC_CTL = crate::Reg<pad_pe04_func_ctl::PAD_PE04_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe04_func_ctl;
#[doc = "PAD_PE04_PAD_CTL register accessor: an alias for `Reg<PAD_PE04_PAD_CTL_SPEC>`"]
pub type PAD_PE04_PAD_CTL = crate::Reg<pad_pe04_pad_ctl::PAD_PE04_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe04_pad_ctl;
#[doc = "PAD_PE05_FUNC_CTL register accessor: an alias for `Reg<PAD_PE05_FUNC_CTL_SPEC>`"]
pub type PAD_PE05_FUNC_CTL = crate::Reg<pad_pe05_func_ctl::PAD_PE05_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe05_func_ctl;
#[doc = "PAD_PE05_PAD_CTL register accessor: an alias for `Reg<PAD_PE05_PAD_CTL_SPEC>`"]
pub type PAD_PE05_PAD_CTL = crate::Reg<pad_pe05_pad_ctl::PAD_PE05_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe05_pad_ctl;
#[doc = "PAD_PE06_FUNC_CTL register accessor: an alias for `Reg<PAD_PE06_FUNC_CTL_SPEC>`"]
pub type PAD_PE06_FUNC_CTL = crate::Reg<pad_pe06_func_ctl::PAD_PE06_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe06_func_ctl;
#[doc = "PAD_PE06_PAD_CTL register accessor: an alias for `Reg<PAD_PE06_PAD_CTL_SPEC>`"]
pub type PAD_PE06_PAD_CTL = crate::Reg<pad_pe06_pad_ctl::PAD_PE06_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe06_pad_ctl;
#[doc = "PAD_PE07_FUNC_CTL register accessor: an alias for `Reg<PAD_PE07_FUNC_CTL_SPEC>`"]
pub type PAD_PE07_FUNC_CTL = crate::Reg<pad_pe07_func_ctl::PAD_PE07_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe07_func_ctl;
#[doc = "PAD_PE07_PAD_CTL register accessor: an alias for `Reg<PAD_PE07_PAD_CTL_SPEC>`"]
pub type PAD_PE07_PAD_CTL = crate::Reg<pad_pe07_pad_ctl::PAD_PE07_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe07_pad_ctl;
#[doc = "PAD_PE08_FUNC_CTL register accessor: an alias for `Reg<PAD_PE08_FUNC_CTL_SPEC>`"]
pub type PAD_PE08_FUNC_CTL = crate::Reg<pad_pe08_func_ctl::PAD_PE08_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe08_func_ctl;
#[doc = "PAD_PE08_PAD_CTL register accessor: an alias for `Reg<PAD_PE08_PAD_CTL_SPEC>`"]
pub type PAD_PE08_PAD_CTL = crate::Reg<pad_pe08_pad_ctl::PAD_PE08_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe08_pad_ctl;
#[doc = "PAD_PE09_FUNC_CTL register accessor: an alias for `Reg<PAD_PE09_FUNC_CTL_SPEC>`"]
pub type PAD_PE09_FUNC_CTL = crate::Reg<pad_pe09_func_ctl::PAD_PE09_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe09_func_ctl;
#[doc = "PAD_PE09_PAD_CTL register accessor: an alias for `Reg<PAD_PE09_PAD_CTL_SPEC>`"]
pub type PAD_PE09_PAD_CTL = crate::Reg<pad_pe09_pad_ctl::PAD_PE09_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe09_pad_ctl;
#[doc = "PAD_PE10_FUNC_CTL register accessor: an alias for `Reg<PAD_PE10_FUNC_CTL_SPEC>`"]
pub type PAD_PE10_FUNC_CTL = crate::Reg<pad_pe10_func_ctl::PAD_PE10_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe10_func_ctl;
#[doc = "PAD_PE10_PAD_CTL register accessor: an alias for `Reg<PAD_PE10_PAD_CTL_SPEC>`"]
pub type PAD_PE10_PAD_CTL = crate::Reg<pad_pe10_pad_ctl::PAD_PE10_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe10_pad_ctl;
#[doc = "PAD_PE11_FUNC_CTL register accessor: an alias for `Reg<PAD_PE11_FUNC_CTL_SPEC>`"]
pub type PAD_PE11_FUNC_CTL = crate::Reg<pad_pe11_func_ctl::PAD_PE11_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe11_func_ctl;
#[doc = "PAD_PE11_PAD_CTL register accessor: an alias for `Reg<PAD_PE11_PAD_CTL_SPEC>`"]
pub type PAD_PE11_PAD_CTL = crate::Reg<pad_pe11_pad_ctl::PAD_PE11_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe11_pad_ctl;
#[doc = "PAD_PE12_FUNC_CTL register accessor: an alias for `Reg<PAD_PE12_FUNC_CTL_SPEC>`"]
pub type PAD_PE12_FUNC_CTL = crate::Reg<pad_pe12_func_ctl::PAD_PE12_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe12_func_ctl;
#[doc = "PAD_PE12_PAD_CTL register accessor: an alias for `Reg<PAD_PE12_PAD_CTL_SPEC>`"]
pub type PAD_PE12_PAD_CTL = crate::Reg<pad_pe12_pad_ctl::PAD_PE12_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe12_pad_ctl;
#[doc = "PAD_PE13_FUNC_CTL register accessor: an alias for `Reg<PAD_PE13_FUNC_CTL_SPEC>`"]
pub type PAD_PE13_FUNC_CTL = crate::Reg<pad_pe13_func_ctl::PAD_PE13_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe13_func_ctl;
#[doc = "PAD_PE13_PAD_CTL register accessor: an alias for `Reg<PAD_PE13_PAD_CTL_SPEC>`"]
pub type PAD_PE13_PAD_CTL = crate::Reg<pad_pe13_pad_ctl::PAD_PE13_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe13_pad_ctl;
#[doc = "PAD_PE14_FUNC_CTL register accessor: an alias for `Reg<PAD_PE14_FUNC_CTL_SPEC>`"]
pub type PAD_PE14_FUNC_CTL = crate::Reg<pad_pe14_func_ctl::PAD_PE14_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe14_func_ctl;
#[doc = "PAD_PE14_PAD_CTL register accessor: an alias for `Reg<PAD_PE14_PAD_CTL_SPEC>`"]
pub type PAD_PE14_PAD_CTL = crate::Reg<pad_pe14_pad_ctl::PAD_PE14_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe14_pad_ctl;
#[doc = "PAD_PE15_FUNC_CTL register accessor: an alias for `Reg<PAD_PE15_FUNC_CTL_SPEC>`"]
pub type PAD_PE15_FUNC_CTL = crate::Reg<pad_pe15_func_ctl::PAD_PE15_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe15_func_ctl;
#[doc = "PAD_PE15_PAD_CTL register accessor: an alias for `Reg<PAD_PE15_PAD_CTL_SPEC>`"]
pub type PAD_PE15_PAD_CTL = crate::Reg<pad_pe15_pad_ctl::PAD_PE15_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe15_pad_ctl;
#[doc = "PAD_PE16_FUNC_CTL register accessor: an alias for `Reg<PAD_PE16_FUNC_CTL_SPEC>`"]
pub type PAD_PE16_FUNC_CTL = crate::Reg<pad_pe16_func_ctl::PAD_PE16_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe16_func_ctl;
#[doc = "PAD_PE16_PAD_CTL register accessor: an alias for `Reg<PAD_PE16_PAD_CTL_SPEC>`"]
pub type PAD_PE16_PAD_CTL = crate::Reg<pad_pe16_pad_ctl::PAD_PE16_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe16_pad_ctl;
#[doc = "PAD_PE17_FUNC_CTL register accessor: an alias for `Reg<PAD_PE17_FUNC_CTL_SPEC>`"]
pub type PAD_PE17_FUNC_CTL = crate::Reg<pad_pe17_func_ctl::PAD_PE17_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe17_func_ctl;
#[doc = "PAD_PE17_PAD_CTL register accessor: an alias for `Reg<PAD_PE17_PAD_CTL_SPEC>`"]
pub type PAD_PE17_PAD_CTL = crate::Reg<pad_pe17_pad_ctl::PAD_PE17_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe17_pad_ctl;
#[doc = "PAD_PE18_FUNC_CTL register accessor: an alias for `Reg<PAD_PE18_FUNC_CTL_SPEC>`"]
pub type PAD_PE18_FUNC_CTL = crate::Reg<pad_pe18_func_ctl::PAD_PE18_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe18_func_ctl;
#[doc = "PAD_PE18_PAD_CTL register accessor: an alias for `Reg<PAD_PE18_PAD_CTL_SPEC>`"]
pub type PAD_PE18_PAD_CTL = crate::Reg<pad_pe18_pad_ctl::PAD_PE18_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe18_pad_ctl;
#[doc = "PAD_PE19_FUNC_CTL register accessor: an alias for `Reg<PAD_PE19_FUNC_CTL_SPEC>`"]
pub type PAD_PE19_FUNC_CTL = crate::Reg<pad_pe19_func_ctl::PAD_PE19_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe19_func_ctl;
#[doc = "PAD_PE19_PAD_CTL register accessor: an alias for `Reg<PAD_PE19_PAD_CTL_SPEC>`"]
pub type PAD_PE19_PAD_CTL = crate::Reg<pad_pe19_pad_ctl::PAD_PE19_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe19_pad_ctl;
#[doc = "PAD_PE20_FUNC_CTL register accessor: an alias for `Reg<PAD_PE20_FUNC_CTL_SPEC>`"]
pub type PAD_PE20_FUNC_CTL = crate::Reg<pad_pe20_func_ctl::PAD_PE20_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe20_func_ctl;
#[doc = "PAD_PE20_PAD_CTL register accessor: an alias for `Reg<PAD_PE20_PAD_CTL_SPEC>`"]
pub type PAD_PE20_PAD_CTL = crate::Reg<pad_pe20_pad_ctl::PAD_PE20_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe20_pad_ctl;
#[doc = "PAD_PE21_FUNC_CTL register accessor: an alias for `Reg<PAD_PE21_FUNC_CTL_SPEC>`"]
pub type PAD_PE21_FUNC_CTL = crate::Reg<pad_pe21_func_ctl::PAD_PE21_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe21_func_ctl;
#[doc = "PAD_PE21_PAD_CTL register accessor: an alias for `Reg<PAD_PE21_PAD_CTL_SPEC>`"]
pub type PAD_PE21_PAD_CTL = crate::Reg<pad_pe21_pad_ctl::PAD_PE21_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe21_pad_ctl;
#[doc = "PAD_PE22_FUNC_CTL register accessor: an alias for `Reg<PAD_PE22_FUNC_CTL_SPEC>`"]
pub type PAD_PE22_FUNC_CTL = crate::Reg<pad_pe22_func_ctl::PAD_PE22_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe22_func_ctl;
#[doc = "PAD_PE22_PAD_CTL register accessor: an alias for `Reg<PAD_PE22_PAD_CTL_SPEC>`"]
pub type PAD_PE22_PAD_CTL = crate::Reg<pad_pe22_pad_ctl::PAD_PE22_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe22_pad_ctl;
#[doc = "PAD_PE23_FUNC_CTL register accessor: an alias for `Reg<PAD_PE23_FUNC_CTL_SPEC>`"]
pub type PAD_PE23_FUNC_CTL = crate::Reg<pad_pe23_func_ctl::PAD_PE23_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe23_func_ctl;
#[doc = "PAD_PE23_PAD_CTL register accessor: an alias for `Reg<PAD_PE23_PAD_CTL_SPEC>`"]
pub type PAD_PE23_PAD_CTL = crate::Reg<pad_pe23_pad_ctl::PAD_PE23_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe23_pad_ctl;
#[doc = "PAD_PE24_FUNC_CTL register accessor: an alias for `Reg<PAD_PE24_FUNC_CTL_SPEC>`"]
pub type PAD_PE24_FUNC_CTL = crate::Reg<pad_pe24_func_ctl::PAD_PE24_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe24_func_ctl;
#[doc = "PAD_PE24_PAD_CTL register accessor: an alias for `Reg<PAD_PE24_PAD_CTL_SPEC>`"]
pub type PAD_PE24_PAD_CTL = crate::Reg<pad_pe24_pad_ctl::PAD_PE24_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe24_pad_ctl;
#[doc = "PAD_PE25_FUNC_CTL register accessor: an alias for `Reg<PAD_PE25_FUNC_CTL_SPEC>`"]
pub type PAD_PE25_FUNC_CTL = crate::Reg<pad_pe25_func_ctl::PAD_PE25_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe25_func_ctl;
#[doc = "PAD_PE25_PAD_CTL register accessor: an alias for `Reg<PAD_PE25_PAD_CTL_SPEC>`"]
pub type PAD_PE25_PAD_CTL = crate::Reg<pad_pe25_pad_ctl::PAD_PE25_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe25_pad_ctl;
#[doc = "PAD_PE26_FUNC_CTL register accessor: an alias for `Reg<PAD_PE26_FUNC_CTL_SPEC>`"]
pub type PAD_PE26_FUNC_CTL = crate::Reg<pad_pe26_func_ctl::PAD_PE26_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe26_func_ctl;
#[doc = "PAD_PE26_PAD_CTL register accessor: an alias for `Reg<PAD_PE26_PAD_CTL_SPEC>`"]
pub type PAD_PE26_PAD_CTL = crate::Reg<pad_pe26_pad_ctl::PAD_PE26_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe26_pad_ctl;
#[doc = "PAD_PE27_FUNC_CTL register accessor: an alias for `Reg<PAD_PE27_FUNC_CTL_SPEC>`"]
pub type PAD_PE27_FUNC_CTL = crate::Reg<pad_pe27_func_ctl::PAD_PE27_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe27_func_ctl;
#[doc = "PAD_PE27_PAD_CTL register accessor: an alias for `Reg<PAD_PE27_PAD_CTL_SPEC>`"]
pub type PAD_PE27_PAD_CTL = crate::Reg<pad_pe27_pad_ctl::PAD_PE27_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe27_pad_ctl;
#[doc = "PAD_PE28_FUNC_CTL register accessor: an alias for `Reg<PAD_PE28_FUNC_CTL_SPEC>`"]
pub type PAD_PE28_FUNC_CTL = crate::Reg<pad_pe28_func_ctl::PAD_PE28_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe28_func_ctl;
#[doc = "PAD_PE28_PAD_CTL register accessor: an alias for `Reg<PAD_PE28_PAD_CTL_SPEC>`"]
pub type PAD_PE28_PAD_CTL = crate::Reg<pad_pe28_pad_ctl::PAD_PE28_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe28_pad_ctl;
#[doc = "PAD_PE29_FUNC_CTL register accessor: an alias for `Reg<PAD_PE29_FUNC_CTL_SPEC>`"]
pub type PAD_PE29_FUNC_CTL = crate::Reg<pad_pe29_func_ctl::PAD_PE29_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe29_func_ctl;
#[doc = "PAD_PE29_PAD_CTL register accessor: an alias for `Reg<PAD_PE29_PAD_CTL_SPEC>`"]
pub type PAD_PE29_PAD_CTL = crate::Reg<pad_pe29_pad_ctl::PAD_PE29_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe29_pad_ctl;
#[doc = "PAD_PE30_FUNC_CTL register accessor: an alias for `Reg<PAD_PE30_FUNC_CTL_SPEC>`"]
pub type PAD_PE30_FUNC_CTL = crate::Reg<pad_pe30_func_ctl::PAD_PE30_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe30_func_ctl;
#[doc = "PAD_PE30_PAD_CTL register accessor: an alias for `Reg<PAD_PE30_PAD_CTL_SPEC>`"]
pub type PAD_PE30_PAD_CTL = crate::Reg<pad_pe30_pad_ctl::PAD_PE30_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe30_pad_ctl;
#[doc = "PAD_PE31_FUNC_CTL register accessor: an alias for `Reg<PAD_PE31_FUNC_CTL_SPEC>`"]
pub type PAD_PE31_FUNC_CTL = crate::Reg<pad_pe31_func_ctl::PAD_PE31_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pe31_func_ctl;
#[doc = "PAD_PE31_PAD_CTL register accessor: an alias for `Reg<PAD_PE31_PAD_CTL_SPEC>`"]
pub type PAD_PE31_PAD_CTL = crate::Reg<pad_pe31_pad_ctl::PAD_PE31_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pe31_pad_ctl;
#[doc = "PAD_PF00_FUNC_CTL register accessor: an alias for `Reg<PAD_PF00_FUNC_CTL_SPEC>`"]
pub type PAD_PF00_FUNC_CTL = crate::Reg<pad_pf00_func_ctl::PAD_PF00_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pf00_func_ctl;
#[doc = "PAD_PF00_PAD_CTL register accessor: an alias for `Reg<PAD_PF00_PAD_CTL_SPEC>`"]
pub type PAD_PF00_PAD_CTL = crate::Reg<pad_pf00_pad_ctl::PAD_PF00_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pf00_pad_ctl;
#[doc = "PAD_PF01_FUNC_CTL register accessor: an alias for `Reg<PAD_PF01_FUNC_CTL_SPEC>`"]
pub type PAD_PF01_FUNC_CTL = crate::Reg<pad_pf01_func_ctl::PAD_PF01_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pf01_func_ctl;
#[doc = "PAD_PF01_PAD_CTL register accessor: an alias for `Reg<PAD_PF01_PAD_CTL_SPEC>`"]
pub type PAD_PF01_PAD_CTL = crate::Reg<pad_pf01_pad_ctl::PAD_PF01_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pf01_pad_ctl;
#[doc = "PAD_PF02_FUNC_CTL register accessor: an alias for `Reg<PAD_PF02_FUNC_CTL_SPEC>`"]
pub type PAD_PF02_FUNC_CTL = crate::Reg<pad_pf02_func_ctl::PAD_PF02_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pf02_func_ctl;
#[doc = "PAD_PF02_PAD_CTL register accessor: an alias for `Reg<PAD_PF02_PAD_CTL_SPEC>`"]
pub type PAD_PF02_PAD_CTL = crate::Reg<pad_pf02_pad_ctl::PAD_PF02_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pf02_pad_ctl;
#[doc = "PAD_PF03_FUNC_CTL register accessor: an alias for `Reg<PAD_PF03_FUNC_CTL_SPEC>`"]
pub type PAD_PF03_FUNC_CTL = crate::Reg<pad_pf03_func_ctl::PAD_PF03_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pf03_func_ctl;
#[doc = "PAD_PF03_PAD_CTL register accessor: an alias for `Reg<PAD_PF03_PAD_CTL_SPEC>`"]
pub type PAD_PF03_PAD_CTL = crate::Reg<pad_pf03_pad_ctl::PAD_PF03_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pf03_pad_ctl;
#[doc = "PAD_PF04_FUNC_CTL register accessor: an alias for `Reg<PAD_PF04_FUNC_CTL_SPEC>`"]
pub type PAD_PF04_FUNC_CTL = crate::Reg<pad_pf04_func_ctl::PAD_PF04_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pf04_func_ctl;
#[doc = "PAD_PF04_PAD_CTL register accessor: an alias for `Reg<PAD_PF04_PAD_CTL_SPEC>`"]
pub type PAD_PF04_PAD_CTL = crate::Reg<pad_pf04_pad_ctl::PAD_PF04_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pf04_pad_ctl;
#[doc = "PAD_PF05_FUNC_CTL register accessor: an alias for `Reg<PAD_PF05_FUNC_CTL_SPEC>`"]
pub type PAD_PF05_FUNC_CTL = crate::Reg<pad_pf05_func_ctl::PAD_PF05_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pf05_func_ctl;
#[doc = "PAD_PF05_PAD_CTL register accessor: an alias for `Reg<PAD_PF05_PAD_CTL_SPEC>`"]
pub type PAD_PF05_PAD_CTL = crate::Reg<pad_pf05_pad_ctl::PAD_PF05_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pf05_pad_ctl;
#[doc = "PAD_PF06_FUNC_CTL register accessor: an alias for `Reg<PAD_PF06_FUNC_CTL_SPEC>`"]
pub type PAD_PF06_FUNC_CTL = crate::Reg<pad_pf06_func_ctl::PAD_PF06_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pf06_func_ctl;
#[doc = "PAD_PF06_PAD_CTL register accessor: an alias for `Reg<PAD_PF06_PAD_CTL_SPEC>`"]
pub type PAD_PF06_PAD_CTL = crate::Reg<pad_pf06_pad_ctl::PAD_PF06_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pf06_pad_ctl;
#[doc = "PAD_PF07_FUNC_CTL register accessor: an alias for `Reg<PAD_PF07_FUNC_CTL_SPEC>`"]
pub type PAD_PF07_FUNC_CTL = crate::Reg<pad_pf07_func_ctl::PAD_PF07_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pf07_func_ctl;
#[doc = "PAD_PF07_PAD_CTL register accessor: an alias for `Reg<PAD_PF07_PAD_CTL_SPEC>`"]
pub type PAD_PF07_PAD_CTL = crate::Reg<pad_pf07_pad_ctl::PAD_PF07_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pf07_pad_ctl;
#[doc = "PAD_PF08_FUNC_CTL register accessor: an alias for `Reg<PAD_PF08_FUNC_CTL_SPEC>`"]
pub type PAD_PF08_FUNC_CTL = crate::Reg<pad_pf08_func_ctl::PAD_PF08_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pf08_func_ctl;
#[doc = "PAD_PF08_PAD_CTL register accessor: an alias for `Reg<PAD_PF08_PAD_CTL_SPEC>`"]
pub type PAD_PF08_PAD_CTL = crate::Reg<pad_pf08_pad_ctl::PAD_PF08_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pf08_pad_ctl;
#[doc = "PAD_PF09_FUNC_CTL register accessor: an alias for `Reg<PAD_PF09_FUNC_CTL_SPEC>`"]
pub type PAD_PF09_FUNC_CTL = crate::Reg<pad_pf09_func_ctl::PAD_PF09_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pf09_func_ctl;
#[doc = "PAD_PF09_PAD_CTL register accessor: an alias for `Reg<PAD_PF09_PAD_CTL_SPEC>`"]
pub type PAD_PF09_PAD_CTL = crate::Reg<pad_pf09_pad_ctl::PAD_PF09_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pf09_pad_ctl;
#[doc = "PAD_PF10_FUNC_CTL register accessor: an alias for `Reg<PAD_PF10_FUNC_CTL_SPEC>`"]
pub type PAD_PF10_FUNC_CTL = crate::Reg<pad_pf10_func_ctl::PAD_PF10_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pf10_func_ctl;
#[doc = "PAD_PF10_PAD_CTL register accessor: an alias for `Reg<PAD_PF10_PAD_CTL_SPEC>`"]
pub type PAD_PF10_PAD_CTL = crate::Reg<pad_pf10_pad_ctl::PAD_PF10_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pf10_pad_ctl;
#[doc = "PAD_PX00_FUNC_CTL register accessor: an alias for `Reg<PAD_PX00_FUNC_CTL_SPEC>`"]
pub type PAD_PX00_FUNC_CTL = crate::Reg<pad_px00_func_ctl::PAD_PX00_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_px00_func_ctl;
#[doc = "PAD_PX00_PAD_CTL register accessor: an alias for `Reg<PAD_PX00_PAD_CTL_SPEC>`"]
pub type PAD_PX00_PAD_CTL = crate::Reg<pad_px00_pad_ctl::PAD_PX00_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_px00_pad_ctl;
#[doc = "PAD_PX01_FUNC_CTL register accessor: an alias for `Reg<PAD_PX01_FUNC_CTL_SPEC>`"]
pub type PAD_PX01_FUNC_CTL = crate::Reg<pad_px01_func_ctl::PAD_PX01_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_px01_func_ctl;
#[doc = "PAD_PX01_PAD_CTL register accessor: an alias for `Reg<PAD_PX01_PAD_CTL_SPEC>`"]
pub type PAD_PX01_PAD_CTL = crate::Reg<pad_px01_pad_ctl::PAD_PX01_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_px01_pad_ctl;
#[doc = "PAD_PX02_FUNC_CTL register accessor: an alias for `Reg<PAD_PX02_FUNC_CTL_SPEC>`"]
pub type PAD_PX02_FUNC_CTL = crate::Reg<pad_px02_func_ctl::PAD_PX02_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_px02_func_ctl;
#[doc = "PAD_PX02_PAD_CTL register accessor: an alias for `Reg<PAD_PX02_PAD_CTL_SPEC>`"]
pub type PAD_PX02_PAD_CTL = crate::Reg<pad_px02_pad_ctl::PAD_PX02_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_px02_pad_ctl;
#[doc = "PAD_PX03_FUNC_CTL register accessor: an alias for `Reg<PAD_PX03_FUNC_CTL_SPEC>`"]
pub type PAD_PX03_FUNC_CTL = crate::Reg<pad_px03_func_ctl::PAD_PX03_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_px03_func_ctl;
#[doc = "PAD_PX03_PAD_CTL register accessor: an alias for `Reg<PAD_PX03_PAD_CTL_SPEC>`"]
pub type PAD_PX03_PAD_CTL = crate::Reg<pad_px03_pad_ctl::PAD_PX03_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_px03_pad_ctl;
#[doc = "PAD_PX04_FUNC_CTL register accessor: an alias for `Reg<PAD_PX04_FUNC_CTL_SPEC>`"]
pub type PAD_PX04_FUNC_CTL = crate::Reg<pad_px04_func_ctl::PAD_PX04_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_px04_func_ctl;
#[doc = "PAD_PX04_PAD_CTL register accessor: an alias for `Reg<PAD_PX04_PAD_CTL_SPEC>`"]
pub type PAD_PX04_PAD_CTL = crate::Reg<pad_px04_pad_ctl::PAD_PX04_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_px04_pad_ctl;
#[doc = "PAD_PX05_FUNC_CTL register accessor: an alias for `Reg<PAD_PX05_FUNC_CTL_SPEC>`"]
pub type PAD_PX05_FUNC_CTL = crate::Reg<pad_px05_func_ctl::PAD_PX05_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_px05_func_ctl;
#[doc = "PAD_PX05_PAD_CTL register accessor: an alias for `Reg<PAD_PX05_PAD_CTL_SPEC>`"]
pub type PAD_PX05_PAD_CTL = crate::Reg<pad_px05_pad_ctl::PAD_PX05_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_px05_pad_ctl;
#[doc = "PAD_PX06_FUNC_CTL register accessor: an alias for `Reg<PAD_PX06_FUNC_CTL_SPEC>`"]
pub type PAD_PX06_FUNC_CTL = crate::Reg<pad_px06_func_ctl::PAD_PX06_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_px06_func_ctl;
#[doc = "PAD_PX06_PAD_CTL register accessor: an alias for `Reg<PAD_PX06_PAD_CTL_SPEC>`"]
pub type PAD_PX06_PAD_CTL = crate::Reg<pad_px06_pad_ctl::PAD_PX06_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_px06_pad_ctl;
#[doc = "PAD_PX07_FUNC_CTL register accessor: an alias for `Reg<PAD_PX07_FUNC_CTL_SPEC>`"]
pub type PAD_PX07_FUNC_CTL = crate::Reg<pad_px07_func_ctl::PAD_PX07_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_px07_func_ctl;
#[doc = "PAD_PX07_PAD_CTL register accessor: an alias for `Reg<PAD_PX07_PAD_CTL_SPEC>`"]
pub type PAD_PX07_PAD_CTL = crate::Reg<pad_px07_pad_ctl::PAD_PX07_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_px07_pad_ctl;
#[doc = "PAD_PX08_FUNC_CTL register accessor: an alias for `Reg<PAD_PX08_FUNC_CTL_SPEC>`"]
pub type PAD_PX08_FUNC_CTL = crate::Reg<pad_px08_func_ctl::PAD_PX08_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_px08_func_ctl;
#[doc = "PAD_PX08_PAD_CTL register accessor: an alias for `Reg<PAD_PX08_PAD_CTL_SPEC>`"]
pub type PAD_PX08_PAD_CTL = crate::Reg<pad_px08_pad_ctl::PAD_PX08_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_px08_pad_ctl;
#[doc = "PAD_PX09_FUNC_CTL register accessor: an alias for `Reg<PAD_PX09_FUNC_CTL_SPEC>`"]
pub type PAD_PX09_FUNC_CTL = crate::Reg<pad_px09_func_ctl::PAD_PX09_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_px09_func_ctl;
#[doc = "PAD_PX09_PAD_CTL register accessor: an alias for `Reg<PAD_PX09_PAD_CTL_SPEC>`"]
pub type PAD_PX09_PAD_CTL = crate::Reg<pad_px09_pad_ctl::PAD_PX09_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_px09_pad_ctl;
#[doc = "PAD_PX10_FUNC_CTL register accessor: an alias for `Reg<PAD_PX10_FUNC_CTL_SPEC>`"]
pub type PAD_PX10_FUNC_CTL = crate::Reg<pad_px10_func_ctl::PAD_PX10_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_px10_func_ctl;
#[doc = "PAD_PX10_PAD_CTL register accessor: an alias for `Reg<PAD_PX10_PAD_CTL_SPEC>`"]
pub type PAD_PX10_PAD_CTL = crate::Reg<pad_px10_pad_ctl::PAD_PX10_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_px10_pad_ctl;
#[doc = "PAD_PX11_FUNC_CTL register accessor: an alias for `Reg<PAD_PX11_FUNC_CTL_SPEC>`"]
pub type PAD_PX11_FUNC_CTL = crate::Reg<pad_px11_func_ctl::PAD_PX11_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_px11_func_ctl;
#[doc = "PAD_PX11_PAD_CTL register accessor: an alias for `Reg<PAD_PX11_PAD_CTL_SPEC>`"]
pub type PAD_PX11_PAD_CTL = crate::Reg<pad_px11_pad_ctl::PAD_PX11_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_px11_pad_ctl;
#[doc = "PAD_PY00_FUNC_CTL register accessor: an alias for `Reg<PAD_PY00_FUNC_CTL_SPEC>`"]
pub type PAD_PY00_FUNC_CTL = crate::Reg<pad_py00_func_ctl::PAD_PY00_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_py00_func_ctl;
#[doc = "PAD_PY00_PAD_CTL register accessor: an alias for `Reg<PAD_PY00_PAD_CTL_SPEC>`"]
pub type PAD_PY00_PAD_CTL = crate::Reg<pad_py00_pad_ctl::PAD_PY00_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_py00_pad_ctl;
#[doc = "PAD_PY01_FUNC_CTL register accessor: an alias for `Reg<PAD_PY01_FUNC_CTL_SPEC>`"]
pub type PAD_PY01_FUNC_CTL = crate::Reg<pad_py01_func_ctl::PAD_PY01_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_py01_func_ctl;
#[doc = "PAD_PY01_PAD_CTL register accessor: an alias for `Reg<PAD_PY01_PAD_CTL_SPEC>`"]
pub type PAD_PY01_PAD_CTL = crate::Reg<pad_py01_pad_ctl::PAD_PY01_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_py01_pad_ctl;
#[doc = "PAD_PY02_FUNC_CTL register accessor: an alias for `Reg<PAD_PY02_FUNC_CTL_SPEC>`"]
pub type PAD_PY02_FUNC_CTL = crate::Reg<pad_py02_func_ctl::PAD_PY02_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_py02_func_ctl;
#[doc = "PAD_PY02_PAD_CTL register accessor: an alias for `Reg<PAD_PY02_PAD_CTL_SPEC>`"]
pub type PAD_PY02_PAD_CTL = crate::Reg<pad_py02_pad_ctl::PAD_PY02_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_py02_pad_ctl;
#[doc = "PAD_PY03_FUNC_CTL register accessor: an alias for `Reg<PAD_PY03_FUNC_CTL_SPEC>`"]
pub type PAD_PY03_FUNC_CTL = crate::Reg<pad_py03_func_ctl::PAD_PY03_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_py03_func_ctl;
#[doc = "PAD_PY03_PAD_CTL register accessor: an alias for `Reg<PAD_PY03_PAD_CTL_SPEC>`"]
pub type PAD_PY03_PAD_CTL = crate::Reg<pad_py03_pad_ctl::PAD_PY03_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_py03_pad_ctl;
#[doc = "PAD_PY04_FUNC_CTL register accessor: an alias for `Reg<PAD_PY04_FUNC_CTL_SPEC>`"]
pub type PAD_PY04_FUNC_CTL = crate::Reg<pad_py04_func_ctl::PAD_PY04_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_py04_func_ctl;
#[doc = "PAD_PY04_PAD_CTL register accessor: an alias for `Reg<PAD_PY04_PAD_CTL_SPEC>`"]
pub type PAD_PY04_PAD_CTL = crate::Reg<pad_py04_pad_ctl::PAD_PY04_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_py04_pad_ctl;
#[doc = "PAD_PY05_FUNC_CTL register accessor: an alias for `Reg<PAD_PY05_FUNC_CTL_SPEC>`"]
pub type PAD_PY05_FUNC_CTL = crate::Reg<pad_py05_func_ctl::PAD_PY05_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_py05_func_ctl;
#[doc = "PAD_PY05_PAD_CTL register accessor: an alias for `Reg<PAD_PY05_PAD_CTL_SPEC>`"]
pub type PAD_PY05_PAD_CTL = crate::Reg<pad_py05_pad_ctl::PAD_PY05_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_py05_pad_ctl;
#[doc = "PAD_PY06_FUNC_CTL register accessor: an alias for `Reg<PAD_PY06_FUNC_CTL_SPEC>`"]
pub type PAD_PY06_FUNC_CTL = crate::Reg<pad_py06_func_ctl::PAD_PY06_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_py06_func_ctl;
#[doc = "PAD_PY06_PAD_CTL register accessor: an alias for `Reg<PAD_PY06_PAD_CTL_SPEC>`"]
pub type PAD_PY06_PAD_CTL = crate::Reg<pad_py06_pad_ctl::PAD_PY06_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_py06_pad_ctl;
#[doc = "PAD_PY07_FUNC_CTL register accessor: an alias for `Reg<PAD_PY07_FUNC_CTL_SPEC>`"]
pub type PAD_PY07_FUNC_CTL = crate::Reg<pad_py07_func_ctl::PAD_PY07_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_py07_func_ctl;
#[doc = "PAD_PY07_PAD_CTL register accessor: an alias for `Reg<PAD_PY07_PAD_CTL_SPEC>`"]
pub type PAD_PY07_PAD_CTL = crate::Reg<pad_py07_pad_ctl::PAD_PY07_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_py07_pad_ctl;
#[doc = "PAD_PY08_FUNC_CTL register accessor: an alias for `Reg<PAD_PY08_FUNC_CTL_SPEC>`"]
pub type PAD_PY08_FUNC_CTL = crate::Reg<pad_py08_func_ctl::PAD_PY08_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_py08_func_ctl;
#[doc = "PAD_PY08_PAD_CTL register accessor: an alias for `Reg<PAD_PY08_PAD_CTL_SPEC>`"]
pub type PAD_PY08_PAD_CTL = crate::Reg<pad_py08_pad_ctl::PAD_PY08_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_py08_pad_ctl;
#[doc = "PAD_PY09_FUNC_CTL register accessor: an alias for `Reg<PAD_PY09_FUNC_CTL_SPEC>`"]
pub type PAD_PY09_FUNC_CTL = crate::Reg<pad_py09_func_ctl::PAD_PY09_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_py09_func_ctl;
#[doc = "PAD_PY09_PAD_CTL register accessor: an alias for `Reg<PAD_PY09_PAD_CTL_SPEC>`"]
pub type PAD_PY09_PAD_CTL = crate::Reg<pad_py09_pad_ctl::PAD_PY09_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_py09_pad_ctl;
#[doc = "PAD_PY10_FUNC_CTL register accessor: an alias for `Reg<PAD_PY10_FUNC_CTL_SPEC>`"]
pub type PAD_PY10_FUNC_CTL = crate::Reg<pad_py10_func_ctl::PAD_PY10_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_py10_func_ctl;
#[doc = "PAD_PY10_PAD_CTL register accessor: an alias for `Reg<PAD_PY10_PAD_CTL_SPEC>`"]
pub type PAD_PY10_PAD_CTL = crate::Reg<pad_py10_pad_ctl::PAD_PY10_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_py10_pad_ctl;
#[doc = "PAD_PY11_FUNC_CTL register accessor: an alias for `Reg<PAD_PY11_FUNC_CTL_SPEC>`"]
pub type PAD_PY11_FUNC_CTL = crate::Reg<pad_py11_func_ctl::PAD_PY11_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_py11_func_ctl;
#[doc = "PAD_PY11_PAD_CTL register accessor: an alias for `Reg<PAD_PY11_PAD_CTL_SPEC>`"]
pub type PAD_PY11_PAD_CTL = crate::Reg<pad_py11_pad_ctl::PAD_PY11_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_py11_pad_ctl;
#[doc = "PAD_PZ00_FUNC_CTL register accessor: an alias for `Reg<PAD_PZ00_FUNC_CTL_SPEC>`"]
pub type PAD_PZ00_FUNC_CTL = crate::Reg<pad_pz00_func_ctl::PAD_PZ00_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pz00_func_ctl;
#[doc = "PAD_PZ00_PAD_CTL register accessor: an alias for `Reg<PAD_PZ00_PAD_CTL_SPEC>`"]
pub type PAD_PZ00_PAD_CTL = crate::Reg<pad_pz00_pad_ctl::PAD_PZ00_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pz00_pad_ctl;
#[doc = "PAD_PZ01_FUNC_CTL register accessor: an alias for `Reg<PAD_PZ01_FUNC_CTL_SPEC>`"]
pub type PAD_PZ01_FUNC_CTL = crate::Reg<pad_pz01_func_ctl::PAD_PZ01_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pz01_func_ctl;
#[doc = "PAD_PZ01_PAD_CTL register accessor: an alias for `Reg<PAD_PZ01_PAD_CTL_SPEC>`"]
pub type PAD_PZ01_PAD_CTL = crate::Reg<pad_pz01_pad_ctl::PAD_PZ01_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pz01_pad_ctl;
#[doc = "PAD_PZ02_FUNC_CTL register accessor: an alias for `Reg<PAD_PZ02_FUNC_CTL_SPEC>`"]
pub type PAD_PZ02_FUNC_CTL = crate::Reg<pad_pz02_func_ctl::PAD_PZ02_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pz02_func_ctl;
#[doc = "PAD_PZ02_PAD_CTL register accessor: an alias for `Reg<PAD_PZ02_PAD_CTL_SPEC>`"]
pub type PAD_PZ02_PAD_CTL = crate::Reg<pad_pz02_pad_ctl::PAD_PZ02_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pz02_pad_ctl;
#[doc = "PAD_PZ03_FUNC_CTL register accessor: an alias for `Reg<PAD_PZ03_FUNC_CTL_SPEC>`"]
pub type PAD_PZ03_FUNC_CTL = crate::Reg<pad_pz03_func_ctl::PAD_PZ03_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pz03_func_ctl;
#[doc = "PAD_PZ03_PAD_CTL register accessor: an alias for `Reg<PAD_PZ03_PAD_CTL_SPEC>`"]
pub type PAD_PZ03_PAD_CTL = crate::Reg<pad_pz03_pad_ctl::PAD_PZ03_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pz03_pad_ctl;
#[doc = "PAD_PZ04_FUNC_CTL register accessor: an alias for `Reg<PAD_PZ04_FUNC_CTL_SPEC>`"]
pub type PAD_PZ04_FUNC_CTL = crate::Reg<pad_pz04_func_ctl::PAD_PZ04_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pz04_func_ctl;
#[doc = "PAD_PZ04_PAD_CTL register accessor: an alias for `Reg<PAD_PZ04_PAD_CTL_SPEC>`"]
pub type PAD_PZ04_PAD_CTL = crate::Reg<pad_pz04_pad_ctl::PAD_PZ04_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pz04_pad_ctl;
#[doc = "PAD_PZ05_FUNC_CTL register accessor: an alias for `Reg<PAD_PZ05_FUNC_CTL_SPEC>`"]
pub type PAD_PZ05_FUNC_CTL = crate::Reg<pad_pz05_func_ctl::PAD_PZ05_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pz05_func_ctl;
#[doc = "PAD_PZ05_PAD_CTL register accessor: an alias for `Reg<PAD_PZ05_PAD_CTL_SPEC>`"]
pub type PAD_PZ05_PAD_CTL = crate::Reg<pad_pz05_pad_ctl::PAD_PZ05_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pz05_pad_ctl;
#[doc = "PAD_PZ06_FUNC_CTL register accessor: an alias for `Reg<PAD_PZ06_FUNC_CTL_SPEC>`"]
pub type PAD_PZ06_FUNC_CTL = crate::Reg<pad_pz06_func_ctl::PAD_PZ06_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pz06_func_ctl;
#[doc = "PAD_PZ06_PAD_CTL register accessor: an alias for `Reg<PAD_PZ06_PAD_CTL_SPEC>`"]
pub type PAD_PZ06_PAD_CTL = crate::Reg<pad_pz06_pad_ctl::PAD_PZ06_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pz06_pad_ctl;
#[doc = "PAD_PZ07_FUNC_CTL register accessor: an alias for `Reg<PAD_PZ07_FUNC_CTL_SPEC>`"]
pub type PAD_PZ07_FUNC_CTL = crate::Reg<pad_pz07_func_ctl::PAD_PZ07_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pz07_func_ctl;
#[doc = "PAD_PZ07_PAD_CTL register accessor: an alias for `Reg<PAD_PZ07_PAD_CTL_SPEC>`"]
pub type PAD_PZ07_PAD_CTL = crate::Reg<pad_pz07_pad_ctl::PAD_PZ07_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pz07_pad_ctl;
#[doc = "PAD_PZ08_FUNC_CTL register accessor: an alias for `Reg<PAD_PZ08_FUNC_CTL_SPEC>`"]
pub type PAD_PZ08_FUNC_CTL = crate::Reg<pad_pz08_func_ctl::PAD_PZ08_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pz08_func_ctl;
#[doc = "PAD_PZ08_PAD_CTL register accessor: an alias for `Reg<PAD_PZ08_PAD_CTL_SPEC>`"]
pub type PAD_PZ08_PAD_CTL = crate::Reg<pad_pz08_pad_ctl::PAD_PZ08_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pz08_pad_ctl;
#[doc = "PAD_PZ09_FUNC_CTL register accessor: an alias for `Reg<PAD_PZ09_FUNC_CTL_SPEC>`"]
pub type PAD_PZ09_FUNC_CTL = crate::Reg<pad_pz09_func_ctl::PAD_PZ09_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pz09_func_ctl;
#[doc = "PAD_PZ09_PAD_CTL register accessor: an alias for `Reg<PAD_PZ09_PAD_CTL_SPEC>`"]
pub type PAD_PZ09_PAD_CTL = crate::Reg<pad_pz09_pad_ctl::PAD_PZ09_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pz09_pad_ctl;
#[doc = "PAD_PZ10_FUNC_CTL register accessor: an alias for `Reg<PAD_PZ10_FUNC_CTL_SPEC>`"]
pub type PAD_PZ10_FUNC_CTL = crate::Reg<pad_pz10_func_ctl::PAD_PZ10_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pz10_func_ctl;
#[doc = "PAD_PZ10_PAD_CTL register accessor: an alias for `Reg<PAD_PZ10_PAD_CTL_SPEC>`"]
pub type PAD_PZ10_PAD_CTL = crate::Reg<pad_pz10_pad_ctl::PAD_PZ10_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pz10_pad_ctl;
#[doc = "PAD_PZ11_FUNC_CTL register accessor: an alias for `Reg<PAD_PZ11_FUNC_CTL_SPEC>`"]
pub type PAD_PZ11_FUNC_CTL = crate::Reg<pad_pz11_func_ctl::PAD_PZ11_FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod pad_pz11_func_ctl;
#[doc = "PAD_PZ11_PAD_CTL register accessor: an alias for `Reg<PAD_PZ11_PAD_CTL_SPEC>`"]
pub type PAD_PZ11_PAD_CTL = crate::Reg<pad_pz11_pad_ctl::PAD_PZ11_PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_pz11_pad_ctl;
