#[doc = "Register `SEQ_QUE_CFG1` reader"]
pub struct R(crate::R<SEQ_QUE_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ_QUE_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ_QUE_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ_QUE_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ_QUE_CFG1` writer"]
pub struct W(crate::W<SEQ_QUE_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ_QUE_CFG1_SPEC>;
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
impl From<crate::W<SEQ_QUE_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ_QUE_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQ_INT_EN` reader - interrupt enable for current conversion"]
pub type SEQ_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `SEQ_INT_EN` writer - interrupt enable for current conversion"]
pub type SEQ_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_QUE_CFG1_SPEC, bool, O>;
#[doc = "Field `CHAN_NUM_4_0` reader - channel number for current conversion"]
pub type CHAN_NUM_4_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHAN_NUM_4_0` writer - channel number for current conversion"]
pub type CHAN_NUM_4_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEQ_QUE_CFG1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 5 - interrupt enable for current conversion"]
    #[inline(always)]
    pub fn seq_int_en(&self) -> SEQ_INT_EN_R {
        SEQ_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 0:4 - channel number for current conversion"]
    #[inline(always)]
    pub fn chan_num_4_0(&self) -> CHAN_NUM_4_0_R {
        CHAN_NUM_4_0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - interrupt enable for current conversion"]
    #[inline(always)]
    pub fn seq_int_en(&mut self) -> SEQ_INT_EN_W<5> {
        SEQ_INT_EN_W::new(self)
    }
    #[doc = "Bits 0:4 - channel number for current conversion"]
    #[inline(always)]
    pub fn chan_num_4_0(&mut self) -> CHAN_NUM_4_0_W<0> {
        CHAN_NUM_4_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_que_cfg1](index.html) module"]
pub struct SEQ_QUE_CFG1_SPEC;
impl crate::RegisterSpec for SEQ_QUE_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq_que_cfg1::R](R) reader structure"]
impl crate::Readable for SEQ_QUE_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq_que_cfg1::W](W) writer structure"]
impl crate::Writable for SEQ_QUE_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQ_QUE_CFG1 to value 0"]
impl crate::Resettable for SEQ_QUE_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
