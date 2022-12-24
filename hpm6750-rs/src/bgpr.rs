#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Generic control"]
    pub batt_gpr0: BATT_GPR0,
    #[doc = "0x04 - Generic control"]
    pub batt_gpr1: BATT_GPR1,
    #[doc = "0x08 - Generic control"]
    pub batt_gpr2: BATT_GPR2,
    #[doc = "0x0c - Generic control"]
    pub batt_gpr3: BATT_GPR3,
    #[doc = "0x10 - Generic control"]
    pub batt_gpr4: BATT_GPR4,
    #[doc = "0x14 - Generic control"]
    pub batt_gpr5: BATT_GPR5,
    #[doc = "0x18 - Generic control"]
    pub batt_gpr6: BATT_GPR6,
    #[doc = "0x1c - Generic control"]
    pub batt_gpr7: BATT_GPR7,
}
#[doc = "BATT_GPR0 (rw) register accessor: an alias for `Reg<BATT_GPR0_SPEC>`"]
pub type BATT_GPR0 = crate::Reg<batt_gpr0::BATT_GPR0_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr0;
#[doc = "BATT_GPR1 (rw) register accessor: an alias for `Reg<BATT_GPR1_SPEC>`"]
pub type BATT_GPR1 = crate::Reg<batt_gpr1::BATT_GPR1_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr1;
#[doc = "BATT_GPR2 (rw) register accessor: an alias for `Reg<BATT_GPR2_SPEC>`"]
pub type BATT_GPR2 = crate::Reg<batt_gpr2::BATT_GPR2_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr2;
#[doc = "BATT_GPR3 (rw) register accessor: an alias for `Reg<BATT_GPR3_SPEC>`"]
pub type BATT_GPR3 = crate::Reg<batt_gpr3::BATT_GPR3_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr3;
#[doc = "BATT_GPR4 (rw) register accessor: an alias for `Reg<BATT_GPR4_SPEC>`"]
pub type BATT_GPR4 = crate::Reg<batt_gpr4::BATT_GPR4_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr4;
#[doc = "BATT_GPR5 (rw) register accessor: an alias for `Reg<BATT_GPR5_SPEC>`"]
pub type BATT_GPR5 = crate::Reg<batt_gpr5::BATT_GPR5_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr5;
#[doc = "BATT_GPR6 (rw) register accessor: an alias for `Reg<BATT_GPR6_SPEC>`"]
pub type BATT_GPR6 = crate::Reg<batt_gpr6::BATT_GPR6_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr6;
#[doc = "BATT_GPR7 (rw) register accessor: an alias for `Reg<BATT_GPR7_SPEC>`"]
pub type BATT_GPR7 = crate::Reg<batt_gpr7::BATT_GPR7_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr7;
