#[doc = "Register `MONOH` reader"]
pub struct R(crate::R<MONOH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MONOH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MONOH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MONOH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MONOH` writer"]
pub struct W(crate::W<MONOH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MONOH_SPEC>;
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
impl From<crate::W<MONOH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MONOH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER` reader - high part of monotonica counter, write to this counter will cause counter increase by 1 if low part overflow"]
pub type COUNTER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNTER` writer - high part of monotonica counter, write to this counter will cause counter increase by 1 if low part overflow"]
pub type COUNTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MONOH_SPEC, u16, u16, 16, O>;
#[doc = "Field `EPOCH` reader - Fuse value for high part of monotonica"]
pub type EPOCH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EPOCH` writer - Fuse value for high part of monotonica"]
pub type EPOCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MONOH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - high part of monotonica counter, write to this counter will cause counter increase by 1 if low part overflow"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Fuse value for high part of monotonica"]
    #[inline(always)]
    pub fn epoch(&self) -> EPOCH_R {
        EPOCH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - high part of monotonica counter, write to this counter will cause counter increase by 1 if low part overflow"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> COUNTER_W<0> {
        COUNTER_W::new(self)
    }
    #[doc = "Bits 16:31 - Fuse value for high part of monotonica"]
    #[inline(always)]
    #[must_use]
    pub fn epoch(&mut self) -> EPOCH_W<16> {
        EPOCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High part of monotonic counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [monoh](index.html) module"]
pub struct MONOH_SPEC;
impl crate::RegisterSpec for MONOH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [monoh::R](R) reader structure"]
impl crate::Readable for MONOH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [monoh::W](W) writer structure"]
impl crate::Writable for MONOH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MONOH to value 0"]
impl crate::Resettable for MONOH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
