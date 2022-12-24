#[doc = "Register `DEC_CTRL1` reader"]
pub struct R(crate::R<DEC_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEC_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEC_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEC_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEC_CTRL1` writer"]
pub struct W(crate::W<DEC_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEC_CTRL1_SPEC>;
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
impl From<crate::W<DEC_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEC_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZCR_LOW` reader - ZCR low limit"]
pub type ZCR_LOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ZCR_LOW` writer - ZCR low limit"]
pub type ZCR_LOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEC_CTRL1_SPEC, u16, u16, 11, O>;
#[doc = "Field `ZCR_HIGH` reader - ZCR high limit"]
pub type ZCR_HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ZCR_HIGH` writer - ZCR high limit"]
pub type ZCR_HIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEC_CTRL1_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - ZCR low limit"]
    #[inline(always)]
    pub fn zcr_low(&self) -> ZCR_LOW_R {
        ZCR_LOW_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - ZCR high limit"]
    #[inline(always)]
    pub fn zcr_high(&self) -> ZCR_HIGH_R {
        ZCR_HIGH_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - ZCR low limit"]
    #[inline(always)]
    #[must_use]
    pub fn zcr_low(&mut self) -> ZCR_LOW_W<0> {
        ZCR_LOW_W::new(self)
    }
    #[doc = "Bits 11:21 - ZCR high limit"]
    #[inline(always)]
    #[must_use]
    pub fn zcr_high(&mut self) -> ZCR_HIGH_W<11> {
        ZCR_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Decision Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dec_ctrl1](index.html) module"]
pub struct DEC_CTRL1_SPEC;
impl crate::RegisterSpec for DEC_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dec_ctrl1::R](R) reader structure"]
impl crate::Readable for DEC_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dec_ctrl1::W](W) writer structure"]
impl crate::Writable for DEC_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEC_CTRL1 to value 0"]
impl crate::Resettable for DEC_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
