#[doc = "Register `DMASA_UV2` reader"]
pub struct R(crate::R<DMASA_UV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASA_UV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASA_UV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASA_UV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASA_UV2` writer"]
pub struct W(crate::W<DMASA_UV2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASA_UV2_SPEC>;
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
impl From<crate::W<DMASA_UV2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASA_UV2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTR` reader - Two Plane UV Buffer Start Address 2"]
pub type PTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PTR` writer - Two Plane UV Buffer Start Address 2"]
pub type PTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMASA_UV2_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Two Plane UV Buffer Start Address 2"]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Two Plane UV Buffer Start Address 2"]
    #[inline(always)]
    pub fn ptr(&mut self) -> PTR_W<2> {
        PTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pixel UV DMA Frame Buffer 2 Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasa_uv2](index.html) module"]
pub struct DMASA_UV2_SPEC;
impl crate::RegisterSpec for DMASA_UV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmasa_uv2::R](R) reader structure"]
impl crate::Readable for DMASA_UV2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmasa_uv2::W](W) writer structure"]
impl crate::Writable for DMASA_UV2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMASA_UV2 to value 0"]
impl crate::Resettable for DMASA_UV2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
