#[doc = "Register `RGB2YUV_COEF2` reader"]
pub struct R(crate::R<RGB2YUV_COEF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGB2YUV_COEF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGB2YUV_COEF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGB2YUV_COEF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RGB2YUV_COEF2` writer"]
pub struct W(crate::W<RGB2YUV_COEF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGB2YUV_COEF2_SPEC>;
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
impl From<crate::W<RGB2YUV_COEF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGB2YUV_COEF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C3` reader - CSC parameters C3"]
pub type C3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C3` writer - CSC parameters C3"]
pub type C3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGB2YUV_COEF2_SPEC, u16, u16, 11, O>;
#[doc = "Field `C2` reader - CSC parameters C2"]
pub type C2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C2` writer - CSC parameters C2"]
pub type C2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGB2YUV_COEF2_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - CSC parameters C3"]
    #[inline(always)]
    pub fn c3(&self) -> C3_R {
        C3_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - CSC parameters C2"]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - CSC parameters C3"]
    #[inline(always)]
    #[must_use]
    pub fn c3(&mut self) -> C3_W<0> {
        C3_W::new(self)
    }
    #[doc = "Bits 16:26 - CSC parameters C2"]
    #[inline(always)]
    #[must_use]
    pub fn c2(&mut self) -> C2_W<16> {
        C2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RGB2YUV coefficients Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgb2yuv_coef2](index.html) module"]
pub struct RGB2YUV_COEF2_SPEC;
impl crate::RegisterSpec for RGB2YUV_COEF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgb2yuv_coef2::R](R) reader structure"]
impl crate::Readable for RGB2YUV_COEF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rgb2yuv_coef2::W](W) writer structure"]
impl crate::Writable for RGB2YUV_COEF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RGB2YUV_COEF2 to value 0"]
impl crate::Resettable for RGB2YUV_COEF2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
