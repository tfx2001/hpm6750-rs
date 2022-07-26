#[doc = "Register `RGB2YUV_COEF4` reader"]
pub struct R(crate::R<RGB2YUV_COEF4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGB2YUV_COEF4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGB2YUV_COEF4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGB2YUV_COEF4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RGB2YUV_COEF4` writer"]
pub struct W(crate::W<RGB2YUV_COEF4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGB2YUV_COEF4_SPEC>;
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
impl From<crate::W<RGB2YUV_COEF4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGB2YUV_COEF4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C8` reader - CSC parameters C8"]
pub type C8_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C8` writer - CSC parameters C8"]
pub type C8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGB2YUV_COEF4_SPEC, u16, u16, 11, O>;
#[doc = "Field `C7` reader - CSC parameters C7"]
pub type C7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C7` writer - CSC parameters C7"]
pub type C7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGB2YUV_COEF4_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 16:26 - CSC parameters C8"]
    #[inline(always)]
    pub fn c8(&self) -> C8_R {
        C8_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10 - CSC parameters C7"]
    #[inline(always)]
    pub fn c7(&self) -> C7_R {
        C7_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:26 - CSC parameters C8"]
    #[inline(always)]
    pub fn c8(&mut self) -> C8_W<16> {
        C8_W::new(self)
    }
    #[doc = "Bits 0:10 - CSC parameters C7"]
    #[inline(always)]
    pub fn c7(&mut self) -> C7_W<0> {
        C7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RGB2YUV coefficients Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgb2yuv_coef4](index.html) module"]
pub struct RGB2YUV_COEF4_SPEC;
impl crate::RegisterSpec for RGB2YUV_COEF4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgb2yuv_coef4::R](R) reader structure"]
impl crate::Readable for RGB2YUV_COEF4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rgb2yuv_coef4::W](W) writer structure"]
impl crate::Writable for RGB2YUV_COEF4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RGB2YUV_COEF4 to value 0"]
impl crate::Resettable for RGB2YUV_COEF4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
