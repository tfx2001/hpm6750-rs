#[doc = "Register `FUSE_LOCK_LOCK03` reader"]
pub struct R(crate::R<FUSE_LOCK_LOCK03_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUSE_LOCK_LOCK03_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUSE_LOCK_LOCK03_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUSE_LOCK_LOCK03_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUSE_LOCK_LOCK03` writer"]
pub struct W(crate::W<FUSE_LOCK_LOCK03_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUSE_LOCK_LOCK03_SPEC>;
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
impl From<crate::W<FUSE_LOCK_LOCK03_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUSE_LOCK_LOCK03_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - lock for fuse array, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked"]
pub type LOCK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOCK` writer - lock for fuse array, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked"]
pub type LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FUSE_LOCK_LOCK03_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - lock for fuse array, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - lock for fuse array, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<0> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fuse lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fuse_lock_lock03](index.html) module"]
pub struct FUSE_LOCK_LOCK03_SPEC;
impl crate::RegisterSpec for FUSE_LOCK_LOCK03_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fuse_lock_lock03::R](R) reader structure"]
impl crate::Readable for FUSE_LOCK_LOCK03_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fuse_lock_lock03::W](W) writer structure"]
impl crate::Writable for FUSE_LOCK_LOCK03_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FUSE_LOCK_LOCK03 to value 0"]
impl crate::Resettable for FUSE_LOCK_LOCK03_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
