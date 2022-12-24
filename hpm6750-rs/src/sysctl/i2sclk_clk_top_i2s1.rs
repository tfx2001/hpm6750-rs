#[doc = "Register `I2SCLK_CLK_TOP_I2S1` reader"]
pub struct R(crate::R<I2SCLK_CLK_TOP_I2S1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCLK_CLK_TOP_I2S1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCLK_CLK_TOP_I2S1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCLK_CLK_TOP_I2S1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SCLK_CLK_TOP_I2S1` writer"]
pub struct W(crate::W<I2SCLK_CLK_TOP_I2S1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCLK_CLK_TOP_I2S1_SPEC>;
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
impl From<crate::W<I2SCLK_CLK_TOP_I2S1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCLK_CLK_TOP_I2S1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUX` reader - clock source selection 0: ahb clock 1: i2s clock 0 2: i2s clock 1 3: i2s clock 2"]
pub type MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUX` writer - clock source selection 0: ahb clock 1: i2s clock 0 2: i2s clock 1 3: i2s clock 2"]
pub type MUX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2SCLK_CLK_TOP_I2S1_SPEC, u8, u8, 3, O>;
#[doc = "Field `LOC_BUSY` reader - local busy 0: a change is pending for current node 1: current node is changing status"]
pub type LOC_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `GLB_BUSY` reader - global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
pub type GLB_BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 8:10 - clock source selection 0: ahb clock 1: i2s clock 0 2: i2s clock 1 3: i2s clock 2"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 30 - local busy 0: a change is pending for current node 1: current node is changing status"]
    #[inline(always)]
    pub fn loc_busy(&self) -> LOC_BUSY_R {
        LOC_BUSY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    #[inline(always)]
    pub fn glb_busy(&self) -> GLB_BUSY_R {
        GLB_BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - clock source selection 0: ahb clock 1: i2s clock 0 2: i2s clock 1 3: i2s clock 2"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MUX_W<8> {
        MUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sclk_clk_top_i2s1](index.html) module"]
pub struct I2SCLK_CLK_TOP_I2S1_SPEC;
impl crate::RegisterSpec for I2SCLK_CLK_TOP_I2S1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sclk_clk_top_i2s1::R](R) reader structure"]
impl crate::Readable for I2SCLK_CLK_TOP_I2S1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sclk_clk_top_i2s1::W](W) writer structure"]
impl crate::Writable for I2SCLK_CLK_TOP_I2S1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SCLK_CLK_TOP_I2S1 to value 0"]
impl crate::Resettable for I2SCLK_CLK_TOP_I2S1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
