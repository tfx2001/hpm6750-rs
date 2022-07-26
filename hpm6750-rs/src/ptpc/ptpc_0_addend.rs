#[doc = "Register `PTPC_0_ADDEND` reader"]
pub struct R(crate::R<PTPC_0_ADDEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPC_0_ADDEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPC_0_ADDEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPC_0_ADDEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPC_0_ADDEND` writer"]
pub struct W(crate::W<PTPC_0_ADDEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPC_0_ADDEND_SPEC>;
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
impl From<crate::W<PTPC_0_ADDEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPC_0_ADDEND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDEND` reader - used in fine update mode only"]
pub type ADDEND_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDEND` writer - used in fine update mode only"]
pub type ADDEND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PTPC_0_ADDEND_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - used in fine update mode only"]
    #[inline(always)]
    pub fn addend(&self) -> ADDEND_R {
        ADDEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - used in fine update mode only"]
    #[inline(always)]
    pub fn addend(&mut self) -> ADDEND_W<0> {
        ADDEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpc_0_addend](index.html) module"]
pub struct PTPC_0_ADDEND_SPEC;
impl crate::RegisterSpec for PTPC_0_ADDEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptpc_0_addend::R](R) reader structure"]
impl crate::Readable for PTPC_0_ADDEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptpc_0_addend::W](W) writer structure"]
impl crate::Writable for PTPC_0_ADDEND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTPC_0_ADDEND to value 0"]
impl crate::Resettable for PTPC_0_ADDEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
