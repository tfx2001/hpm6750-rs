#[doc = "Register `ENDPTSTAT` reader"]
pub struct R(crate::R<ENDPTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDPTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDPTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDPTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERBR` reader - ERBR Endpoint Receive Buffer Ready -- Read Only. One bit for each endpoint indicates status of the respective endpoint buffer. This bit is set to a one by the hardware as a response to receiving a command from a corresponding bit in the ENDPRIME register. There is always a delay between setting a bit in the ENDPRIME register and endpoint indicating ready. This delay time varies based upon the current USB traffic and the number of bits set in the ENDPRIME register. Buffer ready is cleared by USB reset, by the USB DMA system, or through the ENDPTFLUSH register. NOTE: These bits are momentarily cleared by hardware during hardware endpoint re-priming operations when a dTD is retired, and the dQH is updated. ERBR\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type ERBR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETBR` reader - ETBR Endpoint Transmit Buffer Ready -- Read Only. One bit for each endpoint indicates status of the respective endpoint buffer. This bit is set to one by the hardware as a response to receiving a command from a corresponding bit in the ENDPTPRIME register. There is always a delay between setting a bit in the ENDPTPRIME register and endpoint indicating ready. This delay time varies based upon the current USB traffic and the number of bits set in the ENDPRIME register. Buffer ready is cleared by USB reset, by the USB DMA system, or through the ENDPTFLUSH register. NOTE: These bits are momentarily cleared by hardware during hardware endpoint re-priming operations when a dTD is retired, and the dQH is updated. ETBR\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type ETBR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - ERBR Endpoint Receive Buffer Ready -- Read Only. One bit for each endpoint indicates status of the respective endpoint buffer. This bit is set to a one by the hardware as a response to receiving a command from a corresponding bit in the ENDPRIME register. There is always a delay between setting a bit in the ENDPRIME register and endpoint indicating ready. This delay time varies based upon the current USB traffic and the number of bits set in the ENDPRIME register. Buffer ready is cleared by USB reset, by the USB DMA system, or through the ENDPTFLUSH register. NOTE: These bits are momentarily cleared by hardware during hardware endpoint re-priming operations when a dTD is retired, and the dQH is updated. ERBR\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn erbr(&self) -> ERBR_R {
        ERBR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ETBR Endpoint Transmit Buffer Ready -- Read Only. One bit for each endpoint indicates status of the respective endpoint buffer. This bit is set to one by the hardware as a response to receiving a command from a corresponding bit in the ENDPTPRIME register. There is always a delay between setting a bit in the ENDPTPRIME register and endpoint indicating ready. This delay time varies based upon the current USB traffic and the number of bits set in the ENDPRIME register. Buffer ready is cleared by USB reset, by the USB DMA system, or through the ENDPTFLUSH register. NOTE: These bits are momentarily cleared by hardware during hardware endpoint re-priming operations when a dTD is retired, and the dQH is updated. ETBR\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn etbr(&self) -> ETBR_R {
        ETBR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptstat](index.html) module"]
pub struct ENDPTSTAT_SPEC;
impl crate::RegisterSpec for ENDPTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [endptstat::R](R) reader structure"]
impl crate::Readable for ENDPTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ENDPTSTAT to value 0"]
impl crate::Resettable for ENDPTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
