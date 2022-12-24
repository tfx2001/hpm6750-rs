#[doc = "Register `KEYADDR` reader"]
pub struct R(crate::R<KEYADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYADDR` writer"]
pub struct W(crate::W<KEYADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYADDR_SPEC>;
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
impl From<crate::W<KEYADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBWRD` reader - Key subword pointer. The valid indices are 0-3. After each write to the key data register, this field increments; To write a key, the software must first write the desired key index/subword to this register."]
pub type SUBWRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUBWRD` writer - Key subword pointer. The valid indices are 0-3. After each write to the key data register, this field increments; To write a key, the software must first write the desired key index/subword to this register."]
pub type SUBWRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYADDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `INDEX` reader - To write a key to the SDP KEY RAM, the software must first write the desired key index/subword to this register. Key index pointer. The valid indices are 0-\\[number_keys\\]. In the SDP, there is a 16x128 key ram can store 16 AES128 keys or 8 AES 256 Keys; this index is for addressing the 16 128-bit key addresses."]
pub type INDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INDEX` writer - To write a key to the SDP KEY RAM, the software must first write the desired key index/subword to this register. Key index pointer. The valid indices are 0-\\[number_keys\\]. In the SDP, there is a 16x128 key ram can store 16 AES128 keys or 8 AES 256 Keys; this index is for addressing the 16 128-bit key addresses."]
pub type INDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYADDR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1 - Key subword pointer. The valid indices are 0-3. After each write to the key data register, this field increments; To write a key, the software must first write the desired key index/subword to this register."]
    #[inline(always)]
    pub fn subwrd(&self) -> SUBWRD_R {
        SUBWRD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:23 - To write a key to the SDP KEY RAM, the software must first write the desired key index/subword to this register. Key index pointer. The valid indices are 0-\\[number_keys\\]. In the SDP, there is a 16x128 key ram can store 16 AES128 keys or 8 AES 256 Keys; this index is for addressing the 16 128-bit key addresses."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Key subword pointer. The valid indices are 0-3. After each write to the key data register, this field increments; To write a key, the software must first write the desired key index/subword to this register."]
    #[inline(always)]
    #[must_use]
    pub fn subwrd(&mut self) -> SUBWRD_W<0> {
        SUBWRD_W::new(self)
    }
    #[doc = "Bits 16:23 - To write a key to the SDP KEY RAM, the software must first write the desired key index/subword to this register. Key index pointer. The valid indices are 0-\\[number_keys\\]. In the SDP, there is a 16x128 key ram can store 16 AES128 keys or 8 AES 256 Keys; this index is for addressing the 16 128-bit key addresses."]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> INDEX_W<16> {
        INDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyaddr](index.html) module"]
pub struct KEYADDR_SPEC;
impl crate::RegisterSpec for KEYADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyaddr::R](R) reader structure"]
impl crate::Readable for KEYADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyaddr::W](W) writer structure"]
impl crate::Writable for KEYADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYADDR to value 0x40"]
impl crate::Resettable for KEYADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
