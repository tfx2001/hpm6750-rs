#[doc = "Register `CHABORT` writer"]
pub struct W(crate::W<CHABORT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHABORT_SPEC>;
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
impl From<crate::W<CHABORT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHABORT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHABORT` writer - Write 1 to bit n to abort channel n. The bits should only be set when the corresponding channels are enabled. Otherwise, the writes will be ignored for channels that are not enabled. (N: Number of channels)"]
pub type CHABORT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHABORT_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Write 1 to bit n to abort channel n. The bits should only be set when the corresponding channels are enabled. Otherwise, the writes will be ignored for channels that are not enabled. (N: Number of channels)"]
    #[inline(always)]
    pub fn chabort(&mut self) -> CHABORT_W<0> {
        CHABORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Abort Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chabort](index.html) module"]
pub struct CHABORT_SPEC;
impl crate::RegisterSpec for CHABORT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chabort::W](W) writer structure"]
impl crate::Writable for CHABORT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHABORT to value 0"]
impl crate::Resettable for CHABORT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
