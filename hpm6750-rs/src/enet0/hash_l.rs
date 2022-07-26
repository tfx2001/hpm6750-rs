#[doc = "Register `HASH_L` reader"]
pub struct R(crate::R<HASH_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_L` writer"]
pub struct W(crate::W<HASH_L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_L_SPEC>;
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
impl From<crate::W<HASH_L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTL` reader - Hash Table Low This field contains the lower 32 bits of the Hash table."]
pub type HTL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HTL` writer - Hash Table Low This field contains the lower 32 bits of the Hash table."]
pub type HTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_L_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Hash Table Low This field contains the lower 32 bits of the Hash table."]
    #[inline(always)]
    pub fn htl(&self) -> HTL_R {
        HTL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table Low This field contains the lower 32 bits of the Hash table."]
    #[inline(always)]
    pub fn htl(&mut self) -> HTL_W<0> {
        HTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash Table Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_l](index.html) module"]
pub struct HASH_L_SPEC;
impl crate::RegisterSpec for HASH_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_l::R](R) reader structure"]
impl crate::Readable for HASH_L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_l::W](W) writer structure"]
impl crate::Writable for HASH_L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH_L to value 0"]
impl crate::Resettable for HASH_L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
