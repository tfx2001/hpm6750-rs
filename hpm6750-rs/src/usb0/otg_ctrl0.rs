#[doc = "Register `OTG_CTRL0` reader"]
pub struct R(crate::R<OTG_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_CTRL0` writer"]
pub struct W(crate::W<OTG_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_CTRL0_SPEC>;
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
impl From<crate::W<OTG_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTG_WKDPDMCHG_EN` reader - No description avaiable"]
pub type OTG_WKDPDMCHG_EN_R = crate::BitReader<bool>;
#[doc = "Field `OTG_WKDPDMCHG_EN` writer - No description avaiable"]
pub type OTG_WKDPDMCHG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_CTRL0_SPEC, bool, O>;
#[doc = "Field `AUTORESUME_EN` reader - No description avaiable"]
pub type AUTORESUME_EN_R = crate::BitReader<bool>;
#[doc = "Field `AUTORESUME_EN` writer - No description avaiable"]
pub type AUTORESUME_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_CTRL0_SPEC, bool, O>;
#[doc = "Field `OTG_VBUS_WAKEUP_EN` reader - No description avaiable"]
pub type OTG_VBUS_WAKEUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `OTG_VBUS_WAKEUP_EN` writer - No description avaiable"]
pub type OTG_VBUS_WAKEUP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_CTRL0_SPEC, bool, O>;
#[doc = "Field `OTG_ID_WAKEUP_EN` reader - No description avaiable"]
pub type OTG_ID_WAKEUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `OTG_ID_WAKEUP_EN` writer - No description avaiable"]
pub type OTG_ID_WAKEUP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_CTRL0_SPEC, bool, O>;
#[doc = "Field `OTG_VBUS_SOURCE_SEL` reader - No description avaiable"]
pub type OTG_VBUS_SOURCE_SEL_R = crate::BitReader<bool>;
#[doc = "Field `OTG_VBUS_SOURCE_SEL` writer - No description avaiable"]
pub type OTG_VBUS_SOURCE_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OTG_CTRL0_SPEC, bool, O>;
#[doc = "Field `OTG_UTMI_SUSPENDM_SW` reader - default 0 for naneng usbphy"]
pub type OTG_UTMI_SUSPENDM_SW_R = crate::BitReader<bool>;
#[doc = "Field `OTG_UTMI_SUSPENDM_SW` writer - default 0 for naneng usbphy"]
pub type OTG_UTMI_SUSPENDM_SW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OTG_CTRL0_SPEC, bool, O>;
#[doc = "Field `OTG_UTMI_RESET_SW` reader - default 1 for naneng usbphy"]
pub type OTG_UTMI_RESET_SW_R = crate::BitReader<bool>;
#[doc = "Field `OTG_UTMI_RESET_SW` writer - default 1 for naneng usbphy"]
pub type OTG_UTMI_RESET_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_CTRL0_SPEC, bool, O>;
#[doc = "Field `OTG_WAKEUP_INT_ENABLE` reader - No description avaiable"]
pub type OTG_WAKEUP_INT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `OTG_WAKEUP_INT_ENABLE` writer - No description avaiable"]
pub type OTG_WAKEUP_INT_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OTG_CTRL0_SPEC, bool, O>;
#[doc = "Field `OTG_POWER_MASK` reader - No description avaiable"]
pub type OTG_POWER_MASK_R = crate::BitReader<bool>;
#[doc = "Field `OTG_POWER_MASK` writer - No description avaiable"]
pub type OTG_POWER_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_CTRL0_SPEC, bool, O>;
#[doc = "Field `OTG_OVER_CUR_POL` reader - No description avaiable"]
pub type OTG_OVER_CUR_POL_R = crate::BitReader<bool>;
#[doc = "Field `OTG_OVER_CUR_POL` writer - No description avaiable"]
pub type OTG_OVER_CUR_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_CTRL0_SPEC, bool, O>;
#[doc = "Field `OTG_OVER_CUR_DIS` reader - No description avaiable"]
pub type OTG_OVER_CUR_DIS_R = crate::BitReader<bool>;
#[doc = "Field `OTG_OVER_CUR_DIS` writer - No description avaiable"]
pub type OTG_OVER_CUR_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_CTRL0_SPEC, bool, O>;
#[doc = "Field `SER_MODE_SUSPEND_EN` reader - for naneng usbphy, only switch to serial mode when suspend"]
pub type SER_MODE_SUSPEND_EN_R = crate::BitReader<bool>;
#[doc = "Field `SER_MODE_SUSPEND_EN` writer - for naneng usbphy, only switch to serial mode when suspend"]
pub type SER_MODE_SUSPEND_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OTG_CTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 25 - No description avaiable"]
    #[inline(always)]
    pub fn otg_wkdpdmchg_en(&self) -> OTG_WKDPDMCHG_EN_R {
        OTG_WKDPDMCHG_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 19 - No description avaiable"]
    #[inline(always)]
    pub fn autoresume_en(&self) -> AUTORESUME_EN_R {
        AUTORESUME_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 17 - No description avaiable"]
    #[inline(always)]
    pub fn otg_vbus_wakeup_en(&self) -> OTG_VBUS_WAKEUP_EN_R {
        OTG_VBUS_WAKEUP_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - No description avaiable"]
    #[inline(always)]
    pub fn otg_id_wakeup_en(&self) -> OTG_ID_WAKEUP_EN_R {
        OTG_ID_WAKEUP_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 13 - No description avaiable"]
    #[inline(always)]
    pub fn otg_vbus_source_sel(&self) -> OTG_VBUS_SOURCE_SEL_R {
        OTG_VBUS_SOURCE_SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - default 0 for naneng usbphy"]
    #[inline(always)]
    pub fn otg_utmi_suspendm_sw(&self) -> OTG_UTMI_SUSPENDM_SW_R {
        OTG_UTMI_SUSPENDM_SW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - default 1 for naneng usbphy"]
    #[inline(always)]
    pub fn otg_utmi_reset_sw(&self) -> OTG_UTMI_RESET_SW_R {
        OTG_UTMI_RESET_SW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - No description avaiable"]
    #[inline(always)]
    pub fn otg_wakeup_int_enable(&self) -> OTG_WAKEUP_INT_ENABLE_R {
        OTG_WAKEUP_INT_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - No description avaiable"]
    #[inline(always)]
    pub fn otg_power_mask(&self) -> OTG_POWER_MASK_R {
        OTG_POWER_MASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - No description avaiable"]
    #[inline(always)]
    pub fn otg_over_cur_pol(&self) -> OTG_OVER_CUR_POL_R {
        OTG_OVER_CUR_POL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - No description avaiable"]
    #[inline(always)]
    pub fn otg_over_cur_dis(&self) -> OTG_OVER_CUR_DIS_R {
        OTG_OVER_CUR_DIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 4 - for naneng usbphy, only switch to serial mode when suspend"]
    #[inline(always)]
    pub fn ser_mode_suspend_en(&self) -> SER_MODE_SUSPEND_EN_R {
        SER_MODE_SUSPEND_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - No description avaiable"]
    #[inline(always)]
    pub fn otg_wkdpdmchg_en(&mut self) -> OTG_WKDPDMCHG_EN_W<25> {
        OTG_WKDPDMCHG_EN_W::new(self)
    }
    #[doc = "Bit 19 - No description avaiable"]
    #[inline(always)]
    pub fn autoresume_en(&mut self) -> AUTORESUME_EN_W<19> {
        AUTORESUME_EN_W::new(self)
    }
    #[doc = "Bit 17 - No description avaiable"]
    #[inline(always)]
    pub fn otg_vbus_wakeup_en(&mut self) -> OTG_VBUS_WAKEUP_EN_W<17> {
        OTG_VBUS_WAKEUP_EN_W::new(self)
    }
    #[doc = "Bit 16 - No description avaiable"]
    #[inline(always)]
    pub fn otg_id_wakeup_en(&mut self) -> OTG_ID_WAKEUP_EN_W<16> {
        OTG_ID_WAKEUP_EN_W::new(self)
    }
    #[doc = "Bit 13 - No description avaiable"]
    #[inline(always)]
    pub fn otg_vbus_source_sel(&mut self) -> OTG_VBUS_SOURCE_SEL_W<13> {
        OTG_VBUS_SOURCE_SEL_W::new(self)
    }
    #[doc = "Bit 12 - default 0 for naneng usbphy"]
    #[inline(always)]
    pub fn otg_utmi_suspendm_sw(&mut self) -> OTG_UTMI_SUSPENDM_SW_W<12> {
        OTG_UTMI_SUSPENDM_SW_W::new(self)
    }
    #[doc = "Bit 11 - default 1 for naneng usbphy"]
    #[inline(always)]
    pub fn otg_utmi_reset_sw(&mut self) -> OTG_UTMI_RESET_SW_W<11> {
        OTG_UTMI_RESET_SW_W::new(self)
    }
    #[doc = "Bit 10 - No description avaiable"]
    #[inline(always)]
    pub fn otg_wakeup_int_enable(&mut self) -> OTG_WAKEUP_INT_ENABLE_W<10> {
        OTG_WAKEUP_INT_ENABLE_W::new(self)
    }
    #[doc = "Bit 9 - No description avaiable"]
    #[inline(always)]
    pub fn otg_power_mask(&mut self) -> OTG_POWER_MASK_W<9> {
        OTG_POWER_MASK_W::new(self)
    }
    #[doc = "Bit 8 - No description avaiable"]
    #[inline(always)]
    pub fn otg_over_cur_pol(&mut self) -> OTG_OVER_CUR_POL_W<8> {
        OTG_OVER_CUR_POL_W::new(self)
    }
    #[doc = "Bit 7 - No description avaiable"]
    #[inline(always)]
    pub fn otg_over_cur_dis(&mut self) -> OTG_OVER_CUR_DIS_W<7> {
        OTG_OVER_CUR_DIS_W::new(self)
    }
    #[doc = "Bit 4 - for naneng usbphy, only switch to serial mode when suspend"]
    #[inline(always)]
    pub fn ser_mode_suspend_en(&mut self) -> SER_MODE_SUSPEND_EN_W<4> {
        SER_MODE_SUSPEND_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_ctrl0](index.html) module"]
pub struct OTG_CTRL0_SPEC;
impl crate::RegisterSpec for OTG_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_ctrl0::R](R) reader structure"]
impl crate::Readable for OTG_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_ctrl0::W](W) writer structure"]
impl crate::Writable for OTG_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_CTRL0 to value 0"]
impl crate::Resettable for OTG_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
