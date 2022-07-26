#[doc = "Register `CTRL3` reader"]
pub struct R(crate::R<CTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL3` writer"]
pub struct W(crate::W<CTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL3_SPEC>;
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
impl From<crate::W<CTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENET1_REFCLK_OE` reader - No description avaiable"]
pub type ENET1_REFCLK_OE_R = crate::BitReader<bool>;
#[doc = "Field `ENET1_REFCLK_OE` writer - No description avaiable"]
pub type ENET1_REFCLK_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL3_SPEC, bool, O>;
#[doc = "Field `ENET1_PHY_INTF_SEL` reader - No description avaiable"]
pub type ENET1_PHY_INTF_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENET1_PHY_INTF_SEL` writer - No description avaiable"]
pub type ENET1_PHY_INTF_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL3_SPEC, u8, u8, 3, O>;
#[doc = "Field `ENET1_FLOWCTRL` reader - No description avaiable"]
pub type ENET1_FLOWCTRL_R = crate::BitReader<bool>;
#[doc = "Field `ENET1_FLOWCTRL` writer - No description avaiable"]
pub type ENET1_FLOWCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL3_SPEC, bool, O>;
#[doc = "Field `ENET1_RMII_TXCLK_SEL` reader - No description avaiable"]
pub type ENET1_RMII_TXCLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `ENET1_RMII_TXCLK_SEL` writer - No description avaiable"]
pub type ENET1_RMII_TXCLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 19 - No description avaiable"]
    #[inline(always)]
    pub fn enet1_refclk_oe(&self) -> ENET1_REFCLK_OE_R {
        ENET1_REFCLK_OE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 13:15 - No description avaiable"]
    #[inline(always)]
    pub fn enet1_phy_intf_sel(&self) -> ENET1_PHY_INTF_SEL_R {
        ENET1_PHY_INTF_SEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 12 - No description avaiable"]
    #[inline(always)]
    pub fn enet1_flowctrl(&self) -> ENET1_FLOWCTRL_R {
        ENET1_FLOWCTRL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 10 - No description avaiable"]
    #[inline(always)]
    pub fn enet1_rmii_txclk_sel(&self) -> ENET1_RMII_TXCLK_SEL_R {
        ENET1_RMII_TXCLK_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 19 - No description avaiable"]
    #[inline(always)]
    pub fn enet1_refclk_oe(&mut self) -> ENET1_REFCLK_OE_W<19> {
        ENET1_REFCLK_OE_W::new(self)
    }
    #[doc = "Bits 13:15 - No description avaiable"]
    #[inline(always)]
    pub fn enet1_phy_intf_sel(&mut self) -> ENET1_PHY_INTF_SEL_W<13> {
        ENET1_PHY_INTF_SEL_W::new(self)
    }
    #[doc = "Bit 12 - No description avaiable"]
    #[inline(always)]
    pub fn enet1_flowctrl(&mut self) -> ENET1_FLOWCTRL_W<12> {
        ENET1_FLOWCTRL_W::new(self)
    }
    #[doc = "Bit 10 - No description avaiable"]
    #[inline(always)]
    pub fn enet1_rmii_txclk_sel(&mut self) -> ENET1_RMII_TXCLK_SEL_W<10> {
        ENET1_RMII_TXCLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl3](index.html) module"]
pub struct CTRL3_SPEC;
impl crate::RegisterSpec for CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl3::R](R) reader structure"]
impl crate::Readable for CTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl3::W](W) writer structure"]
impl crate::Writable for CTRL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL3 to value 0"]
impl crate::Resettable for CTRL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
