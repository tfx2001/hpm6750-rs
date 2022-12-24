#[doc = "Register `EALCAP` reader"]
pub struct R(crate::R<EALCAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EALCAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EALCAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EALCAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ALC` reader - Arbitration Lost Capture (bit position in the frame where the arbitration has been lost)"]
pub type ALC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KOER` reader - Kind Of ERror (Error code) 000 - no error 001 - BIT ERROR 010 - FORM ERROR 011 - STUFF ERROR 100 - ACKNOWLEDGEMENT ERROR 101 - CRC ERROR 110 - OTHER ERROR(dominant bits after own error flag, received active Error Flag too long,dominant bit during Passive-Error-Flag after ACK error) 111 - not used KOER is updated with each new error. Therefore it stays untouched when frames aresuccessfully transmitted or received."]
pub type KOER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Arbitration Lost Capture (bit position in the frame where the arbitration has been lost)"]
    #[inline(always)]
    pub fn alc(&self) -> ALC_R {
        ALC_R::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - Kind Of ERror (Error code) 000 - no error 001 - BIT ERROR 010 - FORM ERROR 011 - STUFF ERROR 100 - ACKNOWLEDGEMENT ERROR 101 - CRC ERROR 110 - OTHER ERROR(dominant bits after own error flag, received active Error Flag too long,dominant bit during Passive-Error-Flag after ACK error) 111 - not used KOER is updated with each new error. Therefore it stays untouched when frames aresuccessfully transmitted or received."]
    #[inline(always)]
    pub fn koer(&self) -> KOER_R {
        KOER_R::new((self.bits >> 5) & 7)
    }
}
#[doc = "Error and Arbitration Lost Capture Register EALCAP\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ealcap](index.html) module"]
pub struct EALCAP_SPEC;
impl crate::RegisterSpec for EALCAP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ealcap::R](R) reader structure"]
impl crate::Readable for EALCAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EALCAP to value 0"]
impl crate::Resettable for EALCAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
