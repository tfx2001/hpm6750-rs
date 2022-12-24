#[doc = "Register `TXD_DATA0` writer"]
pub struct W(crate::W<TXD_DATA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXD_DATA0_SPEC>;
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
impl From<crate::W<TXD_DATA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXD_DATA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D` writer - No description avaiable"]
pub type D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXD_DATA0_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn d(&mut self) -> D_W<0> {
        D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Data0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txd_data0](index.html) module"]
pub struct TXD_DATA0_SPEC;
impl crate::RegisterSpec for TXD_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txd_data0::W](W) writer structure"]
impl crate::Writable for TXD_DATA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXD_DATA0 to value 0"]
impl crate::Resettable for TXD_DATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
