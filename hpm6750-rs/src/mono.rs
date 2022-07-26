#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low part of monotonic counter"]
    pub monol: crate::Reg<monol::MONOL_SPEC>,
    #[doc = "0x04 - High part of monotonic counter"]
    pub monoh: crate::Reg<monoh::MONOH_SPEC>,
}
#[doc = "MONOL register accessor: an alias for `Reg<MONOL_SPEC>`"]
pub type MONOL = crate::Reg<monol::MONOL_SPEC>;
#[doc = "Low part of monotonic counter"]
pub mod monol;
#[doc = "MONOH register accessor: an alias for `Reg<MONOH_SPEC>`"]
pub type MONOH = crate::Reg<monoh::MONOH_SPEC>;
#[doc = "High part of monotonic counter"]
pub mod monoh;
