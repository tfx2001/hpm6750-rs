#[doc = "Register `DATSZ` reader"]
pub struct R(crate::R<DATSZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATSZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATSZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATSZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATSZ` writer"]
pub struct W(crate::W<DATSZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATSZ_SPEC>;
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
impl From<crate::W<DATSZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATSZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATSZ` reader - Data Size in Byte When IP command is not a write/read operation, DATSZ field would be ignored. 000b - 4 001b - 1 010b - 2 011b - 3 100b - 4 101b - 4 110b - 4 111b - 4"]
pub type DATSZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATSZ` writer - Data Size in Byte When IP command is not a write/read operation, DATSZ field would be ignored. 000b - 4 001b - 1 010b - 2 011b - 3 100b - 4 101b - 4 110b - 4 111b - 4"]
pub type DATSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATSZ_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Data Size in Byte When IP command is not a write/read operation, DATSZ field would be ignored. 000b - 4 001b - 1 010b - 2 011b - 3 100b - 4 101b - 4 110b - 4 111b - 4"]
    #[inline(always)]
    pub fn datsz(&self) -> DATSZ_R {
        DATSZ_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data Size in Byte When IP command is not a write/read operation, DATSZ field would be ignored. 000b - 4 001b - 1 010b - 2 011b - 3 100b - 4 101b - 4 110b - 4 111b - 4"]
    #[inline(always)]
    pub fn datsz(&mut self) -> DATSZ_W<0> {
        DATSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Command Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datsz](index.html) module"]
pub struct DATSZ_SPEC;
impl crate::RegisterSpec for DATSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datsz::R](R) reader structure"]
impl crate::Readable for DATSZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datsz::W](W) writer structure"]
impl crate::Writable for DATSZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATSZ to value 0"]
impl crate::Resettable for DATSZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
