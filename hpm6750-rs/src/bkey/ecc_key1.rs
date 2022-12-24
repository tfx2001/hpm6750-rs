#[doc = "Register `ECC_KEY1` reader"]
pub struct R(crate::R<ECC_KEY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_KEY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_KEY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_KEY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECC_KEY1` writer"]
pub struct W(crate::W<ECC_KEY1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECC_KEY1_SPEC>;
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
impl From<crate::W<ECC_KEY1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECC_KEY1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECC` reader - Parity check bits for key0"]
pub type ECC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ECC` writer - Parity check bits for key0"]
pub type ECC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECC_KEY1_SPEC, u16, u16, 16, O>;
#[doc = "Field `RLOCK` reader - read lock to key0 0: key read enable 1: key always read as 0"]
pub type RLOCK_R = crate::BitReader<bool>;
#[doc = "Field `RLOCK` writer - read lock to key0 0: key read enable 1: key always read as 0"]
pub type RLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECC_KEY1_SPEC, bool, O>;
#[doc = "Field `WLOCK` reader - write lock to key0 0: write enable 1: write ignored"]
pub type WLOCK_R = crate::BitReader<bool>;
#[doc = "Field `WLOCK` writer - write lock to key0 0: write enable 1: write ignored"]
pub type WLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECC_KEY1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Parity check bits for key0"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - read lock to key0 0: key read enable 1: key always read as 0"]
    #[inline(always)]
    pub fn rlock(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - write lock to key0 0: write enable 1: write ignored"]
    #[inline(always)]
    pub fn wlock(&self) -> WLOCK_R {
        WLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Parity check bits for key0"]
    #[inline(always)]
    #[must_use]
    pub fn ecc(&mut self) -> ECC_W<0> {
        ECC_W::new(self)
    }
    #[doc = "Bit 30 - read lock to key0 0: key read enable 1: key always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn rlock(&mut self) -> RLOCK_W<30> {
        RLOCK_W::new(self)
    }
    #[doc = "Bit 31 - write lock to key0 0: write enable 1: write ignored"]
    #[inline(always)]
    #[must_use]
    pub fn wlock(&mut self) -> WLOCK_W<31> {
        WLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key 1 ECC and access control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_key1](index.html) module"]
pub struct ECC_KEY1_SPEC;
impl crate::RegisterSpec for ECC_KEY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_key1::R](R) reader structure"]
impl crate::Readable for ECC_KEY1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecc_key1::W](W) writer structure"]
impl crate::Writable for ECC_KEY1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECC_KEY1 to value 0"]
impl crate::Resettable for ECC_KEY1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
