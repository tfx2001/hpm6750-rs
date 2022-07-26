#[doc = "Register `TBSLOT` reader"]
pub struct R(crate::R<TBSLOT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBSLOT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBSLOT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBSLOT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBSLOT` writer"]
pub struct W(crate::W<TBSLOT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBSLOT_SPEC>;
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
impl From<crate::W<TBSLOT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBSLOT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBE` reader - set TB slot to “Empty” 1 - slot selected by TBPTR shall be marked as “empty” 0 - no actionTBE is automatically reset to 0 as soon as the slot is marked as empty and TSFF=0. If atransmission from this slot is active, then TBE stays set as long as either the transmission completes or after a transmission error or arbitration loss the transmissionis not active any more. If both TBF and TBE are set, then TBE wins"]
pub type TBE_R = crate::BitReader<bool>;
#[doc = "Field `TBE` writer - set TB slot to “Empty” 1 - slot selected by TBPTR shall be marked as “empty” 0 - no actionTBE is automatically reset to 0 as soon as the slot is marked as empty and TSFF=0. If atransmission from this slot is active, then TBE stays set as long as either the transmission completes or after a transmission error or arbitration loss the transmissionis not active any more. If both TBF and TBE are set, then TBE wins"]
pub type TBE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TBSLOT_SPEC, bool, O>;
#[doc = "Field `TBF` reader - set TB slot to “Filled” 1 - slot selected by TBPTR shall be marked as “filled” 0 - no actionTBF is automatically reset to 0 as soon as the slot is marked as filled and TSFF=1. If both TBF and TBE are set, then TBE wins."]
pub type TBF_R = crate::BitReader<bool>;
#[doc = "Field `TBF` writer - set TB slot to “Filled” 1 - slot selected by TBPTR shall be marked as “filled” 0 - no actionTBF is automatically reset to 0 as soon as the slot is marked as filled and TSFF=1. If both TBF and TBE are set, then TBE wins."]
pub type TBF_W<'a, const O: u8> = crate::BitWriter<'a, u8, TBSLOT_SPEC, bool, O>;
#[doc = "Field `TBPTR` reader - Pointer to a TB message slot. 0x00 - Pointer to the PTB others - Pointer to a slot in the STB The message slot pointed to by TBPTR is readable / writable using the TBUF registers. Write access is only possible if TSFF=0. Setting TBF to 1 marks the selected slot asfilled and setting TBE to 1 marks the selected slot as empty. TBSEL and TSNEXT are unused in TTCAN mode and have no meaning. TBPTR can only point to buffer slots, that exist in the hardware. Unusable bits ofTBPTR are fixed to 0. TBPTR is limited to the PTB and 63 STB slots. More slots cannot be used in TTCANmode.If TBPTR is too big and points to a slot that is not available, then TBF and TBE arereset automatically and no action takes place."]
pub type TBPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TBPTR` writer - Pointer to a TB message slot. 0x00 - Pointer to the PTB others - Pointer to a slot in the STB The message slot pointed to by TBPTR is readable / writable using the TBUF registers. Write access is only possible if TSFF=0. Setting TBF to 1 marks the selected slot asfilled and setting TBE to 1 marks the selected slot as empty. TBSEL and TSNEXT are unused in TTCAN mode and have no meaning. TBPTR can only point to buffer slots, that exist in the hardware. Unusable bits ofTBPTR are fixed to 0. TBPTR is limited to the PTB and 63 STB slots. More slots cannot be used in TTCANmode.If TBPTR is too big and points to a slot that is not available, then TBF and TBE arereset automatically and no action takes place."]
pub type TBPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TBSLOT_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 7 - set TB slot to “Empty” 1 - slot selected by TBPTR shall be marked as “empty” 0 - no actionTBE is automatically reset to 0 as soon as the slot is marked as empty and TSFF=0. If atransmission from this slot is active, then TBE stays set as long as either the transmission completes or after a transmission error or arbitration loss the transmissionis not active any more. If both TBF and TBE are set, then TBE wins"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - set TB slot to “Filled” 1 - slot selected by TBPTR shall be marked as “filled” 0 - no actionTBF is automatically reset to 0 as soon as the slot is marked as filled and TSFF=1. If both TBF and TBE are set, then TBE wins."]
    #[inline(always)]
    pub fn tbf(&self) -> TBF_R {
        TBF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 0:5 - Pointer to a TB message slot. 0x00 - Pointer to the PTB others - Pointer to a slot in the STB The message slot pointed to by TBPTR is readable / writable using the TBUF registers. Write access is only possible if TSFF=0. Setting TBF to 1 marks the selected slot asfilled and setting TBE to 1 marks the selected slot as empty. TBSEL and TSNEXT are unused in TTCAN mode and have no meaning. TBPTR can only point to buffer slots, that exist in the hardware. Unusable bits ofTBPTR are fixed to 0. TBPTR is limited to the PTB and 63 STB slots. More slots cannot be used in TTCANmode.If TBPTR is too big and points to a slot that is not available, then TBF and TBE arereset automatically and no action takes place."]
    #[inline(always)]
    pub fn tbptr(&self) -> TBPTR_R {
        TBPTR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - set TB slot to “Empty” 1 - slot selected by TBPTR shall be marked as “empty” 0 - no actionTBE is automatically reset to 0 as soon as the slot is marked as empty and TSFF=0. If atransmission from this slot is active, then TBE stays set as long as either the transmission completes or after a transmission error or arbitration loss the transmissionis not active any more. If both TBF and TBE are set, then TBE wins"]
    #[inline(always)]
    pub fn tbe(&mut self) -> TBE_W<7> {
        TBE_W::new(self)
    }
    #[doc = "Bit 6 - set TB slot to “Filled” 1 - slot selected by TBPTR shall be marked as “filled” 0 - no actionTBF is automatically reset to 0 as soon as the slot is marked as filled and TSFF=1. If both TBF and TBE are set, then TBE wins."]
    #[inline(always)]
    pub fn tbf(&mut self) -> TBF_W<6> {
        TBF_W::new(self)
    }
    #[doc = "Bits 0:5 - Pointer to a TB message slot. 0x00 - Pointer to the PTB others - Pointer to a slot in the STB The message slot pointed to by TBPTR is readable / writable using the TBUF registers. Write access is only possible if TSFF=0. Setting TBF to 1 marks the selected slot asfilled and setting TBE to 1 marks the selected slot as empty. TBSEL and TSNEXT are unused in TTCAN mode and have no meaning. TBPTR can only point to buffer slots, that exist in the hardware. Unusable bits ofTBPTR are fixed to 0. TBPTR is limited to the PTB and 63 STB slots. More slots cannot be used in TTCANmode.If TBPTR is too big and points to a slot that is not available, then TBF and TBE arereset automatically and no action takes place."]
    #[inline(always)]
    pub fn tbptr(&mut self) -> TBPTR_W<0> {
        TBPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TTCAN: TB Slot Pointer TBSLOT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbslot](index.html) module"]
pub struct TBSLOT_SPEC;
impl crate::RegisterSpec for TBSLOT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tbslot::R](R) reader structure"]
impl crate::Readable for TBSLOT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbslot::W](W) writer structure"]
impl crate::Writable for TBSLOT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBSLOT to value 0"]
impl crate::Resettable for TBSLOT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
