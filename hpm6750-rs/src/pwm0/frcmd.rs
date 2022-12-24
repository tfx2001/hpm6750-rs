#[doc = "Register `FRCMD` reader"]
pub struct R(crate::R<FRCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRCMD` writer"]
pub struct W(crate::W<FRCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRCMD_SPEC>;
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
impl From<crate::W<FRCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRCMD` reader - 2bit for each PWM output channel (0~7); 00: force output 0 01: force output 1 10: output highz 11: no force"]
pub type FRCMD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRCMD` writer - 2bit for each PWM output channel (0~7); 00: force output 0 01: force output 1 10: output highz 11: no force"]
pub type FRCMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRCMD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 2bit for each PWM output channel (0~7); 00: force output 0 01: force output 1 10: output highz 11: no force"]
    #[inline(always)]
    pub fn frcmd(&self) -> FRCMD_R {
        FRCMD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 2bit for each PWM output channel (0~7); 00: force output 0 01: force output 1 10: output highz 11: no force"]
    #[inline(always)]
    #[must_use]
    pub fn frcmd(&mut self) -> FRCMD_W<0> {
        FRCMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force output mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frcmd](index.html) module"]
pub struct FRCMD_SPEC;
impl crate::RegisterSpec for FRCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frcmd::R](R) reader structure"]
impl crate::Readable for FRCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frcmd::W](W) writer structure"]
impl crate::Writable for FRCMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRCMD to value 0"]
impl crate::Resettable for FRCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
