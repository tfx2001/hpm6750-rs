#[doc = "Register `PLL_PLL4_CFG0` reader"]
pub struct R(crate::R<PLL_PLL4_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_PLL4_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_PLL4_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_PLL4_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_PLL4_CFG0` writer"]
pub struct W(crate::W<PLL_PLL4_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_PLL4_CFG0_SPEC>;
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
impl From<crate::W<PLL_PLL4_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_PLL4_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SS_RSTPTR` reader - reset pointer, for sscg, lock when lock_en\\[31\\]&~pll_ana_pd&~pll_lock_comb"]
pub type SS_RSTPTR_R = crate::BitReader<bool>;
#[doc = "Field `SS_RSTPTR` writer - reset pointer, for sscg, lock when lock_en\\[31\\]&~pll_ana_pd&~pll_lock_comb"]
pub type SS_RSTPTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_PLL4_CFG0_SPEC, bool, O>;
#[doc = "Field `REFDIV` reader - refclk diverder, lock when lock_en\\[24\\]&~pll_ana_pd"]
pub type REFDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFDIV` writer - refclk diverder, lock when lock_en\\[24\\]&~pll_ana_pd"]
pub type REFDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_PLL4_CFG0_SPEC, u8, u8, 6, O>;
#[doc = "Field `POSTDIV1` reader - lock when lock_en\\[20\\]&~pll_ana_pd"]
pub type POSTDIV1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POSTDIV1` writer - lock when lock_en\\[20\\]&~pll_ana_pd"]
pub type POSTDIV1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_PLL4_CFG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `SS_SPREAD` reader - lock when lock_en\\[14\\]&~pll_ana_pd"]
pub type SS_SPREAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SS_SPREAD` writer - lock when lock_en\\[14\\]&~pll_ana_pd"]
pub type SS_SPREAD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_PLL4_CFG0_SPEC, u8, u8, 5, O>;
#[doc = "Field `SS_DIVVAL` reader - sscg divval, lock when lock_en\\[8\\]&~pll_ana_pd"]
pub type SS_DIVVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SS_DIVVAL` writer - sscg divval, lock when lock_en\\[8\\]&~pll_ana_pd"]
pub type SS_DIVVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_PLL4_CFG0_SPEC, u8, u8, 6, O>;
#[doc = "Field `SS_DOWNSPREAD` reader - Downspread control 1’b0 –> Center-Spread 1’b1 –> Downspread"]
pub type SS_DOWNSPREAD_R = crate::BitReader<bool>;
#[doc = "Field `SS_DOWNSPREAD` writer - Downspread control 1’b0 –> Center-Spread 1’b1 –> Downspread"]
pub type SS_DOWNSPREAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_PLL4_CFG0_SPEC, bool, O>;
#[doc = "Field `SS_RESET` reader - No description avaiable"]
pub type SS_RESET_R = crate::BitReader<bool>;
#[doc = "Field `SS_RESET` writer - No description avaiable"]
pub type SS_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_PLL4_CFG0_SPEC, bool, O>;
#[doc = "Field `SS_DISABLE_SSCG` reader - No description avaiable"]
pub type SS_DISABLE_SSCG_R = crate::BitReader<bool>;
#[doc = "Field `SS_DISABLE_SSCG` writer - No description avaiable"]
pub type SS_DISABLE_SSCG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_PLL4_CFG0_SPEC, bool, O>;
#[doc = "Field `DSMPD` reader - 1: int mode; 0: frac mode"]
pub type DSMPD_R = crate::BitReader<bool>;
#[doc = "Field `DSMPD` writer - 1: int mode; 0: frac mode"]
pub type DSMPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_PLL4_CFG0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - reset pointer, for sscg, lock when lock_en\\[31\\]&~pll_ana_pd&~pll_lock_comb"]
    #[inline(always)]
    pub fn ss_rstptr(&self) -> SS_RSTPTR_R {
        SS_RSTPTR_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 24:29 - refclk diverder, lock when lock_en\\[24\\]&~pll_ana_pd"]
    #[inline(always)]
    pub fn refdiv(&self) -> REFDIV_R {
        REFDIV_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 20:22 - lock when lock_en\\[20\\]&~pll_ana_pd"]
    #[inline(always)]
    pub fn postdiv1(&self) -> POSTDIV1_R {
        POSTDIV1_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 14:18 - lock when lock_en\\[14\\]&~pll_ana_pd"]
    #[inline(always)]
    pub fn ss_spread(&self) -> SS_SPREAD_R {
        SS_SPREAD_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - sscg divval, lock when lock_en\\[8\\]&~pll_ana_pd"]
    #[inline(always)]
    pub fn ss_divval(&self) -> SS_DIVVAL_R {
        SS_DIVVAL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Downspread control 1’b0 –> Center-Spread 1’b1 –> Downspread"]
    #[inline(always)]
    pub fn ss_downspread(&self) -> SS_DOWNSPREAD_R {
        SS_DOWNSPREAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - No description avaiable"]
    #[inline(always)]
    pub fn ss_reset(&self) -> SS_RESET_R {
        SS_RESET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - No description avaiable"]
    #[inline(always)]
    pub fn ss_disable_sscg(&self) -> SS_DISABLE_SSCG_R {
        SS_DISABLE_SSCG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: int mode; 0: frac mode"]
    #[inline(always)]
    pub fn dsmpd(&self) -> DSMPD_R {
        DSMPD_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - reset pointer, for sscg, lock when lock_en\\[31\\]&~pll_ana_pd&~pll_lock_comb"]
    #[inline(always)]
    pub fn ss_rstptr(&mut self) -> SS_RSTPTR_W<31> {
        SS_RSTPTR_W::new(self)
    }
    #[doc = "Bits 24:29 - refclk diverder, lock when lock_en\\[24\\]&~pll_ana_pd"]
    #[inline(always)]
    pub fn refdiv(&mut self) -> REFDIV_W<24> {
        REFDIV_W::new(self)
    }
    #[doc = "Bits 20:22 - lock when lock_en\\[20\\]&~pll_ana_pd"]
    #[inline(always)]
    pub fn postdiv1(&mut self) -> POSTDIV1_W<20> {
        POSTDIV1_W::new(self)
    }
    #[doc = "Bits 14:18 - lock when lock_en\\[14\\]&~pll_ana_pd"]
    #[inline(always)]
    pub fn ss_spread(&mut self) -> SS_SPREAD_W<14> {
        SS_SPREAD_W::new(self)
    }
    #[doc = "Bits 8:13 - sscg divval, lock when lock_en\\[8\\]&~pll_ana_pd"]
    #[inline(always)]
    pub fn ss_divval(&mut self) -> SS_DIVVAL_W<8> {
        SS_DIVVAL_W::new(self)
    }
    #[doc = "Bit 7 - Downspread control 1’b0 –> Center-Spread 1’b1 –> Downspread"]
    #[inline(always)]
    pub fn ss_downspread(&mut self) -> SS_DOWNSPREAD_W<7> {
        SS_DOWNSPREAD_W::new(self)
    }
    #[doc = "Bit 6 - No description avaiable"]
    #[inline(always)]
    pub fn ss_reset(&mut self) -> SS_RESET_W<6> {
        SS_RESET_W::new(self)
    }
    #[doc = "Bit 5 - No description avaiable"]
    #[inline(always)]
    pub fn ss_disable_sscg(&mut self) -> SS_DISABLE_SSCG_W<5> {
        SS_DISABLE_SSCG_W::new(self)
    }
    #[doc = "Bit 3 - 1: int mode; 0: frac mode"]
    #[inline(always)]
    pub fn dsmpd(&mut self) -> DSMPD_W<3> {
        DSMPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLx config0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_pll4_cfg0](index.html) module"]
pub struct PLL_PLL4_CFG0_SPEC;
impl crate::RegisterSpec for PLL_PLL4_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_pll4_cfg0::R](R) reader structure"]
impl crate::Readable for PLL_PLL4_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_pll4_cfg0::W](W) writer structure"]
impl crate::Writable for PLL_PLL4_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_PLL4_CFG0 to value 0x0014_0460"]
impl crate::Resettable for PLL_PLL4_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0014_0460
    }
}
