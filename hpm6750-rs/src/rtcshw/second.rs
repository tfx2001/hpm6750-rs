#[doc = "Register `SECOND` reader"]
pub struct R(crate::R<SECOND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECOND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECOND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECOND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECOND` writer"]
pub struct W(crate::W<SECOND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECOND_SPEC>;
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
impl From<crate::W<SECOND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECOND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECOND` reader - second counter"]
pub type SECOND_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SECOND` writer - second counter"]
pub type SECOND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECOND_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - second counter"]
    #[inline(always)]
    pub fn second(&self) -> SECOND_R {
        SECOND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - second counter"]
    #[inline(always)]
    pub fn second(&mut self) -> SECOND_W<0> {
        SECOND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Second counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [second](index.html) module"]
pub struct SECOND_SPEC;
impl crate::RegisterSpec for SECOND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [second::R](R) reader structure"]
impl crate::Readable for SECOND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [second::W](W) writer structure"]
impl crate::Writable for SECOND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECOND to value 0"]
impl crate::Resettable for SECOND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
