#[doc = "Register `LAYER_3_BG_CL` reader"]
pub struct R(crate::R<LAYER_3_BG_CL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYER_3_BG_CL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYER_3_BG_CL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYER_3_BG_CL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYER_3_BG_CL` writer"]
pub struct W(crate::W<LAYER_3_BG_CL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYER_3_BG_CL_SPEC>;
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
impl From<crate::W<LAYER_3_BG_CL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYER_3_BG_CL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARGB` reader - ARGB8888. It is only useful in the last active stage in the pipeline."]
pub type ARGB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ARGB` writer - ARGB8888. It is only useful in the last active stage in the pipeline."]
pub type ARGB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LAYER_3_BG_CL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ARGB8888. It is only useful in the last active stage in the pipeline."]
    #[inline(always)]
    pub fn argb(&self) -> ARGB_R {
        ARGB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ARGB8888. It is only useful in the last active stage in the pipeline."]
    #[inline(always)]
    pub fn argb(&mut self) -> ARGB_W<0> {
        ARGB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer Background Color Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layer_3_bg_cl](index.html) module"]
pub struct LAYER_3_BG_CL_SPEC;
impl crate::RegisterSpec for LAYER_3_BG_CL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layer_3_bg_cl::R](R) reader structure"]
impl crate::Readable for LAYER_3_BG_CL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layer_3_bg_cl::W](W) writer structure"]
impl crate::Writable for LAYER_3_BG_CL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LAYER_3_BG_CL to value 0"]
impl crate::Resettable for LAYER_3_BG_CL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
