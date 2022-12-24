#[doc = "Register `ENDPTSETUPSTAT` reader"]
pub struct R(crate::R<ENDPTSETUPSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDPTSETUPSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDPTSETUPSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDPTSETUPSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDPTSETUPSTAT` writer"]
pub struct W(crate::W<ENDPTSETUPSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDPTSETUPSTAT_SPEC>;
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
impl From<crate::W<ENDPTSETUPSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDPTSETUPSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENDPTSETUPSTAT` reader - ENDPTSETUPSTAT Setup Endpoint Status. For every setup transaction that is received, a corresponding bit in this register is set to one. Software must clear or acknowledge the setup transfer by writing a one to a respective bit after it has read the setup data from Queue head. The response to a setup packet as in the order of operations and total response time is crucial to limit bus time outs while the setup lock out mechanism is engaged. This register is only used in device mode."]
pub type ENDPTSETUPSTAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENDPTSETUPSTAT` writer - ENDPTSETUPSTAT Setup Endpoint Status. For every setup transaction that is received, a corresponding bit in this register is set to one. Software must clear or acknowledge the setup transfer by writing a one to a respective bit after it has read the setup data from Queue head. The response to a setup packet as in the order of operations and total response time is crucial to limit bus time outs while the setup lock out mechanism is engaged. This register is only used in device mode."]
pub type ENDPTSETUPSTAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENDPTSETUPSTAT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ENDPTSETUPSTAT Setup Endpoint Status. For every setup transaction that is received, a corresponding bit in this register is set to one. Software must clear or acknowledge the setup transfer by writing a one to a respective bit after it has read the setup data from Queue head. The response to a setup packet as in the order of operations and total response time is crucial to limit bus time outs while the setup lock out mechanism is engaged. This register is only used in device mode."]
    #[inline(always)]
    pub fn endptsetupstat(&self) -> ENDPTSETUPSTAT_R {
        ENDPTSETUPSTAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ENDPTSETUPSTAT Setup Endpoint Status. For every setup transaction that is received, a corresponding bit in this register is set to one. Software must clear or acknowledge the setup transfer by writing a one to a respective bit after it has read the setup data from Queue head. The response to a setup packet as in the order of operations and total response time is crucial to limit bus time outs while the setup lock out mechanism is engaged. This register is only used in device mode."]
    #[inline(always)]
    #[must_use]
    pub fn endptsetupstat(&mut self) -> ENDPTSETUPSTAT_W<0> {
        ENDPTSETUPSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Setup Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptsetupstat](index.html) module"]
pub struct ENDPTSETUPSTAT_SPEC;
impl crate::RegisterSpec for ENDPTSETUPSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [endptsetupstat::R](R) reader structure"]
impl crate::Readable for ENDPTSETUPSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endptsetupstat::W](W) writer structure"]
impl crate::Writable for ENDPTSETUPSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENDPTSETUPSTAT to value 0"]
impl crate::Resettable for ENDPTSETUPSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
