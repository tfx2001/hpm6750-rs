#[doc = "Register `PLL_PLL0_CFG1` reader"]
pub struct R(crate::R<PLL_PLL0_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_PLL0_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_PLL0_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_PLL0_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_PLL0_CFG1` writer"]
pub struct W(crate::W<PLL_PLL0_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_PLL0_CFG1_SPEC>;
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
impl From<crate::W<PLL_PLL0_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_PLL0_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLCTRL_HW_EN` reader - 1: hardware controll PLL settings, software can update register, but result unknown; suggested only update fbdiv and frac value 0: full software control PLL settings"]
pub type PLLCTRL_HW_EN_R = crate::BitReader<bool>;
#[doc = "Field `PLLCTRL_HW_EN` writer - 1: hardware controll PLL settings, software can update register, but result unknown; suggested only update fbdiv and frac value 0: full software control PLL settings"]
pub type PLLCTRL_HW_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_PLL0_CFG1_SPEC, bool, O>;
#[doc = "Field `CLKEN_SW` reader - the clock enable used to gate pll output, should be set after lock, and clear before power down pll. pll_clock_enable = pllctrl_hw_en ? (pll_lock_comb & enable & pll_clk_enable_chg) : clken_sw;"]
pub type CLKEN_SW_R = crate::BitReader<bool>;
#[doc = "Field `CLKEN_SW` writer - the clock enable used to gate pll output, should be set after lock, and clear before power down pll. pll_clock_enable = pllctrl_hw_en ? (pll_lock_comb & enable & pll_clk_enable_chg) : clken_sw;"]
pub type CLKEN_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_PLL0_CFG1_SPEC, bool, O>;
#[doc = "Field `PLLPD_SW` reader - pll power down. pll_ana_pd = pllctrl_hw_en ? (pll_pd_soc|pll_pd_chg) : pllpd_sw; pll_pd_soc is just delay of soc enable, for soc to control pll on/off; pll_pd_chg is used to power down pll when div_chg_mode is 1, if software update pll parameter(fbdiv or frac), pll_ctrl will power down pll, update parameter, then power up pll. response to soc will not de-asserted at this sequence"]
pub type PLLPD_SW_R = crate::BitReader<bool>;
#[doc = "Field `PLLPD_SW` writer - pll power down. pll_ana_pd = pllctrl_hw_en ? (pll_pd_soc|pll_pd_chg) : pllpd_sw; pll_pd_soc is just delay of soc enable, for soc to control pll on/off; pll_pd_chg is used to power down pll when div_chg_mode is 1, if software update pll parameter(fbdiv or frac), pll_ctrl will power down pll, update parameter, then power up pll. response to soc will not de-asserted at this sequence"]
pub type PLLPD_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_PLL0_CFG1_SPEC, bool, O>;
#[doc = "Field `LOCK_CNT_CFG` reader - used to wait lock if set larger than lock time; default 1500 24M cycle if refdiv is 1, 4500 cycle if refdiv is 3"]
pub type LOCK_CNT_CFG_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_CNT_CFG` writer - used to wait lock if set larger than lock time; default 1500 24M cycle if refdiv is 1, 4500 cycle if refdiv is 3"]
pub type LOCK_CNT_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_PLL0_CFG1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - 1: hardware controll PLL settings, software can update register, but result unknown; suggested only update fbdiv and frac value 0: full software control PLL settings"]
    #[inline(always)]
    pub fn pllctrl_hw_en(&self) -> PLLCTRL_HW_EN_R {
        PLLCTRL_HW_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 26 - the clock enable used to gate pll output, should be set after lock, and clear before power down pll. pll_clock_enable = pllctrl_hw_en ? (pll_lock_comb & enable & pll_clk_enable_chg) : clken_sw;"]
    #[inline(always)]
    pub fn clken_sw(&self) -> CLKEN_SW_R {
        CLKEN_SW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - pll power down. pll_ana_pd = pllctrl_hw_en ? (pll_pd_soc|pll_pd_chg) : pllpd_sw; pll_pd_soc is just delay of soc enable, for soc to control pll on/off; pll_pd_chg is used to power down pll when div_chg_mode is 1, if software update pll parameter(fbdiv or frac), pll_ctrl will power down pll, update parameter, then power up pll. response to soc will not de-asserted at this sequence"]
    #[inline(always)]
    pub fn pllpd_sw(&self) -> PLLPD_SW_R {
        PLLPD_SW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 15 - used to wait lock if set larger than lock time; default 1500 24M cycle if refdiv is 1, 4500 cycle if refdiv is 3"]
    #[inline(always)]
    pub fn lock_cnt_cfg(&self) -> LOCK_CNT_CFG_R {
        LOCK_CNT_CFG_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 1: hardware controll PLL settings, software can update register, but result unknown; suggested only update fbdiv and frac value 0: full software control PLL settings"]
    #[inline(always)]
    pub fn pllctrl_hw_en(&mut self) -> PLLCTRL_HW_EN_W<31> {
        PLLCTRL_HW_EN_W::new(self)
    }
    #[doc = "Bit 26 - the clock enable used to gate pll output, should be set after lock, and clear before power down pll. pll_clock_enable = pllctrl_hw_en ? (pll_lock_comb & enable & pll_clk_enable_chg) : clken_sw;"]
    #[inline(always)]
    pub fn clken_sw(&mut self) -> CLKEN_SW_W<26> {
        CLKEN_SW_W::new(self)
    }
    #[doc = "Bit 25 - pll power down. pll_ana_pd = pllctrl_hw_en ? (pll_pd_soc|pll_pd_chg) : pllpd_sw; pll_pd_soc is just delay of soc enable, for soc to control pll on/off; pll_pd_chg is used to power down pll when div_chg_mode is 1, if software update pll parameter(fbdiv or frac), pll_ctrl will power down pll, update parameter, then power up pll. response to soc will not de-asserted at this sequence"]
    #[inline(always)]
    pub fn pllpd_sw(&mut self) -> PLLPD_SW_W<25> {
        PLLPD_SW_W::new(self)
    }
    #[doc = "Bit 15 - used to wait lock if set larger than lock time; default 1500 24M cycle if refdiv is 1, 4500 cycle if refdiv is 3"]
    #[inline(always)]
    pub fn lock_cnt_cfg(&mut self) -> LOCK_CNT_CFG_W<15> {
        LOCK_CNT_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLx config1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_pll0_cfg1](index.html) module"]
pub struct PLL_PLL0_CFG1_SPEC;
impl crate::RegisterSpec for PLL_PLL0_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_pll0_cfg1::R](R) reader structure"]
impl crate::Readable for PLL_PLL0_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_pll0_cfg1::W](W) writer structure"]
impl crate::Writable for PLL_PLL0_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_PLL0_CFG1 to value 0x8000_0000"]
impl crate::Resettable for PLL_PLL0_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
