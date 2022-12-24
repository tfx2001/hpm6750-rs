#[doc = "Register `USBMODE` reader"]
pub struct R(crate::R<USBMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBMODE` writer"]
pub struct W(crate::W<USBMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBMODE_SPEC>;
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
impl From<crate::W<USBMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CM` reader - CM Controller Mode - R/WO. Controller mode is defaulted to the proper mode for host only and device only implementations. For those designs that contain both host & device capability, the controller defaults to an idle state and needs to be initialized to the desired operating mode after reset. For combination host/ device controllers, this register can only be written once after reset. If it is necessary to switch modes, software must reset the controller by writing to the RESET bit in the USBCMD register before reprogramming this register. For OTG controller core, reset value is '00b'. 00 - Idle \\[Default for combination host/device\\]
01 - Reserved 10 - Device Controller \\[Default for device only controller\\]
11 - Host Controller \\[Default for host only controller\\]"]
pub type CM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CM` writer - CM Controller Mode - R/WO. Controller mode is defaulted to the proper mode for host only and device only implementations. For those designs that contain both host & device capability, the controller defaults to an idle state and needs to be initialized to the desired operating mode after reset. For combination host/ device controllers, this register can only be written once after reset. If it is necessary to switch modes, software must reset the controller by writing to the RESET bit in the USBCMD register before reprogramming this register. For OTG controller core, reset value is '00b'. 00 - Idle \\[Default for combination host/device\\]
01 - Reserved 10 - Device Controller \\[Default for device only controller\\]
11 - Host Controller \\[Default for host only controller\\]"]
pub type CM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBMODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `ES` reader - ES Endian Select - Read/Write. This bit can change the byte alignment of the transfer buffers to match the host microprocessor. The bit fields in the microprocessor interface and the data structures are unaffected by the value of this bit because they are based upon the 32-bit word. Bit Meaning 0 - Little Endian \\[Default\\]
1 - Big Endian"]
pub type ES_R = crate::BitReader<bool>;
#[doc = "Field `ES` writer - ES Endian Select - Read/Write. This bit can change the byte alignment of the transfer buffers to match the host microprocessor. The bit fields in the microprocessor interface and the data structures are unaffected by the value of this bit because they are based upon the 32-bit word. Bit Meaning 0 - Little Endian \\[Default\\]
1 - Big Endian"]
pub type ES_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBMODE_SPEC, bool, O>;
#[doc = "Field `SLOM` reader - SLOM Setup Lockout Mode. In device mode, this bit controls behavior of the setup lock mechanism. See Control Endpoint Operation Model . 0 - Setup Lockouts On (default); 1 - Setup Lockouts Off. DCD requires use of Setup Data Buffer Tripwire in USBCMD."]
pub type SLOM_R = crate::BitReader<bool>;
#[doc = "Field `SLOM` writer - SLOM Setup Lockout Mode. In device mode, this bit controls behavior of the setup lock mechanism. See Control Endpoint Operation Model . 0 - Setup Lockouts On (default); 1 - Setup Lockouts Off. DCD requires use of Setup Data Buffer Tripwire in USBCMD."]
pub type SLOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBMODE_SPEC, bool, O>;
#[doc = "Field `SDIS` reader - SDIS Stream Disable Mode. (0 - Inactive \\[default\\]; 1 - Active) Device Mode: Setting to a '1' disables double priming on both RX and TX for low bandwidth systems. This mode ensures that when the RX and TX buffers are sufficient to contain an entire packet that the standard double buffering scheme is disabled to prevent overruns/underruns in bandwidth limited systems. Note: In High Speed Mode, all packets received are responded to with a NYET handshake when stream disable is active. Host Mode: Setting to a '1' ensures that overruns/underruns of the latency FIFO are eliminated for low bandwidth systems where the RX and TX buffers are sufficient to contain the entire packet. Enabling stream disable also has the effect of ensuring the TX latency is filled to capacity before the packet is launched onto the USB. NOTE: Time duration to pre-fill the FIFO becomes significant when stream disable is active. See TXFILLTUNING and TXTTFILLTUNING \\[MPH Only\\]
to characterize the adjustments needed for the scheduler when using this feature. NOTE: The use of this feature substantially limits of the overall USB performance that can be achieved."]
pub type SDIS_R = crate::BitReader<bool>;
#[doc = "Field `SDIS` writer - SDIS Stream Disable Mode. (0 - Inactive \\[default\\]; 1 - Active) Device Mode: Setting to a '1' disables double priming on both RX and TX for low bandwidth systems. This mode ensures that when the RX and TX buffers are sufficient to contain an entire packet that the standard double buffering scheme is disabled to prevent overruns/underruns in bandwidth limited systems. Note: In High Speed Mode, all packets received are responded to with a NYET handshake when stream disable is active. Host Mode: Setting to a '1' ensures that overruns/underruns of the latency FIFO are eliminated for low bandwidth systems where the RX and TX buffers are sufficient to contain the entire packet. Enabling stream disable also has the effect of ensuring the TX latency is filled to capacity before the packet is launched onto the USB. NOTE: Time duration to pre-fill the FIFO becomes significant when stream disable is active. See TXFILLTUNING and TXTTFILLTUNING \\[MPH Only\\]
to characterize the adjustments needed for the scheduler when using this feature. NOTE: The use of this feature substantially limits of the overall USB performance that can be achieved."]
pub type SDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBMODE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - CM Controller Mode - R/WO. Controller mode is defaulted to the proper mode for host only and device only implementations. For those designs that contain both host & device capability, the controller defaults to an idle state and needs to be initialized to the desired operating mode after reset. For combination host/ device controllers, this register can only be written once after reset. If it is necessary to switch modes, software must reset the controller by writing to the RESET bit in the USBCMD register before reprogramming this register. For OTG controller core, reset value is '00b'. 00 - Idle \\[Default for combination host/device\\]
01 - Reserved 10 - Device Controller \\[Default for device only controller\\]
11 - Host Controller \\[Default for host only controller\\]"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - ES Endian Select - Read/Write. This bit can change the byte alignment of the transfer buffers to match the host microprocessor. The bit fields in the microprocessor interface and the data structures are unaffected by the value of this bit because they are based upon the 32-bit word. Bit Meaning 0 - Little Endian \\[Default\\]
1 - Big Endian"]
    #[inline(always)]
    pub fn es(&self) -> ES_R {
        ES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SLOM Setup Lockout Mode. In device mode, this bit controls behavior of the setup lock mechanism. See Control Endpoint Operation Model . 0 - Setup Lockouts On (default); 1 - Setup Lockouts Off. DCD requires use of Setup Data Buffer Tripwire in USBCMD."]
    #[inline(always)]
    pub fn slom(&self) -> SLOM_R {
        SLOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SDIS Stream Disable Mode. (0 - Inactive \\[default\\]; 1 - Active) Device Mode: Setting to a '1' disables double priming on both RX and TX for low bandwidth systems. This mode ensures that when the RX and TX buffers are sufficient to contain an entire packet that the standard double buffering scheme is disabled to prevent overruns/underruns in bandwidth limited systems. Note: In High Speed Mode, all packets received are responded to with a NYET handshake when stream disable is active. Host Mode: Setting to a '1' ensures that overruns/underruns of the latency FIFO are eliminated for low bandwidth systems where the RX and TX buffers are sufficient to contain the entire packet. Enabling stream disable also has the effect of ensuring the TX latency is filled to capacity before the packet is launched onto the USB. NOTE: Time duration to pre-fill the FIFO becomes significant when stream disable is active. See TXFILLTUNING and TXTTFILLTUNING \\[MPH Only\\]
to characterize the adjustments needed for the scheduler when using this feature. NOTE: The use of this feature substantially limits of the overall USB performance that can be achieved."]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CM Controller Mode - R/WO. Controller mode is defaulted to the proper mode for host only and device only implementations. For those designs that contain both host & device capability, the controller defaults to an idle state and needs to be initialized to the desired operating mode after reset. For combination host/ device controllers, this register can only be written once after reset. If it is necessary to switch modes, software must reset the controller by writing to the RESET bit in the USBCMD register before reprogramming this register. For OTG controller core, reset value is '00b'. 00 - Idle \\[Default for combination host/device\\]
01 - Reserved 10 - Device Controller \\[Default for device only controller\\]
11 - Host Controller \\[Default for host only controller\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<0> {
        CM_W::new(self)
    }
    #[doc = "Bit 2 - ES Endian Select - Read/Write. This bit can change the byte alignment of the transfer buffers to match the host microprocessor. The bit fields in the microprocessor interface and the data structures are unaffected by the value of this bit because they are based upon the 32-bit word. Bit Meaning 0 - Little Endian \\[Default\\]
1 - Big Endian"]
    #[inline(always)]
    #[must_use]
    pub fn es(&mut self) -> ES_W<2> {
        ES_W::new(self)
    }
    #[doc = "Bit 3 - SLOM Setup Lockout Mode. In device mode, this bit controls behavior of the setup lock mechanism. See Control Endpoint Operation Model . 0 - Setup Lockouts On (default); 1 - Setup Lockouts Off. DCD requires use of Setup Data Buffer Tripwire in USBCMD."]
    #[inline(always)]
    #[must_use]
    pub fn slom(&mut self) -> SLOM_W<3> {
        SLOM_W::new(self)
    }
    #[doc = "Bit 4 - SDIS Stream Disable Mode. (0 - Inactive \\[default\\]; 1 - Active) Device Mode: Setting to a '1' disables double priming on both RX and TX for low bandwidth systems. This mode ensures that when the RX and TX buffers are sufficient to contain an entire packet that the standard double buffering scheme is disabled to prevent overruns/underruns in bandwidth limited systems. Note: In High Speed Mode, all packets received are responded to with a NYET handshake when stream disable is active. Host Mode: Setting to a '1' ensures that overruns/underruns of the latency FIFO are eliminated for low bandwidth systems where the RX and TX buffers are sufficient to contain the entire packet. Enabling stream disable also has the effect of ensuring the TX latency is filled to capacity before the packet is launched onto the USB. NOTE: Time duration to pre-fill the FIFO becomes significant when stream disable is active. See TXFILLTUNING and TXTTFILLTUNING \\[MPH Only\\]
to characterize the adjustments needed for the scheduler when using this feature. NOTE: The use of this feature substantially limits of the overall USB performance that can be achieved."]
    #[inline(always)]
    #[must_use]
    pub fn sdis(&mut self) -> SDIS_W<4> {
        SDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbmode](index.html) module"]
pub struct USBMODE_SPEC;
impl crate::RegisterSpec for USBMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbmode::R](R) reader structure"]
impl crate::Readable for USBMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbmode::W](W) writer structure"]
impl crate::Writable for USBMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBMODE to value 0"]
impl crate::Resettable for USBMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
