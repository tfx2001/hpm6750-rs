#[doc = "Register `PPS_2_TGTTM_SEC` reader"]
pub struct R(crate::R<PPS_2_TGTTM_SEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPS_2_TGTTM_SEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPS_2_TGTTM_SEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPS_2_TGTTM_SEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPS_2_TGTTM_SEC` writer"]
pub struct W(crate::W<PPS_2_TGTTM_SEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPS_2_TGTTM_SEC_SPEC>;
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
impl From<crate::W<PPS_2_TGTTM_SEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPS_2_TGTTM_SEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSTRH1` reader - PPS1 Target Time Seconds Register This register stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[14:13\\], TRGTMODSEL1, of Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled)."]
pub type TSTRH1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSTRH1` writer - PPS1 Target Time Seconds Register This register stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[14:13\\], TRGTMODSEL1, of Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled)."]
pub type TSTRH1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PPS_2_TGTTM_SEC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - PPS1 Target Time Seconds Register This register stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[14:13\\], TRGTMODSEL1, of Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled)."]
    #[inline(always)]
    pub fn tstrh1(&self) -> TSTRH1_R {
        TSTRH1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPS1 Target Time Seconds Register This register stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[14:13\\], TRGTMODSEL1, of Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled)."]
    #[inline(always)]
    pub fn tstrh1(&mut self) -> TSTRH1_W<0> {
        TSTRH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPS2 Target Time Seconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pps_2_tgttm_sec](index.html) module"]
pub struct PPS_2_TGTTM_SEC_SPEC;
impl crate::RegisterSpec for PPS_2_TGTTM_SEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pps_2_tgttm_sec::R](R) reader structure"]
impl crate::Readable for PPS_2_TGTTM_SEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pps_2_tgttm_sec::W](W) writer structure"]
impl crate::Writable for PPS_2_TGTTM_SEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPS_2_TGTTM_SEC to value 0"]
impl crate::Resettable for PPS_2_TGTTM_SEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
