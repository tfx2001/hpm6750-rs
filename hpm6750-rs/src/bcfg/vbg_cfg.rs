#[doc = "Register `VBG_CFG` reader"]
pub struct R(crate::R<VBG_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBG_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBG_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBG_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VBG_CFG` writer"]
pub struct W(crate::W<VBG_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VBG_CFG_SPEC>;
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
impl From<crate::W<VBG_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VBG_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBG_TRIMMED` reader - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
pub type VBG_TRIMMED_R = crate::BitReader<bool>;
#[doc = "Field `VBG_TRIMMED` writer - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
pub type VBG_TRIMMED_W<'a, const O: u8> = crate::BitWriter<'a, u32, VBG_CFG_SPEC, bool, O>;
#[doc = "Field `LP_MODE` reader - Bandgap works in low power mode 0: not in low power mode 1: bandgap work in low power mode"]
pub type LP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `LP_MODE` writer - Bandgap works in low power mode 0: not in low power mode 1: bandgap work in low power mode"]
pub type LP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VBG_CFG_SPEC, bool, O>;
#[doc = "Field `POWER_SAVE` reader - Bandgap works in power save mode 0: not in power save mode 1: bandgap work in power save mode"]
pub type POWER_SAVE_R = crate::BitReader<bool>;
#[doc = "Field `POWER_SAVE` writer - Bandgap works in power save mode 0: not in power save mode 1: bandgap work in power save mode"]
pub type POWER_SAVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VBG_CFG_SPEC, bool, O>;
#[doc = "Field `VBG_1P0` reader - Bandgap 1.0V output trim"]
pub type VBG_1P0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBG_1P0` writer - Bandgap 1.0V output trim"]
pub type VBG_1P0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VBG_CFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `VBG_P65` reader - Bandgap 0.65V output trim"]
pub type VBG_P65_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBG_P65` writer - Bandgap 0.65V output trim"]
pub type VBG_P65_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VBG_CFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `VBG_P50` reader - Bandgap 0.50V output trim"]
pub type VBG_P50_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBG_P50` writer - Bandgap 0.50V output trim"]
pub type VBG_P50_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VBG_CFG_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 31 - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
    #[inline(always)]
    pub fn vbg_trimmed(&self) -> VBG_TRIMMED_R {
        VBG_TRIMMED_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 25 - Bandgap works in low power mode 0: not in low power mode 1: bandgap work in low power mode"]
    #[inline(always)]
    pub fn lp_mode(&self) -> LP_MODE_R {
        LP_MODE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - Bandgap works in power save mode 0: not in power save mode 1: bandgap work in power save mode"]
    #[inline(always)]
    pub fn power_save(&self) -> POWER_SAVE_R {
        POWER_SAVE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Bandgap 1.0V output trim"]
    #[inline(always)]
    pub fn vbg_1p0(&self) -> VBG_1P0_R {
        VBG_1P0_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Bandgap 0.65V output trim"]
    #[inline(always)]
    pub fn vbg_p65(&self) -> VBG_P65_R {
        VBG_P65_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - Bandgap 0.50V output trim"]
    #[inline(always)]
    pub fn vbg_p50(&self) -> VBG_P50_R {
        VBG_P50_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
    #[inline(always)]
    pub fn vbg_trimmed(&mut self) -> VBG_TRIMMED_W<31> {
        VBG_TRIMMED_W::new(self)
    }
    #[doc = "Bit 25 - Bandgap works in low power mode 0: not in low power mode 1: bandgap work in low power mode"]
    #[inline(always)]
    pub fn lp_mode(&mut self) -> LP_MODE_W<25> {
        LP_MODE_W::new(self)
    }
    #[doc = "Bit 24 - Bandgap works in power save mode 0: not in power save mode 1: bandgap work in power save mode"]
    #[inline(always)]
    pub fn power_save(&mut self) -> POWER_SAVE_W<24> {
        POWER_SAVE_W::new(self)
    }
    #[doc = "Bits 16:20 - Bandgap 1.0V output trim"]
    #[inline(always)]
    pub fn vbg_1p0(&mut self) -> VBG_1P0_W<16> {
        VBG_1P0_W::new(self)
    }
    #[doc = "Bits 8:12 - Bandgap 0.65V output trim"]
    #[inline(always)]
    pub fn vbg_p65(&mut self) -> VBG_P65_W<8> {
        VBG_P65_W::new(self)
    }
    #[doc = "Bits 0:4 - Bandgap 0.50V output trim"]
    #[inline(always)]
    pub fn vbg_p50(&mut self) -> VBG_P50_W<0> {
        VBG_P50_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bandgap config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbg_cfg](index.html) module"]
pub struct VBG_CFG_SPEC;
impl crate::RegisterSpec for VBG_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vbg_cfg::R](R) reader structure"]
impl crate::Readable for VBG_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vbg_cfg::W](W) writer structure"]
impl crate::Writable for VBG_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VBG_CFG to value 0"]
impl crate::Resettable for VBG_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
