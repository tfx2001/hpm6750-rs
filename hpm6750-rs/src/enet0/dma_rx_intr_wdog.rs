#[doc = "Register `DMA_RX_INTR_WDOG` reader"]
pub struct R(crate::R<DMA_RX_INTR_WDOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RX_INTR_WDOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RX_INTR_WDOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RX_INTR_WDOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_RX_INTR_WDOG` writer"]
pub struct W(crate::W<DMA_RX_INTR_WDOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_RX_INTR_WDOG_SPEC>;
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
impl From<crate::W<DMA_RX_INTR_WDOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_RX_INTR_WDOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIWT` reader - RI Watchdog Timer Count This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\]. When the watchdog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\]
of any received frame."]
pub type RIWT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RIWT` writer - RI Watchdog Timer Count This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\]. When the watchdog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\]
of any received frame."]
pub type RIWT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_RX_INTR_WDOG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - RI Watchdog Timer Count This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\]. When the watchdog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\]
of any received frame."]
    #[inline(always)]
    pub fn riwt(&self) -> RIWT_R {
        RIWT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RI Watchdog Timer Count This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\]. When the watchdog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\]
of any received frame."]
    #[inline(always)]
    pub fn riwt(&mut self) -> RIWT_W<0> {
        RIWT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Interrupt Watchdog Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_rx_intr_wdog](index.html) module"]
pub struct DMA_RX_INTR_WDOG_SPEC;
impl crate::RegisterSpec for DMA_RX_INTR_WDOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_rx_intr_wdog::R](R) reader structure"]
impl crate::Readable for DMA_RX_INTR_WDOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_rx_intr_wdog::W](W) writer structure"]
impl crate::Writable for DMA_RX_INTR_WDOG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_RX_INTR_WDOG to value 0"]
impl crate::Resettable for DMA_RX_INTR_WDOG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
