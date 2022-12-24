#[doc = "Register `PL_GPIOY_CLEAR` reader"]
pub struct R(crate::R<PL_GPIOY_CLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PL_GPIOY_CLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PL_GPIOY_CLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PL_GPIOY_CLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PL_GPIOY_CLEAR` writer"]
pub struct W(crate::W<PL_GPIOY_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PL_GPIOY_CLEAR_SPEC>;
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
impl From<crate::W<PL_GPIOY_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PL_GPIOY_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ_POL` reader - GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
pub type IRQ_POL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IRQ_POL` writer - GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
pub type IRQ_POL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PL_GPIOY_CLEAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    #[inline(always)]
    pub fn irq_pol(&self) -> IRQ_POL_R {
        IRQ_POL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn irq_pol(&mut self) -> IRQ_POL_W<0> {
        IRQ_POL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO interrupt polarity clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl_gpioy_clear](index.html) module"]
pub struct PL_GPIOY_CLEAR_SPEC;
impl crate::RegisterSpec for PL_GPIOY_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pl_gpioy_clear::R](R) reader structure"]
impl crate::Readable for PL_GPIOY_CLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pl_gpioy_clear::W](W) writer structure"]
impl crate::Writable for PL_GPIOY_CLEAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PL_GPIOY_CLEAR to value 0"]
impl crate::Resettable for PL_GPIOY_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
