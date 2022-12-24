#[doc = "Register `RGB2YUV_COEF0` reader"]
pub struct R(crate::R<RGB2YUV_COEF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGB2YUV_COEF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGB2YUV_COEF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGB2YUV_COEF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RGB2YUV_COEF0` writer"]
pub struct W(crate::W<RGB2YUV_COEF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGB2YUV_COEF0_SPEC>;
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
impl From<crate::W<RGB2YUV_COEF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGB2YUV_COEF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Y_OFFSET` reader - CSC parameters Y_OFFSET"]
pub type Y_OFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `Y_OFFSET` writer - CSC parameters Y_OFFSET"]
pub type Y_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RGB2YUV_COEF0_SPEC, u16, u16, 9, O>;
#[doc = "Field `UV_OFFSET` reader - CSC parameters UV_OFFSET"]
pub type UV_OFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UV_OFFSET` writer - CSC parameters UV_OFFSET"]
pub type UV_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RGB2YUV_COEF0_SPEC, u16, u16, 9, O>;
#[doc = "Field `C0` reader - CSC parameters C0"]
pub type C0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C0` writer - CSC parameters C0"]
pub type C0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGB2YUV_COEF0_SPEC, u16, u16, 11, O>;
#[doc = "Field `ENABLE` reader - Asserted to enable this RGB2YUV CSC stage"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Asserted to enable this RGB2YUV CSC stage"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGB2YUV_COEF0_SPEC, bool, O>;
#[doc = "Field `YCBCR_MODE` reader - Asserted to use YCrCb mode"]
pub type YCBCR_MODE_R = crate::BitReader<bool>;
#[doc = "Field `YCBCR_MODE` writer - Asserted to use YCrCb mode"]
pub type YCBCR_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGB2YUV_COEF0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - CSC parameters Y_OFFSET"]
    #[inline(always)]
    pub fn y_offset(&self) -> Y_OFFSET_R {
        Y_OFFSET_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - CSC parameters UV_OFFSET"]
    #[inline(always)]
    pub fn uv_offset(&self) -> UV_OFFSET_R {
        UV_OFFSET_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:28 - CSC parameters C0"]
    #[inline(always)]
    pub fn c0(&self) -> C0_R {
        C0_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bit 30 - Asserted to enable this RGB2YUV CSC stage"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Asserted to use YCrCb mode"]
    #[inline(always)]
    pub fn ycbcr_mode(&self) -> YCBCR_MODE_R {
        YCBCR_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - CSC parameters Y_OFFSET"]
    #[inline(always)]
    #[must_use]
    pub fn y_offset(&mut self) -> Y_OFFSET_W<0> {
        Y_OFFSET_W::new(self)
    }
    #[doc = "Bits 9:17 - CSC parameters UV_OFFSET"]
    #[inline(always)]
    #[must_use]
    pub fn uv_offset(&mut self) -> UV_OFFSET_W<9> {
        UV_OFFSET_W::new(self)
    }
    #[doc = "Bits 18:28 - CSC parameters C0"]
    #[inline(always)]
    #[must_use]
    pub fn c0(&mut self) -> C0_W<18> {
        C0_W::new(self)
    }
    #[doc = "Bit 30 - Asserted to enable this RGB2YUV CSC stage"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<30> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 31 - Asserted to use YCrCb mode"]
    #[inline(always)]
    #[must_use]
    pub fn ycbcr_mode(&mut self) -> YCBCR_MODE_W<31> {
        YCBCR_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RGB2YUV coefficients register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgb2yuv_coef0](index.html) module"]
pub struct RGB2YUV_COEF0_SPEC;
impl crate::RegisterSpec for RGB2YUV_COEF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgb2yuv_coef0::R](R) reader structure"]
impl crate::Readable for RGB2YUV_COEF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rgb2yuv_coef0::W](W) writer structure"]
impl crate::Writable for RGB2YUV_COEF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RGB2YUV_COEF0 to value 0"]
impl crate::Resettable for RGB2YUV_COEF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
