#[doc = "Register `MTIME` reader"]
pub struct R(crate::R<MTIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTIME` writer"]
pub struct W(crate::W<MTIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTIME_SPEC>;
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
impl From<crate::W<MTIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MTIME` reader - Machine time"]
pub type MTIME_R = crate::FieldReader<u64, u64>;
#[doc = "Field `MTIME` writer - Machine time"]
pub type MTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u64, MTIME_SPEC, u64, u64, 64, O>;
impl R {
    #[doc = "Bits 0:63 - Machine time"]
    #[inline(always)]
    pub fn mtime(&self) -> MTIME_R {
        MTIME_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63 - Machine time"]
    #[inline(always)]
    #[must_use]
    pub fn mtime(&mut self) -> MTIME_W<0> {
        MTIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Machine Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtime](index.html) module"]
pub struct MTIME_SPEC;
impl crate::RegisterSpec for MTIME_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [mtime::R](R) reader structure"]
impl crate::Readable for MTIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtime::W](W) writer structure"]
impl crate::Writable for MTIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTIME to value 0x0002_0210"]
impl crate::Resettable for MTIME_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0210;
}
