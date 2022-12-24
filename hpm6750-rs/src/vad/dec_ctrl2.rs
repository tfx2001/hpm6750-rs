#[doc = "Register `DEC_CTRL2` reader"]
pub struct R(crate::R<DEC_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEC_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEC_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEC_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEC_CTRL2` writer"]
pub struct W(crate::W<DEC_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEC_CTRL2_SPEC>;
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
impl From<crate::W<DEC_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEC_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMP_LOW` reader - amplitude low limit"]
pub type AMP_LOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AMP_LOW` writer - amplitude low limit"]
pub type AMP_LOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEC_CTRL2_SPEC, u16, u16, 16, O>;
#[doc = "Field `AMP_HIGH` reader - amplitude high limit"]
pub type AMP_HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AMP_HIGH` writer - amplitude high limit"]
pub type AMP_HIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEC_CTRL2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - amplitude low limit"]
    #[inline(always)]
    pub fn amp_low(&self) -> AMP_LOW_R {
        AMP_LOW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - amplitude high limit"]
    #[inline(always)]
    pub fn amp_high(&self) -> AMP_HIGH_R {
        AMP_HIGH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - amplitude low limit"]
    #[inline(always)]
    #[must_use]
    pub fn amp_low(&mut self) -> AMP_LOW_W<0> {
        AMP_LOW_W::new(self)
    }
    #[doc = "Bits 16:31 - amplitude high limit"]
    #[inline(always)]
    #[must_use]
    pub fn amp_high(&mut self) -> AMP_HIGH_W<16> {
        AMP_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Decision Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dec_ctrl2](index.html) module"]
pub struct DEC_CTRL2_SPEC;
impl crate::RegisterSpec for DEC_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dec_ctrl2::R](R) reader structure"]
impl crate::Readable for DEC_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dec_ctrl2::W](W) writer structure"]
impl crate::Writable for DEC_CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEC_CTRL2 to value 0"]
impl crate::Resettable for DEC_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
