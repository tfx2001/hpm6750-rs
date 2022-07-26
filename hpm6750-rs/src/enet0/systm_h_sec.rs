#[doc = "Register `SYSTM_H_SEC` reader"]
pub struct R(crate::R<SYSTM_H_SEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTM_H_SEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTM_H_SEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTM_H_SEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTM_H_SEC` writer"]
pub struct W(crate::W<SYSTM_H_SEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTM_H_SEC_SPEC>;
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
impl From<crate::W<SYSTM_H_SEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTM_H_SEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSHWR` reader - Timestamp Higher Word Register This field contains the most significant 16-bits of the timestamp seconds value. This register is optional and can be selected using the Enable IEEE 1588 Higher Word Register option during core configuration. The register is directly written to initialize the value. This register is incremented when there is an overflow from the 32-bits of the System Time - Seconds register."]
pub type TSHWR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSHWR` writer - Timestamp Higher Word Register This field contains the most significant 16-bits of the timestamp seconds value. This register is optional and can be selected using the Enable IEEE 1588 Higher Word Register option during core configuration. The register is directly written to initialize the value. This register is incremented when there is an overflow from the 32-bits of the System Time - Seconds register."]
pub type TSHWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSTM_H_SEC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Higher Word Register This field contains the most significant 16-bits of the timestamp seconds value. This register is optional and can be selected using the Enable IEEE 1588 Higher Word Register option during core configuration. The register is directly written to initialize the value. This register is incremented when there is an overflow from the 32-bits of the System Time - Seconds register."]
    #[inline(always)]
    pub fn tshwr(&self) -> TSHWR_R {
        TSHWR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp Higher Word Register This field contains the most significant 16-bits of the timestamp seconds value. This register is optional and can be selected using the Enable IEEE 1588 Higher Word Register option during core configuration. The register is directly written to initialize the value. This register is incremented when there is an overflow from the 32-bits of the System Time - Seconds register."]
    #[inline(always)]
    pub fn tshwr(&mut self) -> TSHWR_W<0> {
        TSHWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Time - Higher Word Seconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systm_h_sec](index.html) module"]
pub struct SYSTM_H_SEC_SPEC;
impl crate::RegisterSpec for SYSTM_H_SEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systm_h_sec::R](R) reader structure"]
impl crate::Readable for SYSTM_H_SEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [systm_h_sec::W](W) writer structure"]
impl crate::Writable for SYSTM_H_SEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSTM_H_SEC to value 0"]
impl crate::Resettable for SYSTM_H_SEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
