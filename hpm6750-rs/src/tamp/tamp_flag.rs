#[doc = "Register `TAMP_FLAG` reader"]
pub struct R(crate::R<TAMP_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_FLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_FLAG` writer"]
pub struct W(crate::W<TAMP_FLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_FLAG_SPEC>;
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
impl From<crate::W<TAMP_FLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_FLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLAG` reader - tamper flag, each bit represents one tamper pin, write 1 to clear the flag Note, clear can only be cleared when tamper disapeared"]
pub type FLAG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLAG` writer - tamper flag, each bit represents one tamper pin, write 1 to clear the flag Note, clear can only be cleared when tamper disapeared"]
pub type FLAG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_FLAG_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - tamper flag, each bit represents one tamper pin, write 1 to clear the flag Note, clear can only be cleared when tamper disapeared"]
    #[inline(always)]
    pub fn flag(&self) -> FLAG_R {
        FLAG_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - tamper flag, each bit represents one tamper pin, write 1 to clear the flag Note, clear can only be cleared when tamper disapeared"]
    #[inline(always)]
    pub fn flag(&mut self) -> FLAG_W<0> {
        FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_flag](index.html) module"]
pub struct TAMP_FLAG_SPEC;
impl crate::RegisterSpec for TAMP_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_flag::R](R) reader structure"]
impl crate::Readable for TAMP_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_flag::W](W) writer structure"]
impl crate::Writable for TAMP_FLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMP_FLAG to value 0"]
impl crate::Resettable for TAMP_FLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
