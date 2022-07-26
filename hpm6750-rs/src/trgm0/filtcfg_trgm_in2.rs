#[doc = "Register `FILTCFG_TRGM_IN2` reader"]
pub struct R(crate::R<FILTCFG_TRGM_IN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTCFG_TRGM_IN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTCFG_TRGM_IN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTCFG_TRGM_IN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTCFG_TRGM_IN2` writer"]
pub struct W(crate::W<FILTCFG_TRGM_IN2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTCFG_TRGM_IN2_SPEC>;
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
impl From<crate::W<FILTCFG_TRGM_IN2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTCFG_TRGM_IN2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTINV` reader - 1- Filter will invert the output 0- Filter will not invert the output"]
pub type OUTINV_R = crate::BitReader<bool>;
#[doc = "Field `OUTINV` writer - 1- Filter will invert the output 0- Filter will not invert the output"]
pub type OUTINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FILTCFG_TRGM_IN2_SPEC, bool, O>;
#[doc = "Field `MODE` reader - This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTCFG_TRGM_IN2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SYNCEN` reader - set to enable sychronization input signal with TRGM clock"]
pub type SYNCEN_R = crate::BitReader<bool>;
#[doc = "Field `SYNCEN` writer - set to enable sychronization input signal with TRGM clock"]
pub type SYNCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FILTCFG_TRGM_IN2_SPEC, bool, O>;
#[doc = "Field `FILTLEN` reader - This bitfields defines the filter counter length."]
pub type FILTLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FILTLEN` writer - This bitfields defines the filter counter length."]
pub type FILTLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FILTCFG_TRGM_IN2_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bit 16 - 1- Filter will invert the output 0- Filter will not invert the output"]
    #[inline(always)]
    pub fn outinv(&self) -> OUTINV_R {
        OUTINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 13:15 - This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 12 - set to enable sychronization input signal with TRGM clock"]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 0:11 - This bitfields defines the filter counter length."]
    #[inline(always)]
    pub fn filtlen(&self) -> FILTLEN_R {
        FILTLEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - 1- Filter will invert the output 0- Filter will not invert the output"]
    #[inline(always)]
    pub fn outinv(&mut self) -> OUTINV_W<16> {
        OUTINV_W::new(self)
    }
    #[doc = "Bits 13:15 - This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<13> {
        MODE_W::new(self)
    }
    #[doc = "Bit 12 - set to enable sychronization input signal with TRGM clock"]
    #[inline(always)]
    pub fn syncen(&mut self) -> SYNCEN_W<12> {
        SYNCEN_W::new(self)
    }
    #[doc = "Bits 0:11 - This bitfields defines the filter counter length."]
    #[inline(always)]
    pub fn filtlen(&mut self) -> FILTLEN_W<0> {
        FILTLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filtcfg_trgm_in2](index.html) module"]
pub struct FILTCFG_TRGM_IN2_SPEC;
impl crate::RegisterSpec for FILTCFG_TRGM_IN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filtcfg_trgm_in2::R](R) reader structure"]
impl crate::Readable for FILTCFG_TRGM_IN2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filtcfg_trgm_in2::W](W) writer structure"]
impl crate::Writable for FILTCFG_TRGM_IN2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILTCFG_TRGM_IN2 to value 0"]
impl crate::Resettable for FILTCFG_TRGM_IN2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
