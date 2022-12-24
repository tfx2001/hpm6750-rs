#[doc = "Register `DEBUGGING` reader"]
pub struct R(crate::R<DEBUGGING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUGGING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUGGING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUGGING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPESTS` reader - MAC GMII or MII Receive Protocol Engine Status When high, this bit indicates that the MAC GMII or MII receive protocol engine is actively receiving data and not in IDLE state."]
pub type RPESTS_R = crate::BitReader<bool>;
#[doc = "Field `RFCFCSTS` reader - MAC Receive Frame FIFO Controller Status When high, this field indicates the active state of the small FIFO Read and Write controllers of the MAC Receive Frame Controller Module. - RFCFCSTS\\[1\\]
represents the status of small FIFO Read controller. - RFCFCSTS\\[0\\]
represents the status of small FIFO Write controller."]
pub type RFCFCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWCSTS` reader - MTL Rx FIFO Write Controller Active Status When high, this bit indicates that the MTL Rx FIFO Write Controller is active and is transferring a received frame to the FIFO."]
pub type RWCSTS_R = crate::BitReader<bool>;
#[doc = "Field `RRCSTS` reader - MTL RxFIFO Read Controller State This field gives the state of the Rx FIFO read Controller: - 00: IDLE state - 01: Reading frame data - 10: Reading frame status (or timestamp) - 11: Flushing the frame data and status"]
pub type RRCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFSTS` reader - MTL RxFIFO Fill-Level Status This field gives the status of the fill-level of the Rx FIFO: - 00: Rx FIFO Empty - 01: Rx FIFO fill-level below flow-control deactivate threshold - 10: Rx FIFO fill-level above flow-control activate threshold - 11: Rx FIFO Full"]
pub type RXFSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TPESTS` reader - MAC GMII or MII Transmit Protocol Engine Status When high, this bit indicates that the MAC GMII or MII transmit protocol engine is actively transmitting data and is not in the IDLE state."]
pub type TPESTS_R = crate::BitReader<bool>;
#[doc = "Field `TFCSTS` reader - MAC Transmit Frame Controller Status This field indicates the state of the MAC Transmit Frame Controller module: - 00: IDLE state - 01: Waiting for status of previous frame or IFG or backoff period to be over - 10: Generating and transmitting a Pause frame (in the full-duplex mode) - 11: Transferring input frame for transmission"]
pub type TFCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXPAUSED` reader - MAC Transmitter in Pause When high, this bit indicates that the MAC transmitter is in the Pause condition (in the full-duplex-only mode) and hence does not schedule any frame for transmission."]
pub type TXPAUSED_R = crate::BitReader<bool>;
#[doc = "Field `TRCSTS` reader - MTL Tx FIFO Read Controller Status This field indicates the state of the Tx FIFO Read Controller: - 00: IDLE state - 01: READ state (transferring data to the MAC transmitter) - 10: Waiting for TxStatus from the MAC transmitter - 11: Writing the received TxStatus or flushing the Tx FIFO"]
pub type TRCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWCSTS` reader - MTL Tx FIFO Write Controller Status When high, this bit indicates that the MTL Tx FIFO Write Controller is active and is transferring data to the Tx FIFO."]
pub type TWCSTS_R = crate::BitReader<bool>;
#[doc = "Field `TXFSTS` reader - MTL Tx FIFO Not Empty Status When high, this bit indicates that the MTL Tx FIFO is not empty and some data is left for transmission."]
pub type TXFSTS_R = crate::BitReader<bool>;
#[doc = "Field `TXSTSFSTS` reader - MTL TxStatus FIFO Full Status When high, this bit indicates that the MTL TxStatus FIFO is full. Therefore, the MTL cannot accept any more frames for transmission. This bit is reserved in the GMAC-AHB and GMAC-DMA configurations."]
pub type TXSTSFSTS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - MAC GMII or MII Receive Protocol Engine Status When high, this bit indicates that the MAC GMII or MII receive protocol engine is actively receiving data and not in IDLE state."]
    #[inline(always)]
    pub fn rpests(&self) -> RPESTS_R {
        RPESTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Frame FIFO Controller Status When high, this field indicates the active state of the small FIFO Read and Write controllers of the MAC Receive Frame Controller Module. - RFCFCSTS\\[1\\]
represents the status of small FIFO Read controller. - RFCFCSTS\\[0\\]
represents the status of small FIFO Write controller."]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - MTL Rx FIFO Write Controller Active Status When high, this bit indicates that the MTL Rx FIFO Write Controller is active and is transferring a received frame to the FIFO."]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - MTL RxFIFO Read Controller State This field gives the state of the Rx FIFO read Controller: - 00: IDLE state - 01: Reading frame data - 10: Reading frame status (or timestamp) - 11: Flushing the frame data and status"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9 - MTL RxFIFO Fill-Level Status This field gives the status of the fill-level of the Rx FIFO: - 00: Rx FIFO Empty - 01: Rx FIFO fill-level below flow-control deactivate threshold - 10: Rx FIFO fill-level above flow-control activate threshold - 11: Rx FIFO Full"]
    #[inline(always)]
    pub fn rxfsts(&self) -> RXFSTS_R {
        RXFSTS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - MAC GMII or MII Transmit Protocol Engine Status When high, this bit indicates that the MAC GMII or MII transmit protocol engine is actively transmitting data and is not in the IDLE state."]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - MAC Transmit Frame Controller Status This field indicates the state of the MAC Transmit Frame Controller module: - 00: IDLE state - 01: Waiting for status of previous frame or IFG or backoff period to be over - 10: Generating and transmitting a Pause frame (in the full-duplex mode) - 11: Transferring input frame for transmission"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - MAC Transmitter in Pause When high, this bit indicates that the MAC transmitter is in the Pause condition (in the full-duplex-only mode) and hence does not schedule any frame for transmission."]
    #[inline(always)]
    pub fn txpaused(&self) -> TXPAUSED_R {
        TXPAUSED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - MTL Tx FIFO Read Controller Status This field indicates the state of the Tx FIFO Read Controller: - 00: IDLE state - 01: READ state (transferring data to the MAC transmitter) - 10: Waiting for TxStatus from the MAC transmitter - 11: Writing the received TxStatus or flushing the Tx FIFO"]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - MTL Tx FIFO Write Controller Status When high, this bit indicates that the MTL Tx FIFO Write Controller is active and is transferring data to the Tx FIFO."]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - MTL Tx FIFO Not Empty Status When high, this bit indicates that the MTL Tx FIFO is not empty and some data is left for transmission."]
    #[inline(always)]
    pub fn txfsts(&self) -> TXFSTS_R {
        TXFSTS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MTL TxStatus FIFO Full Status When high, this bit indicates that the MTL TxStatus FIFO is full. Therefore, the MTL cannot accept any more frames for transmission. This bit is reserved in the GMAC-AHB and GMAC-DMA configurations."]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debugging](index.html) module"]
pub struct DEBUGGING_SPEC;
impl crate::RegisterSpec for DEBUGGING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debugging::R](R) reader structure"]
impl crate::Readable for DEBUGGING_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEBUGGING to value 0"]
impl crate::Resettable for DEBUGGING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
