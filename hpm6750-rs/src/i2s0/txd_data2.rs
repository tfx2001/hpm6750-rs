#[doc = "Register `TXD_DATA2` writer"]
pub struct W(crate::W<TXD_DATA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXD_DATA2_SPEC>;
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
impl From<crate::W<TXD_DATA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXD_DATA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D` writer - No description avaiable"]
pub type D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXD_DATA2_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
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
#[doc = "Tx Data2\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txd_data2](index.html) module"]
pub struct TXD_DATA2_SPEC;
impl crate::RegisterSpec for TXD_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txd_data2::W](W) writer structure"]
impl crate::Writable for TXD_DATA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXD_DATA2 to value 0"]
impl crate::Resettable for TXD_DATA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
