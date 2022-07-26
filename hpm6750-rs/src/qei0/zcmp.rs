#[doc = "Register `ZCMP` reader"]
pub struct R(crate::R<ZCMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ZCMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ZCMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ZCMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ZCMP` writer"]
pub struct W(crate::W<ZCMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ZCMP_SPEC>;
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
impl From<crate::W<ZCMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ZCMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZCMP` reader - zcnt postion compare value"]
pub type ZCMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ZCMP` writer - zcnt postion compare value"]
pub type ZCMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ZCMP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - zcnt postion compare value"]
    #[inline(always)]
    pub fn zcmp(&self) -> ZCMP_R {
        ZCMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - zcnt postion compare value"]
    #[inline(always)]
    pub fn zcmp(&mut self) -> ZCMP_W<0> {
        ZCMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Z comparator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [zcmp](index.html) module"]
pub struct ZCMP_SPEC;
impl crate::RegisterSpec for ZCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [zcmp::R](R) reader structure"]
impl crate::Readable for ZCMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [zcmp::W](W) writer structure"]
impl crate::Writable for ZCMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ZCMP to value 0"]
impl crate::Resettable for ZCMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
