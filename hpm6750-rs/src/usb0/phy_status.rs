#[doc = "Register `PHY_STATUS` reader"]
pub struct R(crate::R<PHY_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHY_STATUS` writer"]
pub struct W(crate::W<PHY_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_STATUS_SPEC>;
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
impl From<crate::W<PHY_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTMI_CLK_VALID` reader - No description avaiable"]
pub type UTMI_CLK_VALID_R = crate::BitReader<bool>;
#[doc = "Field `UTMI_CLK_VALID` writer - No description avaiable"]
pub type UTMI_CLK_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_STATUS_SPEC, bool, O>;
#[doc = "Field `LINE_STATE` reader - No description avaiable"]
pub type LINE_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINE_STATE` writer - No description avaiable"]
pub type LINE_STATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHY_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `HOST_DISCONNECT` reader - No description avaiable"]
pub type HOST_DISCONNECT_R = crate::BitReader<bool>;
#[doc = "Field `HOST_DISCONNECT` writer - No description avaiable"]
pub type HOST_DISCONNECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_STATUS_SPEC, bool, O>;
#[doc = "Field `ID_DIG` reader - No description avaiable"]
pub type ID_DIG_R = crate::BitReader<bool>;
#[doc = "Field `ID_DIG` writer - No description avaiable"]
pub type ID_DIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_STATUS_SPEC, bool, O>;
#[doc = "Field `UTMI_SESS_VALID` reader - No description avaiable"]
pub type UTMI_SESS_VALID_R = crate::BitReader<bool>;
#[doc = "Field `UTMI_SESS_VALID` writer - No description avaiable"]
pub type UTMI_SESS_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_STATUS_SPEC, bool, O>;
#[doc = "Field `VBUS_VALID` reader - No description avaiable"]
pub type VBUS_VALID_R = crate::BitReader<bool>;
#[doc = "Field `VBUS_VALID` writer - No description avaiable"]
pub type VBUS_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - No description avaiable"]
    #[inline(always)]
    pub fn utmi_clk_valid(&self) -> UTMI_CLK_VALID_R {
        UTMI_CLK_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 6:7 - No description avaiable"]
    #[inline(always)]
    pub fn line_state(&self) -> LINE_STATE_R {
        LINE_STATE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 5 - No description avaiable"]
    #[inline(always)]
    pub fn host_disconnect(&self) -> HOST_DISCONNECT_R {
        HOST_DISCONNECT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - No description avaiable"]
    #[inline(always)]
    pub fn id_dig(&self) -> ID_DIG_R {
        ID_DIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn utmi_sess_valid(&self) -> UTMI_SESS_VALID_R {
        UTMI_SESS_VALID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn vbus_valid(&self) -> VBUS_VALID_R {
        VBUS_VALID_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - No description avaiable"]
    #[inline(always)]
    pub fn utmi_clk_valid(&mut self) -> UTMI_CLK_VALID_W<31> {
        UTMI_CLK_VALID_W::new(self)
    }
    #[doc = "Bits 6:7 - No description avaiable"]
    #[inline(always)]
    pub fn line_state(&mut self) -> LINE_STATE_W<6> {
        LINE_STATE_W::new(self)
    }
    #[doc = "Bit 5 - No description avaiable"]
    #[inline(always)]
    pub fn host_disconnect(&mut self) -> HOST_DISCONNECT_W<5> {
        HOST_DISCONNECT_W::new(self)
    }
    #[doc = "Bit 4 - No description avaiable"]
    #[inline(always)]
    pub fn id_dig(&mut self) -> ID_DIG_W<4> {
        ID_DIG_W::new(self)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn utmi_sess_valid(&mut self) -> UTMI_SESS_VALID_W<2> {
        UTMI_SESS_VALID_W::new(self)
    }
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn vbus_valid(&mut self) -> VBUS_VALID_W<0> {
        VBUS_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_status](index.html) module"]
pub struct PHY_STATUS_SPEC;
impl crate::RegisterSpec for PHY_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_status::R](R) reader structure"]
impl crate::Readable for PHY_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_status::W](W) writer structure"]
impl crate::Writable for PHY_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PHY_STATUS to value 0"]
impl crate::Resettable for PHY_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
