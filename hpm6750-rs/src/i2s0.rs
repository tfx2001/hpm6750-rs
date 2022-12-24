#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Rx FIFO Filling Level"]
    pub rfifo_fillings: RFIFO_FILLINGS,
    #[doc = "0x08 - Tx FIFO Filling Level"]
    pub tfifo_fillings: TFIFO_FILLINGS,
    #[doc = "0x0c - TX/RX FIFO Threshold setting."]
    pub fifo_thresh: FIFO_THRESH,
    #[doc = "0x10 - Status Registers"]
    pub sta: STA,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - Rx Data0"]
    pub rxd_data0: RXD_DATA0,
    #[doc = "0x24 - Rx Data1"]
    pub rxd_data1: RXD_DATA1,
    #[doc = "0x28 - Rx Data2"]
    pub rxd_data2: RXD_DATA2,
    #[doc = "0x2c - Rx Data3"]
    pub rxd_data3: RXD_DATA3,
    #[doc = "0x30 - Tx Data0"]
    pub txd_data0: TXD_DATA0,
    #[doc = "0x34 - Tx Data1"]
    pub txd_data1: TXD_DATA1,
    #[doc = "0x38 - Tx Data2"]
    pub txd_data2: TXD_DATA2,
    #[doc = "0x3c - Tx Data3"]
    pub txd_data3: TXD_DATA3,
    _reserved13: [u8; 0x10],
    #[doc = "0x50 - Configruation Regsiters"]
    pub cfgr: CFGR,
    _reserved14: [u8; 0x04],
    #[doc = "0x58 - Misc configuration Registers"]
    pub misc_cfgr: MISC_CFGR,
    _reserved15: [u8; 0x04],
    #[doc = "0x60 - Rx Slots Enable for Rx Data0"]
    pub rxdslot_data0: RXDSLOT_DATA0,
    #[doc = "0x64 - Rx Slots Enable for Rx Data1"]
    pub rxdslot_data1: RXDSLOT_DATA1,
    #[doc = "0x68 - Rx Slots Enable for Rx Data2"]
    pub rxdslot_data2: RXDSLOT_DATA2,
    #[doc = "0x6c - Rx Slots Enable for Rx Data3"]
    pub rxdslot_data3: RXDSLOT_DATA3,
    #[doc = "0x70 - Tx Slots Enable for Tx Data0."]
    pub txdslot_data0: TXDSLOT_DATA0,
    #[doc = "0x74 - Tx Slots Enable for Tx Data1."]
    pub txdslot_data1: TXDSLOT_DATA1,
    #[doc = "0x78 - Tx Slots Enable for Tx Data2."]
    pub txdslot_data2: TXDSLOT_DATA2,
    #[doc = "0x7c - Tx Slots Enable for Tx Data3."]
    pub txdslot_data3: TXDSLOT_DATA3,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "RFIFO_FILLINGS (r) register accessor: an alias for `Reg<RFIFO_FILLINGS_SPEC>`"]
