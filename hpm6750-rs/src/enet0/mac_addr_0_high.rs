#[doc = "Register `MAC_ADDR_0_HIGH` reader"]
pub struct R(crate::R<MAC_ADDR_0_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDR_0_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_ADDR_0_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_ADDR_0_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDR_0_HIGH` writer"]
pub struct W(crate::W<MAC_ADDR_0_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDR_0_HIGH_SPEC>;
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
impl From<crate::W<MAC_ADDR_0_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_ADDR_0_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRHI` reader - MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the first 6-byte MAC address. The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
pub type ADDRHI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDRHI` writer - MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the first 6-byte MAC address. The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
pub type ADDRHI_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAC_ADDR_0_HIGH_SPEC, u16, u16, 16, O>;
#[doc = "Field `AE` reader - Address Enable This bit is always set to 1."]
pub type AE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the first 6-byte MAC address. The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Address Enable This bit is always set to 1."]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the first 6-byte MAC address. The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
    #[inline(always)]
    #[must_use]
    pub fn addrhi(&mut self) -> ADDRHI_W<0> {
        ADDRHI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Address 0 High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_addr_0_high](index.html) module"]
pub struct MAC_ADDR_0_HIGH_SPEC;
impl crate::RegisterSpec for MAC_ADDR_0_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_addr_0_high::R](R) reader structure"]
impl crate::Readable for MAC_ADDR_0_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_addr_0_high::W](W) writer structure"]
impl crate::Writable for MAC_ADDR_0_HIGH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_ADDR_0_HIGH to value 0"]
impl crate::Resettable for MAC_ADDR_0_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
