#[doc = "Register `SHLK` reader"]
pub struct R(crate::R<SHLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHLK` writer"]
pub struct W(crate::W<SHLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHLK_SPEC>;
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
impl From<crate::W<SHLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHLK` reader - write 1 to lock all shawdow register, wirte access is not permitted"]
pub type SHLK_R = crate::BitReader<bool>;
#[doc = "Field `SHLK` writer - write 1 to lock all shawdow register, wirte access is not permitted"]
pub type SHLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHLK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - write 1 to lock all shawdow register, wirte access is not permitted"]
    #[inline(always)]
    pub fn shlk(&self) -> SHLK_R {
        SHLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - write 1 to lock all shawdow register, wirte access is not permitted"]
    #[inline(always)]
    pub fn shlk(&mut self) -> SHLK_W<31> {
        SHLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow registers lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shlk](index.html) module"]
pub struct SHLK_SPEC;
impl crate::RegisterSpec for SHLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shlk::R](R) reader structure"]
impl crate::Readable for SHLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shlk::W](W) writer structure"]
impl crate::Writable for SHLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHLK to value 0"]
impl crate::Resettable for SHLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
