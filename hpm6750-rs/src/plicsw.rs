#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    #[doc = "0x1000 - Pending status"]
    pub pending: PENDING,
    _reserved1: [u8; 0x0ffc],
    #[doc = "0x2000 - Interrupt enable"]
    pub inten: INTEN,
    _reserved2: [u8; 0x001f_e000],
    #[doc = "0x200004 - Claim and complete."]
    pub claim: CLAIM,
}
#[doc = "PENDING (rw) register accessor: an alias for `Reg<PENDING_SPEC>`"]
pub type PENDING = crate::Reg<pending::PENDING_SPEC>;
#[doc = "Pending status"]
pub mod pending;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt enable"]
pub mod inten;
#[doc = "CLAIM (rw) register accessor: an alias for `Reg<CLAIM_SPEC>`"]
pub type CLAIM = crate::Reg<claim::CLAIM_SPEC>;
#[doc = "Claim and complete."]
pub mod claim;
