#[doc = "Register `UNLOCK` reader"]
pub struct R(crate::R<UNLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UNLOCK` writer"]
pub struct W(crate::W<UNLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UNLOCK_SPEC>;
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
impl From<crate::W<UNLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UNLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNLOCK` reader - unlock word for fuse array operation write \"OPEN\" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly"]
pub type UNLOCK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UNLOCK` writer - unlock word for fuse array operation write \"OPEN\" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly"]
pub type UNLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UNLOCK_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - unlock word for fuse array operation write \"OPEN\" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly"]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - unlock word for fuse array operation write \"OPEN\" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly"]
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W<0> {
        UNLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UNLOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unlock](index.html) module"]
pub struct UNLOCK_SPEC;
impl crate::RegisterSpec for UNLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [unlock::R](R) reader structure"]
impl crate::Readable for UNLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [unlock::W](W) writer structure"]
impl crate::Writable for UNLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UNLOCK to value 0"]
impl crate::Resettable for UNLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
