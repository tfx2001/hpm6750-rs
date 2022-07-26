#[doc = "Register `VLAN_TAG_INC_RPL` reader"]
pub struct R(crate::R<VLAN_TAG_INC_RPL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLAN_TAG_INC_RPL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLAN_TAG_INC_RPL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLAN_TAG_INC_RPL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VLAN_TAG_INC_RPL` writer"]
pub struct W(crate::W<VLAN_TAG_INC_RPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLAN_TAG_INC_RPL_SPEC>;
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
impl From<crate::W<VLAN_TAG_INC_RPL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLAN_TAG_INC_RPL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSVL` reader - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted frames. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the transmitted frames."]
pub type CSVL_R = crate::BitReader<bool>;
#[doc = "Field `CSVL` writer - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted frames. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the transmitted frames."]
pub type CSVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, VLAN_TAG_INC_RPL_SPEC, bool, O>;
#[doc = "Field `VLP` reader - VLAN Priority Control When this bit is set, the control Bits \\[17:16\\]
are used for VLAN deletion, insertion, or replacement. When this bit is reset, the mti_vlan_ctrl_i control input is used, and Bits \\[17:16\\]
are ignored."]
pub type VLP_R = crate::BitReader<bool>;
#[doc = "Field `VLP` writer - VLAN Priority Control When this bit is set, the control Bits \\[17:16\\]
are used for VLAN deletion, insertion, or replacement. When this bit is reset, the mti_vlan_ctrl_i control input is used, and Bits \\[17:16\\]
are ignored."]
pub type VLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, VLAN_TAG_INC_RPL_SPEC, bool, O>;
#[doc = "Field `VLC` reader - VLAN Tag Control in Transmit Frames - 2’b00: No VLAN tag deletion, insertion, or replacement - 2’b01: VLAN tag deletion The MAC removes the VLAN type (bytes 13 and 14) and VLAN tag (bytes 15 and 16) of all transmitted frames with VLAN tags. - 2’b10: VLAN tag insertion The MAC inserts VLT in bytes 15 and 16 of the frame after inserting the Type value (0x8100/0x88a8) in bytes 13 and 14. This operation is performed on all transmitted frames, irrespective of whether they already have a VLAN tag. - 2’b11: VLAN tag replacement The MAC replaces VLT in bytes 15 and 16 of all VLAN-type transmitted frames (Bytes 13 and 14 are 0x8100/0x88a8). Note: Changes to this field take effect only on the start of a frame. If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value."]
pub type VLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VLC` writer - VLAN Tag Control in Transmit Frames - 2’b00: No VLAN tag deletion, insertion, or replacement - 2’b01: VLAN tag deletion The MAC removes the VLAN type (bytes 13 and 14) and VLAN tag (bytes 15 and 16) of all transmitted frames with VLAN tags. - 2’b10: VLAN tag insertion The MAC inserts VLT in bytes 15 and 16 of the frame after inserting the Type value (0x8100/0x88a8) in bytes 13 and 14. This operation is performed on all transmitted frames, irrespective of whether they already have a VLAN tag. - 2’b11: VLAN tag replacement The MAC replaces VLT in bytes 15 and 16 of all VLAN-type transmitted frames (Bytes 13 and 14 are 0x8100/0x88a8). Note: Changes to this field take effect only on the start of a frame. If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value."]
pub type VLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VLAN_TAG_INC_RPL_SPEC, u8, u8, 2, O>;
#[doc = "Field `VLT` reader - VLAN Tag for Transmit Frames This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. Bits\\[15:13\\]
are the User Priority, Bit 12 is the CFI/DEI, and Bits\\[11:0\\]
are the VLAN tag’s VID field."]
pub type VLT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VLT` writer - VLAN Tag for Transmit Frames This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. Bits\\[15:13\\]
are the User Priority, Bit 12 is the CFI/DEI, and Bits\\[11:0\\]
are the VLAN tag’s VID field."]
pub type VLT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VLAN_TAG_INC_RPL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 19 - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted frames. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the transmitted frames."]
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - VLAN Priority Control When this bit is set, the control Bits \\[17:16\\]
are used for VLAN deletion, insertion, or replacement. When this bit is reset, the mti_vlan_ctrl_i control input is used, and Bits \\[17:16\\]
are ignored."]
    #[inline(always)]
    pub fn vlp(&self) -> VLP_R {
        VLP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames - 2’b00: No VLAN tag deletion, insertion, or replacement - 2’b01: VLAN tag deletion The MAC removes the VLAN type (bytes 13 and 14) and VLAN tag (bytes 15 and 16) of all transmitted frames with VLAN tags. - 2’b10: VLAN tag insertion The MAC inserts VLT in bytes 15 and 16 of the frame after inserting the Type value (0x8100/0x88a8) in bytes 13 and 14. This operation is performed on all transmitted frames, irrespective of whether they already have a VLAN tag. - 2’b11: VLAN tag replacement The MAC replaces VLT in bytes 15 and 16 of all VLAN-type transmitted frames (Bytes 13 and 14 are 0x8100/0x88a8). Note: Changes to this field take effect only on the start of a frame. If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value."]
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. Bits\\[15:13\\]
are the User Priority, Bit 12 is the CFI/DEI, and Bits\\[11:0\\]
are the VLAN tag’s VID field."]
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 19 - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted frames. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the transmitted frames."]
    #[inline(always)]
    pub fn csvl(&mut self) -> CSVL_W<19> {
        CSVL_W::new(self)
    }
    #[doc = "Bit 18 - VLAN Priority Control When this bit is set, the control Bits \\[17:16\\]
are used for VLAN deletion, insertion, or replacement. When this bit is reset, the mti_vlan_ctrl_i control input is used, and Bits \\[17:16\\]
are ignored."]
    #[inline(always)]
    pub fn vlp(&mut self) -> VLP_W<18> {
        VLP_W::new(self)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames - 2’b00: No VLAN tag deletion, insertion, or replacement - 2’b01: VLAN tag deletion The MAC removes the VLAN type (bytes 13 and 14) and VLAN tag (bytes 15 and 16) of all transmitted frames with VLAN tags. - 2’b10: VLAN tag insertion The MAC inserts VLT in bytes 15 and 16 of the frame after inserting the Type value (0x8100/0x88a8) in bytes 13 and 14. This operation is performed on all transmitted frames, irrespective of whether they already have a VLAN tag. - 2’b11: VLAN tag replacement The MAC replaces VLT in bytes 15 and 16 of all VLAN-type transmitted frames (Bytes 13 and 14 are 0x8100/0x88a8). Note: Changes to this field take effect only on the start of a frame. If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value."]
    #[inline(always)]
    pub fn vlc(&mut self) -> VLC_W<16> {
        VLC_W::new(self)
    }
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. Bits\\[15:13\\]
are the User Priority, Bit 12 is the CFI/DEI, and Bits\\[11:0\\]
are the VLAN tag’s VID field."]
    #[inline(always)]
    pub fn vlt(&mut self) -> VLT_W<0> {
        VLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VLAN Tag Inclusion or Replacement Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlan_tag_inc_rpl](index.html) module"]
pub struct VLAN_TAG_INC_RPL_SPEC;
impl crate::RegisterSpec for VLAN_TAG_INC_RPL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vlan_tag_inc_rpl::R](R) reader structure"]
impl crate::Readable for VLAN_TAG_INC_RPL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vlan_tag_inc_rpl::W](W) writer structure"]
impl crate::Writable for VLAN_TAG_INC_RPL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VLAN_TAG_INC_RPL to value 0"]
impl crate::Resettable for VLAN_TAG_INC_RPL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
