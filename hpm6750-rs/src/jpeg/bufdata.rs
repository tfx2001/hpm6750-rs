#[doc = "Register `BUFDATA` reader"]
pub struct R(crate::R<BUFDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUFDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUFDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUFDATA` writer"]
pub struct W(crate::W<BUFDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUFDATA_SPEC>;
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
impl From<crate::W<BUFDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUFDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - The data write-to/read-from buffer. The n-th address read will be actually the data written for n-1 th address, and the actual stored location is n-1 th address."]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - The data write-to/read-from buffer. The n-th address read will be actually the data written for n-1 th address, and the actual stored location is n-1 th address."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUFDATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The data write-to/read-from buffer. The n-th address read will be actually the data written for n-1 th address, and the actual stored location is n-1 th address."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The data write-to/read-from buffer. The n-th address read will be actually the data written for n-1 th address, and the actual stored location is n-1 th address."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buf Access Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bufdata](index.html) module"]
pub struct BUFDATA_SPEC;
impl crate::RegisterSpec for BUFDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bufdata::R](R) reader structure"]
impl crate::Readable for BUFDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bufdata::W](W) writer structure"]
impl crate::Writable for BUFDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUFDATA to value 0"]
impl crate::Resettable for BUFDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
