#[doc = "Register `HASH_H` reader"]
pub struct R(crate::R<HASH_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_H` writer"]
pub struct W(crate::W<HASH_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_H_SPEC>;
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
impl From<crate::W<HASH_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTH` reader - Hash Table High This field contains the upper 32 bits of the Hash table."]
pub type HTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HTH` writer - Hash Table High This field contains the upper 32 bits of the Hash table."]
pub type HTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_H_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Hash Table High This field contains the upper 32 bits of the Hash table."]
    #[inline(always)]
    pub fn hth(&self) -> HTH_R {
        HTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table High This field contains the upper 32 bits of the Hash table."]
    #[inline(always)]
    #[must_use]
    pub fn hth(&mut self) -> HTH_W<0> {
        HTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash Table High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_h](index.html) module"]
pub struct HASH_H_SPEC;
impl crate::RegisterSpec for HASH_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_h::R](R) reader structure"]
impl crate::Readable for HASH_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_h::W](W) writer structure"]
impl crate::Writable for HASH_H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASH_H to value 0"]
impl crate::Resettable for HASH_H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
