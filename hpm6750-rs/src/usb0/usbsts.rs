#[doc = "Register `USBSTS` reader"]
pub struct R(crate::R<USBSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBSTS` writer"]
pub struct W(crate::W<USBSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBSTS_SPEC>;
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
impl From<crate::W<USBSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UI` reader - UI USB Interrupt (USBINT) - R/WC. This bit is set by the Host/Device Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set. This bit is also set by the Host/Device Controller when a short packet is detected. A short packet is when the actual number of bytes received was less than the expected number of bytes."]
pub type UI_R = crate::BitReader<bool>;
#[doc = "Field `UI` writer - UI USB Interrupt (USBINT) - R/WC. This bit is set by the Host/Device Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set. This bit is also set by the Host/Device Controller when a short packet is detected. A short packet is when the actual number of bytes received was less than the expected number of bytes."]
pub type UI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `UEI` reader - UEI USB Error Interrupt (USBERRINT) - R/WC. When completion of a USB transaction results in an error condition, this bit is set by the Host/Device Controller. This bit is set along with the USBINT bit, if the TD on which the error interrupt occurred also had its interrupt on complete (IOC) bit set."]
pub type UEI_R = crate::BitReader<bool>;
#[doc = "Field `UEI` writer - UEI USB Error Interrupt (USBERRINT) - R/WC. When completion of a USB transaction results in an error condition, this bit is set by the Host/Device Controller. This bit is set along with the USBINT bit, if the TD on which the error interrupt occurred also had its interrupt on complete (IOC) bit set."]
pub type UEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `PCI` reader - PCI Port Change Detect - R/WC. The Host Controller sets this bit to a one when on any port a Connect Status occurs, a Port Enable/Disable Change occurs, or the Force Port Resume bit is set as the result of a J-K transition on the suspended port. The Device Controller sets this bit to a one when the port controller enters the full or high-speed operational state. When the port controller exits the full or high-speed operation states due to Reset or Suspend events, the notification mechanisms are the USB Reset Received bit and the DCSuspend bits Respectively."]
pub type PCI_R = crate::BitReader<bool>;
#[doc = "Field `PCI` writer - PCI Port Change Detect - R/WC. The Host Controller sets this bit to a one when on any port a Connect Status occurs, a Port Enable/Disable Change occurs, or the Force Port Resume bit is set as the result of a J-K transition on the suspended port. The Device Controller sets this bit to a one when the port controller enters the full or high-speed operational state. When the port controller exits the full or high-speed operation states due to Reset or Suspend events, the notification mechanisms are the USB Reset Received bit and the DCSuspend bits Respectively."]
pub type PCI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `FRI` reader - FRI Frame List Rollover - R/WC. The Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example. If the frame list size (as programmed in the Frame List Size field of the USB_n_USBCMD register) is 1024, the Frame Index Register rolls over every time FRINDEX \\[13\\]
toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FHINDEX \\[12\\]
toggles. Only used in host operation mode."]
pub type FRI_R = crate::BitReader<bool>;
#[doc = "Field `FRI` writer - FRI Frame List Rollover - R/WC. The Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example. If the frame list size (as programmed in the Frame List Size field of the USB_n_USBCMD register) is 1024, the Frame Index Register rolls over every time FRINDEX \\[13\\]
toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FHINDEX \\[12\\]
toggles. Only used in host operation mode."]
pub type FRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `SEI` reader - System Error – RWC. Default = 0b. In the BVCI implementation of the USBHS core, this bit is not used, and will always be cleared to '0b'. In the AMBA implementation, this bit will be set to '1b' when an Error response is seen by the master interface (HRESP\\[1:0\\]=ERROR)"]
pub type SEI_R = crate::BitReader<bool>;
#[doc = "Field `SEI` writer - System Error – RWC. Default = 0b. In the BVCI implementation of the USBHS core, this bit is not used, and will always be cleared to '0b'. In the AMBA implementation, this bit will be set to '1b' when an Error response is seen by the master interface (HRESP\\[1:0\\]=ERROR)"]
pub type SEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `AAI` reader - AAI Interrupt on Async Advance - R/WC. System software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the n_USBCMD register. This status bit indicates the assertion of that interrupt source. Only used in host operation mode."]
pub type AAI_R = crate::BitReader<bool>;
#[doc = "Field `AAI` writer - AAI Interrupt on Async Advance - R/WC. System software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the n_USBCMD register. This status bit indicates the assertion of that interrupt source. Only used in host operation mode."]
pub type AAI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `URI` reader - URI USB Reset Received - R/WC. When the device controller detects a USB Reset and enters the default state, this bit will be set to a one. Software can write a 1 to this bit to clear the USB Reset Received status bit. Only used in device operation mode."]
pub type URI_R = crate::BitReader<bool>;
#[doc = "Field `URI` writer - URI USB Reset Received - R/WC. When the device controller detects a USB Reset and enters the default state, this bit will be set to a one. Software can write a 1 to this bit to clear the USB Reset Received status bit. Only used in device operation mode."]
pub type URI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `SRI` reader - SRI SOF Received - R/WC. When the device controller detects a Start Of (micro) Frame, this bit will be set to a one. When a SOF is extremely late, the device controller will automatically set this bit to indicate that an SOF was expected. Therefore, this bit will be set roughly every 1ms in device FS mode and every 125ms in HS mode and will be synchronized to the actual SOF that is received. Because the device controller is initialized to FS before connect, this bit will be set at an interval of 1ms during the prelude to connect and chirp. In host mode, this bit will be set every 125us and can be used by host controller driver as a time base. Software writes a 1 to this bit to clear it."]
pub type SRI_R = crate::BitReader<bool>;
#[doc = "Field `SRI` writer - SRI SOF Received - R/WC. When the device controller detects a Start Of (micro) Frame, this bit will be set to a one. When a SOF is extremely late, the device controller will automatically set this bit to indicate that an SOF was expected. Therefore, this bit will be set roughly every 1ms in device FS mode and every 125ms in HS mode and will be synchronized to the actual SOF that is received. Because the device controller is initialized to FS before connect, this bit will be set at an interval of 1ms during the prelude to connect and chirp. In host mode, this bit will be set every 125us and can be used by host controller driver as a time base. Software writes a 1 to this bit to clear it."]
pub type SRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `SLI` reader - SLI DCSuspend - R/WC. When a controller enters a suspend state from an active state, this bit will be set to a one. The device controller clears the bit upon exiting from a suspend state. Only used in device operation mode."]
pub type SLI_R = crate::BitReader<bool>;
#[doc = "Field `SLI` writer - SLI DCSuspend - R/WC. When a controller enters a suspend state from an active state, this bit will be set to a one. The device controller clears the bit upon exiting from a suspend state. Only used in device operation mode."]
pub type SLI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `HCH` reader - HCH HCHaIted - Read Only. This bit is a zero whenever the Run/Stop bit is a one. The Controller sets this bit to one after it has stopped executing because of the Run/Stop bit being set to 0, either by software or by the Controller hardware (for example, an internal error). Only used in the host operation mode. Default value is '0b' for OTG core . This is because OTG core is not operating as host in default. Please see CM bit in USB_n_USBMODE register. NOTE: HCH bit reset value: '0b' for OTG controller core ."]
pub type HCH_R = crate::BitReader<bool>;
#[doc = "Field `RCL` reader - RCL Reclamation - Read Only. This is a read-only status bit used to detect an empty asynchronous schedule. Only used in the host operation mode."]
pub type RCL_R = crate::BitReader<bool>;
#[doc = "Field `PS` reader - PS Periodic Schedule Status - Read Only. This bit reports the current real status of the Periodic Schedule. When set to zero the periodic schedule is disabled, and if set to one the status is enabled. The Host Controller is not required to immediately disable or enable the Periodic Schedule when software transitions the Periodic Schedule Enable bit in the USBCMD register. When this bit and the Periodic Schedule Enable bit are the same value, the Periodic Schedule is either enabled (1) or disabled (0). Only used in the host operation mode."]
pub type PS_R = crate::BitReader<bool>;
#[doc = "Field `AS` reader - AS Asynchronous Schedule Status - Read Only. This bit reports the current real status of the Asynchronous Schedule. When set to zero the asynchronous schedule status is disabled and if set to one the status is enabled. The Host Controller is not required to immediately disable or enable the Asynchronous Schedule when software transitions the Asynchronous Schedule Enable bit in the USBCMD register. When this bit and the Asynchronous Schedule Enable bit are the same value, the Asynchronous Schedule is either enabled (1) or disabled (0). Only used in the host operation mode."]
pub type AS_R = crate::BitReader<bool>;
#[doc = "Field `NAKI` reader - NAKI NAK Interrupt Bit--RO. This bit is set by hardware when for a particular endpoint both the TX/RX Endpoint NAK bit and corresponding TX/RX Endpoint NAK Enable bit are set. This bit is automatically cleared by hardware when all Enabled TX/RX Endpoint NAK bits are cleared."]
pub type NAKI_R = crate::BitReader<bool>;
#[doc = "Field `UAI` reader - USB Host Asynchronous Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set AND the TD was from the asynchronous schedule. This bit is also set by the Host when a short packet is detected and the packet is on the asynchronous schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero"]
pub type UAI_R = crate::BitReader<bool>;
#[doc = "Field `UAI` writer - USB Host Asynchronous Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set AND the TD was from the asynchronous schedule. This bit is also set by the Host when a short packet is detected and the packet is on the asynchronous schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero"]
pub type UAI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `UPI` reader - USB Host Periodic Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set and the TD was from the periodic schedule. This bit is also set by the Host Controller when a short packet is detected and the packet is on the periodic schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero."]
pub type UPI_R = crate::BitReader<bool>;
#[doc = "Field `UPI` writer - USB Host Periodic Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set and the TD was from the periodic schedule. This bit is also set by the Host Controller when a short packet is detected and the packet is on the periodic schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero."]
pub type UPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `TI0` reader - TI0 General Purpose Timer Interrupt 0(GPTINT0)--R/WC. This bit is set when the counter in the GPTIMER0CTRL register transitions to zero, writing a one to this bit clears it."]
pub type TI0_R = crate::BitReader<bool>;
#[doc = "Field `TI0` writer - TI0 General Purpose Timer Interrupt 0(GPTINT0)--R/WC. This bit is set when the counter in the GPTIMER0CTRL register transitions to zero, writing a one to this bit clears it."]
pub type TI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `TI1` reader - TI1 General Purpose Timer Interrupt 1(GPTINT1)--R/WC. This bit is set when the counter in the GPTIMER1CTRL register transitions to zero, writing a one to this bit will clear it."]
pub type TI1_R = crate::BitReader<bool>;
#[doc = "Field `TI1` writer - TI1 General Purpose Timer Interrupt 1(GPTINT1)--R/WC. This bit is set when the counter in the GPTIMER1CTRL register transitions to zero, writing a one to this bit will clear it."]
pub type TI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UI USB Interrupt (USBINT) - R/WC. This bit is set by the Host/Device Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set. This bit is also set by the Host/Device Controller when a short packet is detected. A short packet is when the actual number of bytes received was less than the expected number of bytes."]
    #[inline(always)]
    pub fn ui(&self) -> UI_R {
        UI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UEI USB Error Interrupt (USBERRINT) - R/WC. When completion of a USB transaction results in an error condition, this bit is set by the Host/Device Controller. This bit is set along with the USBINT bit, if the TD on which the error interrupt occurred also had its interrupt on complete (IOC) bit set."]
    #[inline(always)]
    pub fn uei(&self) -> UEI_R {
        UEI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCI Port Change Detect - R/WC. The Host Controller sets this bit to a one when on any port a Connect Status occurs, a Port Enable/Disable Change occurs, or the Force Port Resume bit is set as the result of a J-K transition on the suspended port. The Device Controller sets this bit to a one when the port controller enters the full or high-speed operational state. When the port controller exits the full or high-speed operation states due to Reset or Suspend events, the notification mechanisms are the USB Reset Received bit and the DCSuspend bits Respectively."]
    #[inline(always)]
    pub fn pci(&self) -> PCI_R {
        PCI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FRI Frame List Rollover - R/WC. The Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example. If the frame list size (as programmed in the Frame List Size field of the USB_n_USBCMD register) is 1024, the Frame Index Register rolls over every time FRINDEX \\[13\\]
toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FHINDEX \\[12\\]
toggles. Only used in host operation mode."]
    #[inline(always)]
    pub fn fri(&self) -> FRI_R {
        FRI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - System Error – RWC. Default = 0b. In the BVCI implementation of the USBHS core, this bit is not used, and will always be cleared to '0b'. In the AMBA implementation, this bit will be set to '1b' when an Error response is seen by the master interface (HRESP\\[1:0\\]=ERROR)"]
    #[inline(always)]
    pub fn sei(&self) -> SEI_R {
        SEI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AAI Interrupt on Async Advance - R/WC. System software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the n_USBCMD register. This status bit indicates the assertion of that interrupt source. Only used in host operation mode."]
    #[inline(always)]
    pub fn aai(&self) -> AAI_R {
        AAI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - URI USB Reset Received - R/WC. When the device controller detects a USB Reset and enters the default state, this bit will be set to a one. Software can write a 1 to this bit to clear the USB Reset Received status bit. Only used in device operation mode."]
    #[inline(always)]
    pub fn uri(&self) -> URI_R {
        URI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SRI SOF Received - R/WC. When the device controller detects a Start Of (micro) Frame, this bit will be set to a one. When a SOF is extremely late, the device controller will automatically set this bit to indicate that an SOF was expected. Therefore, this bit will be set roughly every 1ms in device FS mode and every 125ms in HS mode and will be synchronized to the actual SOF that is received. Because the device controller is initialized to FS before connect, this bit will be set at an interval of 1ms during the prelude to connect and chirp. In host mode, this bit will be set every 125us and can be used by host controller driver as a time base. Software writes a 1 to this bit to clear it."]
    #[inline(always)]
    pub fn sri(&self) -> SRI_R {
        SRI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SLI DCSuspend - R/WC. When a controller enters a suspend state from an active state, this bit will be set to a one. The device controller clears the bit upon exiting from a suspend state. Only used in device operation mode."]
    #[inline(always)]
    pub fn sli(&self) -> SLI_R {
        SLI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - HCH HCHaIted - Read Only. This bit is a zero whenever the Run/Stop bit is a one. The Controller sets this bit to one after it has stopped executing because of the Run/Stop bit being set to 0, either by software or by the Controller hardware (for example, an internal error). Only used in the host operation mode. Default value is '0b' for OTG core . This is because OTG core is not operating as host in default. Please see CM bit in USB_n_USBMODE register. NOTE: HCH bit reset value: '0b' for OTG controller core ."]
    #[inline(always)]
    pub fn hch(&self) -> HCH_R {
        HCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RCL Reclamation - Read Only. This is a read-only status bit used to detect an empty asynchronous schedule. Only used in the host operation mode."]
    #[inline(always)]
    pub fn rcl(&self) -> RCL_R {
        RCL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PS Periodic Schedule Status - Read Only. This bit reports the current real status of the Periodic Schedule. When set to zero the periodic schedule is disabled, and if set to one the status is enabled. The Host Controller is not required to immediately disable or enable the Periodic Schedule when software transitions the Periodic Schedule Enable bit in the USBCMD register. When this bit and the Periodic Schedule Enable bit are the same value, the Periodic Schedule is either enabled (1) or disabled (0). Only used in the host operation mode."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AS Asynchronous Schedule Status - Read Only. This bit reports the current real status of the Asynchronous Schedule. When set to zero the asynchronous schedule status is disabled and if set to one the status is enabled. The Host Controller is not required to immediately disable or enable the Asynchronous Schedule when software transitions the Asynchronous Schedule Enable bit in the USBCMD register. When this bit and the Asynchronous Schedule Enable bit are the same value, the Asynchronous Schedule is either enabled (1) or disabled (0). Only used in the host operation mode."]
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NAKI NAK Interrupt Bit--RO. This bit is set by hardware when for a particular endpoint both the TX/RX Endpoint NAK bit and corresponding TX/RX Endpoint NAK Enable bit are set. This bit is automatically cleared by hardware when all Enabled TX/RX Endpoint NAK bits are cleared."]
    #[inline(always)]
    pub fn naki(&self) -> NAKI_R {
        NAKI_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - USB Host Asynchronous Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set AND the TD was from the asynchronous schedule. This bit is also set by the Host when a short packet is detected and the packet is on the asynchronous schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero"]
    #[inline(always)]
    pub fn uai(&self) -> UAI_R {
        UAI_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USB Host Periodic Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set and the TD was from the periodic schedule. This bit is also set by the Host Controller when a short packet is detected and the packet is on the periodic schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero."]
    #[inline(always)]
    pub fn upi(&self) -> UPI_R {
        UPI_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - TI0 General Purpose Timer Interrupt 0(GPTINT0)--R/WC. This bit is set when the counter in the GPTIMER0CTRL register transitions to zero, writing a one to this bit clears it."]
    #[inline(always)]
    pub fn ti0(&self) -> TI0_R {
        TI0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TI1 General Purpose Timer Interrupt 1(GPTINT1)--R/WC. This bit is set when the counter in the GPTIMER1CTRL register transitions to zero, writing a one to this bit will clear it."]
    #[inline(always)]
    pub fn ti1(&self) -> TI1_R {
        TI1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UI USB Interrupt (USBINT) - R/WC. This bit is set by the Host/Device Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set. This bit is also set by the Host/Device Controller when a short packet is detected. A short packet is when the actual number of bytes received was less than the expected number of bytes."]
    #[inline(always)]
    #[must_use]
    pub fn ui(&mut self) -> UI_W<0> {
        UI_W::new(self)
    }
    #[doc = "Bit 1 - UEI USB Error Interrupt (USBERRINT) - R/WC. When completion of a USB transaction results in an error condition, this bit is set by the Host/Device Controller. This bit is set along with the USBINT bit, if the TD on which the error interrupt occurred also had its interrupt on complete (IOC) bit set."]
    #[inline(always)]
    #[must_use]
    pub fn uei(&mut self) -> UEI_W<1> {
        UEI_W::new(self)
    }
    #[doc = "Bit 2 - PCI Port Change Detect - R/WC. The Host Controller sets this bit to a one when on any port a Connect Status occurs, a Port Enable/Disable Change occurs, or the Force Port Resume bit is set as the result of a J-K transition on the suspended port. The Device Controller sets this bit to a one when the port controller enters the full or high-speed operational state. When the port controller exits the full or high-speed operation states due to Reset or Suspend events, the notification mechanisms are the USB Reset Received bit and the DCSuspend bits Respectively."]
    #[inline(always)]
    #[must_use]
    pub fn pci(&mut self) -> PCI_W<2> {
        PCI_W::new(self)
    }
    #[doc = "Bit 3 - FRI Frame List Rollover - R/WC. The Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example. If the frame list size (as programmed in the Frame List Size field of the USB_n_USBCMD register) is 1024, the Frame Index Register rolls over every time FRINDEX \\[13\\]
toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FHINDEX \\[12\\]
toggles. Only used in host operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn fri(&mut self) -> FRI_W<3> {
        FRI_W::new(self)
    }
    #[doc = "Bit 4 - System Error – RWC. Default = 0b. In the BVCI implementation of the USBHS core, this bit is not used, and will always be cleared to '0b'. In the AMBA implementation, this bit will be set to '1b' when an Error response is seen by the master interface (HRESP\\[1:0\\]=ERROR)"]
    #[inline(always)]
    #[must_use]
    pub fn sei(&mut self) -> SEI_W<4> {
        SEI_W::new(self)
    }
    #[doc = "Bit 5 - AAI Interrupt on Async Advance - R/WC. System software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the n_USBCMD register. This status bit indicates the assertion of that interrupt source. Only used in host operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn aai(&mut self) -> AAI_W<5> {
        AAI_W::new(self)
    }
    #[doc = "Bit 6 - URI USB Reset Received - R/WC. When the device controller detects a USB Reset and enters the default state, this bit will be set to a one. Software can write a 1 to this bit to clear the USB Reset Received status bit. Only used in device operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn uri(&mut self) -> URI_W<6> {
        URI_W::new(self)
    }
    #[doc = "Bit 7 - SRI SOF Received - R/WC. When the device controller detects a Start Of (micro) Frame, this bit will be set to a one. When a SOF is extremely late, the device controller will automatically set this bit to indicate that an SOF was expected. Therefore, this bit will be set roughly every 1ms in device FS mode and every 125ms in HS mode and will be synchronized to the actual SOF that is received. Because the device controller is initialized to FS before connect, this bit will be set at an interval of 1ms during the prelude to connect and chirp. In host mode, this bit will be set every 125us and can be used by host controller driver as a time base. Software writes a 1 to this bit to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn sri(&mut self) -> SRI_W<7> {
        SRI_W::new(self)
    }
    #[doc = "Bit 8 - SLI DCSuspend - R/WC. When a controller enters a suspend state from an active state, this bit will be set to a one. The device controller clears the bit upon exiting from a suspend state. Only used in device operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn sli(&mut self) -> SLI_W<8> {
        SLI_W::new(self)
    }
    #[doc = "Bit 18 - USB Host Asynchronous Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set AND the TD was from the asynchronous schedule. This bit is also set by the Host when a short packet is detected and the packet is on the asynchronous schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero"]
    #[inline(always)]
    #[must_use]
    pub fn uai(&mut self) -> UAI_W<18> {
        UAI_W::new(self)
    }
    #[doc = "Bit 19 - USB Host Periodic Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set and the TD was from the periodic schedule. This bit is also set by the Host Controller when a short packet is detected and the packet is on the periodic schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero."]
    #[inline(always)]
    #[must_use]
    pub fn upi(&mut self) -> UPI_W<19> {
        UPI_W::new(self)
    }
    #[doc = "Bit 24 - TI0 General Purpose Timer Interrupt 0(GPTINT0)--R/WC. This bit is set when the counter in the GPTIMER0CTRL register transitions to zero, writing a one to this bit clears it."]
    #[inline(always)]
    #[must_use]
    pub fn ti0(&mut self) -> TI0_W<24> {
        TI0_W::new(self)
    }
    #[doc = "Bit 25 - TI1 General Purpose Timer Interrupt 1(GPTINT1)--R/WC. This bit is set when the counter in the GPTIMER1CTRL register transitions to zero, writing a one to this bit will clear it."]
    #[inline(always)]
    #[must_use]
    pub fn ti1(&mut self) -> TI1_W<25> {
        TI1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbsts](index.html) module"]
pub struct USBSTS_SPEC;
impl crate::RegisterSpec for USBSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbsts::R](R) reader structure"]
impl crate::Readable for USBSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbsts::W](W) writer structure"]
impl crate::Writable for USBSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBSTS to value 0"]
impl crate::Resettable for USBSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
