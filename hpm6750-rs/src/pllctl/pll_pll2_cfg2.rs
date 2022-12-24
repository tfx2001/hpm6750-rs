#[doc = "Register `PLL_PLL2_CFG2` reader"]
pub struct R(crate::R<PLL_PLL2_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_PLL2_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_PLL2_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_PLL2_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_PLL2_CFG2` writer"]
pub struct W(crate::W<PLL_PLL2_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_PLL2_CFG2_SPEC>;
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
impl From<crate::W<PLL_PLL2_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_PLL2_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBDIV_INT` reader - fbdiv used in int mode"]
pub type FBDIV_INT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FBDIV_INT` writer - fbdiv used in int mode"]
pub type FBDIV_INT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_PLL2_CFG2_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - fbdiv used in int mode"]
    #[inline(always)]
    pub fn fbdiv_int(&self) -> FBDIV_INT_R {
        FBDIV_INT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - fbdiv used in int mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbdiv_int(&mut self) -> FBDIV_INT_W<0> {
        FBDIV_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLx config2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_pll2_cfg2](index.html) module"]
pub struct PLL_PLL2_CFG2_SPEC;
impl crate::RegisterSpec for PLL_PLL2_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_pll2_cfg2::R](R) reader structure"]
impl crate::Readable for PLL_PLL2_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_pll2_cfg2::W](W) writer structure"]
impl crate::Writable for PLL_PLL2_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_PLL2_CFG2 to value 0"]
impl crate::Resettable for PLL_PLL2_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
