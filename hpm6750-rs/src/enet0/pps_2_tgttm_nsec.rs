#[doc = "Register `PPS_2_TGTTM_NSEC` reader"]
pub struct R(crate::R<PPS_2_TGTTM_NSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPS_2_TGTTM_NSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPS_2_TGTTM_NSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPS_2_TGTTM_NSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPS_2_TGTTM_NSEC` writer"]
pub struct W(crate::W<PPS_2_TGTTM_NSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPS_2_TGTTM_NSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PPS_2_TGTTM_NSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPS_2_TGTTM_NSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTSL1` reader - Target Time Low for PPS1 Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL1 field (Bits \\[14:13\\]) in Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled). This value should not exceed 0x3B9A_C9FF when Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register). The actual start or stop time of the PPS signal output may have an error margin up to one unit of sub-second increment value."]
pub type TTSL1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TTSL1` writer - Target Time Low for PPS1 Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL1 field (Bits \\[14:13\\]) in Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled). This value should not exceed 0x3B9A_C9FF when Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register). The actual start or stop time of the PPS signal output may have an error margin up to one unit of sub-second increment value."]
pub type TTSL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PPS_2_TGTTM_NSEC_SPEC, u32, u32, 31, O>;
#[doc = "Field `TRGTBUSY1` reader - PPS1 Target Time Register Busy The MAC sets this bit when the PPSCMD1 field (Bits \\[10:8\\]) in Register 459 (PPS Control Register) is programmed to 010 or 011. Programming the PPSCMD1 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted."]
pub type TRGTBUSY1_R = crate::BitReader<bool>;
#[doc = "Field `TRGTBUSY1` writer - PPS1 Target Time Register Busy The MAC sets this bit when the PPSCMD1 field (Bits \\[10:8\\]) in Register 459 (PPS Control Register) is programmed to 010 or 011. Programming the PPSCMD1 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted."]
pub type TRGTBUSY1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPS_2_TGTTM_NSEC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:30 - Target Time Low for PPS1 Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL1 field (Bits \\[14:13\\]) in Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled). This value should not exceed 0x3B9A_C9FF when Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register). The actual start or stop time of the PPS signal output may have an error margin up to one unit of sub-second increment value."]
    #[inline(always)]
    pub fn ttsl1(&self) -> TTSL1_R {
        TTSL1_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - PPS1 Target Time Register Busy The MAC sets this bit when the PPSCMD1 field (Bits \\[10:8\\]) in Register 459 (PPS Control Register) is programmed to 010 or 011. Programming the PPSCMD1 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted."]
    #[inline(always)]
    pub fn trgtbusy1(&self) -> TRGTBUSY1_R {
        TRGTBUSY1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Time Low for PPS1 Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL1 field (Bits \\[14:13\\]) in Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled). This value should not exceed 0x3B9A_C9FF when Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register). The actual start or stop time of the PPS signal output may have an error margin up to one unit of sub-second increment value."]
    #[inline(always)]
    #[must_use]
    pub fn ttsl1(&mut self) -> TTSL1_W<0> {
        TTSL1_W::new(self)
    }
    #[doc = "Bit 31 - PPS1 Target Time Register Busy The MAC sets this bit when the PPSCMD1 field (Bits \\[10:8\\]) in Register 459 (PPS Control Register) is programmed to 010 or 011. Programming the PPSCMD1 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted."]
    #[inline(always)]
    #[must_use]
    pub fn trgtbusy1(&mut self) -> TRGTBUSY1_W<31> {
        TRGTBUSY1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPS Target Time Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pps_2_tgttm_nsec](index.html) module"]
pub struct PPS_2_TGTTM_NSEC_SPEC;
impl crate::RegisterSpec for PPS_2_TGTTM_NSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pps_2_tgttm_nsec::R](R) reader structure"]
impl crate::Readable for PPS_2_TGTTM_NSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pps_2_tgttm_nsec::W](W) writer structure"]
impl crate::Writable for PPS_2_TGTTM_NSEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPS_2_TGTTM_NSEC to value 0"]
impl crate::Resettable for PPS_2_TGTTM_NSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
