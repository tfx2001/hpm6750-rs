#[doc = "Register `YUV2RGB_COEF0` reader"]
pub struct R(crate::R<YUV2RGB_COEF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<YUV2RGB_COEF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<YUV2RGB_COEF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<YUV2RGB_COEF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `YUV2RGB_COEF0` writer"]
pub struct W(crate::W<YUV2RGB_COEF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<YUV2RGB_COEF0_SPEC>;
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
impl From<crate::W<YUV2RGB_COEF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<YUV2RGB_COEF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Y_OFFSET` reader - Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
pub type Y_OFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `Y_OFFSET` writer - Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
pub type Y_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, YUV2RGB_COEF0_SPEC, u16, u16, 9, O>;
#[doc = "Field `UV_OFFSET` reader - Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
pub type UV_OFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UV_OFFSET` writer - Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
pub type UV_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, YUV2RGB_COEF0_SPEC, u16, u16, 9, O>;
#[doc = "Field `C0` reader - Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
pub type C0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C0` writer - Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
pub type C0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, YUV2RGB_COEF0_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:8 - Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
    #[inline(always)]
    pub fn y_offset(&self) -> Y_OFFSET_R {
        Y_OFFSET_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
    #[inline(always)]
    pub fn uv_offset(&self) -> UV_OFFSET_R {
        UV_OFFSET_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:28 - Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    #[inline(always)]
    pub fn c0(&self) -> C0_R {
        C0_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
    #[inline(always)]
    #[must_use]
    pub fn y_offset(&mut self) -> Y_OFFSET_W<0> {
        Y_OFFSET_W::new(self)
    }
    #[doc = "Bits 9:17 - Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
    #[inline(always)]
    #[must_use]
    pub fn uv_offset(&mut self) -> UV_OFFSET_W<9> {
        UV_OFFSET_W::new(self)
    }
    #[doc = "Bits 18:28 - Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    #[inline(always)]
    #[must_use]
    pub fn c0(&mut self) -> C0_W<18> {
        C0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "YUV2RGB coefficients register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [yuv2rgb_coef0](index.html) module"]
pub struct YUV2RGB_COEF0_SPEC;
impl crate::RegisterSpec for YUV2RGB_COEF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [yuv2rgb_coef0::R](R) reader structure"]
impl crate::Readable for YUV2RGB_COEF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [yuv2rgb_coef0::W](W) writer structure"]
impl crate::Writable for YUV2RGB_COEF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets YUV2RGB_COEF0 to value 0"]
impl crate::Resettable for YUV2RGB_COEF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
