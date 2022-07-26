#[doc = "Register `DCDC_CURRENT` reader"]
pub struct R(crate::R<DCDC_CURRENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC_CURRENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC_CURRENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC_CURRENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDC_CURRENT` writer"]
pub struct W(crate::W<DCDC_CURRENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC_CURRENT_SPEC>;
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
impl From<crate::W<DCDC_CURRENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC_CURRENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESTI_EN` reader - enable current measure"]
pub type ESTI_EN_R = crate::BitReader<bool>;
#[doc = "Field `ESTI_EN` writer - enable current measure"]
pub type ESTI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_CURRENT_SPEC, bool, O>;
#[doc = "Field `VALID` reader - Current level valid 0: data is invalid 1: data is valid"]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `LEVEL` reader - DCDC current level, current level is num * 50mA"]
pub type LEVEL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 15 - enable current measure"]
    #[inline(always)]
    pub fn esti_en(&self) -> ESTI_EN_R {
        ESTI_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 8 - Current level valid 0: data is invalid 1: data is valid"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:4 - DCDC current level, current level is num * 50mA"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - enable current measure"]
    #[inline(always)]
    pub fn esti_en(&mut self) -> ESTI_EN_W<15> {
        ESTI_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC current estimation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc_current](index.html) module"]
pub struct DCDC_CURRENT_SPEC;
impl crate::RegisterSpec for DCDC_CURRENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc_current::R](R) reader structure"]
impl crate::Readable for DCDC_CURRENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc_current::W](W) writer structure"]
impl crate::Writable for DCDC_CURRENT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDC_CURRENT to value 0"]
impl crate::Resettable for DCDC_CURRENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
