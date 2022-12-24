#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENET0_RMII_TXCLK_SEL` reader - No description avaiable"]
pub type ENET0_RMII_TXCLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `ENET0_RMII_TXCLK_SEL` writer - No description avaiable"]
pub type ENET0_RMII_TXCLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, bool, O>;
#[doc = "Field `ENET0_FLOWCTRL` reader - No description avaiable"]
pub type ENET0_FLOWCTRL_R = crate::BitReader<bool>;
#[doc = "Field `ENET0_FLOWCTRL` writer - No description avaiable"]
pub type ENET0_FLOWCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, bool, O>;
#[doc = "Field `ENET0_PHY_INF_SEL` reader - No description avaiable"]
pub type ENET0_PHY_INF_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENET0_PHY_INF_SEL` writer - No description avaiable"]
pub type ENET0_PHY_INF_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL2_SPEC, u8, u8, 3, O>;
#[doc = "Field `ENET0_REFCLK_OE` reader - No description avaiable"]
pub type ENET0_REFCLK_OE_R = crate::BitReader<bool>;
#[doc = "Field `ENET0_REFCLK_OE` writer - No description avaiable"]
pub type ENET0_REFCLK_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, bool, O>;
#[doc = "Field `ENET0_IRQ_EN` reader - No description avaiable"]
pub type ENET0_IRQ_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENET0_IRQ_EN` writer - No description avaiable"]
pub type ENET0_IRQ_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 10 - No description avaiable"]
    #[inline(always)]
    pub fn enet0_rmii_txclk_sel(&self) -> ENET0_RMII_TXCLK_SEL_R {
        ENET0_RMII_TXCLK_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - No description avaiable"]
    #[inline(always)]
    pub fn enet0_flowctrl(&self) -> ENET0_FLOWCTRL_R {
        ENET0_FLOWCTRL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - No description avaiable"]
    #[inline(always)]
    pub fn enet0_phy_inf_sel(&self) -> ENET0_PHY_INF_SEL_R {
        ENET0_PHY_INF_SEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 19 - No description avaiable"]
    #[inline(always)]
    pub fn enet0_refclk_oe(&self) -> ENET0_REFCLK_OE_R {
        ENET0_REFCLK_OE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 27:31 - No description avaiable"]
    #[inline(always)]
    pub fn enet0_irq_en(&self) -> ENET0_IRQ_EN_R {
        ENET0_IRQ_EN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 10 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn enet0_rmii_txclk_sel(&mut self) -> ENET0_RMII_TXCLK_SEL_W<10> {
        ENET0_RMII_TXCLK_SEL_W::new(self)
    }
    #[doc = "Bit 12 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn enet0_flowctrl(&mut self) -> ENET0_FLOWCTRL_W<12> {
        ENET0_FLOWCTRL_W::new(self)
    }
    #[doc = "Bits 13:15 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn enet0_phy_inf_sel(&mut self) -> ENET0_PHY_INF_SEL_W<13> {
        ENET0_PHY_INF_SEL_W::new(self)
    }
    #[doc = "Bit 19 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn enet0_refclk_oe(&mut self) -> ENET0_REFCLK_OE_W<19> {
        ENET0_REFCLK_OE_W::new(self)
    }
    #[doc = "Bits 27:31 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn enet0_irq_en(&mut self) -> ENET0_IRQ_EN_W<27> {
        ENET0_IRQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
