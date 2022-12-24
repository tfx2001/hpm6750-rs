#[doc = "Register `TGTTM_SEC` reader"]
pub struct R(crate::R<TGTTM_SEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TGTTM_SEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TGTTM_SEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TGTTM_SEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TGTTM_SEC` writer"]
pub struct W(crate::W<TGTTM_SEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TGTTM_SEC_SPEC>;
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
impl From<crate::W<TGTTM_SEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TGTTM_SEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSTR` reader - Target Time Seconds Register This register stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[6:5\\]
of Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled)."]
pub type TSTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSTR` writer - Target Time Seconds Register This register stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[6:5\\]
of Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled)."]
pub type TSTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TGTTM_SEC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Target Time Seconds Register This register stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[6:5\\]
of Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled)."]
    #[inline(always)]
    pub fn tstr(&self) -> TSTR_R {
        TSTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target Time Seconds Register This register stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[6:5\\]
of Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled)."]
    #[inline(always)]
    #[must_use]
    pub fn tstr(&mut self) -> TSTR_W<0> {
        TSTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Target Time Seconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tgttm_sec](index.html) module"]
pub struct TGTTM_SEC_SPEC;
impl crate::RegisterSpec for TGTTM_SEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tgttm_sec::R](R) reader structure"]
impl crate::Readable for TGTTM_SEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tgttm_sec::W](W) writer structure"]
impl crate::Writable for TGTTM_SEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TGTTM_SEC to value 0"]
impl crate::Resettable for TGTTM_SEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
