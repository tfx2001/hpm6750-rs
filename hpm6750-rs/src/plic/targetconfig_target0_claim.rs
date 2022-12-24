#[doc = "Register `TARGETCONFIG_TARGET0_CLAIM` reader"]
pub struct R(crate::R<TARGETCONFIG_TARGET0_CLAIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGETCONFIG_TARGET0_CLAIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGETCONFIG_TARGET0_CLAIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGETCONFIG_TARGET0_CLAIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGETCONFIG_TARGET0_CLAIM` writer"]
pub struct W(crate::W<TARGETCONFIG_TARGET0_CLAIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGETCONFIG_TARGET0_CLAIM_SPEC>;
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
impl From<crate::W<TARGETCONFIG_TARGET0_CLAIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGETCONFIG_TARGET0_CLAIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERRUPT_ID` reader - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
pub type INTERRUPT_ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INTERRUPT_ID` writer - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
pub type INTERRUPT_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TARGETCONFIG_TARGET0_CLAIM_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
    #[inline(always)]
    pub fn interrupt_id(&self) -> INTERRUPT_ID_R {
        INTERRUPT_ID_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W<0> {
        INTERRUPT_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Target claim and complete\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targetconfig_target0_claim](index.html) module"]
pub struct TARGETCONFIG_TARGET0_CLAIM_SPEC;
impl crate::RegisterSpec for TARGETCONFIG_TARGET0_CLAIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [targetconfig_target0_claim::R](R) reader structure"]
impl crate::Readable for TARGETCONFIG_TARGET0_CLAIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [targetconfig_target0_claim::W](W) writer structure"]
impl crate::Writable for TARGETCONFIG_TARGET0_CLAIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARGETCONFIG_TARGET0_CLAIM to value 0"]
impl crate::Resettable for TARGETCONFIG_TARGET0_CLAIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
