#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Generic control"]
    pub pmic_gpr00: PMIC_GPR00,
    #[doc = "0x04 - Generic control"]
    pub pmic_gpr01: PMIC_GPR01,
    #[doc = "0x08 - Generic control"]
    pub pmic_gpr02: PMIC_GPR02,
    #[doc = "0x0c - Generic control"]
    pub pmic_gpr03: PMIC_GPR03,
    #[doc = "0x10 - Generic control"]
    pub pmic_gpr04: PMIC_GPR04,
    #[doc = "0x14 - Generic control"]
    pub pmic_gpr05: PMIC_GPR05,
    #[doc = "0x18 - Generic control"]
    pub pmic_gpr06: PMIC_GPR06,
    #[doc = "0x1c - Generic control"]
    pub pmic_gpr07: PMIC_GPR07,
    #[doc = "0x20 - Generic control"]
    pub pmic_gpr08: PMIC_GPR08,
    #[doc = "0x24 - Generic control"]
    pub pmic_gpr09: PMIC_GPR09,
    #[doc = "0x28 - Generic control"]
    pub pmic_gpr10: PMIC_GPR10,
    #[doc = "0x2c - Generic control"]
    pub pmic_gpr11: PMIC_GPR11,
    #[doc = "0x30 - Generic control"]
    pub pmic_gpr12: PMIC_GPR12,
    #[doc = "0x34 - Generic control"]
    pub pmic_gpr13: PMIC_GPR13,
    #[doc = "0x38 - Generic control"]
    pub pmic_gpr14: PMIC_GPR14,
    #[doc = "0x3c - Generic control"]
    pub pmic_gpr15: PMIC_GPR15,
}
#[doc = "PMIC_GPR00 (rw) register accessor: an alias for `Reg<PMIC_GPR00_SPEC>`"]
pub type PMIC_GPR00 = crate::Reg<pmic_gpr00::PMIC_GPR00_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr00;
#[doc = "PMIC_GPR01 (rw) register accessor: an alias for `Reg<PMIC_GPR01_SPEC>`"]
pub type PMIC_GPR01 = crate::Reg<pmic_gpr01::PMIC_GPR01_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr01;
#[doc = "PMIC_GPR02 (rw) register accessor: an alias for `Reg<PMIC_GPR02_SPEC>`"]
pub type PMIC_GPR02 = crate::Reg<pmic_gpr02::PMIC_GPR02_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr02;
#[doc = "PMIC_GPR03 (rw) register accessor: an alias for `Reg<PMIC_GPR03_SPEC>`"]
pub type PMIC_GPR03 = crate::Reg<pmic_gpr03::PMIC_GPR03_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr03;
#[doc = "PMIC_GPR04 (rw) register accessor: an alias for `Reg<PMIC_GPR04_SPEC>`"]
pub type PMIC_GPR04 = crate::Reg<pmic_gpr04::PMIC_GPR04_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr04;
#[doc = "PMIC_GPR05 (rw) register accessor: an alias for `Reg<PMIC_GPR05_SPEC>`"]
pub type PMIC_GPR05 = crate::Reg<pmic_gpr05::PMIC_GPR05_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr05;
#[doc = "PMIC_GPR06 (rw) register accessor: an alias for `Reg<PMIC_GPR06_SPEC>`"]
pub type PMIC_GPR06 = crate::Reg<pmic_gpr06::PMIC_GPR06_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr06;
#[doc = "PMIC_GPR07 (rw) register accessor: an alias for `Reg<PMIC_GPR07_SPEC>`"]
pub type PMIC_GPR07 = crate::Reg<pmic_gpr07::PMIC_GPR07_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr07;
#[doc = "PMIC_GPR08 (rw) register accessor: an alias for `Reg<PMIC_GPR08_SPEC>`"]
pub type PMIC_GPR08 = crate::Reg<pmic_gpr08::PMIC_GPR08_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr08;
#[doc = "PMIC_GPR09 (rw) register accessor: an alias for `Reg<PMIC_GPR09_SPEC>`"]
pub type PMIC_GPR09 = crate::Reg<pmic_gpr09::PMIC_GPR09_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr09;
#[doc = "PMIC_GPR10 (rw) register accessor: an alias for `Reg<PMIC_GPR10_SPEC>`"]
pub type PMIC_GPR10 = crate::Reg<pmic_gpr10::PMIC_GPR10_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr10;
#[doc = "PMIC_GPR11 (rw) register accessor: an alias for `Reg<PMIC_GPR11_SPEC>`"]
pub type PMIC_GPR11 = crate::Reg<pmic_gpr11::PMIC_GPR11_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr11;
#[doc = "PMIC_GPR12 (rw) register accessor: an alias for `Reg<PMIC_GPR12_SPEC>`"]
pub type PMIC_GPR12 = crate::Reg<pmic_gpr12::PMIC_GPR12_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr12;
#[doc = "PMIC_GPR13 (rw) register accessor: an alias for `Reg<PMIC_GPR13_SPEC>`"]
pub type PMIC_GPR13 = crate::Reg<pmic_gpr13::PMIC_GPR13_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr13;
#[doc = "PMIC_GPR14 (rw) register accessor: an alias for `Reg<PMIC_GPR14_SPEC>`"]
pub type PMIC_GPR14 = crate::Reg<pmic_gpr14::PMIC_GPR14_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr14;
#[doc = "PMIC_GPR15 (rw) register accessor: an alias for `Reg<PMIC_GPR15_SPEC>`"]
pub type PMIC_GPR15 = crate::Reg<pmic_gpr15::PMIC_GPR15_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr15;
