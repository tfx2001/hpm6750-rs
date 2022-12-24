#[doc = "Register `ENDPTCTRL_ENDPTCTRL6` reader"]
pub struct R(crate::R<ENDPTCTRL_ENDPTCTRL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDPTCTRL_ENDPTCTRL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDPTCTRL_ENDPTCTRL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDPTCTRL_ENDPTCTRL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDPTCTRL_ENDPTCTRL6` writer"]
pub struct W(crate::W<ENDPTCTRL_ENDPTCTRL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDPTCTRL_ENDPTCTRL6_SPEC>;
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
impl From<crate::W<ENDPTCTRL_ENDPTCTRL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDPTCTRL_ENDPTCTRL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXS` reader - RXS RX Endpoint Stall - Read/Write 0 End Point OK. \\[Default\\]
1 End Point Stalled This bit is set automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpointand this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
pub type RXS_R = crate::BitReader<bool>;
#[doc = "Field `RXS` writer - RXS RX Endpoint Stall - Read/Write 0 End Point OK. \\[Default\\]
1 End Point Stalled This bit is set automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpointand this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
pub type RXS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL_ENDPTCTRL6_SPEC, bool, O>;
#[doc = "Field `RXT` reader - RXT RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
pub type RXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXT` writer - RXT RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
pub type RXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENDPTCTRL_ENDPTCTRL6_SPEC, u8, u8, 2, O>;
#[doc = "Field `RXR` reader - RXR RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device."]
pub type RXR_R = crate::BitReader<bool>;
#[doc = "Field `RXR` writer - RXR RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device."]
pub type RXR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL_ENDPTCTRL6_SPEC, bool, O>;
#[doc = "Field `RXE` reader - RXE RX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
pub type RXE_R = crate::BitReader<bool>;
#[doc = "Field `RXE` writer - RXE RX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
pub type RXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL_ENDPTCTRL6_SPEC, bool, O>;
#[doc = "Field `TXS` reader - TXS TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
pub type TXS_R = crate::BitReader<bool>;
#[doc = "Field `TXS` writer - TXS TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
pub type TXS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL_ENDPTCTRL6_SPEC, bool, O>;
#[doc = "Field `TXT` reader - TXT TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
pub type TXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXT` writer - TXT TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
pub type TXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENDPTCTRL_ENDPTCTRL6_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXR` reader - TXR TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device."]
pub type TXR_R = crate::BitReader<bool>;
#[doc = "Field `TXR` writer - TXR TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device."]
pub type TXR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL_ENDPTCTRL6_SPEC, bool, O>;
#[doc = "Field `TXE` reader - TXE TX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `TXE` writer - TXE TX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
pub type TXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL_ENDPTCTRL6_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RXS RX Endpoint Stall - Read/Write 0 End Point OK. \\[Default\\]
1 End Point Stalled This bit is set automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpointand this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
    #[inline(always)]
    pub fn rxs(&self) -> RXS_R {
        RXS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - RXT RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    pub fn rxt(&self) -> RXT_R {
        RXT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - RXR RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device."]
    #[inline(always)]
    pub fn rxr(&self) -> RXR_R {
        RXR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RXE RX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - TXS TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - TXT TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    pub fn txt(&self) -> TXT_R {
        TXT_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 22 - TXR TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device."]
    #[inline(always)]
    pub fn txr(&self) -> TXR_R {
        TXR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXE TX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXS RX Endpoint Stall - Read/Write 0 End Point OK. \\[Default\\]
1 End Point Stalled This bit is set automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpointand this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
    #[inline(always)]
    #[must_use]
    pub fn rxs(&mut self) -> RXS_W<0> {
        RXS_W::new(self)
    }
    #[doc = "Bits 2:3 - RXT RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxt(&mut self) -> RXT_W<2> {
        RXT_W::new(self)
    }
    #[doc = "Bit 6 - RXR RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device."]
    #[inline(always)]
    #[must_use]
    pub fn rxr(&mut self) -> RXR_W<6> {
        RXR_W::new(self)
    }
    #[doc = "Bit 7 - RXE RX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
    #[inline(always)]
    #[must_use]
    pub fn rxe(&mut self) -> RXE_W<7> {
        RXE_W::new(self)
    }
    #[doc = "Bit 16 - TXS TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
    #[inline(always)]
    #[must_use]
    pub fn txs(&mut self) -> TXS_W<16> {
        TXS_W::new(self)
    }
    #[doc = "Bits 18:19 - TXT TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txt(&mut self) -> TXT_W<18> {
        TXT_W::new(self)
    }
    #[doc = "Bit 22 - TXR TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device."]
    #[inline(always)]
    #[must_use]
    pub fn txr(&mut self) -> TXR_W<22> {
        TXR_W::new(self)
    }
    #[doc = "Bit 23 - TXE TX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<23> {
        TXE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Control0 Register... Endpoint Control7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptctrl_endptctrl6](index.html) module"]
pub struct ENDPTCTRL_ENDPTCTRL6_SPEC;
impl crate::RegisterSpec for ENDPTCTRL_ENDPTCTRL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [endptctrl_endptctrl6::R](R) reader structure"]
impl crate::Readable for ENDPTCTRL_ENDPTCTRL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endptctrl_endptctrl6::W](W) writer structure"]
impl crate::Writable for ENDPTCTRL_ENDPTCTRL6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENDPTCTRL_ENDPTCTRL6 to value 0"]
impl crate::Resettable for ENDPTCTRL_ENDPTCTRL6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
