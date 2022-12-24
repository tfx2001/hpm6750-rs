#[doc = "Register `SYST_NSEC_UPD` reader"]
pub struct R(crate::R<SYST_NSEC_UPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYST_NSEC_UPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYST_NSEC_UPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYST_NSEC_UPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYST_NSEC_UPD` writer"]
pub struct W(crate::W<SYST_NSEC_UPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYST_NSEC_UPD_SPEC>;
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
impl From<crate::W<SYST_NSEC_UPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYST_NSEC_UPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSSS` reader - Timestamp Sub Seconds The value in this field has the sub second representation of time, with an accuracy of 0.46 ns. When Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register), each bit represents 1 ns and the programmed value should not exceed 0x3B9A_C9FF."]
pub type TSSS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSSS` writer - Timestamp Sub Seconds The value in this field has the sub second representation of time, with an accuracy of 0.46 ns. When Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register), each bit represents 1 ns and the programmed value should not exceed 0x3B9A_C9FF."]
pub type TSSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYST_NSEC_UPD_SPEC, u32, u32, 31, O>;
#[doc = "Field `ADDSUB` reader - Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register. When this bit is reset, the time value is added with the contents of the update register."]
pub type ADDSUB_R = crate::BitReader<bool>;
#[doc = "Field `ADDSUB` writer - Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register. When this bit is reset, the time value is added with the contents of the update register."]
pub type ADDSUB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYST_NSEC_UPD_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub Seconds The value in this field has the sub second representation of time, with an accuracy of 0.46 ns. When Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register), each bit represents 1 ns and the programmed value should not exceed 0x3B9A_C9FF."]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register. When this bit is reset, the time value is added with the contents of the update register."]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Timestamp Sub Seconds The value in this field has the sub second representation of time, with an accuracy of 0.46 ns. When Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register), each bit represents 1 ns and the programmed value should not exceed 0x3B9A_C9FF."]
    #[inline(always)]
    #[must_use]
    pub fn tsss(&mut self) -> TSSS_W<0> {
        TSSS_W::new(self)
    }
    #[doc = "Bit 31 - Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register. When this bit is reset, the time value is added with the contents of the update register."]
    #[inline(always)]
    #[must_use]
    pub fn addsub(&mut self) -> ADDSUB_W<31> {
        ADDSUB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Time - Nanoseconds Update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_nsec_upd](index.html) module"]
pub struct SYST_NSEC_UPD_SPEC;
impl crate::RegisterSpec for SYST_NSEC_UPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syst_nsec_upd::R](R) reader structure"]
impl crate::Readable for SYST_NSEC_UPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syst_nsec_upd::W](W) writer structure"]
impl crate::Writable for SYST_NSEC_UPD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYST_NSEC_UPD to value 0"]
impl crate::Resettable for SYST_NSEC_UPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
