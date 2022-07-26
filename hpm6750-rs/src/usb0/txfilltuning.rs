#[doc = "Register `TXFILLTUNING` reader"]
pub struct R(crate::R<TXFILLTUNING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFILLTUNING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFILLTUNING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFILLTUNING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXFILLTUNING` writer"]
pub struct W(crate::W<TXFILLTUNING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFILLTUNING_SPEC>;
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
impl From<crate::W<TXFILLTUNING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFILLTUNING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFIFOTHRES` reader - TXFIFOTHRES FIFO Burst Threshold. (Read/Write) This register controls the number of data bursts that are posted to the TX latency FIFO in host mode before the packet begins on to the bus. The minimum value is 2 and this value should be a low as possible to maximize USB performance. A higher value can be used in systems with unpredictable latency and/or insufficient bandwidth where the FIFO may underrun because the data transferred from the latency FIFO to USB occurs before it can be replenished from system memory. This value is ignored if the Stream Disable bit in USB_n_USBMODE register is set."]
pub type TXFIFOTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFIFOTHRES` writer - TXFIFOTHRES FIFO Burst Threshold. (Read/Write) This register controls the number of data bursts that are posted to the TX latency FIFO in host mode before the packet begins on to the bus. The minimum value is 2 and this value should be a low as possible to maximize USB performance. A higher value can be used in systems with unpredictable latency and/or insufficient bandwidth where the FIFO may underrun because the data transferred from the latency FIFO to USB occurs before it can be replenished from system memory. This value is ignored if the Stream Disable bit in USB_n_USBMODE register is set."]
pub type TXFIFOTHRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TXFILLTUNING_SPEC, u8, u8, 6, O>;
#[doc = "Field `TXSCHHEALTH` reader - TXSCHHEALTH Scheduler Health Counter. (Read/Write To Clear) Table continues on the next page This register increments when the host controller fails to fill the TX latency FIFO to the level programmed by TXFIFOTHRES before running out of time to send the packet before the next Start-Of-Frame. This health counter measures the number of times this occurs to provide feedback to selecting a proper TXSCHOH. Writing to this register will clear the counter and this counter will max. at 31."]
pub type TXSCHHEALTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXSCHHEALTH` writer - TXSCHHEALTH Scheduler Health Counter. (Read/Write To Clear) Table continues on the next page This register increments when the host controller fails to fill the TX latency FIFO to the level programmed by TXFIFOTHRES before running out of time to send the packet before the next Start-Of-Frame. This health counter measures the number of times this occurs to provide feedback to selecting a proper TXSCHOH. Writing to this register will clear the counter and this counter will max. at 31."]
pub type TXSCHHEALTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TXFILLTUNING_SPEC, u8, u8, 5, O>;
#[doc = "Field `TXSCHOH` reader - TXSCHOH Scheduler Overhead. (Read/Write) \\[Default = 0\\]
This register adds an additional fixed offset to the schedule time estimator described above as Tff. As an approximation, the value chosen for this register should limit the number of back-off events captured in the TXSCHHEALTH to less than 10 per second in a highly utilized bus. Choosing a value that is too high for this register is not desired as it can needlessly reduce USB utilization. The time unit represented in this register is 1.267us when a device is connected in High-Speed Mode. The time unit represented in this register is 6.333us when a device is connected in Low/Full Speed Mode. Default value is '08h' for OTG controller core ."]
pub type TXSCHOH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXSCHOH` writer - TXSCHOH Scheduler Overhead. (Read/Write) \\[Default = 0\\]
This register adds an additional fixed offset to the schedule time estimator described above as Tff. As an approximation, the value chosen for this register should limit the number of back-off events captured in the TXSCHHEALTH to less than 10 per second in a highly utilized bus. Choosing a value that is too high for this register is not desired as it can needlessly reduce USB utilization. The time unit represented in this register is 1.267us when a device is connected in High-Speed Mode. The time unit represented in this register is 6.333us when a device is connected in Low/Full Speed Mode. Default value is '08h' for OTG controller core ."]
pub type TXSCHOH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXFILLTUNING_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 16:21 - TXFIFOTHRES FIFO Burst Threshold. (Read/Write) This register controls the number of data bursts that are posted to the TX latency FIFO in host mode before the packet begins on to the bus. The minimum value is 2 and this value should be a low as possible to maximize USB performance. A higher value can be used in systems with unpredictable latency and/or insufficient bandwidth where the FIFO may underrun because the data transferred from the latency FIFO to USB occurs before it can be replenished from system memory. This value is ignored if the Stream Disable bit in USB_n_USBMODE register is set."]
    #[inline(always)]
    pub fn txfifothres(&self) -> TXFIFOTHRES_R {
        TXFIFOTHRES_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - TXSCHHEALTH Scheduler Health Counter. (Read/Write To Clear) Table continues on the next page This register increments when the host controller fails to fill the TX latency FIFO to the level programmed by TXFIFOTHRES before running out of time to send the packet before the next Start-Of-Frame. This health counter measures the number of times this occurs to provide feedback to selecting a proper TXSCHOH. Writing to this register will clear the counter and this counter will max. at 31."]
    #[inline(always)]
    pub fn txschhealth(&self) -> TXSCHHEALTH_R {
        TXSCHHEALTH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:6 - TXSCHOH Scheduler Overhead. (Read/Write) \\[Default = 0\\]
This register adds an additional fixed offset to the schedule time estimator described above as Tff. As an approximation, the value chosen for this register should limit the number of back-off events captured in the TXSCHHEALTH to less than 10 per second in a highly utilized bus. Choosing a value that is too high for this register is not desired as it can needlessly reduce USB utilization. The time unit represented in this register is 1.267us when a device is connected in High-Speed Mode. The time unit represented in this register is 6.333us when a device is connected in Low/Full Speed Mode. Default value is '08h' for OTG controller core ."]
    #[inline(always)]
    pub fn txschoh(&self) -> TXSCHOH_R {
        TXSCHOH_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:21 - TXFIFOTHRES FIFO Burst Threshold. (Read/Write) This register controls the number of data bursts that are posted to the TX latency FIFO in host mode before the packet begins on to the bus. The minimum value is 2 and this value should be a low as possible to maximize USB performance. A higher value can be used in systems with unpredictable latency and/or insufficient bandwidth where the FIFO may underrun because the data transferred from the latency FIFO to USB occurs before it can be replenished from system memory. This value is ignored if the Stream Disable bit in USB_n_USBMODE register is set."]
    #[inline(always)]
    pub fn txfifothres(&mut self) -> TXFIFOTHRES_W<16> {
        TXFIFOTHRES_W::new(self)
    }
    #[doc = "Bits 8:12 - TXSCHHEALTH Scheduler Health Counter. (Read/Write To Clear) Table continues on the next page This register increments when the host controller fails to fill the TX latency FIFO to the level programmed by TXFIFOTHRES before running out of time to send the packet before the next Start-Of-Frame. This health counter measures the number of times this occurs to provide feedback to selecting a proper TXSCHOH. Writing to this register will clear the counter and this counter will max. at 31."]
    #[inline(always)]
    pub fn txschhealth(&mut self) -> TXSCHHEALTH_W<8> {
        TXSCHHEALTH_W::new(self)
    }
    #[doc = "Bits 0:6 - TXSCHOH Scheduler Overhead. (Read/Write) \\[Default = 0\\]
This register adds an additional fixed offset to the schedule time estimator described above as Tff. As an approximation, the value chosen for this register should limit the number of back-off events captured in the TXSCHHEALTH to less than 10 per second in a highly utilized bus. Choosing a value that is too high for this register is not desired as it can needlessly reduce USB utilization. The time unit represented in this register is 1.267us when a device is connected in High-Speed Mode. The time unit represented in this register is 6.333us when a device is connected in Low/Full Speed Mode. Default value is '08h' for OTG controller core ."]
    #[inline(always)]
    pub fn txschoh(&mut self) -> TXSCHOH_W<0> {
        TXSCHOH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX FIFO Fill Tuning Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfilltuning](index.html) module"]
pub struct TXFILLTUNING_SPEC;
impl crate::RegisterSpec for TXFILLTUNING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfilltuning::R](R) reader structure"]
impl crate::Readable for TXFILLTUNING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txfilltuning::W](W) writer structure"]
impl crate::Writable for TXFILLTUNING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXFILLTUNING to value 0"]
impl crate::Resettable for TXFILLTUNING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
