#[doc = "Register `LAYER_4_LAYSIZE` reader"]
pub struct R(crate::R<LAYER_4_LAYSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYER_4_LAYSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYER_4_LAYSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYER_4_LAYSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYER_4_LAYSIZE` writer"]
pub struct W(crate::W<LAYER_4_LAYSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYER_4_LAYSIZE_SPEC>;
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
impl From<crate::W<LAYER_4_LAYSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYER_4_LAYSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIDTH` reader - Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16."]
pub type WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WIDTH` writer - Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16."]
pub type WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYER_4_LAYSIZE_SPEC, u16, u16, 12, O>;
#[doc = "Field `HEIGHT` reader - Height of the layer in pixels"]
pub type HEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HEIGHT` writer - Height of the layer in pixels"]
pub type HEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYER_4_LAYSIZE_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Height of the layer in pixels"]
    #[inline(always)]
    pub fn height(&self) -> HEIGHT_R {
        HEIGHT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<0> {
        WIDTH_W::new(self)
    }
    #[doc = "Bits 16:27 - Height of the layer in pixels"]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HEIGHT_W<16> {
        HEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layer_4_laysize](index.html) module"]
pub struct LAYER_4_LAYSIZE_SPEC;
impl crate::RegisterSpec for LAYER_4_LAYSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layer_4_laysize::R](R) reader structure"]
impl crate::Readable for LAYER_4_LAYSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layer_4_laysize::W](W) writer structure"]
impl crate::Writable for LAYER_4_LAYSIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYER_4_LAYSIZE to value 0"]
impl crate::Resettable for LAYER_4_LAYSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
