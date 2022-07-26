#[doc = "Register `PHY_CTRL0` reader"]
pub struct R(crate::R<PHY_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHY_CTRL0` writer"]
pub struct W(crate::W<PHY_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CTRL0_SPEC>;
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
impl From<crate::W<PHY_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_ID_SEL_N` reader - No description avaiable"]
pub type GPIO_ID_SEL_N_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_ID_SEL_N` writer - No description avaiable"]
pub type GPIO_ID_SEL_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CTRL0_SPEC, bool, O>;
#[doc = "Field `ID_DIG_OVERRIDE` reader - No description avaiable"]
pub type ID_DIG_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `ID_DIG_OVERRIDE` writer - No description avaiable"]
pub type ID_DIG_OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CTRL0_SPEC, bool, O>;
#[doc = "Field `SESS_VALID_OVERRIDE` reader - No description avaiable"]
pub type SESS_VALID_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `SESS_VALID_OVERRIDE` writer - No description avaiable"]
pub type SESS_VALID_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PHY_CTRL0_SPEC, bool, O>;
#[doc = "Field `VBUS_VALID_OVERRIDE` reader - No description avaiable"]
pub type VBUS_VALID_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `VBUS_VALID_OVERRIDE` writer - No description avaiable"]
pub type VBUS_VALID_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PHY_CTRL0_SPEC, bool, O>;
#[doc = "Field `ID_DIG_OVERRIDE_EN` reader - No description avaiable"]
pub type ID_DIG_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `ID_DIG_OVERRIDE_EN` writer - No description avaiable"]
pub type ID_DIG_OVERRIDE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_CTRL0_SPEC, bool, O>;
#[doc = "Field `SESS_VALID_OVERRIDE_EN` reader - No description avaiable"]
pub type SESS_VALID_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `SESS_VALID_OVERRIDE_EN` writer - No description avaiable"]
pub type SESS_VALID_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PHY_CTRL0_SPEC, bool, O>;
#[doc = "Field `VBUS_VALID_OVERRIDE_EN` reader - No description avaiable"]
pub type VBUS_VALID_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `VBUS_VALID_OVERRIDE_EN` writer - No description avaiable"]
pub type VBUS_VALID_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PHY_CTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 25 - No description avaiable"]
    #[inline(always)]
    pub fn gpio_id_sel_n(&self) -> GPIO_ID_SEL_N_R {
        GPIO_ID_SEL_N_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 14 - No description avaiable"]
    #[inline(always)]
    pub fn id_dig_override(&self) -> ID_DIG_OVERRIDE_R {
        ID_DIG_OVERRIDE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - No description avaiable"]
    #[inline(always)]
    pub fn sess_valid_override(&self) -> SESS_VALID_OVERRIDE_R {
        SESS_VALID_OVERRIDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - No description avaiable"]
    #[inline(always)]
    pub fn vbus_valid_override(&self) -> VBUS_VALID_OVERRIDE_R {
        VBUS_VALID_OVERRIDE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn id_dig_override_en(&self) -> ID_DIG_OVERRIDE_EN_R {
        ID_DIG_OVERRIDE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn sess_valid_override_en(&self) -> SESS_VALID_OVERRIDE_EN_R {
        SESS_VALID_OVERRIDE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn vbus_valid_override_en(&self) -> VBUS_VALID_OVERRIDE_EN_R {
        VBUS_VALID_OVERRIDE_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - No description avaiable"]
    #[inline(always)]
    pub fn gpio_id_sel_n(&mut self) -> GPIO_ID_SEL_N_W<25> {
        GPIO_ID_SEL_N_W::new(self)
    }
    #[doc = "Bit 14 - No description avaiable"]
    #[inline(always)]
    pub fn id_dig_override(&mut self) -> ID_DIG_OVERRIDE_W<14> {
        ID_DIG_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 13 - No description avaiable"]
    #[inline(always)]
    pub fn sess_valid_override(&mut self) -> SESS_VALID_OVERRIDE_W<13> {
        SESS_VALID_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 12 - No description avaiable"]
    #[inline(always)]
    pub fn vbus_valid_override(&mut self) -> VBUS_VALID_OVERRIDE_W<12> {
        VBUS_VALID_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn id_dig_override_en(&mut self) -> ID_DIG_OVERRIDE_EN_W<2> {
        ID_DIG_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn sess_valid_override_en(&mut self) -> SESS_VALID_OVERRIDE_EN_W<1> {
        SESS_VALID_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn vbus_valid_override_en(&mut self) -> VBUS_VALID_OVERRIDE_EN_W<0> {
        VBUS_VALID_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_ctrl0](index.html) module"]
pub struct PHY_CTRL0_SPEC;
impl crate::RegisterSpec for PHY_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_ctrl0::R](R) reader structure"]
impl crate::Readable for PHY_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_ctrl0::W](W) writer structure"]
impl crate::Writable for PHY_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PHY_CTRL0 to value 0"]
impl crate::Resettable for PHY_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
