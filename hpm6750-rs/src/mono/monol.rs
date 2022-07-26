#[doc = "Register `MONOL` reader"]
pub struct R(crate::R<MONOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MONOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MONOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MONOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MONOL` writer"]
pub struct W(crate::W<MONOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MONOL_SPEC>;
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
impl From<crate::W<MONOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MONOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER` reader - low part of monotonica counter, write to this counter will cause counter increase by 1"]
pub type COUNTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNTER` writer - low part of monotonica counter, write to this counter will cause counter increase by 1"]
pub type COUNTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MONOL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - low part of monotonica counter, write to this counter will cause counter increase by 1"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - low part of monotonica counter, write to this counter will cause counter increase by 1"]
    #[inline(always)]
    pub fn counter(&mut self) -> COUNTER_W<0> {
        COUNTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low part of monotonic counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [monol](index.html) module"]
pub struct MONOL_SPEC;
impl crate::RegisterSpec for MONOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [monol::R](R) reader structure"]
impl crate::Readable for MONOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [monol::W](W) writer structure"]
impl crate::Writable for MONOL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MONOL to value 0"]
impl crate::Resettable for MONOL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
