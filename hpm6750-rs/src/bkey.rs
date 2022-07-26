#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key data"]
    pub key_0_data_0: crate::Reg<key_0_data_0::KEY_0_DATA_0_SPEC>,
    #[doc = "0x04 - Key data"]
    pub key_0_data_1: crate::Reg<key_0_data_1::KEY_0_DATA_1_SPEC>,
    #[doc = "0x08 - Key data"]
    pub key_0_data_2: crate::Reg<key_0_data_2::KEY_0_DATA_2_SPEC>,
    #[doc = "0x0c - Key data"]
    pub key_0_data_3: crate::Reg<key_0_data_3::KEY_0_DATA_3_SPEC>,
    #[doc = "0x10 - Key data"]
    pub key_0_data_4: crate::Reg<key_0_data_4::KEY_0_DATA_4_SPEC>,
    #[doc = "0x14 - Key data"]
    pub key_0_data_5: crate::Reg<key_0_data_5::KEY_0_DATA_5_SPEC>,
    #[doc = "0x18 - Key data"]
    pub key_0_data_6: crate::Reg<key_0_data_6::KEY_0_DATA_6_SPEC>,
    #[doc = "0x1c - Key data"]
    pub key_0_data_7: crate::Reg<key_0_data_7::KEY_0_DATA_7_SPEC>,
    #[doc = "0x20 - Key data"]
    pub key_1_data_0: crate::Reg<key_1_data_0::KEY_1_DATA_0_SPEC>,
    #[doc = "0x24 - Key data"]
    pub key_1_data_1: crate::Reg<key_1_data_1::KEY_1_DATA_1_SPEC>,
    #[doc = "0x28 - Key data"]
    pub key_1_data_2: crate::Reg<key_1_data_2::KEY_1_DATA_2_SPEC>,
    #[doc = "0x2c - Key data"]
    pub key_1_data_3: crate::Reg<key_1_data_3::KEY_1_DATA_3_SPEC>,
    #[doc = "0x30 - Key data"]
    pub key_1_data_4: crate::Reg<key_1_data_4::KEY_1_DATA_4_SPEC>,
    #[doc = "0x34 - Key data"]
    pub key_1_data_5: crate::Reg<key_1_data_5::KEY_1_DATA_5_SPEC>,
    #[doc = "0x38 - Key data"]
    pub key_1_data_6: crate::Reg<key_1_data_6::KEY_1_DATA_6_SPEC>,
    #[doc = "0x3c - Key data"]
    pub key_1_data_7: crate::Reg<key_1_data_7::KEY_1_DATA_7_SPEC>,
    #[doc = "0x40 - Key ECC and access control"]
    pub ecc_key0: crate::Reg<ecc_key0::ECC_KEY0_SPEC>,
    #[doc = "0x44 - Key 1 ECC and access control"]
    pub ecc_key1: crate::Reg<ecc_key1::ECC_KEY1_SPEC>,
    #[doc = "0x48 - Key selection"]
    pub select: crate::Reg<select::SELECT_SPEC>,
}
#[doc = "KEY_0_DATA_0 register accessor: an alias for `Reg<KEY_0_DATA_0_SPEC>`"]
pub type KEY_0_DATA_0 = crate::Reg<key_0_data_0::KEY_0_DATA_0_SPEC>;
#[doc = "Key data"]
pub mod key_0_data_0;
#[doc = "KEY_0_DATA_1 register accessor: an alias for `Reg<KEY_0_DATA_1_SPEC>`"]
pub type KEY_0_DATA_1 = crate::Reg<key_0_data_1::KEY_0_DATA_1_SPEC>;
#[doc = "Key data"]
pub mod key_0_data_1;
#[doc = "KEY_0_DATA_2 register accessor: an alias for `Reg<KEY_0_DATA_2_SPEC>`"]
pub type KEY_0_DATA_2 = crate::Reg<key_0_data_2::KEY_0_DATA_2_SPEC>;
#[doc = "Key data"]
pub mod key_0_data_2;
#[doc = "KEY_0_DATA_3 register accessor: an alias for `Reg<KEY_0_DATA_3_SPEC>`"]
pub type KEY_0_DATA_3 = crate::Reg<key_0_data_3::KEY_0_DATA_3_SPEC>;
#[doc = "Key data"]
pub mod key_0_data_3;
#[doc = "KEY_0_DATA_4 register accessor: an alias for `Reg<KEY_0_DATA_4_SPEC>`"]
pub type KEY_0_DATA_4 = crate::Reg<key_0_data_4::KEY_0_DATA_4_SPEC>;
#[doc = "Key data"]
pub mod key_0_data_4;
#[doc = "KEY_0_DATA_5 register accessor: an alias for `Reg<KEY_0_DATA_5_SPEC>`"]
pub type KEY_0_DATA_5 = crate::Reg<key_0_data_5::KEY_0_DATA_5_SPEC>;
#[doc = "Key data"]
pub mod key_0_data_5;
#[doc = "KEY_0_DATA_6 register accessor: an alias for `Reg<KEY_0_DATA_6_SPEC>`"]
pub type KEY_0_DATA_6 = crate::Reg<key_0_data_6::KEY_0_DATA_6_SPEC>;
#[doc = "Key data"]
pub mod key_0_data_6;
#[doc = "KEY_0_DATA_7 register accessor: an alias for `Reg<KEY_0_DATA_7_SPEC>`"]
pub type KEY_0_DATA_7 = crate::Reg<key_0_data_7::KEY_0_DATA_7_SPEC>;
#[doc = "Key data"]
pub mod key_0_data_7;
#[doc = "KEY_1_DATA_0 register accessor: an alias for `Reg<KEY_1_DATA_0_SPEC>`"]
pub type KEY_1_DATA_0 = crate::Reg<key_1_data_0::KEY_1_DATA_0_SPEC>;
#[doc = "Key data"]
pub mod key_1_data_0;
#[doc = "KEY_1_DATA_1 register accessor: an alias for `Reg<KEY_1_DATA_1_SPEC>`"]
pub type KEY_1_DATA_1 = crate::Reg<key_1_data_1::KEY_1_DATA_1_SPEC>;
#[doc = "Key data"]
pub mod key_1_data_1;
#[doc = "KEY_1_DATA_2 register accessor: an alias for `Reg<KEY_1_DATA_2_SPEC>`"]
pub type KEY_1_DATA_2 = crate::Reg<key_1_data_2::KEY_1_DATA_2_SPEC>;
#[doc = "Key data"]
pub mod key_1_data_2;
#[doc = "KEY_1_DATA_3 register accessor: an alias for `Reg<KEY_1_DATA_3_SPEC>`"]
pub type KEY_1_DATA_3 = crate::Reg<key_1_data_3::KEY_1_DATA_3_SPEC>;
#[doc = "Key data"]
pub mod key_1_data_3;
#[doc = "KEY_1_DATA_4 register accessor: an alias for `Reg<KEY_1_DATA_4_SPEC>`"]
pub type KEY_1_DATA_4 = crate::Reg<key_1_data_4::KEY_1_DATA_4_SPEC>;
#[doc = "Key data"]
pub mod key_1_data_4;
#[doc = "KEY_1_DATA_5 register accessor: an alias for `Reg<KEY_1_DATA_5_SPEC>`"]
pub type KEY_1_DATA_5 = crate::Reg<key_1_data_5::KEY_1_DATA_5_SPEC>;
#[doc = "Key data"]
pub mod key_1_data_5;
#[doc = "KEY_1_DATA_6 register accessor: an alias for `Reg<KEY_1_DATA_6_SPEC>`"]
pub type KEY_1_DATA_6 = crate::Reg<key_1_data_6::KEY_1_DATA_6_SPEC>;
#[doc = "Key data"]
pub mod key_1_data_6;
#[doc = "KEY_1_DATA_7 register accessor: an alias for `Reg<KEY_1_DATA_7_SPEC>`"]
pub type KEY_1_DATA_7 = crate::Reg<key_1_data_7::KEY_1_DATA_7_SPEC>;
#[doc = "Key data"]
pub mod key_1_data_7;
#[doc = "ECC_KEY0 register accessor: an alias for `Reg<ECC_KEY0_SPEC>`"]
pub type ECC_KEY0 = crate::Reg<ecc_key0::ECC_KEY0_SPEC>;
#[doc = "Key ECC and access control"]
pub mod ecc_key0;
#[doc = "ECC_KEY1 register accessor: an alias for `Reg<ECC_KEY1_SPEC>`"]
pub type ECC_KEY1 = crate::Reg<ecc_key1::ECC_KEY1_SPEC>;
#[doc = "Key 1 ECC and access control"]
pub mod ecc_key1;
#[doc = "SELECT register accessor: an alias for `Reg<SELECT_SPEC>`"]
pub type SELECT = crate::Reg<select::SELECT_SPEC>;
#[doc = "Key selection"]
pub mod select;
