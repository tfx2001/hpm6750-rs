#[doc = "Register `OUT_PS_0_LRC` reader"]
pub struct R(crate::R<OUT_PS_0_LRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_PS_0_LRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_PS_0_LRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_PS_0_LRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_PS_0_LRC` writer"]
pub struct W(crate::W<OUT_PS_0_LRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_PS_0_LRC_SPEC>;
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
impl From<crate::W<OUT_PS_0_LRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_PS_0_LRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Y` reader - This field indicates the lower right Y-coordinate (in pixels) of the processed surface in the output frame buffer."]
pub type Y_R = crate::FieldReader<u16, u16>;
#[doc = "Field `Y` writer - This field indicates the lower right Y-coordinate (in pixels) of the processed surface in the output frame buffer."]
pub type Y_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_PS_0_LRC_SPEC, u16, u16, 14, O>;
#[doc = "Field `X` reader - This field indicates the lower right X-coordinate (in pixels) of the processed surface in the output frame buffer."]
pub type X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `X` writer - This field indicates the lower right X-coordinate (in pixels) of the processed surface in the output frame buffer."]
pub type X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_PS_0_LRC_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 16:29 - This field indicates the lower right Y-coordinate (in pixels) of the processed surface in the output frame buffer."]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 0:13 - This field indicates the lower right X-coordinate (in pixels) of the processed surface in the output frame buffer."]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:29 - This field indicates the lower right Y-coordinate (in pixels) of the processed surface in the output frame buffer."]
    #[inline(always)]
    pub fn y(&mut self) -> Y_W<16> {
        Y_W::new(self)
    }
    #[doc = "Bits 0:13 - This field indicates the lower right X-coordinate (in pixels) of the processed surface in the output frame buffer."]
    #[inline(always)]
    pub fn x(&mut self) -> X_W<0> {
        X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer Lower Right Corner Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_ps_0_lrc](index.html) module"]
pub struct OUT_PS_0_LRC_SPEC;
impl crate::RegisterSpec for OUT_PS_0_LRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_ps_0_lrc::R](R) reader structure"]
impl crate::Readable for OUT_PS_0_LRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_ps_0_lrc::W](W) writer structure"]
impl crate::Writable for OUT_PS_0_LRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_PS_0_LRC to value 0"]
impl crate::Resettable for OUT_PS_0_LRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
