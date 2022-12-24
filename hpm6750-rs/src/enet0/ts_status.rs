#[doc = "Register `TS_STATUS` reader"]
pub struct R(crate::R<TS_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TS_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TS_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TS_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSSOVF` reader - No description avaiable"]
pub type TSSOVF_R = crate::BitReader<bool>;
#[doc = "Field `TSTARGT` reader - No description avaiable"]
pub type TSTARGT_R = crate::BitReader<bool>;
#[doc = "Field `AUXTSTRIG` reader - No description avaiable"]
pub type AUXTSTRIG_R = crate::BitReader<bool>;
#[doc = "Field `TSTRGTERR` reader - No description avaiable"]
pub type TSTRGTERR_R = crate::BitReader<bool>;
#[doc = "Field `TSTARGT1` reader - No description avaiable"]
pub type TSTARGT1_R = crate::BitReader<bool>;
#[doc = "Field `TSTRGTERR1` reader - No description avaiable"]
pub type TSTRGTERR1_R = crate::BitReader<bool>;
#[doc = "Field `TSTARGT2` reader - No description avaiable"]
pub type TSTARGT2_R = crate::BitReader<bool>;
#[doc = "Field `TSTRGTERR2` reader - No description avaiable"]
pub type TSTRGTERR2_R = crate::BitReader<bool>;
#[doc = "Field `TSTARGT3` reader - Timestamp Target Time Reached for Target Time PPS3 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 496 (PPS3 Target Time High Register) and Register 497 (PPS3 Target Time Low Register)."]
pub type TSTARGT3_R = crate::BitReader<bool>;
#[doc = "Field `TSTRGTERR3` reader - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 496 and Register 497, is already elapsed. This bit is cleared when read by the application."]
pub type TSTRGTERR3_R = crate::BitReader<bool>;
#[doc = "Field `ATSSTN` reader - Auxiliary Timestamp Snapshot Trigger Identifier These bits identify the Auxiliary trigger inputs for which the timestamp available in the Auxiliary Snapshot Register is applicable. When more than one bit is set at the same time, it means that corresponding auxiliary triggers were sampled at the same clock. These bits are applicable only if the number of Auxiliary snapshots is more than one. One bit is assigned for each trigger as shown in the following list: - Bit 16: Auxiliary trigger 0 - Bit 17: Auxiliary trigger 1 - Bit 18: Auxiliary trigger 2 - Bit 19: Auxiliary trigger 3 The software can read this register to find the triggers that are set when the timestamp is taken."]
pub type ATSSTN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATSSTM` reader - Auxiliary Timestamp Snapshot Trigger Missed This bit is set when the Auxiliary timestamp snapshot FIFO is full and external trigger was set. This indicates that the latest snapshot is not stored in the FIFO. This bit is valid only if the Add IEEE 1588 Auxiliary Snapshot option is selected during core configuration."]
pub type ATSSTM_R = crate::BitReader<bool>;
#[doc = "Field `ATSNS` reader - Number of Auxiliary Timestamp Snapshots This field indicates the number of Snapshots available in the FIFO. A value equal to the selected depth of FIFO (4, 8, or 16) indicates that the Auxiliary Snapshot FIFO is full. These bits are cleared (to 00000) when the Auxiliary snapshot FIFO clear bit is set. This bit is valid only if the Add IEEE 1588 Auxiliary Snapshot option is selected during core configuration."]
pub type ATSNS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn tstargt(&self) -> TSTARGT_R {
        TSTARGT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn auxtstrig(&self) -> AUXTSTRIG_R {
        AUXTSTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No description avaiable"]
    #[inline(always)]
    pub fn tstrgterr(&self) -> TSTRGTERR_R {
        TSTRGTERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No description avaiable"]
    #[inline(always)]
    pub fn tstargt1(&self) -> TSTARGT1_R {
        TSTARGT1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No description avaiable"]
    #[inline(always)]
    pub fn tstrgterr1(&self) -> TSTRGTERR1_R {
        TSTRGTERR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - No description avaiable"]
    #[inline(always)]
    pub fn tstargt2(&self) -> TSTARGT2_R {
        TSTARGT2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No description avaiable"]
    #[inline(always)]
    pub fn tstrgterr2(&self) -> TSTRGTERR2_R {
        TSTRGTERR2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timestamp Target Time Reached for Target Time PPS3 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 496 (PPS3 Target Time High Register) and Register 497 (PPS3 Target Time Low Register)."]
    #[inline(always)]
    pub fn tstargt3(&self) -> TSTARGT3_R {
        TSTARGT3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 496 and Register 497, is already elapsed. This bit is cleared when read by the application."]
    #[inline(always)]
    pub fn tstrgterr3(&self) -> TSTRGTERR3_R {
        TSTRGTERR3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Auxiliary Timestamp Snapshot Trigger Identifier These bits identify the Auxiliary trigger inputs for which the timestamp available in the Auxiliary Snapshot Register is applicable. When more than one bit is set at the same time, it means that corresponding auxiliary triggers were sampled at the same clock. These bits are applicable only if the number of Auxiliary snapshots is more than one. One bit is assigned for each trigger as shown in the following list: - Bit 16: Auxiliary trigger 0 - Bit 17: Auxiliary trigger 1 - Bit 18: Auxiliary trigger 2 - Bit 19: Auxiliary trigger 3 The software can read this register to find the triggers that are set when the timestamp is taken."]
    #[inline(always)]
    pub fn atsstn(&self) -> ATSSTN_R {
        ATSSTN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Auxiliary Timestamp Snapshot Trigger Missed This bit is set when the Auxiliary timestamp snapshot FIFO is full and external trigger was set. This indicates that the latest snapshot is not stored in the FIFO. This bit is valid only if the Add IEEE 1588 Auxiliary Snapshot option is selected during core configuration."]
    #[inline(always)]
    pub fn atsstm(&self) -> ATSSTM_R {
        ATSSTM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Number of Auxiliary Timestamp Snapshots This field indicates the number of Snapshots available in the FIFO. A value equal to the selected depth of FIFO (4, 8, or 16) indicates that the Auxiliary Snapshot FIFO is full. These bits are cleared (to 00000) when the Auxiliary snapshot FIFO clear bit is set. This bit is valid only if the Add IEEE 1588 Auxiliary Snapshot option is selected during core configuration."]
    #[inline(always)]
    pub fn atsns(&self) -> ATSNS_R {
        ATSNS_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
#[doc = "Timestamp Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ts_status](index.html) module"]
pub struct TS_STATUS_SPEC;
impl crate::RegisterSpec for TS_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ts_status::R](R) reader structure"]
impl crate::Readable for TS_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TS_STATUS to value 0"]
impl crate::Resettable for TS_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
