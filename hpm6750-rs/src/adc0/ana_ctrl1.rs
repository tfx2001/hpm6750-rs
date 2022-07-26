#[doc = "Register `ANA_CTRL1` reader"]
pub struct R(crate::R<ANA_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_CTRL1` writer"]
pub struct W(crate::W<ANA_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_CTRL1_SPEC>;
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
impl From<crate::W<ANA_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELRES` reader - 11-12bit 10-10bit 01-8bit 00-6bit"]
pub type SELRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELRES` writer - 11-12bit 10-10bit 01-8bit 00-6bit"]
pub type SELRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANA_CTRL1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 6:7 - 11-12bit 10-10bit 01-8bit 00-6bit"]
    #[inline(always)]
    pub fn selres(&self) -> SELRES_R {
        SELRES_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - 11-12bit 10-10bit 01-8bit 00-6bit"]
    #[inline(always)]
    pub fn selres(&mut self) -> SELRES_W<6> {
        SELRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_ctrl1](index.html) module"]
pub struct ANA_CTRL1_SPEC;
impl crate::RegisterSpec for ANA_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_ctrl1::R](R) reader structure"]
impl crate::Readable for ANA_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_ctrl1::W](W) writer structure"]
impl crate::Writable for ANA_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANA_CTRL1 to value 0"]
impl crate::Resettable for ANA_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
