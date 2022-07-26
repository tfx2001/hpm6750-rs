#[doc = "Register `PHY_CTRL1` reader"]
pub struct R(crate::R<PHY_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHY_CTRL1` writer"]
pub struct W(crate::W<PHY_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CTRL1_SPEC>;
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
impl From<crate::W<PHY_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTMI_CFG_RST_N` reader - No description avaiable"]
pub type UTMI_CFG_RST_N_R = crate::BitReader<bool>;
#[doc = "Field `UTMI_CFG_RST_N` writer - No description avaiable"]
pub type UTMI_CFG_RST_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CTRL1_SPEC, bool, O>;
#[doc = "Field `UTMI_OTG_SUSPENDM` reader - OTG suspend, not utmi_suspendm"]
pub type UTMI_OTG_SUSPENDM_R = crate::BitReader<bool>;
#[doc = "Field `UTMI_OTG_SUSPENDM` writer - OTG suspend, not utmi_suspendm"]
pub type UTMI_OTG_SUSPENDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 20 - No description avaiable"]
    #[inline(always)]
    pub fn utmi_cfg_rst_n(&self) -> UTMI_CFG_RST_N_R {
        UTMI_CFG_RST_N_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 1 - OTG suspend, not utmi_suspendm"]
    #[inline(always)]
    pub fn utmi_otg_suspendm(&self) -> UTMI_OTG_SUSPENDM_R {
        UTMI_OTG_SUSPENDM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - No description avaiable"]
    #[inline(always)]
    pub fn utmi_cfg_rst_n(&mut self) -> UTMI_CFG_RST_N_W<20> {
        UTMI_CFG_RST_N_W::new(self)
    }
    #[doc = "Bit 1 - OTG suspend, not utmi_suspendm"]
    #[inline(always)]
    pub fn utmi_otg_suspendm(&mut self) -> UTMI_OTG_SUSPENDM_W<1> {
        UTMI_OTG_SUSPENDM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_ctrl1](index.html) module"]
pub struct PHY_CTRL1_SPEC;
impl crate::RegisterSpec for PHY_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_ctrl1::R](R) reader structure"]
impl crate::Readable for PHY_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_ctrl1::W](W) writer structure"]
impl crate::Writable for PHY_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PHY_CTRL1 to value 0"]
impl crate::Resettable for PHY_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
