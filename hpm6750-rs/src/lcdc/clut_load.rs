#[doc = "Register `CLUT_LOAD` reader"]
pub struct R(crate::R<CLUT_LOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLUT_LOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLUT_LOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLUT_LOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLUT_LOAD` writer"]
pub struct W(crate::W<CLUT_LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLUT_LOAD_SPEC>;
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
impl From<crate::W<CLUT_LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLUT_LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL_NUM` reader - Selected CLUT Number The SEL_CLUT_NUM is used to select which plane's CLUT need to be updated. The hardware can only backup one CLUT setting and load, so the SEL_CLUT_NUM can't be changed when CLUT_LOAD\\[UPDATE_EN\\]
is 1. . 3'h0 - PLANE 0 . 3'h1 - PLANE 1 . ------ . 3'h7 - PLANE 7 CLUT 8 can be modified via APB even when display is on. Currently CLUT for plane 0..7 cannot be modified via APB when display is on. Can only be updated via CLUT_LOAD\\[UPDATE_EN\\]
bit."]
pub type SEL_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL_NUM` writer - Selected CLUT Number The SEL_CLUT_NUM is used to select which plane's CLUT need to be updated. The hardware can only backup one CLUT setting and load, so the SEL_CLUT_NUM can't be changed when CLUT_LOAD\\[UPDATE_EN\\]
is 1. . 3'h0 - PLANE 0 . 3'h1 - PLANE 1 . ------ . 3'h7 - PLANE 7 CLUT 8 can be modified via APB even when display is on. Currently CLUT for plane 0..7 cannot be modified via APB when display is on. Can only be updated via CLUT_LOAD\\[UPDATE_EN\\]
bit."]
pub type SEL_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLUT_LOAD_SPEC, u8, u8, 3, O>;
#[doc = "Field `UPDATE_EN` reader - CLUT Update Enable The bit is written to 1 when software want to update the Color Look Up Tables during display. If set to 1, software update selected CLUT due to SEL_CLUT_NUM setting, the table will be copied from CLUT8 during vertical blanking period after SHADOW_LOAD_EN is set to 1. If set to 0, software can update CLUT8 directly according to the CLUT memory map. Hardware will automatically clear this bit when selected CLUT is updated according to SEL_CLUT_NUM."]
pub type UPDATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `UPDATE_EN` writer - CLUT Update Enable The bit is written to 1 when software want to update the Color Look Up Tables during display. If set to 1, software update selected CLUT due to SEL_CLUT_NUM setting, the table will be copied from CLUT8 during vertical blanking period after SHADOW_LOAD_EN is set to 1. If set to 0, software can update CLUT8 directly according to the CLUT memory map. Hardware will automatically clear this bit when selected CLUT is updated according to SEL_CLUT_NUM."]
pub type UPDATE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLUT_LOAD_SPEC, bool, O>;
impl R {
    #[doc = "Bits 4:6 - Selected CLUT Number The SEL_CLUT_NUM is used to select which plane's CLUT need to be updated. The hardware can only backup one CLUT setting and load, so the SEL_CLUT_NUM can't be changed when CLUT_LOAD\\[UPDATE_EN\\]
is 1. . 3'h0 - PLANE 0 . 3'h1 - PLANE 1 . ------ . 3'h7 - PLANE 7 CLUT 8 can be modified via APB even when display is on. Currently CLUT for plane 0..7 cannot be modified via APB when display is on. Can only be updated via CLUT_LOAD\\[UPDATE_EN\\]
bit."]
    #[inline(always)]
    pub fn sel_num(&self) -> SEL_NUM_R {
        SEL_NUM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 0 - CLUT Update Enable The bit is written to 1 when software want to update the Color Look Up Tables during display. If set to 1, software update selected CLUT due to SEL_CLUT_NUM setting, the table will be copied from CLUT8 during vertical blanking period after SHADOW_LOAD_EN is set to 1. If set to 0, software can update CLUT8 directly according to the CLUT memory map. Hardware will automatically clear this bit when selected CLUT is updated according to SEL_CLUT_NUM."]
    #[inline(always)]
    pub fn update_en(&self) -> UPDATE_EN_R {
        UPDATE_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - Selected CLUT Number The SEL_CLUT_NUM is used to select which plane's CLUT need to be updated. The hardware can only backup one CLUT setting and load, so the SEL_CLUT_NUM can't be changed when CLUT_LOAD\\[UPDATE_EN\\]
is 1. . 3'h0 - PLANE 0 . 3'h1 - PLANE 1 . ------ . 3'h7 - PLANE 7 CLUT 8 can be modified via APB even when display is on. Currently CLUT for plane 0..7 cannot be modified via APB when display is on. Can only be updated via CLUT_LOAD\\[UPDATE_EN\\]
bit."]
    #[inline(always)]
    pub fn sel_num(&mut self) -> SEL_NUM_W<4> {
        SEL_NUM_W::new(self)
    }
    #[doc = "Bit 0 - CLUT Update Enable The bit is written to 1 when software want to update the Color Look Up Tables during display. If set to 1, software update selected CLUT due to SEL_CLUT_NUM setting, the table will be copied from CLUT8 during vertical blanking period after SHADOW_LOAD_EN is set to 1. If set to 0, software can update CLUT8 directly according to the CLUT memory map. Hardware will automatically clear this bit when selected CLUT is updated according to SEL_CLUT_NUM."]
    #[inline(always)]
    pub fn update_en(&mut self) -> UPDATE_EN_W<0> {
        UPDATE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clut Load Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clut_load](index.html) module"]
pub struct CLUT_LOAD_SPEC;
impl crate::RegisterSpec for CLUT_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clut_load::R](R) reader structure"]
impl crate::Readable for CLUT_LOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clut_load::W](W) writer structure"]
impl crate::Writable for CLUT_LOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLUT_LOAD to value 0"]
impl crate::Resettable for CLUT_LOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
