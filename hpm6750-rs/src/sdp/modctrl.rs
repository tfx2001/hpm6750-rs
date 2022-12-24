#[doc = "Register `MODCTRL` reader"]
pub struct R(crate::R<MODCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODCTRL` writer"]
pub struct W(crate::W<MODCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODCTRL_SPEC>;
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
impl From<crate::W<MODCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYSWP` reader - Decide whether the SDP byteswaps the Key (big-endian data). When all bits are set, the data is assumed to be in the big-endian format"]
pub type KEYSWP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEYSWP` writer - Decide whether the SDP byteswaps the Key (big-endian data). When all bits are set, the data is assumed to be in the big-endian format"]
pub type KEYSWP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DOUTSWP` reader - Decide whether the SDP byteswaps the output data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format"]
pub type DOUTSWP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOUTSWP` writer - Decide whether the SDP byteswaps the output data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format"]
pub type DOUTSWP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DINSWP` reader - Decide whether the SDP byteswaps the input data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format"]
pub type DINSWP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DINSWP` writer - Decide whether the SDP byteswaps the input data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format"]
pub type DINSWP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `HASOUT` reader - When hashing is enabled, this bit controls the input or output data of the AES engine is hashed. 0 INPUT HASH 1 OUTPUT HASH"]
pub type HASOUT_R = crate::BitReader<bool>;
#[doc = "Field `HASOUT` writer - When hashing is enabled, this bit controls the input or output data of the AES engine is hashed. 0 INPUT HASH 1 OUTPUT HASH"]
pub type HASOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODCTRL_SPEC, bool, O>;
#[doc = "Field `HASCHK` reader - HASH Check Enable Bit. 1x1, HASH check need, hash result will compare with the HASHRSLT 0-7 registers; 1x0, HASH check is not enabled, HASHRSLT0-7 store the HASH result. For SHA1, will use HASHRSLT0-3 words, and HASH 256 will use HASH0-7 words."]
pub type HASCHK_R = crate::BitReader<bool>;
#[doc = "Field `HASCHK` writer - HASH Check Enable Bit. 1x1, HASH check need, hash result will compare with the HASHRSLT 0-7 registers; 1x0, HASH check is not enabled, HASHRSLT0-7 store the HASH result. For SHA1, will use HASHRSLT0-3 words, and HASH 256 will use HASH0-7 words."]
pub type HASCHK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODCTRL_SPEC, bool, O>;
#[doc = "Field `CRCEN` reader - CRC enable. 1x1, CRC is enabled. 1x0, CRC is disabled."]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - CRC enable. 1x1, CRC is enabled. 1x0, CRC is disabled."]
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODCTRL_SPEC, bool, O>;
#[doc = "Field `HASALG` reader - HASH Algorithem selection. 0x0 SHA1 — 0x1 CRC32 — 0x2 SHA256 —"]
pub type HASALG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HASALG` writer - HASH Algorithem selection. 0x0 SHA1 — 0x1 CRC32 — 0x2 SHA256 —"]
pub type HASALG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AESDIR` reader - AES direction 1x1, AES Decryption 1x0, AES Encryption."]
pub type AESDIR_R = crate::BitReader<bool>;
#[doc = "Field `AESDIR` writer - AES direction 1x1, AES Decryption 1x0, AES Encryption."]
pub type AESDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODCTRL_SPEC, bool, O>;
#[doc = "Field `AESKS` reader - AES Key Selection. These regisgers are being used to select the AES key that stored in the 16x128 key ram of the SDP, or select the key from the OTP. Detail as following: 0x00: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 bit from this address and 128 bit key from next address as 256 bit AES key. 0x01: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. .... 0x0E: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 from this add and 128 from next add for the AES key. 0x0F: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. 0x20: kman_sk0\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk0\\[255:0\\]
as AES key. 0x21: kman_sk0\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x22: kman_sk1\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk1\\[255:0\\]
as AES key. 0x23: kman_sk1\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x24: kman_sk2\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk2\\[255:0\\]
as AES key. 0x25: kman_sk2\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x26: kman_sk3\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk3\\[255:0\\]
as AES key. 0x27: kman_sk3\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x30: exip0_key\\[127:0\\]
from OTP for AES128; AES256 will use exip0_key\\[255:0\\]
as AES key. 0x31: exip0_key\\[255:128\\]
from OTP for AES128; not valid for AES256. 0x32: exip1_key\\[127:0\\]
from OTP for AES128; AES256 will use exip1_key\\[255:0\\]
as AES key. 0x33: exip1_key\\[255:128\\]
from OTP for AES128; not valid for AES256. Other values, reserved."]
pub type AESKS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AESKS` writer - AES Key Selection. These regisgers are being used to select the AES key that stored in the 16x128 key ram of the SDP, or select the key from the OTP. Detail as following: 0x00: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 bit from this address and 128 bit key from next address as 256 bit AES key. 0x01: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. .... 0x0E: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 from this add and 128 from next add for the AES key. 0x0F: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. 0x20: kman_sk0\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk0\\[255:0\\]
as AES key. 0x21: kman_sk0\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x22: kman_sk1\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk1\\[255:0\\]
as AES key. 0x23: kman_sk1\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x24: kman_sk2\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk2\\[255:0\\]
as AES key. 0x25: kman_sk2\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x26: kman_sk3\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk3\\[255:0\\]
as AES key. 0x27: kman_sk3\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x30: exip0_key\\[127:0\\]
from OTP for AES128; AES256 will use exip0_key\\[255:0\\]
as AES key. 0x31: exip0_key\\[255:128\\]
from OTP for AES128; not valid for AES256. 0x32: exip1_key\\[127:0\\]
from OTP for AES128; AES256 will use exip1_key\\[255:0\\]
as AES key. 0x33: exip1_key\\[255:128\\]
from OTP for AES128; not valid for AES256. Other values, reserved."]
pub type AESKS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODCTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `AESMOD` reader - AES mode selection. 0x0 = ECB; 0x1 = CBC; Others, reserved."]
pub type AESMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AESMOD` writer - AES mode selection. 0x0 = ECB; 0x1 = CBC; Others, reserved."]
pub type AESMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AESALG` reader - AES algorithem selection. 0x0 = AES 128; 0x1 = AES 256; Others, reserved."]
pub type AESALG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AESALG` writer - AES algorithem selection. 0x0 = AES 128; 0x1 = AES 256; Others, reserved."]
pub type AESALG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODCTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Decide whether the SDP byteswaps the Key (big-endian data). When all bits are set, the data is assumed to be in the big-endian format"]
    #[inline(always)]
    pub fn keyswp(&self) -> KEYSWP_R {
        KEYSWP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Decide whether the SDP byteswaps the output data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format"]
    #[inline(always)]
    pub fn doutswp(&self) -> DOUTSWP_R {
        DOUTSWP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Decide whether the SDP byteswaps the input data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format"]
    #[inline(always)]
    pub fn dinswp(&self) -> DINSWP_R {
        DINSWP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 9 - When hashing is enabled, this bit controls the input or output data of the AES engine is hashed. 0 INPUT HASH 1 OUTPUT HASH"]
    #[inline(always)]
    pub fn hasout(&self) -> HASOUT_R {
        HASOUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HASH Check Enable Bit. 1x1, HASH check need, hash result will compare with the HASHRSLT 0-7 registers; 1x0, HASH check is not enabled, HASHRSLT0-7 store the HASH result. For SHA1, will use HASHRSLT0-3 words, and HASH 256 will use HASH0-7 words."]
    #[inline(always)]
    pub fn haschk(&self) -> HASCHK_R {
        HASCHK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CRC enable. 1x1, CRC is enabled. 1x0, CRC is disabled."]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - HASH Algorithem selection. 0x0 SHA1 — 0x1 CRC32 — 0x2 SHA256 —"]
    #[inline(always)]
    pub fn hasalg(&self) -> HASALG_R {
        HASALG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - AES direction 1x1, AES Decryption 1x0, AES Encryption."]
    #[inline(always)]
    pub fn aesdir(&self) -> AESDIR_R {
        AESDIR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:23 - AES Key Selection. These regisgers are being used to select the AES key that stored in the 16x128 key ram of the SDP, or select the key from the OTP. Detail as following: 0x00: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 bit from this address and 128 bit key from next address as 256 bit AES key. 0x01: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. .... 0x0E: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 from this add and 128 from next add for the AES key. 0x0F: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. 0x20: kman_sk0\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk0\\[255:0\\]
as AES key. 0x21: kman_sk0\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x22: kman_sk1\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk1\\[255:0\\]
as AES key. 0x23: kman_sk1\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x24: kman_sk2\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk2\\[255:0\\]
as AES key. 0x25: kman_sk2\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x26: kman_sk3\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk3\\[255:0\\]
as AES key. 0x27: kman_sk3\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x30: exip0_key\\[127:0\\]
from OTP for AES128; AES256 will use exip0_key\\[255:0\\]
as AES key. 0x31: exip0_key\\[255:128\\]
from OTP for AES128; not valid for AES256. 0x32: exip1_key\\[127:0\\]
from OTP for AES128; AES256 will use exip1_key\\[255:0\\]
as AES key. 0x33: exip1_key\\[255:128\\]
from OTP for AES128; not valid for AES256. Other values, reserved."]
    #[inline(always)]
    pub fn aesks(&self) -> AESKS_R {
        AESKS_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - AES mode selection. 0x0 = ECB; 0x1 = CBC; Others, reserved."]
    #[inline(always)]
    pub fn aesmod(&self) -> AESMOD_R {
        AESMOD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - AES algorithem selection. 0x0 = AES 128; 0x1 = AES 256; Others, reserved."]
    #[inline(always)]
    pub fn aesalg(&self) -> AESALG_R {
        AESALG_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Decide whether the SDP byteswaps the Key (big-endian data). When all bits are set, the data is assumed to be in the big-endian format"]
    #[inline(always)]
    #[must_use]
    pub fn keyswp(&mut self) -> KEYSWP_W<0> {
        KEYSWP_W::new(self)
    }
    #[doc = "Bits 2:3 - Decide whether the SDP byteswaps the output data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format"]
    #[inline(always)]
    #[must_use]
    pub fn doutswp(&mut self) -> DOUTSWP_W<2> {
        DOUTSWP_W::new(self)
    }
    #[doc = "Bits 4:5 - Decide whether the SDP byteswaps the input data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format"]
    #[inline(always)]
    #[must_use]
    pub fn dinswp(&mut self) -> DINSWP_W<4> {
        DINSWP_W::new(self)
    }
    #[doc = "Bit 9 - When hashing is enabled, this bit controls the input or output data of the AES engine is hashed. 0 INPUT HASH 1 OUTPUT HASH"]
    #[inline(always)]
    #[must_use]
    pub fn hasout(&mut self) -> HASOUT_W<9> {
        HASOUT_W::new(self)
    }
    #[doc = "Bit 10 - HASH Check Enable Bit. 1x1, HASH check need, hash result will compare with the HASHRSLT 0-7 registers; 1x0, HASH check is not enabled, HASHRSLT0-7 store the HASH result. For SHA1, will use HASHRSLT0-3 words, and HASH 256 will use HASH0-7 words."]
    #[inline(always)]
    #[must_use]
    pub fn haschk(&mut self) -> HASCHK_W<10> {
        HASCHK_W::new(self)
    }
    #[doc = "Bit 11 - CRC enable. 1x1, CRC is enabled. 1x0, CRC is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<11> {
        CRCEN_W::new(self)
    }
    #[doc = "Bits 12:15 - HASH Algorithem selection. 0x0 SHA1 — 0x1 CRC32 — 0x2 SHA256 —"]
    #[inline(always)]
    #[must_use]
    pub fn hasalg(&mut self) -> HASALG_W<12> {
        HASALG_W::new(self)
    }
    #[doc = "Bit 16 - AES direction 1x1, AES Decryption 1x0, AES Encryption."]
    #[inline(always)]
    #[must_use]
    pub fn aesdir(&mut self) -> AESDIR_W<16> {
        AESDIR_W::new(self)
    }
    #[doc = "Bits 18:23 - AES Key Selection. These regisgers are being used to select the AES key that stored in the 16x128 key ram of the SDP, or select the key from the OTP. Detail as following: 0x00: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 bit from this address and 128 bit key from next address as 256 bit AES key. 0x01: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. .... 0x0E: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 from this add and 128 from next add for the AES key. 0x0F: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. 0x20: kman_sk0\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk0\\[255:0\\]
as AES key. 0x21: kman_sk0\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x22: kman_sk1\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk1\\[255:0\\]
as AES key. 0x23: kman_sk1\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x24: kman_sk2\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk2\\[255:0\\]
as AES key. 0x25: kman_sk2\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x26: kman_sk3\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk3\\[255:0\\]
as AES key. 0x27: kman_sk3\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x30: exip0_key\\[127:0\\]
from OTP for AES128; AES256 will use exip0_key\\[255:0\\]
as AES key. 0x31: exip0_key\\[255:128\\]
from OTP for AES128; not valid for AES256. 0x32: exip1_key\\[127:0\\]
from OTP for AES128; AES256 will use exip1_key\\[255:0\\]
as AES key. 0x33: exip1_key\\[255:128\\]
from OTP for AES128; not valid for AES256. Other values, reserved."]
    #[inline(always)]
    #[must_use]
    pub fn aesks(&mut self) -> AESKS_W<18> {
        AESKS_W::new(self)
    }
    #[doc = "Bits 24:27 - AES mode selection. 0x0 = ECB; 0x1 = CBC; Others, reserved."]
    #[inline(always)]
    #[must_use]
    pub fn aesmod(&mut self) -> AESMOD_W<24> {
        AESMOD_W::new(self)
    }
    #[doc = "Bits 28:31 - AES algorithem selection. 0x0 = AES 128; 0x1 = AES 256; Others, reserved."]
    #[inline(always)]
    #[must_use]
    pub fn aesalg(&mut self) -> AESALG_W<28> {
        AESALG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mod control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modctrl](index.html) module"]
pub struct MODCTRL_SPEC;
impl crate::RegisterSpec for MODCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modctrl::R](R) reader structure"]
impl crate::Readable for MODCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modctrl::W](W) writer structure"]
impl crate::Writable for MODCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODCTRL to value 0"]
impl crate::Resettable for MODCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
