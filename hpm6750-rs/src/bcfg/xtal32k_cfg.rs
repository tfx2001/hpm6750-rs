#[doc = "Register `XTAL32K_CFG` reader"]
pub struct R(crate::R<XTAL32K_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32K_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32K_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32K_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL32K_CFG` writer"]
pub struct W(crate::W<XTAL32K_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32K_CFG_SPEC>;
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
impl From<crate::W<XTAL32K_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32K_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HYST_EN` reader - crystal 32k hysteres enable"]
pub type HYST_EN_R = crate::BitReader<bool>;
#[doc = "Field `HYST_EN` writer - crystal 32k hysteres enable"]
pub type HYST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTAL32K_CFG_SPEC, bool, O>;
#[doc = "Field `GMSEL` reader - crystal 32k gm selection"]
pub type GMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GMSEL` writer - crystal 32k gm selection"]
pub type GMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTAL32K_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CFG` reader - crystal 32k config"]
pub type CFG_R = crate::BitReader<bool>;
#[doc = "Field `CFG` writer - crystal 32k config"]
pub type CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTAL32K_CFG_SPEC, bool, O>;
#[doc = "Field `AMP` reader - crystal 32k amplifier"]
pub type AMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AMP` writer - crystal 32k amplifier"]
pub type AMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTAL32K_CFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 12 - crystal 32k hysteres enable"]
    #[inline(always)]
    pub fn hyst_en(&self) -> HYST_EN_R {
        HYST_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 8:9 - crystal 32k gm selection"]
    #[inline(always)]
    pub fn gmsel(&self) -> GMSEL_R {
        GMSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 4 - crystal 32k config"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:1 - crystal 32k amplifier"]
    #[inline(always)]
    pub fn amp(&self) -> AMP_R {
        AMP_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - crystal 32k hysteres enable"]
    #[inline(always)]
    pub fn hyst_en(&mut self) -> HYST_EN_W<12> {
        HYST_EN_W::new(self)
    }
    #[doc = "Bits 8:9 - crystal 32k gm selection"]
    #[inline(always)]
    pub fn gmsel(&mut self) -> GMSEL_W<8> {
        GMSEL_W::new(self)
    }
    #[doc = "Bit 4 - crystal 32k config"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CFG_W<4> {
        CFG_W::new(self)
    }
    #[doc = "Bits 0:1 - crystal 32k amplifier"]
    #[inline(always)]
    pub fn amp(&mut self) -> AMP_W<0> {
        AMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTAL 32K config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32k_cfg](index.html) module"]
pub struct XTAL32K_CFG_SPEC;
impl crate::RegisterSpec for XTAL32K_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal32k_cfg::R](R) reader structure"]
impl crate::Readable for XTAL32K_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32k_cfg::W](W) writer structure"]
impl crate::Writable for XTAL32K_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL32K_CFG to value 0"]
impl crate::Resettable for XTAL32K_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
