#[doc = "Register `BUF_PARA` reader"]
pub struct R(crate::R<BUF_PARA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_PARA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_PARA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_PARA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF_PARA` writer"]
pub struct W(crate::W<BUF_PARA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_PARA_SPEC>;
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
impl From<crate::W<BUF_PARA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_PARA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINEBSP_STRIDE` reader - Line Blank Space Stride. Indicates the space between the end of line image storage and the start of a new line storage in the frame buffer. The width of the line storage in frame buffer(in double words) minus the width of the image(in double words) is the stride. The stride should be double words aligned. The embedded DMA controller will skip the stride before starting to write the next row of the image."]
pub type LINEBSP_STRIDE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINEBSP_STRIDE` writer - Line Blank Space Stride. Indicates the space between the end of line image storage and the start of a new line storage in the frame buffer. The width of the line storage in frame buffer(in double words) minus the width of the image(in double words) is the stride. The stride should be double words aligned. The embedded DMA controller will skip the stride before starting to write the next row of the image."]
pub type LINEBSP_STRIDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BUF_PARA_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Line Blank Space Stride. Indicates the space between the end of line image storage and the start of a new line storage in the frame buffer. The width of the line storage in frame buffer(in double words) minus the width of the image(in double words) is the stride. The stride should be double words aligned. The embedded DMA controller will skip the stride before starting to write the next row of the image."]
    #[inline(always)]
    pub fn linebsp_stride(&self) -> LINEBSP_STRIDE_R {
        LINEBSP_STRIDE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Line Blank Space Stride. Indicates the space between the end of line image storage and the start of a new line storage in the frame buffer. The width of the line storage in frame buffer(in double words) minus the width of the image(in double words) is the stride. The stride should be double words aligned. The embedded DMA controller will skip the stride before starting to write the next row of the image."]
    #[inline(always)]
    pub fn linebsp_stride(&mut self) -> LINEBSP_STRIDE_W<0> {
        LINEBSP_STRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer Parameters Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_para](index.html) module"]
pub struct BUF_PARA_SPEC;
impl crate::RegisterSpec for BUF_PARA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_para::R](R) reader structure"]
impl crate::Readable for BUF_PARA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_para::W](W) writer structure"]
impl crate::Writable for BUF_PARA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF_PARA to value 0"]
impl crate::Resettable for BUF_PARA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
