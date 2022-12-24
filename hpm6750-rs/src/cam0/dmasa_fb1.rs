#[doc = "Register `DMASA_FB1` reader"]
pub struct R(crate::R<DMASA_FB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASA_FB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASA_FB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASA_FB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASA_FB1` writer"]
pub struct W(crate::W<DMASA_FB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASA_FB1_SPEC>;
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
impl From<crate::W<DMASA_FB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASA_FB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTR` reader - DMA Start Address in Frame Buffer1. Indicates the start address to write data. The embedded DMA controller will read data from RxFIFO and write it from this address through AHB bus. The address should be double words aligned. In Two-Plane Mode, Y buffer1"]
pub type PTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PTR` writer - DMA Start Address in Frame Buffer1. Indicates the start address to write data. The embedded DMA controller will read data from RxFIFO and write it from this address through AHB bus. The address should be double words aligned. In Two-Plane Mode, Y buffer1"]
pub type PTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMASA_FB1_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - DMA Start Address in Frame Buffer1. Indicates the start address to write data. The embedded DMA controller will read data from RxFIFO and write it from this address through AHB bus. The address should be double words aligned. In Two-Plane Mode, Y buffer1"]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - DMA Start Address in Frame Buffer1. Indicates the start address to write data. The embedded DMA controller will read data from RxFIFO and write it from this address through AHB bus. The address should be double words aligned. In Two-Plane Mode, Y buffer1"]
    #[inline(always)]
    #[must_use]
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
#[doc = "Pixel DMA Frame Buffer 1 Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasa_fb1](index.html) module"]
pub struct DMASA_FB1_SPEC;
impl crate::RegisterSpec for DMASA_FB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmasa_fb1::R](R) reader structure"]
impl crate::Readable for DMASA_FB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmasa_fb1::W](W) writer structure"]
impl crate::Writable for DMASA_FB1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMASA_FB1 to value 0"]
impl crate::Resettable for DMASA_FB1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
