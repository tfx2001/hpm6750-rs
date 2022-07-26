#[doc = "Register `DEVICEADDR` reader"]
pub struct R(crate::R<DEVICEADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICEADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICEADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICEADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVICEADDR` writer"]
pub struct W(crate::W<DEVICEADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVICEADDR_SPEC>;
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
impl From<crate::W<DEVICEADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVICEADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBADR` reader - USBADR Device Address. These bits correspond to the USB device address"]
pub type USBADR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBADR` writer - USBADR Device Address. These bits correspond to the USB device address"]
pub type USBADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEVICEADDR_SPEC, u8, u8, 7, O>;
#[doc = "Field `USBADRA` reader - USBADRA Device Address Advance. Default=0. When this bit is '0', any writes to USBADR are instantaneous. When this bit is written to a '1' at the same time or before USBADR is written, the write to the USBADR field is staged and held in a hidden register. After an IN occurs on endpoint 0 and is ACKed, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: 1) IN is ACKed to endpoint 0. (USBADR is updated from staging register). 2) OUT/SETUP occur to endpoint 0. (USBADR is not updated). 3) Device Reset occurs (USBADR is reset to 0). NOTE: After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write of the device address within 2ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2ms USB requirement."]
pub type USBADRA_R = crate::BitReader<bool>;
#[doc = "Field `USBADRA` writer - USBADRA Device Address Advance. Default=0. When this bit is '0', any writes to USBADR are instantaneous. When this bit is written to a '1' at the same time or before USBADR is written, the write to the USBADR field is staged and held in a hidden register. After an IN occurs on endpoint 0 and is ACKed, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: 1) IN is ACKed to endpoint 0. (USBADR is updated from staging register). 2) OUT/SETUP occur to endpoint 0. (USBADR is not updated). 3) Device Reset occurs (USBADR is reset to 0). NOTE: After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write of the device address within 2ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2ms USB requirement."]
pub type USBADRA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVICEADDR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 25:31 - USBADR Device Address. These bits correspond to the USB device address"]
    #[inline(always)]
    pub fn usbadr(&self) -> USBADR_R {
        USBADR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - USBADRA Device Address Advance. Default=0. When this bit is '0', any writes to USBADR are instantaneous. When this bit is written to a '1' at the same time or before USBADR is written, the write to the USBADR field is staged and held in a hidden register. After an IN occurs on endpoint 0 and is ACKed, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: 1) IN is ACKed to endpoint 0. (USBADR is updated from staging register). 2) OUT/SETUP occur to endpoint 0. (USBADR is not updated). 3) Device Reset occurs (USBADR is reset to 0). NOTE: After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write of the device address within 2ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2ms USB requirement."]
    #[inline(always)]
    pub fn usbadra(&self) -> USBADRA_R {
        USBADRA_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 25:31 - USBADR Device Address. These bits correspond to the USB device address"]
    #[inline(always)]
    pub fn usbadr(&mut self) -> USBADR_W<25> {
        USBADR_W::new(self)
    }
    #[doc = "Bit 24 - USBADRA Device Address Advance. Default=0. When this bit is '0', any writes to USBADR are instantaneous. When this bit is written to a '1' at the same time or before USBADR is written, the write to the USBADR field is staged and held in a hidden register. After an IN occurs on endpoint 0 and is ACKed, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: 1) IN is ACKed to endpoint 0. (USBADR is updated from staging register). 2) OUT/SETUP occur to endpoint 0. (USBADR is not updated). 3) Device Reset occurs (USBADR is reset to 0). NOTE: After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write of the device address within 2ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2ms USB requirement."]
    #[inline(always)]
    pub fn usbadra(&mut self) -> USBADRA_W<24> {
        USBADRA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceaddr](index.html) module"]
pub struct DEVICEADDR_SPEC;
impl crate::RegisterSpec for DEVICEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deviceaddr::R](R) reader structure"]
impl crate::Readable for DEVICEADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [deviceaddr::W](W) writer structure"]
impl crate::Writable for DEVICEADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVICEADDR to value 0"]
impl crate::Resettable for DEVICEADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
