#[doc = "Register `SELECT` reader"]
pub struct R(crate::R<SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SELECT` writer"]
pub struct W(crate::W<SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SELECT_SPEC>;
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
impl From<crate::W<SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELECT` reader - select key, key0 treated as secure key, in non-scure mode, only key1 can be selected 0: select key0 in secure mode, key1 in non-secure mode 1: select key1 in secure or nonsecure mode"]
pub type SELECT_R = crate::BitReader<bool>;
#[doc = "Field `SELECT` writer - select key, key0 treated as secure key, in non-scure mode, only key1 can be selected 0: select key0 in secure mode, key1 in non-secure mode 1: select key1 in secure or nonsecure mode"]
pub type SELECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SELECT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - select key, key0 treated as secure key, in non-scure mode, only key1 can be selected 0: select key0 in secure mode, key1 in non-secure mode 1: select key1 in secure or nonsecure mode"]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - select key, key0 treated as secure key, in non-scure mode, only key1 can be selected 0: select key0 in secure mode, key1 in non-secure mode 1: select key1 in secure or nonsecure mode"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SELECT_W<0> {
        SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [select](index.html) module"]
pub struct SELECT_SPEC;
impl crate::RegisterSpec for SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [select::R](R) reader structure"]
impl crate::Readable for SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [select::W](W) writer structure"]
impl crate::Writable for SELECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SELECT to value 0"]
impl crate::Resettable for SELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
