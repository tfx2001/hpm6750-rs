#[doc = "Register `TXREG` writer"]
pub struct W(crate::W<TXREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXREG_SPEC>;
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
impl From<crate::W<TXREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXREG` writer - Transmit word message to other core."]
pub type TXREG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXREG_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Transmit word message to other core."]
    #[inline(always)]
    pub fn txreg(&mut self) -> TXREG_W<0> {
        TXREG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit word message to other core.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txreg](index.html) module"]
pub struct TXREG_SPEC;
impl crate::RegisterSpec for TXREG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txreg::W](W) writer structure"]
impl crate::Writable for TXREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXREG to value 0"]
impl crate::Resettable for TXREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
