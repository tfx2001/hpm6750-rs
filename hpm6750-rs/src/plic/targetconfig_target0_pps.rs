#[doc = "Register `TARGETCONFIG_TARGET0_PPS` reader"]
pub struct R(crate::R<TARGETCONFIG_TARGET0_PPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGETCONFIG_TARGET0_PPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGETCONFIG_TARGET0_PPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGETCONFIG_TARGET0_PPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGETCONFIG_TARGET0_PPS` writer"]
pub struct W(crate::W<TARGETCONFIG_TARGET0_PPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGETCONFIG_TARGET0_PPS_SPEC>;
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
impl From<crate::W<TARGETCONFIG_TARGET0_PPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGETCONFIG_TARGET0_PPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIORITY_PREEMPTED` reader - Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt."]
pub type PRIORITY_PREEMPTED_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRIORITY_PREEMPTED` writer - Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt."]
pub type PRIORITY_PREEMPTED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TARGETCONFIG_TARGET0_PPS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt."]
    #[inline(always)]
    pub fn priority_preempted(&self) -> PRIORITY_PREEMPTED_R {
        PRIORITY_PREEMPTED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn priority_preempted(&mut self) -> PRIORITY_PREEMPTED_W<0> {
        PRIORITY_PREEMPTED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Preempted priority stack\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targetconfig_target0_pps](index.html) module"]
pub struct TARGETCONFIG_TARGET0_PPS_SPEC;
impl crate::RegisterSpec for TARGETCONFIG_TARGET0_PPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [targetconfig_target0_pps::R](R) reader structure"]
impl crate::Readable for TARGETCONFIG_TARGET0_PPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [targetconfig_target0_pps::W](W) writer structure"]
impl crate::Writable for TARGETCONFIG_TARGET0_PPS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARGETCONFIG_TARGET0_PPS to value 0"]
impl crate::Resettable for TARGETCONFIG_TARGET0_PPS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
