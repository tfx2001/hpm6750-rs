#[doc = "Register `VLAN_TAG` reader"]
pub struct R(crate::R<VLAN_TAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLAN_TAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLAN_TAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLAN_TAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VLAN_TAG` writer"]
pub struct W(crate::W<VLAN_TAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLAN_TAG_SPEC>;
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
impl From<crate::W<VLAN_TAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLAN_TAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTHM` reader - VLAN Tag Hash Table Match Enable When set, the most significant four bits of the VLAN tag’s CRC are used to index the content of Register 354 (VLAN Hash Table Register). A value of 1 in the VLAN Hash Table register, corresponding to the index, indicates that the frame matched the VLAN hash table. When Bit 16 (ETV) is set, the CRC of the 12-bit VLAN Identifier (VID) is used for comparison whereas when ETV is reset, the CRC of the 16-bit VLAN tag is used for comparison. When reset, the VLAN Hash Match operation is not performed."]
pub type VTHM_R = crate::BitReader<bool>;
#[doc = "Field `VTHM` writer - VLAN Tag Hash Table Match Enable When set, the most significant four bits of the VLAN tag’s CRC are used to index the content of Register 354 (VLAN Hash Table Register). A value of 1 in the VLAN Hash Table register, corresponding to the index, indicates that the frame matched the VLAN hash table. When Bit 16 (ETV) is set, the CRC of the 12-bit VLAN Identifier (VID) is used for comparison whereas when ETV is reset, the CRC of the 16-bit VLAN tag is used for comparison. When reset, the VLAN Hash Match operation is not performed."]
pub type VTHM_W<'a, const O: u8> = crate::BitWriter<'a, u32, VLAN_TAG_SPEC, bool, O>;
#[doc = "Field `ESVL` reader - Enable S-VLAN When this bit is set, the MAC transmitter and receiver also consider the S-VLAN (Type = 0x88A8) frames as valid VLAN tagged frames."]
pub type ESVL_R = crate::BitReader<bool>;
#[doc = "Field `ESVL` writer - Enable S-VLAN When this bit is set, the MAC transmitter and receiver also consider the S-VLAN (Type = 0x88A8) frames as valid VLAN tagged frames."]
pub type ESVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, VLAN_TAG_SPEC, bool, O>;
#[doc = "Field `VTIM` reader - VLAN Tag Inverse Match Enable When set, this bit enables the VLAN Tag inverse matching. The frames that do not have matching VLAN Tag are marked as matched. When reset, this bit enables the VLAN Tag perfect matching. The frames with matched VLAN Tag are marked as matched."]
pub type VTIM_R = crate::BitReader<bool>;
#[doc = "Field `VTIM` writer - VLAN Tag Inverse Match Enable When set, this bit enables the VLAN Tag inverse matching. The frames that do not have matching VLAN Tag are marked as matched. When reset, this bit enables the VLAN Tag perfect matching. The frames with matched VLAN Tag are marked as matched."]
pub type VTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, VLAN_TAG_SPEC, bool, O>;
#[doc = "Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison When this bit is set, a 12-bit VLAN identifier is used for comparing and filtering instead of the complete 16-bit VLAN tag. Bits \\[11:0\\]
of VLAN tag are compared with the corresponding field in the received VLAN-tagged frame. Similarly, when enabled, only 12 bits of the VLAN tag in the received frame are used for hash-based VLAN filtering. When this bit is reset, all 16 bits of the 15th and 16th bytes of the received VLAN frame are used for comparison and VLAN hash filtering."]
pub type ETV_R = crate::BitReader<bool>;
#[doc = "Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison When this bit is set, a 12-bit VLAN identifier is used for comparing and filtering instead of the complete 16-bit VLAN tag. Bits \\[11:0\\]
of VLAN tag are compared with the corresponding field in the received VLAN-tagged frame. Similarly, when enabled, only 12 bits of the VLAN tag in the received frame are used for hash-based VLAN filtering. When this bit is reset, all 16 bits of the 15th and 16th bytes of the received VLAN frame are used for comparison and VLAN hash filtering."]
pub type ETV_W<'a, const O: u8> = crate::BitWriter<'a, u32, VLAN_TAG_SPEC, bool, O>;
#[doc = "Field `VL` reader - VLAN Tag Identifier for Receive Frames This field contains the 802.1Q VLAN tag to identify the VLAN frames and is compared to the 15th and 16th bytes of the frames being received for VLAN frames. The following list describes the bits of this field: - Bits \\[15:13\\]: User Priority - Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) - Bits\\[11:0\\]: VLAN tag’s VLAN Identifier (VID) field When the ETV bit is set, only the VID (Bits\\[11:0\\]) is used for comparison. If VL (VL\\[11:0\\]
if ETV is set) is all zeros, the MAC does not check the fifteenth and 16th bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 or 0x88a8 as VLAN frames."]
pub type VL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VL` writer - VLAN Tag Identifier for Receive Frames This field contains the 802.1Q VLAN tag to identify the VLAN frames and is compared to the 15th and 16th bytes of the frames being received for VLAN frames. The following list describes the bits of this field: - Bits \\[15:13\\]: User Priority - Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) - Bits\\[11:0\\]: VLAN tag’s VLAN Identifier (VID) field When the ETV bit is set, only the VID (Bits\\[11:0\\]) is used for comparison. If VL (VL\\[11:0\\]
if ETV is set) is all zeros, the MAC does not check the fifteenth and 16th bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 or 0x88a8 as VLAN frames."]
pub type VL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VLAN_TAG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 19 - VLAN Tag Hash Table Match Enable When set, the most significant four bits of the VLAN tag’s CRC are used to index the content of Register 354 (VLAN Hash Table Register). A value of 1 in the VLAN Hash Table register, corresponding to the index, indicates that the frame matched the VLAN hash table. When Bit 16 (ETV) is set, the CRC of the 12-bit VLAN Identifier (VID) is used for comparison whereas when ETV is reset, the CRC of the 16-bit VLAN tag is used for comparison. When reset, the VLAN Hash Match operation is not performed."]
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable S-VLAN When this bit is set, the MAC transmitter and receiver also consider the S-VLAN (Type = 0x88A8) frames as valid VLAN tagged frames."]
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable When set, this bit enables the VLAN Tag inverse matching. The frames that do not have matching VLAN Tag are marked as matched. When reset, this bit enables the VLAN Tag perfect matching. The frames with matched VLAN Tag are marked as matched."]
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison When this bit is set, a 12-bit VLAN identifier is used for comparing and filtering instead of the complete 16-bit VLAN tag. Bits \\[11:0\\]
of VLAN tag are compared with the corresponding field in the received VLAN-tagged frame. Similarly, when enabled, only 12 bits of the VLAN tag in the received frame are used for hash-based VLAN filtering. When this bit is reset, all 16 bits of the 15th and 16th bytes of the received VLAN frame are used for comparison and VLAN hash filtering."]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames This field contains the 802.1Q VLAN tag to identify the VLAN frames and is compared to the 15th and 16th bytes of the frames being received for VLAN frames. The following list describes the bits of this field: - Bits \\[15:13\\]: User Priority - Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) - Bits\\[11:0\\]: VLAN tag’s VLAN Identifier (VID) field When the ETV bit is set, only the VID (Bits\\[11:0\\]) is used for comparison. If VL (VL\\[11:0\\]
if ETV is set) is all zeros, the MAC does not check the fifteenth and 16th bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 or 0x88a8 as VLAN frames."]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 19 - VLAN Tag Hash Table Match Enable When set, the most significant four bits of the VLAN tag’s CRC are used to index the content of Register 354 (VLAN Hash Table Register). A value of 1 in the VLAN Hash Table register, corresponding to the index, indicates that the frame matched the VLAN hash table. When Bit 16 (ETV) is set, the CRC of the 12-bit VLAN Identifier (VID) is used for comparison whereas when ETV is reset, the CRC of the 16-bit VLAN tag is used for comparison. When reset, the VLAN Hash Match operation is not performed."]
    #[inline(always)]
    pub fn vthm(&mut self) -> VTHM_W<19> {
        VTHM_W::new(self)
    }
    #[doc = "Bit 18 - Enable S-VLAN When this bit is set, the MAC transmitter and receiver also consider the S-VLAN (Type = 0x88A8) frames as valid VLAN tagged frames."]
    #[inline(always)]
    pub fn esvl(&mut self) -> ESVL_W<18> {
        ESVL_W::new(self)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable When set, this bit enables the VLAN Tag inverse matching. The frames that do not have matching VLAN Tag are marked as matched. When reset, this bit enables the VLAN Tag perfect matching. The frames with matched VLAN Tag are marked as matched."]
    #[inline(always)]
    pub fn vtim(&mut self) -> VTIM_W<17> {
        VTIM_W::new(self)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison When this bit is set, a 12-bit VLAN identifier is used for comparing and filtering instead of the complete 16-bit VLAN tag. Bits \\[11:0\\]
of VLAN tag are compared with the corresponding field in the received VLAN-tagged frame. Similarly, when enabled, only 12 bits of the VLAN tag in the received frame are used for hash-based VLAN filtering. When this bit is reset, all 16 bits of the 15th and 16th bytes of the received VLAN frame are used for comparison and VLAN hash filtering."]
    #[inline(always)]
    pub fn etv(&mut self) -> ETV_W<16> {
        ETV_W::new(self)
    }
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames This field contains the 802.1Q VLAN tag to identify the VLAN frames and is compared to the 15th and 16th bytes of the frames being received for VLAN frames. The following list describes the bits of this field: - Bits \\[15:13\\]: User Priority - Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) - Bits\\[11:0\\]: VLAN tag’s VLAN Identifier (VID) field When the ETV bit is set, only the VID (Bits\\[11:0\\]) is used for comparison. If VL (VL\\[11:0\\]
if ETV is set) is all zeros, the MAC does not check the fifteenth and 16th bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 or 0x88a8 as VLAN frames."]
    #[inline(always)]
    pub fn vl(&mut self) -> VL_W<0> {
        VL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VLAN Tag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlan_tag](index.html) module"]
pub struct VLAN_TAG_SPEC;
impl crate::RegisterSpec for VLAN_TAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vlan_tag::R](R) reader structure"]
impl crate::Readable for VLAN_TAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vlan_tag::W](W) writer structure"]
impl crate::Writable for VLAN_TAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VLAN_TAG to value 0"]
impl crate::Resettable for VLAN_TAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
