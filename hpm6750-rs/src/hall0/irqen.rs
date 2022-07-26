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
#[doc = "Field `WDGIE` reader - 1- generate interrupt request when wdg flag set"]
pub type WDGIE_R = crate::BitReader<bool>;
#[doc = "Field `WDGIE` writer - 1- generate interrupt request when wdg flag set"]
pub type WDGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `PHUPTIE` reader - 1- generate interrupt request when phupt flag set"]
pub type PHUPTIE_R = crate::BitReader<bool>;
#[doc = "Field `PHUPTIE` writer - 1- generate interrupt request when phupt flag set"]
pub type PHUPTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `PHPREIE` reader - 1- generate interrupt request when phpre flag set"]
pub type PHPREIE_R = crate::BitReader<bool>;
#[doc = "Field `PHPREIE` writer - 1- generate interrupt request when phpre flag set"]
pub type PHPREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `PHDLYIE` reader - 1- generate interrupt request when phdly flag set"]
pub type PHDLYIE_R = crate::BitReader<bool>;
#[doc = "Field `PHDLYIE` writer - 1- generate interrupt request when phdly flag set"]
pub type PHDLYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `UFIE` reader - 1- generate interrupt request when u flag set"]
pub type UFIE_R = crate::BitReader<bool>;
#[doc = "Field `UFIE` writer - 1- generate interrupt request when u flag set"]
pub type UFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `VFIE` reader - 1- generate interrupt request when v flag set"]
pub type VFIE_R = crate::BitReader<bool>;
#[doc = "Field `VFIE` writer - 1- generate interrupt request when v flag set"]
pub type VFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `WFIE` reader - 1- generate interrupt request when w flag set"]
pub type WFIE_R = crate::BitReader<bool>;
#[doc = "Field `WFIE` writer - 1- generate interrupt request when w flag set"]
pub type WFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - 1- generate interrupt request when wdg flag set"]
    #[inline(always)]
    pub fn wdgie(&self) -> WDGIE_R {
        WDGIE_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- generate interrupt request when phupt flag set"]
    #[inline(always)]
    pub fn phuptie(&self) -> PHUPTIE_R {
        PHUPTIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - 1- generate interrupt request when phpre flag set"]
    #[inline(always)]
    pub fn phpreie(&self) -> PHPREIE_R {
        PHPREIE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - 1- generate interrupt request when phdly flag set"]
    #[inline(always)]
    pub fn phdlyie(&self) -> PHDLYIE_R {
        PHDLYIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 23 - 1- generate interrupt request when u flag set"]
    #[inline(always)]
    pub fn ufie(&self) -> UFIE_R {
        UFIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - 1- generate interrupt request when v flag set"]
    #[inline(always)]
    pub fn vfie(&self) -> VFIE_R {
        VFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - 1- generate interrupt request when w flag set"]
    #[inline(always)]
    pub fn wfie(&self) -> WFIE_R {
        WFIE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 1- generate interrupt request when wdg flag set"]
    #[inline(always)]
    pub fn wdgie(&mut self) -> WDGIE_W<31> {
        WDGIE_W::new(self)
    }
    #[doc = "Bit 30 - 1- generate interrupt request when phupt flag set"]
    #[inline(always)]
    pub fn phuptie(&mut self) -> PHUPTIE_W<30> {
        PHUPTIE_W::new(self)
    }
    #[doc = "Bit 29 - 1- generate interrupt request when phpre flag set"]
    #[inline(always)]
    pub fn phpreie(&mut self) -> PHPREIE_W<29> {
        PHPREIE_W::new(self)
    }
    #[doc = "Bit 28 - 1- generate interrupt request when phdly flag set"]
    #[inline(always)]
    pub fn phdlyie(&mut self) -> PHDLYIE_W<28> {
        PHDLYIE_W::new(self)
    }
    #[doc = "Bit 23 - 1- generate interrupt request when u flag set"]
    #[inline(always)]
    pub fn ufie(&mut self) -> UFIE_W<23> {
        UFIE_W::new(self)
    }
    #[doc = "Bit 22 - 1- generate interrupt request when v flag set"]
    #[inline(always)]
    pub fn vfie(&mut self) -> VFIE_W<22> {
        VFIE_W::new(self)
    }
    #[doc = "Bit 21 - 1- generate interrupt request when w flag set"]
    #[inline(always)]
    pub fn wfie(&mut self) -> WFIE_W<21> {
        WFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt request enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqen](index.html) module"]
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
