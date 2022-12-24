#[doc = "Register `TGTTM_NSEC` reader"]
pub struct R(crate::R<TGTTM_NSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TGTTM_NSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TGTTM_NSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TGTTM_NSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TGTTM_NSEC` writer"]
pub struct W(crate::W<TGTTM_NSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TGTTM_NSEC_SPEC>;
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
impl From<crate::W<TGTTM_NSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TGTTM_NSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTSLO` reader - Target Timestamp Low Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL0 field (Bits \\[6:5\\]) in Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled). This value should not exceed 0x3B9A_C9FF when Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register). The actual start or stop time of the PPS signal output may have an error margin up to one unit of sub-second increment value."]
pub type TTSLO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TTSLO` writer - Target Timestamp Low Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL0 field (Bits \\[6:5\\]) in Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled). This value should not exceed 0x3B9A_C9FF when Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register). The actual start or stop time of the PPS signal output may have an error margin up to one unit of sub-second increment value."]
pub type TTSLO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TGTTM_NSEC_SPEC, u32, u32, 31, O>;
#[doc = "Field `TRGTBUSY` reader - Target Time Register Busy The MAC sets this bit when the PPSCMD field (Bit \\[3:0\\]) in Register 459 (PPS Control Register) is programmed to 010 or 011. Programming the PPSCMD field to 010 or 011, instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted. This bit is reserved when the Enable Flexible Pulse-Per-Second Output feature is not selected."]
pub type TRGTBUSY_R = crate::BitReader<bool>;
#[doc = "Field `TRGTBUSY` writer - Target Time Register Busy The MAC sets this bit when the PPSCMD field (Bit \\[3:0\\]) in Register 459 (PPS Control Register) is programmed to 010 or 011. Programming the PPSCMD field to 010 or 011, instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted. This bit is reserved when the Enable Flexible Pulse-Per-Second Output feature is not selected."]
pub type TRGTBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, TGTTM_NSEC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:30 - Target Timestamp Low Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL0 field (Bits \\[6:5\\]) in Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled). This value should not exceed 0x3B9A_C9FF when Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register). The actual start or stop time of the PPS signal output may have an error margin up to one unit of sub-second increment value."]
    #[inline(always)]
    pub fn ttslo(&self) -> TTSLO_R {
        TTSLO_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Target Time Register Busy The MAC sets this bit when the PPSCMD field (Bit \\[3:0\\]) in Register 459 (PPS Control Register) is programmed to 010 or 011. Programming the PPSCMD field to 010 or 011, instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted. This bit is reserved when the Enable Flexible Pulse-Per-Second Output feature is not selected."]
    #[inline(always)]
    pub fn trgtbusy(&self) -> TRGTBUSY_R {
        TRGTBUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Timestamp Low Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL0 field (Bits \\[6:5\\]) in Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled). This value should not exceed 0x3B9A_C9FF when Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register). The actual start or stop time of the PPS signal output may have an error margin up to one unit of sub-second increment value."]
    #[inline(always)]
    #[must_use]
    pub fn ttslo(&mut self) -> TTSLO_W<0> {
        TTSLO_W::new(self)
    }
    #[doc = "Bit 31 - Target Time Register Busy The MAC sets this bit when the PPSCMD field (Bit \\[3:0\\]) in Register 459 (PPS Control Register) is programmed to 010 or 011. Programming the PPSCMD field to 010 or 011, instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted. This bit is reserved when the Enable Flexible Pulse-Per-Second Output feature is not selected."]
    #[inline(always)]
    #[must_use]
    pub fn trgtbusy(&mut self) -> TRGTBUSY_W<31> {
        TRGTBUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Target Time Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tgttm_nsec](index.html) module"]
pub struct TGTTM_NSEC_SPEC;
impl crate::RegisterSpec for TGTTM_NSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tgttm_nsec::R](R) reader structure"]
impl crate::Readable for TGTTM_NSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tgttm_nsec::W](W) writer structure"]
impl crate::Writable for TGTTM_NSEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TGTTM_NSEC to value 0"]
impl crate::Resettable for TGTTM_NSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
