#[doc = "Register `TAMP_TAMP4_LFSR` writer"]
pub struct W(crate::W<TAMP_TAMP4_LFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_TAMP4_LFSR_SPEC>;
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
impl From<crate::W<TAMP_TAMP4_LFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_TAMP4_LFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFSR` writer - LFSR for active tamper, write only register, always read 0"]
pub type LFSR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TAMP_TAMP4_LFSR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - LFSR for active tamper, write only register, always read 0"]
    #[inline(always)]
    pub fn lfsr(&mut self) -> LFSR_W<0> {
        LFSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper4 LFSR shift register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_tamp4_lfsr](index.html) module"]
pub struct TAMP_TAMP4_LFSR_SPEC;
impl crate::RegisterSpec for TAMP_TAMP4_LFSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tamp_tamp4_lfsr::W](W) writer structure"]
impl crate::Writable for TAMP_TAMP4_LFSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMP_TAMP4_LFSR to value 0"]
impl crate::Resettable for TAMP_TAMP4_LFSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
