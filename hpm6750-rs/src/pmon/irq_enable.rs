#[doc = "Register `IRQ_ENABLE` reader"]
pub struct R(crate::R<IRQ_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_ENABLE` writer"]
pub struct W(crate::W<IRQ_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_ENABLE_SPEC>;
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
impl From<crate::W<IRQ_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - interrupt enable, each bit represents for one monitor 0: monitor interrupt disabled 1: monitor interrupt enabled"]
pub type ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENABLE` writer - interrupt enable, each bit represents for one monitor 0: monitor interrupt disabled 1: monitor interrupt enabled"]
pub type ENABLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQ_ENABLE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - interrupt enable, each bit represents for one monitor 0: monitor interrupt disabled 1: monitor interrupt enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - interrupt enable, each bit represents for one monitor 0: monitor interrupt disabled 1: monitor interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_enable](index.html) module"]
pub struct IRQ_ENABLE_SPEC;
impl crate::RegisterSpec for IRQ_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_enable::R](R) reader structure"]
impl crate::Readable for IRQ_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_enable::W](W) writer structure"]
impl crate::Writable for IRQ_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQ_ENABLE to value 0"]
impl crate::Resettable for IRQ_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
