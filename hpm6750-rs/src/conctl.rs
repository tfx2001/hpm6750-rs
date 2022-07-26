#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No description avaiable"]
    pub ctrl0: crate::Reg<ctrl0::CTRL0_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - No description avaiable"]
    pub ctrl2: crate::Reg<ctrl2::CTRL2_SPEC>,
    #[doc = "0x0c - No description avaiable"]
    pub ctrl3: crate::Reg<ctrl3::CTRL3_SPEC>,
    #[doc = "0x10 - No description avaiable"]
    pub ctrl4: crate::Reg<ctrl4::CTRL4_SPEC>,
    #[doc = "0x14 - No description avaiable"]
    pub ctrl5: crate::Reg<ctrl5::CTRL5_SPEC>,
}
#[doc = "CTRL0 register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "No description avaiable"]
pub mod ctrl0;
#[doc = "CTRL2 register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "No description avaiable"]
pub mod ctrl2;
#[doc = "CTRL3 register accessor: an alias for `Reg<CTRL3_SPEC>`"]
pub type CTRL3 = crate::Reg<ctrl3::CTRL3_SPEC>;
#[doc = "No description avaiable"]
pub mod ctrl3;
#[doc = "CTRL4 register accessor: an alias for `Reg<CTRL4_SPEC>`"]
pub type CTRL4 = crate::Reg<ctrl4::CTRL4_SPEC>;
#[doc = "No description avaiable"]
pub mod ctrl4;
#[doc = "CTRL5 register accessor: an alias for `Reg<CTRL5_SPEC>`"]
pub type CTRL5 = crate::Reg<ctrl5::CTRL5_SPEC>;
#[doc = "No description avaiable"]
pub mod ctrl5;
