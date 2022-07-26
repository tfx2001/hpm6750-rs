#[doc = "Register `BANDGAP` reader"]
pub struct R(crate::R<BANDGAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BANDGAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BANDGAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BANDGAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BANDGAP` writer"]
pub struct W(crate::W<BANDGAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BANDGAP_SPEC>;
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
impl From<crate::W<BANDGAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BANDGAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBG_TRIMMED` reader - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
pub type VBG_TRIMMED_R = crate::BitReader<bool>;
#[doc = "Field `VBG_TRIMMED` writer - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
pub type VBG_TRIMMED_W<'a, const O: u8> = crate::BitWriter<'a, u32, BANDGAP_SPEC, bool, O>;
#[doc = "Field `LOWPOWER_MODE` reader - Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode"]
pub type LOWPOWER_MODE_R = crate::BitReader<bool>;
#[doc = "Field `LOWPOWER_MODE` writer - Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode"]
pub type LOWPOWER_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BANDGAP_SPEC, bool, O>;
#[doc = "Field `POWER_SAVE` reader - Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode"]
pub type POWER_SAVE_R = crate::BitReader<bool>;
#[doc = "Field `POWER_SAVE` writer - Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode"]
pub type POWER_SAVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BANDGAP_SPEC, bool, O>;
#[doc = "Field `VBG_1P0_TRIM` reader - Banggap 1.0V output trim value"]
pub type VBG_1P0_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBG_1P0_TRIM` writer - Banggap 1.0V output trim value"]
pub type VBG_1P0_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BANDGAP_SPEC, u8, u8, 5, O>;
#[doc = "Field `VBG_P65_TRIM` reader - Banggap 1.0V output trim value"]
pub type VBG_P65_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBG_P65_TRIM` writer - Banggap 1.0V output trim value"]
pub type VBG_P65_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BANDGAP_SPEC, u8, u8, 5, O>;
#[doc = "Field `VBG_P50_TRIM` reader - Banggap 1.0V output trim value"]
pub type VBG_P50_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBG_P50_TRIM` writer - Banggap 1.0V output trim value"]
pub type VBG_P50_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BANDGAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 31 - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
    #[inline(always)]
    pub fn vbg_trimmed(&self) -> VBG_TRIMMED_R {
        VBG_TRIMMED_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 25 - Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode"]
    #[inline(always)]
    pub fn lowpower_mode(&self) -> LOWPOWER_MODE_R {
        LOWPOWER_MODE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode"]
    #[inline(always)]
    pub fn power_save(&self) -> POWER_SAVE_R {
        POWER_SAVE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Banggap 1.0V output trim value"]
    #[inline(always)]
    pub fn vbg_1p0_trim(&self) -> VBG_1P0_TRIM_R {
        VBG_1P0_TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Banggap 1.0V output trim value"]
    #[inline(always)]
    pub fn vbg_p65_trim(&self) -> VBG_P65_TRIM_R {
        VBG_P65_TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - Banggap 1.0V output trim value"]
    #[inline(always)]
    pub fn vbg_p50_trim(&self) -> VBG_P50_TRIM_R {
        VBG_P50_TRIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
    #[inline(always)]
    pub fn vbg_trimmed(&mut self) -> VBG_TRIMMED_W<31> {
        VBG_TRIMMED_W::new(self)
    }
    #[doc = "Bit 25 - Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode"]
    #[inline(always)]
    pub fn lowpower_mode(&mut self) -> LOWPOWER_MODE_W<25> {
        LOWPOWER_MODE_W::new(self)
    }
    #[doc = "Bit 24 - Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode"]
    #[inline(always)]
    pub fn power_save(&mut self) -> POWER_SAVE_W<24> {
        POWER_SAVE_W::new(self)
    }
    #[doc = "Bits 16:20 - Banggap 1.0V output trim value"]
    #[inline(always)]
    pub fn vbg_1p0_trim(&mut self) -> VBG_1P0_TRIM_W<16> {
        VBG_1P0_TRIM_W::new(self)
    }
    #[doc = "Bits 8:12 - Banggap 1.0V output trim value"]
    #[inline(always)]
    pub fn vbg_p65_trim(&mut self) -> VBG_P65_TRIM_W<8> {
        VBG_P65_TRIM_W::new(self)
    }
    #[doc = "Bits 0:4 - Banggap 1.0V output trim value"]
    #[inline(always)]
    pub fn vbg_p50_trim(&mut self) -> VBG_P50_TRIM_W<0> {
        VBG_P50_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BANGGAP control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bandgap](index.html) module"]
pub struct BANDGAP_SPEC;
impl crate::RegisterSpec for BANDGAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bandgap::R](R) reader structure"]
impl crate::Readable for BANDGAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bandgap::W](W) writer structure"]
impl crate::Writable for BANDGAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BANDGAP to value 0x0010_1010"]
impl crate::Resettable for BANDGAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_1010
    }
}
