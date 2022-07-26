#[doc = "Register `MAC_ADDR_2_HIGH` reader"]
pub struct R(crate::R<MAC_ADDR_2_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDR_2_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_ADDR_2_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_ADDR_2_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDR_2_HIGH` writer"]
pub struct W(crate::W<MAC_ADDR_2_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDR_2_HIGH_SPEC>;
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
impl From<crate::W<MAC_ADDR_2_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_ADDR_2_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AE` reader - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering."]
pub type AE_R = crate::BitReader<bool>;
#[doc = "Field `AE` writer - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering."]
pub type AE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_ADDR_2_HIGH_SPEC, bool, O>;
#[doc = "Field `SA` reader - Source Address When this bit is set, the MAC Address1\\[47:0\\]
is used to compare with the SA fields of the received frame. When this bit is reset, the MAC Address1\\[47:0\\]
is used to compare with the DA fields of the received frame."]
pub type SA_R = crate::BitReader<bool>;
#[doc = "Field `SA` writer - Source Address When this bit is set, the MAC Address1\\[47:0\\]
is used to compare with the SA fields of the received frame. When this bit is reset, the MAC Address1\\[47:0\\]
is used to compare with the DA fields of the received frame."]
pub type SA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_ADDR_2_HIGH_SPEC, bool, O>;
#[doc = "Field `MBC` reader - Mask Byte Control These bits are mask control bits for comparison of each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: - Bit 29: Register 18\\[15:8\\]
- Bit 28: Register 18\\[7:0\\]
- Bit 27: Register 19\\[31:24\\]
- ... - Bit 24: Register 19\\[7:0\\]
You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
pub type MBC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MBC` writer - Mask Byte Control These bits are mask control bits for comparison of each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: - Bit 29: Register 18\\[15:8\\]
- Bit 28: Register 18\\[7:0\\]
- Bit 27: Register 19\\[31:24\\]
- ... - Bit 24: Register 19\\[7:0\\]
You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
pub type MBC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_ADDR_2_HIGH_SPEC, u8, u8, 6, O>;
#[doc = "Field `ADDRHI` reader - MAC Address1 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the second 6-byte MAC address."]
pub type ADDRHI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDRHI` writer - MAC Address1 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the second 6-byte MAC address."]
pub type ADDRHI_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAC_ADDR_2_HIGH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 31 - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering."]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Source Address When this bit is set, the MAC Address1\\[47:0\\]
is used to compare with the SA fields of the received frame. When this bit is reset, the MAC Address1\\[47:0\\]
is used to compare with the DA fields of the received frame."]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Mask Byte Control These bits are mask control bits for comparison of each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: - Bit 29: Register 18\\[15:8\\]
- Bit 28: Register 18\\[7:0\\]
- Bit 27: Register 19\\[31:24\\]
- ... - Bit 24: Register 19\\[7:0\\]
You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 0:15 - MAC Address1 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the second 6-byte MAC address."]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering."]
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W<31> {
        AE_W::new(self)
    }
    #[doc = "Bit 30 - Source Address When this bit is set, the MAC Address1\\[47:0\\]
is used to compare with the SA fields of the received frame. When this bit is reset, the MAC Address1\\[47:0\\]
is used to compare with the DA fields of the received frame."]
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W<30> {
        SA_W::new(self)
    }
    #[doc = "Bits 24:29 - Mask Byte Control These bits are mask control bits for comparison of each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: - Bit 29: Register 18\\[15:8\\]
- Bit 28: Register 18\\[7:0\\]
- Bit 27: Register 19\\[31:24\\]
- ... - Bit 24: Register 19\\[7:0\\]
You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
    #[inline(always)]
    pub fn mbc(&mut self) -> MBC_W<24> {
        MBC_W::new(self)
    }
    #[doc = "Bits 0:15 - MAC Address1 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the second 6-byte MAC address."]
    #[inline(always)]
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
#[doc = "MAC Address2 High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_addr_2_high](index.html) module"]
pub struct MAC_ADDR_2_HIGH_SPEC;
impl crate::RegisterSpec for MAC_ADDR_2_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_addr_2_high::R](R) reader structure"]
impl crate::Readable for MAC_ADDR_2_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_addr_2_high::W](W) writer structure"]
impl crate::Writable for MAC_ADDR_2_HIGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_ADDR_2_HIGH to value 0"]
impl crate::Resettable for MAC_ADDR_2_HIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
