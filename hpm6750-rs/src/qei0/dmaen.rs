#[doc = "Register `DMAEN` reader"]
pub struct R(crate::R<DMAEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAEN` writer"]
pub struct W(crate::W<DMAEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAEN_SPEC>;
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
impl From<crate::W<DMAEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDGFEN` reader - 1- generate dma request when wdg flag set"]
pub type WDGFEN_R = crate::BitReader<bool>;
#[doc = "Field `WDGFEN` writer - 1- generate dma request when wdg flag set"]
pub type WDGFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEN_SPEC, bool, O>;
#[doc = "Field `HOMEFEN` reader - 1- generate dma request when homef flag set"]
pub type HOMEFEN_R = crate::BitReader<bool>;
#[doc = "Field `HOMEFEN` writer - 1- generate dma request when homef flag set"]
pub type HOMEFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEN_SPEC, bool, O>;
#[doc = "Field `POSCMPFEN` reader - 1- generate dma request when poscmpf flag set"]
pub type POSCMPFEN_R = crate::BitReader<bool>;
#[doc = "Field `POSCMPFEN` writer - 1- generate dma request when poscmpf flag set"]
pub type POSCMPFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEN_SPEC, bool, O>;
#[doc = "Field `ZPHFEN` reader - 1- generate dma request when zphf flag set"]
pub type ZPHFEN_R = crate::BitReader<bool>;
#[doc = "Field `ZPHFEN` writer - 1- generate dma request when zphf flag set"]
pub type ZPHFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - 1- generate dma request when wdg flag set"]
    #[inline(always)]
    pub fn wdgfen(&self) -> WDGFEN_R {
        WDGFEN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- generate dma request when homef flag set"]
    #[inline(always)]
    pub fn homefen(&self) -> HOMEFEN_R {
        HOMEFEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - 1- generate dma request when poscmpf flag set"]
    #[inline(always)]
    pub fn poscmpfen(&self) -> POSCMPFEN_R {
        POSCMPFEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - 1- generate dma request when zphf flag set"]
    #[inline(always)]
    pub fn zphfen(&self) -> ZPHFEN_R {
        ZPHFEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 1- generate dma request when wdg flag set"]
    #[inline(always)]
    pub fn wdgfen(&mut self) -> WDGFEN_W<31> {
        WDGFEN_W::new(self)
    }
    #[doc = "Bit 30 - 1- generate dma request when homef flag set"]
    #[inline(always)]
    pub fn homefen(&mut self) -> HOMEFEN_W<30> {
        HOMEFEN_W::new(self)
    }
    #[doc = "Bit 29 - 1- generate dma request when poscmpf flag set"]
    #[inline(always)]
    pub fn poscmpfen(&mut self) -> POSCMPFEN_W<29> {
        POSCMPFEN_W::new(self)
    }
    #[doc = "Bit 28 - 1- generate dma request when zphf flag set"]
    #[inline(always)]
    pub fn zphfen(&mut self) -> ZPHFEN_W<28> {
        ZPHFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA request enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaen](index.html) module"]
pub struct DMAEN_SPEC;
impl crate::RegisterSpec for DMAEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaen::R](R) reader structure"]
impl crate::Readable for DMAEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaen::W](W) writer structure"]
impl crate::Writable for DMAEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAEN to value 0"]
impl crate::Resettable for DMAEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
