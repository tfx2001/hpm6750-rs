#[doc = "Register `PLL_PLL0_DIV1` reader"]
pub struct R(crate::R<PLL_PLL0_DIV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_PLL0_DIV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_PLL0_DIV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_PLL0_DIV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_PLL0_DIV1` writer"]
pub struct W(crate::W<PLL_PLL0_DIV1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_PLL0_DIV1_SPEC>;
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
impl From<crate::W<PLL_PLL0_DIV1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_PLL0_DIV1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSY` reader - Busy flag 0: divider is working 1: divider is changing status"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `RESPONSE` reader - Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
pub type RESPONSE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` reader - Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DIV` reader - Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_PLL0_DIV1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 31 - Busy flag 0: divider is working 1: divider is changing status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 29 - Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
    #[inline(always)]
    pub fn response(&self) -> RESPONSE_R {
        RESPONSE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 0:7 - Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLx divider1 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_pll0_div1](index.html) module"]
pub struct PLL_PLL0_DIV1_SPEC;
impl crate::RegisterSpec for PLL_PLL0_DIV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_pll0_div1::R](R) reader structure"]
impl crate::Readable for PLL_PLL0_DIV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_pll0_div1::W](W) writer structure"]
impl crate::Writable for PLL_PLL0_DIV1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_PLL0_DIV1 to value 0"]
impl crate::Resettable for PLL_PLL0_DIV1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
