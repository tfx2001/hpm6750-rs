#[doc = "Register `INTR_STATUS` reader"]
pub struct R(crate::R<INTR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIIS` reader - GPI Interrupt Status When the GPIO feature is enabled, this bit is set when any active event (LL or LH) occurs on the GPIS field (Bits \\[3:0\\]) of Register 56 (General Purpose IO Register) and the corresponding GPIE bit is enabled. This bit is cleared on reading lane 0 (GPIS) of Register 56 (General Purpose IO Register). When the GPIO feature is not enabled, this bit is reserved."]
pub type GPIIS_R = crate::BitReader<bool>;
#[doc = "Field `LPIIS` reader - LPI Interrupt Status When the Energy Efficient Ethernet feature is enabled, this bit is set for any LPI state entry or exit in the MAC Transmitter or Receiver. This bit is cleared on reading Bit 0 of Register 12 (LPI Control and Status Register). In all other modes, this bit is reserved."]
pub type LPIIS_R = crate::BitReader<bool>;
#[doc = "Field `TSIS` reader - Timestamp Interrupt Status When the Advanced Timestamp feature is enabled, this bit is set when any of the following conditions is true: - The system time value equals or exceeds the value specified in the Target Time High and Low registers. - There is an overflow in the seconds register. - The Auxiliary snapshot trigger is asserted. This bit is cleared on reading Bit 0 of Register 458 (Timestamp Status Register)."]
pub type TSIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCRXIPIS` reader - MMC Receive Checksum Offload Interrupt Status This bit is set high when an interrupt is generated in the MMC Receive Checksum Offload Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared."]
pub type MMCRXIPIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCTXIS` reader - MMC Transmit Interrupt Status This bit is set high when an interrupt is generated in the MMC Transmit Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared."]
pub type MMCTXIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCRXIS` reader - MMC Receive Interrupt Status This bit is set high when an interrupt is generated in the MMC Receive Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared."]
pub type MMCRXIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCIS` reader - MMC Interrupt Status This bit is set high when any of the Bits \\[7:5\\]
is set high and cleared only when all of these bits are low."]
pub type MMCIS_R = crate::BitReader<bool>;
#[doc = "Field `PMTIS` reader - PMT Interrupt Status This bit is set when a magic packet or remote wake-up frame is received in the power-down mode (see Bits 5 and 6 in the PMT Control and Status Register). This bit is cleared when both Bits\\[6:5\\]
are cleared because of a read operation to the PMT Control and Status register."]
pub type PMTIS_R = crate::BitReader<bool>;
#[doc = "Field `PCSANCIS` reader - PCS Auto-Negotiation Complete This bit is set when the Auto-negotiation is completed in the TBI, RTBI, or SGMII PHY interface (Bit 5 in Register 49 (AN Status Register)). This bit is cleared when you perform a read operation to the AN Status register."]
pub type PCSANCIS_R = crate::BitReader<bool>;
#[doc = "Field `PCSLCHGIS` reader - PCS Link Status Changed This bit is set because of any change in Link Status in the TBI, RTBI, or SGMII PHY interface (Bit 2 in Register 49 (AN Status Register)). This bit is cleared when you perform a read operation on the AN Status register."]
pub type PCSLCHGIS_R = crate::BitReader<bool>;
#[doc = "Field `RGSMIIIS` reader - RGMII or SMII Interrupt Status This bit is set because of any change in value of the Link Status of RGMII or SMII interface (Bit 3 in Register 54 (SGMII/RGMII/SMII Control and Status Register)). This bit is cleared when you perform a read operation on the SGMII/RGMII/SMII Control and Status Register."]
pub type RGSMIIIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 11 - GPI Interrupt Status When the GPIO feature is enabled, this bit is set when any active event (LL or LH) occurs on the GPIS field (Bits \\[3:0\\]) of Register 56 (General Purpose IO Register) and the corresponding GPIE bit is enabled. This bit is cleared on reading lane 0 (GPIS) of Register 56 (General Purpose IO Register). When the GPIO feature is not enabled, this bit is reserved."]
    #[inline(always)]
    pub fn gpiis(&self) -> GPIIS_R {
        GPIIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - LPI Interrupt Status When the Energy Efficient Ethernet feature is enabled, this bit is set for any LPI state entry or exit in the MAC Transmitter or Receiver. This bit is cleared on reading Bit 0 of Register 12 (LPI Control and Status Register). In all other modes, this bit is reserved."]
    #[inline(always)]
    pub fn lpiis(&self) -> LPIIS_R {
        LPIIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Status When the Advanced Timestamp feature is enabled, this bit is set when any of the following conditions is true: - The system time value equals or exceeds the value specified in the Target Time High and Low registers. - There is an overflow in the seconds register. - The Auxiliary snapshot trigger is asserted. This bit is cleared on reading Bit 0 of Register 458 (Timestamp Status Register)."]
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Checksum Offload Interrupt Status This bit is set high when an interrupt is generated in the MMC Receive Checksum Offload Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared."]
    #[inline(always)]
    pub fn mmcrxipis(&self) -> MMCRXIPIS_R {
        MMCRXIPIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit Interrupt Status This bit is set high when an interrupt is generated in the MMC Transmit Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared."]
    #[inline(always)]
    pub fn mmctxis(&self) -> MMCTXIS_R {
        MMCTXIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive Interrupt Status This bit is set high when an interrupt is generated in the MMC Receive Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared."]
    #[inline(always)]
    pub fn mmcrxis(&self) -> MMCRXIS_R {
        MMCRXIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Interrupt Status This bit is set high when any of the Bits \\[7:5\\]
is set high and cleared only when all of these bits are low."]
    #[inline(always)]
    pub fn mmcis(&self) -> MMCIS_R {
        MMCIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - PMT Interrupt Status This bit is set when a magic packet or remote wake-up frame is received in the power-down mode (see Bits 5 and 6 in the PMT Control and Status Register). This bit is cleared when both Bits\\[6:5\\]
are cleared because of a read operation to the PMT Control and Status register."]
    #[inline(always)]
    pub fn pmtis(&self) -> PMTIS_R {
        PMTIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - PCS Auto-Negotiation Complete This bit is set when the Auto-negotiation is completed in the TBI, RTBI, or SGMII PHY interface (Bit 5 in Register 49 (AN Status Register)). This bit is cleared when you perform a read operation to the AN Status register."]
    #[inline(always)]
    pub fn pcsancis(&self) -> PCSANCIS_R {
        PCSANCIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - PCS Link Status Changed This bit is set because of any change in Link Status in the TBI, RTBI, or SGMII PHY interface (Bit 2 in Register 49 (AN Status Register)). This bit is cleared when you perform a read operation on the AN Status register."]
    #[inline(always)]
    pub fn pcslchgis(&self) -> PCSLCHGIS_R {
        PCSLCHGIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - RGMII or SMII Interrupt Status This bit is set because of any change in value of the Link Status of RGMII or SMII interface (Bit 3 in Register 54 (SGMII/RGMII/SMII Control and Status Register)). This bit is cleared when you perform a read operation on the SGMII/RGMII/SMII Control and Status Register."]
    #[inline(always)]
    pub fn rgsmiiis(&self) -> RGSMIIIS_R {
        RGSMIIIS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_status](index.html) module"]
pub struct INTR_STATUS_SPEC;
impl crate::RegisterSpec for INTR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_status::R](R) reader structure"]
impl crate::Readable for INTR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_STATUS to value 0"]
impl crate::Resettable for INTR_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
