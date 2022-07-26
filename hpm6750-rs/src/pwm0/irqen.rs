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
#[doc = "Field `FAULTIRQE` reader - fault condition interrupt enable"]
pub type FAULTIRQE_R = crate::BitReader<bool>;
#[doc = "Field `FAULTIRQE` writer - fault condition interrupt enable"]
pub type FAULTIRQE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `XRLDIRQE` reader - extended reload flag interrupt enable"]
pub type XRLDIRQE_R = crate::BitReader<bool>;
#[doc = "Field `XRLDIRQE` writer - extended reload flag interrupt enable"]
pub type XRLDIRQE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `HALFRLDIRQE` reader - half reload flag interrupt enable"]
pub type HALFRLDIRQE_R = crate::BitReader<bool>;
#[doc = "Field `HALFRLDIRQE` writer - half reload flag interrupt enable"]
pub type HALFRLDIRQE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `RLDIRQE` reader - reload flag interrupt enable"]
pub type RLDIRQE_R = crate::BitReader<bool>;
#[doc = "Field `RLDIRQE` writer - reload flag interrupt enable"]
pub type RLDIRQE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `CMPIRQEX` reader - comparator output compare or input capture flag interrupt enable"]
pub type CMPIRQEX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMPIRQEX` writer - comparator output compare or input capture flag interrupt enable"]
pub type CMPIRQEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQEN_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 27 - fault condition interrupt enable"]
    #[inline(always)]
    pub fn faultirqe(&self) -> FAULTIRQE_R {
        FAULTIRQE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - extended reload flag interrupt enable"]
    #[inline(always)]
    pub fn xrldirqe(&self) -> XRLDIRQE_R {
        XRLDIRQE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - half reload flag interrupt enable"]
    #[inline(always)]
    pub fn halfrldirqe(&self) -> HALFRLDIRQE_R {
        HALFRLDIRQE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - reload flag interrupt enable"]
    #[inline(always)]
    pub fn rldirqe(&self) -> RLDIRQE_R {
        RLDIRQE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 0:23 - comparator output compare or input capture flag interrupt enable"]
    #[inline(always)]
    pub fn cmpirqex(&self) -> CMPIRQEX_R {
        CMPIRQEX_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 27 - fault condition interrupt enable"]
    #[inline(always)]
    pub fn faultirqe(&mut self) -> FAULTIRQE_W<27> {
        FAULTIRQE_W::new(self)
    }
    #[doc = "Bit 26 - extended reload flag interrupt enable"]
    #[inline(always)]
    pub fn xrldirqe(&mut self) -> XRLDIRQE_W<26> {
        XRLDIRQE_W::new(self)
    }
    #[doc = "Bit 25 - half reload flag interrupt enable"]
    #[inline(always)]
    pub fn halfrldirqe(&mut self) -> HALFRLDIRQE_W<25> {
        HALFRLDIRQE_W::new(self)
    }
    #[doc = "Bit 24 - reload flag interrupt enable"]
    #[inline(always)]
    pub fn rldirqe(&mut self) -> RLDIRQE_W<24> {
        RLDIRQE_W::new(self)
    }
    #[doc = "Bits 0:23 - comparator output compare or input capture flag interrupt enable"]
    #[inline(always)]
    pub fn cmpirqex(&mut self) -> CMPIRQEX_W<0> {
        CMPIRQEX_W::new(self)
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
