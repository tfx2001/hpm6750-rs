#[doc = "Register `IRQEN` reader"]
pub struct R(crate::R<IRQEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQEN` writer"]
pub struct W(crate::W<IRQEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQEN_SPEC>;
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
impl From<crate::W<IRQEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDGIE` reader - 1- generate interrupt when wdg flag set"]
pub type WDGIE_R = crate::BitReader<bool>;
#[doc = "Field `WDGIE` writer - 1- generate interrupt when wdg flag set"]
pub type WDGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `HOMEIE` reader - 1- generate interrupt when homef flag set"]
pub type HOMEIE_R = crate::BitReader<bool>;
#[doc = "Field `HOMEIE` writer - 1- generate interrupt when homef flag set"]
pub type HOMEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `POSCMPIE` reader - 1- generate interrupt when poscmpf flag set"]
pub type POSCMPIE_R = crate::BitReader<bool>;
#[doc = "Field `POSCMPIE` writer - 1- generate interrupt when poscmpf flag set"]
pub type POSCMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `ZPHIE` reader - 1- generate interrupt when zphf flag set"]
pub type ZPHIE_R = crate::BitReader<bool>;
#[doc = "Field `ZPHIE` writer - 1- generate interrupt when zphf flag set"]
pub type ZPHIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - 1- generate interrupt when wdg flag set"]
    #[inline(always)]
    pub fn wdgie(&self) -> WDGIE_R {
        WDGIE_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- generate interrupt when homef flag set"]
    #[inline(always)]
    pub fn homeie(&self) -> HOMEIE_R {
        HOMEIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - 1- generate interrupt when poscmpf flag set"]
    #[inline(always)]
    pub fn poscmpie(&self) -> POSCMPIE_R {
        POSCMPIE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - 1- generate interrupt when zphf flag set"]
    #[inline(always)]
    pub fn zphie(&self) -> ZPHIE_R {
        ZPHIE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 1- generate interrupt when wdg flag set"]
    #[inline(always)]
    pub fn wdgie(&mut self) -> WDGIE_W<31> {
        WDGIE_W::new(self)
    }
    #[doc = "Bit 30 - 1- generate interrupt when homef flag set"]
    #[inline(always)]
    pub fn homeie(&mut self) -> HOMEIE_W<30> {
        HOMEIE_W::new(self)
    }
    #[doc = "Bit 29 - 1- generate interrupt when poscmpf flag set"]
    #[inline(always)]
    pub fn poscmpie(&mut self) -> POSCMPIE_W<29> {
        POSCMPIE_W::new(self)
    }
    #[doc = "Bit 28 - 1- generate interrupt when zphf flag set"]
    #[inline(always)]
    pub fn zphie(&mut self) -> ZPHIE_W<28> {
        ZPHIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqen](index.html) module"]
pub struct IRQEN_SPEC;
impl crate::RegisterSpec for IRQEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqen::R](R) reader structure"]
impl crate::Readable for IRQEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqen::W](W) writer structure"]
impl crate::Writable for IRQEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQEN to value 0"]
impl crate::Resettable for IRQEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
