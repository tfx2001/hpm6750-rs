#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Command Registers"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Status Registers"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x08 - Transmit word message to other core."]
    pub txreg: crate::Reg<txreg::TXREG_SPEC>,
    #[doc = "0x0c - Receive word message from other core."]
    pub rxreg: crate::Reg<rxreg::RXREG_SPEC>,
    #[doc = "0x10 - TXFIFO for sending message to other core"]
    pub txwrd_txfifo0: crate::Reg<txwrd_txfifo0::TXWRD_TXFIFO0_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - RXFIFO for receiving message from other core"]
    pub rxwrd_rxfifo0: crate::Reg<rxwrd_rxfifo0::RXWRD_RXFIFO0_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Command Registers"]
pub mod cr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Registers"]
pub mod sr;
#[doc = "TXREG register accessor: an alias for `Reg<TXREG_SPEC>`"]
pub type TXREG = crate::Reg<txreg::TXREG_SPEC>;
#[doc = "Transmit word message to other core."]
pub mod txreg;
#[doc = "RXREG register accessor: an alias for `Reg<RXREG_SPEC>`"]
pub type RXREG = crate::Reg<rxreg::RXREG_SPEC>;
#[doc = "Receive word message from other core."]
pub mod rxreg;
#[doc = "TXWRD_TXFIFO0 register accessor: an alias for `Reg<TXWRD_TXFIFO0_SPEC>`"]
pub type TXWRD_TXFIFO0 = crate::Reg<txwrd_txfifo0::TXWRD_TXFIFO0_SPEC>;
#[doc = "TXFIFO for sending message to other core"]
pub mod txwrd_txfifo0;
#[doc = "RXWRD_RXFIFO0 register accessor: an alias for `Reg<RXWRD_RXFIFO0_SPEC>`"]
pub type RXWRD_RXFIFO0 = crate::Reg<rxwrd_rxfifo0::RXWRD_RXFIFO0_SPEC>;
#[doc = "RXFIFO for receiving message from other core"]
pub mod rxwrd_rxfifo0;
