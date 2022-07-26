#[doc = "Register `PENDING_PENDING1` reader"]
pub struct R(crate::R<PENDING_PENDING1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PENDING_PENDING1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PENDING_PENDING1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PENDING_PENDING1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PENDING_PENDING1` writer"]
pub struct W(crate::W<PENDING_PENDING1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PENDING_PENDING1_SPEC>;
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
impl From<crate::W<PENDING_PENDING1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PENDING_PENDING1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERRUPT` reader - The interrupt pending status of inpterrupt sources. Every interrupt source occupies 1 bit."]
pub type INTERRUPT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTERRUPT` writer - The interrupt pending status of inpterrupt sources. Every interrupt source occupies 1 bit."]
pub type INTERRUPT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PENDING_PENDING1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The interrupt pending status of inpterrupt sources. Every interrupt source occupies 1 bit."]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The interrupt pending status of inpterrupt sources. Every interrupt source occupies 1 bit."]
    #[inline(always)]
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
#[doc = "Pending status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pending_pending1](index.html) module"]
pub struct PENDING_PENDING1_SPEC;
impl crate::RegisterSpec for PENDING_PENDING1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pending_pending1::R](R) reader structure"]
impl crate::Readable for PENDING_PENDING1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pending_pending1::W](W) writer structure"]
impl crate::Writable for PENDING_PENDING1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PENDING_PENDING1 to value 0"]
impl crate::Resettable for PENDING_PENDING1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}