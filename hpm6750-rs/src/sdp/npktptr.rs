#[doc = "Register `NPKTPTR` reader"]
pub struct R(crate::R<NPKTPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NPKTPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NPKTPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NPKTPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NPKTPTR` writer"]
pub struct W(crate::W<NPKTPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NPKTPTR_SPEC>;
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
impl From<crate::W<NPKTPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NPKTPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NPKTPTR` reader - Next Packet Address Pointer"]
pub type NPKTPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NPKTPTR` writer - Next Packet Address Pointer"]
pub type NPKTPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NPKTPTR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Next Packet Address Pointer"]
    #[inline(always)]
    pub fn npktptr(&self) -> NPKTPTR_R {
        NPKTPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Next Packet Address Pointer"]
    #[inline(always)]
    pub fn npktptr(&mut self) -> NPKTPTR_W<0> {
        NPKTPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Next Packet Address Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npktptr](index.html) module"]
pub struct NPKTPTR_SPEC;
impl crate::RegisterSpec for NPKTPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [npktptr::R](R) reader structure"]
impl crate::Readable for NPKTPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [npktptr::W](W) writer structure"]
impl crate::Writable for NPKTPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NPKTPTR to value 0"]
impl crate::Resettable for NPKTPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
