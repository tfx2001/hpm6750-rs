#[doc = "Register `TARGETINT_TARGET1_INTEN_INTEN0` reader"]
pub struct R(crate::R<TARGETINT_TARGET1_INTEN_INTEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGETINT_TARGET1_INTEN_INTEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGETINT_TARGET1_INTEN_INTEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGETINT_TARGET1_INTEN_INTEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGETINT_TARGET1_INTEN_INTEN0` writer"]
pub struct W(crate::W<TARGETINT_TARGET1_INTEN_INTEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGETINT_TARGET1_INTEN_INTEN0_SPEC>;
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
impl From<crate::W<TARGETINT_TARGET1_INTEN_INTEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGETINT_TARGET1_INTEN_INTEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERRUPT` reader - The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit."]
pub type INTERRUPT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTERRUPT` writer - The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit."]
pub type INTERRUPT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TARGETINT_TARGET1_INTEN_INTEN0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit."]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt(&mut self) -> INTERRUPT_W<0> {
        INTERRUPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "supervisor interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targetint_target1_inten_inten0](index.html) module"]
pub struct TARGETINT_TARGET1_INTEN_INTEN0_SPEC;
impl crate::RegisterSpec for TARGETINT_TARGET1_INTEN_INTEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [targetint_target1_inten_inten0::R](R) reader structure"]
impl crate::Readable for TARGETINT_TARGET1_INTEN_INTEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [targetint_target1_inten_inten0::W](W) writer structure"]
impl crate::Writable for TARGETINT_TARGET1_INTEN_INTEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARGETINT_TARGET1_INTEN_INTEN0 to value 0"]
impl crate::Resettable for TARGETINT_TARGET1_INTEN_INTEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
