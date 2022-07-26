#[doc = "Register `WDOG_WTO` reader"]
pub struct R(crate::R<WDOG_WTO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_WTO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOG_WTO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOG_WTO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDOG_WTO` writer"]
pub struct W(crate::W<WDOG_WTO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_WTO_SPEC>;
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
impl From<crate::W<WDOG_WTO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOG_WTO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWE` reader - Programmable Watchdog Enable When this bit is set and Bit 23 (WD) of Register 0 (MAC Configuration Register) is reset, the WTO field (Bits\\[13:0\\]) is used as watchdog timeout for a received frame. When this bit is cleared, the watchdog timeout for a received frame is controlled by the setting of Bit 23 (WD) and Bit 20 (JE) in Register 0 (MAC Configuration Register)."]
pub type PWE_R = crate::BitReader<bool>;
#[doc = "Field `PWE` writer - Programmable Watchdog Enable When this bit is set and Bit 23 (WD) of Register 0 (MAC Configuration Register) is reset, the WTO field (Bits\\[13:0\\]) is used as watchdog timeout for a received frame. When this bit is cleared, the watchdog timeout for a received frame is controlled by the setting of Bit 23 (WD) and Bit 20 (JE) in Register 0 (MAC Configuration Register)."]
pub type PWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDOG_WTO_SPEC, bool, O>;
#[doc = "Field `WTO` reader - Watchdog Timeout When Bit 16 (PWE) is set and Bit 23 (WD) of Register 0 (MAC Configuration Register) is reset, this field is used as watchdog timeout for a received frame. If the length of a received frame exceeds the value of this field, such frame is terminated and declared as an error frame. Note: When Bit 16 (PWE) is set, the value in this field should be more than 1,522 (0x05F2). Otherwise, the IEEE Std 802.3-specified valid tagged frames are declared as error frames and are dropped."]
pub type WTO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WTO` writer - Watchdog Timeout When Bit 16 (PWE) is set and Bit 23 (WD) of Register 0 (MAC Configuration Register) is reset, this field is used as watchdog timeout for a received frame. If the length of a received frame exceeds the value of this field, such frame is terminated and declared as an error frame. Note: When Bit 16 (PWE) is set, the value in this field should be more than 1,522 (0x05F2). Otherwise, the IEEE Std 802.3-specified valid tagged frames are declared as error frames and are dropped."]
pub type WTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDOG_WTO_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bit 16 - Programmable Watchdog Enable When this bit is set and Bit 23 (WD) of Register 0 (MAC Configuration Register) is reset, the WTO field (Bits\\[13:0\\]) is used as watchdog timeout for a received frame. When this bit is cleared, the watchdog timeout for a received frame is controlled by the setting of Bit 23 (WD) and Bit 20 (JE) in Register 0 (MAC Configuration Register)."]
    #[inline(always)]
    pub fn pwe(&self) -> PWE_R {
        PWE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 0:13 - Watchdog Timeout When Bit 16 (PWE) is set and Bit 23 (WD) of Register 0 (MAC Configuration Register) is reset, this field is used as watchdog timeout for a received frame. If the length of a received frame exceeds the value of this field, such frame is terminated and declared as an error frame. Note: When Bit 16 (PWE) is set, the value in this field should be more than 1,522 (0x05F2). Otherwise, the IEEE Std 802.3-specified valid tagged frames are declared as error frames and are dropped."]
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - Programmable Watchdog Enable When this bit is set and Bit 23 (WD) of Register 0 (MAC Configuration Register) is reset, the WTO field (Bits\\[13:0\\]) is used as watchdog timeout for a received frame. When this bit is cleared, the watchdog timeout for a received frame is controlled by the setting of Bit 23 (WD) and Bit 20 (JE) in Register 0 (MAC Configuration Register)."]
    #[inline(always)]
    pub fn pwe(&mut self) -> PWE_W<16> {
        PWE_W::new(self)
    }
    #[doc = "Bits 0:13 - Watchdog Timeout When Bit 16 (PWE) is set and Bit 23 (WD) of Register 0 (MAC Configuration Register) is reset, this field is used as watchdog timeout for a received frame. If the length of a received frame exceeds the value of this field, such frame is terminated and declared as an error frame. Note: When Bit 16 (PWE) is set, the value in this field should be more than 1,522 (0x05F2). Otherwise, the IEEE Std 802.3-specified valid tagged frames are declared as error frames and are dropped."]
    #[inline(always)]
    pub fn wto(&mut self) -> WTO_W<0> {
        WTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_wto](index.html) module"]
pub struct WDOG_WTO_SPEC;
impl crate::RegisterSpec for WDOG_WTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdog_wto::R](R) reader structure"]
impl crate::Readable for WDOG_WTO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog_wto::W](W) writer structure"]
impl crate::Writable for WDOG_WTO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOG_WTO to value 0"]
impl crate::Resettable for WDOG_WTO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
