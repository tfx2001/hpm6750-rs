#[doc = "Register `RGB2YUV_COEF3` reader"]
pub struct R(crate::R<RGB2YUV_COEF3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGB2YUV_COEF3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGB2YUV_COEF3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGB2YUV_COEF3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RGB2YUV_COEF3` writer"]
pub struct W(crate::W<RGB2YUV_COEF3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGB2YUV_COEF3_SPEC>;
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
impl From<crate::W<RGB2YUV_COEF3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGB2YUV_COEF3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C5` reader - CSC parameters C5"]
pub type C5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C5` writer - CSC parameters C5"]
pub type C5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGB2YUV_COEF3_SPEC, u16, u16, 11, O>;
#[doc = "Field `C6` reader - CSC parameters C6"]
pub type C6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C6` writer - CSC parameters C6"]
pub type C6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGB2YUV_COEF3_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - CSC parameters C5"]
    #[inline(always)]
    pub fn c5(&self) -> C5_R {
        C5_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - CSC parameters C6"]
    #[inline(always)]
    pub fn c6(&self) -> C6_R {
        C6_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - CSC parameters C5"]
    #[inline(always)]
    #[must_use]
    pub fn c5(&mut self) -> C5_W<0> {
        C5_W::new(self)
    }
    #[doc = "Bits 16:26 - CSC parameters C6"]
    #[inline(always)]
    #[must_use]
    pub fn c6(&mut self) -> C6_W<16> {
        C6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RGB2YUV coefficients register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgb2yuv_coef3](index.html) module"]
pub struct RGB2YUV_COEF3_SPEC;
impl crate::RegisterSpec for RGB2YUV_COEF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgb2yuv_coef3::R](R) reader structure"]
impl crate::Readable for RGB2YUV_COEF3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rgb2yuv_coef3::W](W) writer structure"]
impl crate::Writable for RGB2YUV_COEF3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RGB2YUV_COEF3 to value 0"]
impl crate::Resettable for RGB2YUV_COEF3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
