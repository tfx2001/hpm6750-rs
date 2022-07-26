#[doc = "Register `PAD_PB08_PAD_CTL` reader"]
pub struct R(crate::R<PAD_PB08_PAD_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_PB08_PAD_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_PB08_PAD_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_PB08_PAD_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_PB08_PAD_CTL` writer"]
pub struct W(crate::W<PAD_PB08_PAD_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_PB08_PAD_CTL_SPEC>;
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
impl From<crate::W<PAD_PB08_PAD_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_PB08_PAD_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MS` reader - pin voltage select, only available in high-speed IO 0: 3.3V 1: 1.8V"]
pub type MS_R = crate::BitReader<bool>;
#[doc = "Field `MS` writer - pin voltage select, only available in high-speed IO 0: 3.3V 1: 1.8V"]
pub type MS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_PB08_PAD_CTL_SPEC, bool, O>;
#[doc = "Field `OD` reader - open drain 0: open drain disable 1: open drain enable"]
pub type OD_R = crate::BitReader<bool>;
#[doc = "Field `OD` writer - open drain 0: open drain disable 1: open drain enable"]
pub type OD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_PB08_PAD_CTL_SPEC, bool, O>;
#[doc = "Field `SMT` reader - schmitt trigger enable, only avaiable in high-speed IO 0: disable 1: enable"]
pub type SMT_R = crate::BitReader<bool>;
#[doc = "Field `SMT` writer - schmitt trigger enable, only avaiable in high-speed IO 0: disable 1: enable"]
pub type SMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_PB08_PAD_CTL_SPEC, bool, O>;
#[doc = "Field `PS` reader - pull select 0: pull down 1: pull up"]
pub type PS_R = crate::BitReader<bool>;
#[doc = "Field `PS` writer - pull select 0: pull down 1: pull up"]
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_PB08_PAD_CTL_SPEC, bool, O>;
#[doc = "Field `PE` reader - pull enable 0: pull disable 1: pull enable"]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `PE` writer - pull enable 0: pull disable 1: pull enable"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_PB08_PAD_CTL_SPEC, bool, O>;
#[doc = "Field `DS` reader - drive strength for high-speed IO 3.3V: 000: 85.61Ohm 001: 61.2 Ohm 010: 42.88Ohm 011: 35.76Ohm 111: 30.67Ohm for high-speed IO 1.8V: 000: 84.07Ohm 001: 60.14Ohm 010: 42.15Ohm 011: 35.19Ohm 111: 30.2 Ohm for general IO: 00: 4mA 01: 8mA 11: 12mA"]
pub type DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS` writer - drive strength for high-speed IO 3.3V: 000: 85.61Ohm 001: 61.2 Ohm 010: 42.88Ohm 011: 35.76Ohm 111: 30.67Ohm for high-speed IO 1.8V: 000: 84.07Ohm 001: 60.14Ohm 010: 42.15Ohm 011: 35.19Ohm 111: 30.2 Ohm for general IO: 00: 4mA 01: 8mA 11: 12mA"]
pub type DS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_PB08_PAD_CTL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 14 - pin voltage select, only available in high-speed IO 0: 3.3V 1: 1.8V"]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - open drain 0: open drain disable 1: open drain enable"]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - schmitt trigger enable, only avaiable in high-speed IO 0: disable 1: enable"]
    #[inline(always)]
    pub fn smt(&self) -> SMT_R {
        SMT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - pull select 0: pull down 1: pull up"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 4 - pull enable 0: pull disable 1: pull enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:2 - drive strength for high-speed IO 3.3V: 000: 85.61Ohm 001: 61.2 Ohm 010: 42.88Ohm 011: 35.76Ohm 111: 30.67Ohm for high-speed IO 1.8V: 000: 84.07Ohm 001: 60.14Ohm 010: 42.15Ohm 011: 35.19Ohm 111: 30.2 Ohm for general IO: 00: 4mA 01: 8mA 11: 12mA"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - pin voltage select, only available in high-speed IO 0: 3.3V 1: 1.8V"]
    #[inline(always)]
    pub fn ms(&mut self) -> MS_W<14> {
        MS_W::new(self)
    }
    #[doc = "Bit 13 - open drain 0: open drain disable 1: open drain enable"]
    #[inline(always)]
    pub fn od(&mut self) -> OD_W<13> {
        OD_W::new(self)
    }
    #[doc = "Bit 12 - schmitt trigger enable, only avaiable in high-speed IO 0: disable 1: enable"]
    #[inline(always)]
    pub fn smt(&mut self) -> SMT_W<12> {
        SMT_W::new(self)
    }
    #[doc = "Bit 11 - pull select 0: pull down 1: pull up"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<11> {
        PS_W::new(self)
    }
    #[doc = "Bit 4 - pull enable 0: pull disable 1: pull enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<4> {
        PE_W::new(self)
    }
    #[doc = "Bits 0:2 - drive strength for high-speed IO 3.3V: 000: 85.61Ohm 001: 61.2 Ohm 010: 42.88Ohm 011: 35.76Ohm 111: 30.67Ohm for high-speed IO 1.8V: 000: 84.07Ohm 001: 60.14Ohm 010: 42.15Ohm 011: 35.19Ohm 111: 30.2 Ohm for general IO: 00: 4mA 01: 8mA 11: 12mA"]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W<0> {
        DS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAD SETTINGS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_pb08_pad_ctl](index.html) module"]
pub struct PAD_PB08_PAD_CTL_SPEC;
impl crate::RegisterSpec for PAD_PB08_PAD_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_pb08_pad_ctl::R](R) reader structure"]
impl crate::Readable for PAD_PB08_PAD_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_pb08_pad_ctl::W](W) writer structure"]
impl crate::Writable for PAD_PB08_PAD_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_PB08_PAD_CTL to value 0x1010"]
impl crate::Resettable for PAD_PB08_PAD_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1010
    }
}
