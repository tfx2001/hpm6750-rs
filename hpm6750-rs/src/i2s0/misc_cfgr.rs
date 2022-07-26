#[doc = "Register `MISC_CFGR` reader"]
pub struct R(crate::R<MISC_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_CFGR` writer"]
pub struct W(crate::W<MISC_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_CFGR_SPEC>;
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
impl From<crate::W<MISC_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCLK_GATEOFF` reader - Gate off the mclk. This mclk is the output of a glitch prone mux, so every time to switch the mclk, the gate off clock should be asserted at first. After the clock is switched, de-assert this bit to ungate off the mclk."]
pub type MCLK_GATEOFF_R = crate::BitReader<bool>;
#[doc = "Field `MCLK_GATEOFF` writer - Gate off the mclk. This mclk is the output of a glitch prone mux, so every time to switch the mclk, the gate off clock should be asserted at first. After the clock is switched, de-assert this bit to ungate off the mclk."]
pub type MCLK_GATEOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC_CFGR_SPEC, bool, O>;
#[doc = "Field `MCLKOE` reader - Master clock output to pad enable 0: Master clock output is disabled 1: Master clock output is enabled Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
pub type MCLKOE_R = crate::BitReader<bool>;
#[doc = "Field `MCLKOE` writer - Master clock output to pad enable 0: Master clock output is disabled 1: Master clock output is enabled Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
pub type MCLKOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC_CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 13 - Gate off the mclk. This mclk is the output of a glitch prone mux, so every time to switch the mclk, the gate off clock should be asserted at first. After the clock is switched, de-assert this bit to ungate off the mclk."]
    #[inline(always)]
    pub fn mclk_gateoff(&self) -> MCLK_GATEOFF_R {
        MCLK_GATEOFF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 0 - Master clock output to pad enable 0: Master clock output is disabled 1: Master clock output is enabled Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
    #[inline(always)]
    pub fn mclkoe(&self) -> MCLKOE_R {
        MCLKOE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Gate off the mclk. This mclk is the output of a glitch prone mux, so every time to switch the mclk, the gate off clock should be asserted at first. After the clock is switched, de-assert this bit to ungate off the mclk."]
    #[inline(always)]
    pub fn mclk_gateoff(&mut self) -> MCLK_GATEOFF_W<13> {
        MCLK_GATEOFF_W::new(self)
    }
    #[doc = "Bit 0 - Master clock output to pad enable 0: Master clock output is disabled 1: Master clock output is enabled Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
    #[inline(always)]
    pub fn mclkoe(&mut self) -> MCLKOE_W<0> {
        MCLKOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Misc configuration Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_cfgr](index.html) module"]
pub struct MISC_CFGR_SPEC;
impl crate::RegisterSpec for MISC_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_cfgr::R](R) reader structure"]
impl crate::Readable for MISC_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_cfgr::W](W) writer structure"]
impl crate::Writable for MISC_CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC_CFGR to value 0x0004_2000"]
impl crate::Resettable for MISC_CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_2000
    }
}
