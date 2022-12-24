#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - software set symmetric key"]
    pub softmkey_sfk0: SOFTMKEY_SFK0,
    #[doc = "0x04 - software set symmetric key"]
    pub softmkey_sfk1: SOFTMKEY_SFK1,
    #[doc = "0x08 - software set symmetric key"]
    pub softmkey_sfk2: SOFTMKEY_SFK2,
    #[doc = "0x0c - software set symmetric key"]
    pub softmkey_sfk3: SOFTMKEY_SFK3,
    #[doc = "0x10 - software set symmetric key"]
    pub softmkey_sfk4: SOFTMKEY_SFK4,
    #[doc = "0x14 - software set symmetric key"]
    pub softmkey_sfk5: SOFTMKEY_SFK5,
    #[doc = "0x18 - software set symmetric key"]
    pub softmkey_sfk6: SOFTMKEY_SFK6,
    #[doc = "0x1c - software set symmetric key"]
    pub softmkey_sfk7: SOFTMKEY_SFK7,
    #[doc = "0x20 - system asymmetric key"]
    pub softpkey_spk0: SOFTPKEY_SPK0,
    #[doc = "0x24 - system asymmetric key"]
    pub softpkey_spk1: SOFTPKEY_SPK1,
    #[doc = "0x28 - system asymmetric key"]
    pub softpkey_spk2: SOFTPKEY_SPK2,
    #[doc = "0x2c - system asymmetric key"]
    pub softpkey_spk3: SOFTPKEY_SPK3,
    #[doc = "0x30 - system asymmetric key"]
    pub softpkey_spk4: SOFTPKEY_SPK4,
    #[doc = "0x34 - system asymmetric key"]
    pub softpkey_spk5: SOFTPKEY_SPK5,
    #[doc = "0x38 - system asymmetric key"]
    pub softpkey_spk6: SOFTPKEY_SPK6,
    #[doc = "0x3c - system asymmetric key"]
    pub softpkey_spk7: SOFTPKEY_SPK7,
    #[doc = "0x40 - secure key generation"]
    pub sec_key_ctl: SEC_KEY_CTL,
    #[doc = "0x44 - non-secure key generation"]
    pub nsc_key_ctl: NSC_KEY_CTL,
    #[doc = "0x48 - Random number interface behavior"]
    pub rng: RNG,
    #[doc = "0x4c - key read out control"]
    pub read_control: READ_CONTROL,
}
#[doc = "SOFTMKEY_SFK0 (rw) register accessor: an alias for `Reg<SOFTMKEY_SFK0_SPEC>`"]
pub type SOFTMKEY_SFK0 = crate::Reg<softmkey_sfk0::SOFTMKEY_SFK0_SPEC>;
#[doc = "software set symmetric key"]
pub mod softmkey_sfk0;
#[doc = "SOFTMKEY_SFK1 (rw) register accessor: an alias for `Reg<SOFTMKEY_SFK1_SPEC>`"]
pub type SOFTMKEY_SFK1 = crate::Reg<softmkey_sfk1::SOFTMKEY_SFK1_SPEC>;
#[doc = "software set symmetric key"]
pub mod softmkey_sfk1;
#[doc = "SOFTMKEY_SFK2 (rw) register accessor: an alias for `Reg<SOFTMKEY_SFK2_SPEC>`"]
pub type SOFTMKEY_SFK2 = crate::Reg<softmkey_sfk2::SOFTMKEY_SFK2_SPEC>;
#[doc = "software set symmetric key"]
pub mod softmkey_sfk2;
#[doc = "SOFTMKEY_SFK3 (rw) register accessor: an alias for `Reg<SOFTMKEY_SFK3_SPEC>`"]
pub type SOFTMKEY_SFK3 = crate::Reg<softmkey_sfk3::SOFTMKEY_SFK3_SPEC>;
#[doc = "software set symmetric key"]
pub mod softmkey_sfk3;
#[doc = "SOFTMKEY_SFK4 (rw) register accessor: an alias for `Reg<SOFTMKEY_SFK4_SPEC>`"]
pub type SOFTMKEY_SFK4 = crate::Reg<softmkey_sfk4::SOFTMKEY_SFK4_SPEC>;
#[doc = "software set symmetric key"]
pub mod softmkey_sfk4;
#[doc = "SOFTMKEY_SFK5 (rw) register accessor: an alias for `Reg<SOFTMKEY_SFK5_SPEC>`"]
pub type SOFTMKEY_SFK5 = crate::Reg<softmkey_sfk5::SOFTMKEY_SFK5_SPEC>;
#[doc = "software set symmetric key"]
pub mod softmkey_sfk5;
#[doc = "SOFTMKEY_SFK6 (rw) register accessor: an alias for `Reg<SOFTMKEY_SFK6_SPEC>`"]
pub type SOFTMKEY_SFK6 = crate::Reg<softmkey_sfk6::SOFTMKEY_SFK6_SPEC>;
#[doc = "software set symmetric key"]
pub mod softmkey_sfk6;
#[doc = "SOFTMKEY_SFK7 (rw) register accessor: an alias for `Reg<SOFTMKEY_SFK7_SPEC>`"]
pub type SOFTMKEY_SFK7 = crate::Reg<softmkey_sfk7::SOFTMKEY_SFK7_SPEC>;
#[doc = "software set symmetric key"]
pub mod softmkey_sfk7;
#[doc = "SOFTPKEY_SPK0 (rw) register accessor: an alias for `Reg<SOFTPKEY_SPK0_SPEC>`"]
pub type SOFTPKEY_SPK0 = crate::Reg<softpkey_spk0::SOFTPKEY_SPK0_SPEC>;
#[doc = "system asymmetric key"]
pub mod softpkey_spk0;
#[doc = "SOFTPKEY_SPK1 (rw) register accessor: an alias for `Reg<SOFTPKEY_SPK1_SPEC>`"]
pub type SOFTPKEY_SPK1 = crate::Reg<softpkey_spk1::SOFTPKEY_SPK1_SPEC>;
#[doc = "system asymmetric key"]
pub mod softpkey_spk1;
#[doc = "SOFTPKEY_SPK2 (rw) register accessor: an alias for `Reg<SOFTPKEY_SPK2_SPEC>`"]
pub type SOFTPKEY_SPK2 = crate::Reg<softpkey_spk2::SOFTPKEY_SPK2_SPEC>;
#[doc = "system asymmetric key"]
pub mod softpkey_spk2;
#[doc = "SOFTPKEY_SPK3 (rw) register accessor: an alias for `Reg<SOFTPKEY_SPK3_SPEC>`"]
pub type SOFTPKEY_SPK3 = crate::Reg<softpkey_spk3::SOFTPKEY_SPK3_SPEC>;
#[doc = "system asymmetric key"]
pub mod softpkey_spk3;
#[doc = "SOFTPKEY_SPK4 (rw) register accessor: an alias for `Reg<SOFTPKEY_SPK4_SPEC>`"]
pub type SOFTPKEY_SPK4 = crate::Reg<softpkey_spk4::SOFTPKEY_SPK4_SPEC>;
#[doc = "system asymmetric key"]
pub mod softpkey_spk4;
#[doc = "SOFTPKEY_SPK5 (rw) register accessor: an alias for `Reg<SOFTPKEY_SPK5_SPEC>`"]
pub type SOFTPKEY_SPK5 = crate::Reg<softpkey_spk5::SOFTPKEY_SPK5_SPEC>;
#[doc = "system asymmetric key"]
pub mod softpkey_spk5;
#[doc = "SOFTPKEY_SPK6 (rw) register accessor: an alias for `Reg<SOFTPKEY_SPK6_SPEC>`"]
pub type SOFTPKEY_SPK6 = crate::Reg<softpkey_spk6::SOFTPKEY_SPK6_SPEC>;
#[doc = "system asymmetric key"]
pub mod softpkey_spk6;
#[doc = "SOFTPKEY_SPK7 (rw) register accessor: an alias for `Reg<SOFTPKEY_SPK7_SPEC>`"]
pub type SOFTPKEY_SPK7 = crate::Reg<softpkey_spk7::SOFTPKEY_SPK7_SPEC>;
#[doc = "system asymmetric key"]
pub mod softpkey_spk7;
#[doc = "SEC_KEY_CTL (rw) register accessor: an alias for `Reg<SEC_KEY_CTL_SPEC>`"]
pub type SEC_KEY_CTL = crate::Reg<sec_key_ctl::SEC_KEY_CTL_SPEC>;
#[doc = "secure key generation"]
pub mod sec_key_ctl;
#[doc = "NSC_KEY_CTL (rw) register accessor: an alias for `Reg<NSC_KEY_CTL_SPEC>`"]
pub type NSC_KEY_CTL = crate::Reg<nsc_key_ctl::NSC_KEY_CTL_SPEC>;
#[doc = "non-secure key generation"]
pub mod nsc_key_ctl;
#[doc = "RNG (rw) register accessor: an alias for `Reg<RNG_SPEC>`"]
pub type RNG = crate::Reg<rng::RNG_SPEC>;
#[doc = "Random number interface behavior"]
pub mod rng;
#[doc = "READ_CONTROL (rw) register accessor: an alias for `Reg<READ_CONTROL_SPEC>`"]
pub type READ_CONTROL = crate::Reg<read_control::READ_CONTROL_SPEC>;
#[doc = "key read out control"]
pub mod read_control;
