#[doc = "Register `CLOCK_CLK_TOP_NTMR0` reader"]
pub struct R(crate::R<CLOCK_CLK_TOP_NTMR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_CLK_TOP_NTMR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_CLK_TOP_NTMR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_CLK_TOP_NTMR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK_CLK_TOP_NTMR0` writer"]
pub struct W(crate::W<CLOCK_CLK_TOP_NTMR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_CLK_TOP_NTMR0_SPEC>;
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
impl From<crate::W<CLOCK_CLK_TOP_NTMR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_CLK_TOP_NTMR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
pub type DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLOCK_CLK_TOP_NTMR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `MUX` reader - clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
pub type MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUX` writer - clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
pub type MUX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLOCK_CLK_TOP_NTMR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `LOC_BUSY` reader - local busy 0: a change is pending for current node 1: current node is changing status"]
pub type LOC_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `GLB_BUSY` reader - global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
pub type GLB_BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 8) & 0x0f) as u8)
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
    #[doc = "Bits 0:7 - clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bits 8:11 - clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
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
#[doc = "Clock setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_clk_top_ntmr0](index.html) module"]
pub struct CLOCK_CLK_TOP_NTMR0_SPEC;
impl crate::RegisterSpec for CLOCK_CLK_TOP_NTMR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_clk_top_ntmr0::R](R) reader structure"]
impl crate::Readable for CLOCK_CLK_TOP_NTMR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_clk_top_ntmr0::W](W) writer structure"]
impl crate::Writable for CLOCK_CLK_TOP_NTMR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCK_CLK_TOP_NTMR0 to value 0"]
impl crate::Resettable for CLOCK_CLK_TOP_NTMR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
