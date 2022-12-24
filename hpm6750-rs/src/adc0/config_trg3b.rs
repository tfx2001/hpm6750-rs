#[doc = "Register `CONFIG_TRG3B` reader"]
pub struct R(crate::R<CONFIG_TRG3B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_TRG3B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_TRG3B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_TRG3B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG_TRG3B` writer"]
pub struct W(crate::W<CONFIG_TRG3B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_TRG3B_SPEC>;
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
impl From<crate::W<CONFIG_TRG3B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_TRG3B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHAN0` reader - channel number for 1st conversion"]
pub type CHAN0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHAN0` writer - channel number for 1st conversion"]
pub type CHAN0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_TRG3B_SPEC, u8, u8, 5, O>;
#[doc = "Field `INTEN0` reader - interupt enable for 1st conversion"]
pub type INTEN0_R = crate::BitReader<bool>;
#[doc = "Field `INTEN0` writer - interupt enable for 1st conversion"]
pub type INTEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_TRG3B_SPEC, bool, O>;
#[doc = "Field `CHAN1` reader - channel number for 2nd conversion"]
pub type CHAN1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHAN1` writer - channel number for 2nd conversion"]
pub type CHAN1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_TRG3B_SPEC, u8, u8, 5, O>;
#[doc = "Field `INTEN1` reader - interupt enable for 2nd conversion"]
pub type INTEN1_R = crate::BitReader<bool>;
#[doc = "Field `INTEN1` writer - interupt enable for 2nd conversion"]
pub type INTEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_TRG3B_SPEC, bool, O>;
#[doc = "Field `CHAN2` reader - channel number for 3rd conversion"]
pub type CHAN2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHAN2` writer - channel number for 3rd conversion"]
pub type CHAN2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_TRG3B_SPEC, u8, u8, 5, O>;
#[doc = "Field `INTEN2` reader - interupt enable for 3rd conversion"]
pub type INTEN2_R = crate::BitReader<bool>;
#[doc = "Field `INTEN2` writer - interupt enable for 3rd conversion"]
pub type INTEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_TRG3B_SPEC, bool, O>;
#[doc = "Field `CHAN3` reader - channel number for 4th conversion"]
pub type CHAN3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHAN3` writer - channel number for 4th conversion"]
pub type CHAN3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_TRG3B_SPEC, u8, u8, 5, O>;
#[doc = "Field `INTEN3` reader - interupt enable for 4th conversion"]
pub type INTEN3_R = crate::BitReader<bool>;
#[doc = "Field `INTEN3` writer - interupt enable for 4th conversion"]
pub type INTEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_TRG3B_SPEC, bool, O>;
#[doc = "Field `TRIG_LEN` writer - length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
pub type TRIG_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_TRG3B_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:4 - channel number for 1st conversion"]
    #[inline(always)]
    pub fn chan0(&self) -> CHAN0_R {
        CHAN0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - interupt enable for 1st conversion"]
    #[inline(always)]
    pub fn inten0(&self) -> INTEN0_R {
        INTEN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12 - channel number for 2nd conversion"]
    #[inline(always)]
    pub fn chan1(&self) -> CHAN1_R {
        CHAN1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - interupt enable for 2nd conversion"]
    #[inline(always)]
    pub fn inten1(&self) -> INTEN1_R {
        INTEN1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - channel number for 3rd conversion"]
    #[inline(always)]
    pub fn chan2(&self) -> CHAN2_R {
        CHAN2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - interupt enable for 3rd conversion"]
    #[inline(always)]
    pub fn inten2(&self) -> INTEN2_R {
        INTEN2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:28 - channel number for 4th conversion"]
    #[inline(always)]
    pub fn chan3(&self) -> CHAN3_R {
        CHAN3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - interupt enable for 4th conversion"]
    #[inline(always)]
    pub fn inten3(&self) -> INTEN3_R {
        INTEN3_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - channel number for 1st conversion"]
    #[inline(always)]
    #[must_use]
    pub fn chan0(&mut self) -> CHAN0_W<0> {
        CHAN0_W::new(self)
    }
    #[doc = "Bit 5 - interupt enable for 1st conversion"]
    #[inline(always)]
    #[must_use]
    pub fn inten0(&mut self) -> INTEN0_W<5> {
        INTEN0_W::new(self)
    }
    #[doc = "Bits 8:12 - channel number for 2nd conversion"]
    #[inline(always)]
    #[must_use]
    pub fn chan1(&mut self) -> CHAN1_W<8> {
        CHAN1_W::new(self)
    }
    #[doc = "Bit 13 - interupt enable for 2nd conversion"]
    #[inline(always)]
    #[must_use]
    pub fn inten1(&mut self) -> INTEN1_W<13> {
        INTEN1_W::new(self)
    }
    #[doc = "Bits 16:20 - channel number for 3rd conversion"]
    #[inline(always)]
    #[must_use]
    pub fn chan2(&mut self) -> CHAN2_W<16> {
        CHAN2_W::new(self)
    }
    #[doc = "Bit 21 - interupt enable for 3rd conversion"]
    #[inline(always)]
    #[must_use]
    pub fn inten2(&mut self) -> INTEN2_W<21> {
        INTEN2_W::new(self)
    }
    #[doc = "Bits 24:28 - channel number for 4th conversion"]
    #[inline(always)]
    #[must_use]
    pub fn chan3(&mut self) -> CHAN3_W<24> {
        CHAN3_W::new(self)
    }
    #[doc = "Bit 29 - interupt enable for 4th conversion"]
    #[inline(always)]
    #[must_use]
    pub fn inten3(&mut self) -> INTEN3_W<29> {
        INTEN3_W::new(self)
    }
    #[doc = "Bits 30:31 - length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn trig_len(&mut self) -> TRIG_LEN_W<30> {
        TRIG_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_trg3b](index.html) module"]
pub struct CONFIG_TRG3B_SPEC;
impl crate::RegisterSpec for CONFIG_TRG3B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config_trg3b::R](R) reader structure"]
impl crate::Readable for CONFIG_TRG3B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config_trg3b::W](W) writer structure"]
impl crate::Writable for CONFIG_TRG3B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG_TRG3B to value 0"]
impl crate::Resettable for CONFIG_TRG3B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