pub type RFIFO_FILLINGS = crate::Reg<rfifo_fillings::RFIFO_FILLINGS_SPEC>;
#[doc = "Rx FIFO Filling Level"]
pub mod rfifo_fillings;
#[doc = "TFIFO_FILLINGS (r) register accessor: an alias for `Reg<TFIFO_FILLINGS_SPEC>`"]
pub type TFIFO_FILLINGS = crate::Reg<tfifo_fillings::TFIFO_FILLINGS_SPEC>;
#[doc = "Tx FIFO Filling Level"]
pub mod tfifo_fillings;
#[doc = "FIFO_THRESH (rw) register accessor: an alias for `Reg<FIFO_THRESH_SPEC>`"]
pub type FIFO_THRESH = crate::Reg<fifo_thresh::FIFO_THRESH_SPEC>;
#[doc = "TX/RX FIFO Threshold setting."]
pub mod fifo_thresh;
#[doc = "STA (rw) register accessor: an alias for `Reg<STA_SPEC>`"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "Status Registers"]
pub mod sta;
#[doc = "RXD_DATA0 (r) register accessor: an alias for `Reg<RXD_DATA0_SPEC>`"]
pub type RXD_DATA0 = crate::Reg<rxd_data0::RXD_DATA0_SPEC>;
#[doc = "Rx Data0"]
pub mod rxd_data0;
#[doc = "RXD_DATA1 (r) register accessor: an alias for `Reg<RXD_DATA1_SPEC>`"]
pub type RXD_DATA1 = crate::Reg<rxd_data1::RXD_DATA1_SPEC>;
#[doc = "Rx Data1"]
pub mod rxd_data1;
#[doc = "RXD_DATA2 (r) register accessor: an alias for `Reg<RXD_DATA2_SPEC>`"]
pub type RXD_DATA2 = crate::Reg<rxd_data2::RXD_DATA2_SPEC>;
#[doc = "Rx Data2"]
pub mod rxd_data2;
#[doc = "RXD_DATA3 (r) register accessor: an alias for `Reg<RXD_DATA3_SPEC>`"]
pub type RXD_DATA3 = crate::Reg<rxd_data3::RXD_DATA3_SPEC>;
#[doc = "Rx Data3"]
pub mod rxd_data3;
#[doc = "TXD_DATA0 (w) register accessor: an alias for `Reg<TXD_DATA0_SPEC>`"]
pub type TXD_DATA0 = crate::Reg<txd_data0::TXD_DATA0_SPEC>;
#[doc = "Tx Data0"]
pub mod txd_data0;
#[doc = "TXD_DATA1 (w) register accessor: an alias for `Reg<TXD_DATA1_SPEC>`"]
pub type TXD_DATA1 = crate::Reg<txd_data1::TXD_DATA1_SPEC>;
#[doc = "Tx Data1"]
pub mod txd_data1;
#[doc = "TXD_DATA2 (w) register accessor: an alias for `Reg<TXD_DATA2_SPEC>`"]
pub type TXD_DATA2 = crate::Reg<txd_data2::TXD_DATA2_SPEC>;
#[doc = "Tx Data2"]
pub mod txd_data2;
#[doc = "TXD_DATA3 (w) register accessor: an alias for `Reg<TXD_DATA3_SPEC>`"]
pub type TXD_DATA3 = crate::Reg<txd_data3::TXD_DATA3_SPEC>;
#[doc = "Tx Data3"]
pub mod txd_data3;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Configruation Regsiters"]
pub mod cfgr;
#[doc = "MISC_CFGR (rw) register accessor: an alias for `Reg<MISC_CFGR_SPEC>`"]
pub type MISC_CFGR = crate::Reg<misc_cfgr::MISC_CFGR_SPEC>;
#[doc = "Misc configuration Registers"]
pub mod misc_cfgr;
#[doc = "RXDSLOT_DATA0 (rw) register accessor: an alias for `Reg<RXDSLOT_DATA0_SPEC>`"]
pub type RXDSLOT_DATA0 = crate::Reg<rxdslot_data0::RXDSLOT_DATA0_SPEC>;
#[doc = "Rx Slots Enable for Rx Data0"]
pub mod rxdslot_data0;
#[doc = "RXDSLOT_DATA1 (rw) register accessor: an alias for `Reg<RXDSLOT_DATA1_SPEC>`"]
pub type RXDSLOT_DATA1 = crate::Reg<rxdslot_data1::RXDSLOT_DATA1_SPEC>;
#[doc = "Rx Slots Enable for Rx Data1"]
pub mod rxdslot_data1;
#[doc = "RXDSLOT_DATA2 (rw) register accessor: an alias for `Reg<RXDSLOT_DATA2_SPEC>`"]
pub type RXDSLOT_DATA2 = crate::Reg<rxdslot_data2::RXDSLOT_DATA2_SPEC>;
#[doc = "Rx Slots Enable for Rx Data2"]
pub mod rxdslot_data2;
#[doc = "RXDSLOT_DATA3 (rw) register accessor: an alias for `Reg<RXDSLOT_DATA3_SPEC>`"]
pub type RXDSLOT_DATA3 = crate::Reg<rxdslot_data3::RXDSLOT_DATA3_SPEC>;
#[doc = "Rx Slots Enable for Rx Data3"]
pub mod rxdslot_data3;
#[doc = "TXDSLOT_DATA0 (rw) register accessor: an alias for `Reg<TXDSLOT_DATA0_SPEC>`"]
pub type TXDSLOT_DATA0 = crate::Reg<txdslot_data0::TXDSLOT_DATA0_SPEC>;
#[doc = "Tx Slots Enable for Tx Data0."]
pub mod txdslot_data0;
#[doc = "TXDSLOT_DATA1 (rw) register accessor: an alias for `Reg<TXDSLOT_DATA1_SPEC>`"]
pub type TXDSLOT_DATA1 = crate::Reg<txdslot_data1::TXDSLOT_DATA1_SPEC>;
#[doc = "Tx Slots Enable for Tx Data1."]
pub mod txdslot_data1;
#[doc = "TXDSLOT_DATA2 (rw) register accessor: an alias for `Reg<TXDSLOT_DATA2_SPEC>`"]
pub type TXDSLOT_DATA2 = crate::Reg<txdslot_data2::TXDSLOT_DATA2_SPEC>;
#[doc = "Tx Slots Enable for Tx Data2."]
pub mod txdslot_data2;
#[doc = "TXDSLOT_DATA3 (rw) register accessor: an alias for `Reg<TXDSLOT_DATA3_SPEC>`"]
pub type TXDSLOT_DATA3 = crate::Reg<txdslot_data3::TXDSLOT_DATA3_SPEC>;
#[doc = "Tx Slots Enable for Tx Data3."]
pub mod txdslot_data3;
