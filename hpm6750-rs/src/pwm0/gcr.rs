#[doc = "Register `GCR` reader"]
pub struct R(crate::R<GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCR` writer"]
pub struct W(crate::W<GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCR_SPEC>;
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
impl From<crate::W<GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAULTI3EN` reader - 1- enable the internal fault input 3"]
pub type FAULTI3EN_R = crate::BitReader<bool>;
#[doc = "Field `FAULTI3EN` writer - 1- enable the internal fault input 3"]
pub type FAULTI3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `FAULTI2EN` reader - 1- enable the internal fault input 2"]
pub type FAULTI2EN_R = crate::BitReader<bool>;
#[doc = "Field `FAULTI2EN` writer - 1- enable the internal fault input 2"]
pub type FAULTI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `FAULTI1EN` reader - 1- enable the internal fault input 1"]
pub type FAULTI1EN_R = crate::BitReader<bool>;
#[doc = "Field `FAULTI1EN` writer - 1- enable the internal fault input 1"]
pub type FAULTI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `FAULTI0EN` reader - 1- enable the internal fault input 0"]
pub type FAULTI0EN_R = crate::BitReader<bool>;
#[doc = "Field `FAULTI0EN` writer - 1- enable the internal fault input 0"]
pub type FAULTI0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `DEBUGFAULT` reader - 1- enable debug mode output protection"]
pub type DEBUGFAULT_R = crate::BitReader<bool>;
#[doc = "Field `DEBUGFAULT` writer - 1- enable debug mode output protection"]
pub type DEBUGFAULT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `FRCPOL` reader - polarity of input pwm_force, 1- active low 0- active high"]
pub type FRCPOL_R = crate::BitReader<bool>;
#[doc = "Field `FRCPOL` writer - polarity of input pwm_force, 1- active low 0- active high"]
pub type FRCPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `HWSHDWEDG` reader - When hardware event is selected as shawdow register effective time and the select comparator is configured as input capture mode. This bit assign its which edge is used as shadow register hardware load event. 1- Falling edge 0- Rising edge"]
pub type HWSHDWEDG_R = crate::BitReader<bool>;
#[doc = "Field `HWSHDWEDG` writer - When hardware event is selected as shawdow register effective time and the select comparator is configured as input capture mode. This bit assign its which edge is used as shadow register hardware load event. 1- Falling edge 0- Rising edge"]
pub type HWSHDWEDG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `CMPSHDWSEL` reader - This bitfield select one of the comparators as hardware event time to load comparator shadow registers"]
pub type CMPSHDWSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPSHDWSEL` writer - This bitfield select one of the comparators as hardware event time to load comparator shadow registers"]
pub type CMPSHDWSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `FAULTRECEDG` reader - When hardware load is selected as output fault recover trigger and the selected channel is capture mode. This bit assign its effective edge of fault recover trigger. 1- Falling edge 0- Rising edge"]
pub type FAULTRECEDG_R = crate::BitReader<bool>;
#[doc = "Field `FAULTRECEDG` writer - When hardware load is selected as output fault recover trigger and the selected channel is capture mode. This bit assign its effective edge of fault recover trigger. 1- Falling edge 0- Rising edge"]
pub type FAULTRECEDG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `FAULTRECHWSEL` reader - Selec one of the 24 comparators as fault output recover trigger."]
pub type FAULTRECHWSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAULTRECHWSEL` writer - Selec one of the 24 comparators as fault output recover trigger."]
pub type FAULTRECHWSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `FAULTE1EN` reader - 1- enable the external fault input 1"]
pub type FAULTE1EN_R = crate::BitReader<bool>;
#[doc = "Field `FAULTE1EN` writer - 1- enable the external fault input 1"]
pub type FAULTE1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `FAULTE0EN` reader - 1- enable the external fault input 0"]
pub type FAULTE0EN_R = crate::BitReader<bool>;
#[doc = "Field `FAULTE0EN` writer - 1- enable the external fault input 0"]
pub type FAULTE0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `FAULTEXPOL` reader - external fault polarity 1-active low 0-active high"]
pub type FAULTEXPOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAULTEXPOL` writer - external fault polarity 1-active low 0-active high"]
pub type FAULTEXPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RLDSYNCEN` reader - 1- pwm timer counter reset to reload value (rld) by synci is enabled"]
pub type RLDSYNCEN_R = crate::BitReader<bool>;
#[doc = "Field `RLDSYNCEN` writer - 1- pwm timer counter reset to reload value (rld) by synci is enabled"]
pub type RLDSYNCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `CEN` reader - 1- enable the pwm timer counter 0- stop the pwm timer counter"]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - 1- enable the pwm timer counter 0- stop the pwm timer counter"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `FAULTCLR` reader - 1- Write 1 to clear the fault condition. The output will recover if FAULTRECTIME is set to 2b'11. User should write 1 to this bit after the active FAULT signal de-assert and before it re-assert again."]
pub type FAULTCLR_R = crate::BitReader<bool>;
#[doc = "Field `FAULTCLR` writer - 1- Write 1 to clear the fault condition. The output will recover if FAULTRECTIME is set to 2b'11. User should write 1 to this bit after the active FAULT signal de-assert and before it re-assert again."]
pub type FAULTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `XRLDSYNCEN` reader - 1- pwm timer extended counter (xcnt) reset to extended reload value (xrld) by synci is enabled"]
pub type XRLDSYNCEN_R = crate::BitReader<bool>;
#[doc = "Field `XRLDSYNCEN` writer - 1- pwm timer extended counter (xcnt) reset to extended reload value (xrld) by synci is enabled"]
pub type XRLDSYNCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `FRCTIME` reader - This bit field select the force effective time 00: force immediately 01: force at main counter reload time 10: force at FRCSYNCI 11: no force"]
pub type FRCTIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRCTIME` writer - This bit field select the force effective time 00: force immediately 01: force at main counter reload time 10: force at FRCSYNCI 11: no force"]
pub type FRCTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SWFRC` reader - 1- write 1 to enable software force, if the frcsrcsel is set to 0, force will take effect"]
pub type SWFRC_R = crate::BitReader<bool>;
#[doc = "Field `SWFRC` writer - 1- write 1 to enable software force, if the frcsrcsel is set to 0, force will take effect"]
pub type SWFRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - 1- enable the internal fault input 3"]
    #[inline(always)]
    pub fn faulti3en(&self) -> FAULTI3EN_R {
        FAULTI3EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- enable the internal fault input 2"]
    #[inline(always)]
    pub fn faulti2en(&self) -> FAULTI2EN_R {
        FAULTI2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - 1- enable the internal fault input 1"]
    #[inline(always)]
    pub fn faulti1en(&self) -> FAULTI1EN_R {
        FAULTI1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - 1- enable the internal fault input 0"]
    #[inline(always)]
    pub fn faulti0en(&self) -> FAULTI0EN_R {
        FAULTI0EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - 1- enable debug mode output protection"]
    #[inline(always)]
    pub fn debugfault(&self) -> DEBUGFAULT_R {
        DEBUGFAULT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - polarity of input pwm_force, 1- active low 0- active high"]
    #[inline(always)]
    pub fn frcpol(&self) -> FRCPOL_R {
        FRCPOL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 24 - When hardware event is selected as shawdow register effective time and the select comparator is configured as input capture mode. This bit assign its which edge is used as shadow register hardware load event. 1- Falling edge 0- Rising edge"]
    #[inline(always)]
    pub fn hwshdwedg(&self) -> HWSHDWEDG_R {
        HWSHDWEDG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 19:23 - This bitfield select one of the comparators as hardware event time to load comparator shadow registers"]
    #[inline(always)]
    pub fn cmpshdwsel(&self) -> CMPSHDWSEL_R {
        CMPSHDWSEL_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 18 - When hardware load is selected as output fault recover trigger and the selected channel is capture mode. This bit assign its effective edge of fault recover trigger. 1- Falling edge 0- Rising edge"]
    #[inline(always)]
    pub fn faultrecedg(&self) -> FAULTRECEDG_R {
        FAULTRECEDG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 13:17 - Selec one of the 24 comparators as fault output recover trigger."]
    #[inline(always)]
    pub fn faultrechwsel(&self) -> FAULTRECHWSEL_R {
        FAULTRECHWSEL_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bit 12 - 1- enable the external fault input 1"]
    #[inline(always)]
    pub fn faulte1en(&self) -> FAULTE1EN_R {
        FAULTE1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - 1- enable the external fault input 0"]
    #[inline(always)]
    pub fn faulte0en(&self) -> FAULTE0EN_R {
        FAULTE0EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 9:10 - external fault polarity 1-active low 0-active high"]
    #[inline(always)]
    pub fn faultexpol(&self) -> FAULTEXPOL_R {
        FAULTEXPOL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 8 - 1- pwm timer counter reset to reload value (rld) by synci is enabled"]
    #[inline(always)]
    pub fn rldsyncen(&self) -> RLDSYNCEN_R {
        RLDSYNCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - 1- enable the pwm timer counter 0- stop the pwm timer counter"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - 1- Write 1 to clear the fault condition. The output will recover if FAULTRECTIME is set to 2b'11. User should write 1 to this bit after the active FAULT signal de-assert and before it re-assert again."]
    #[inline(always)]
    pub fn faultclr(&self) -> FAULTCLR_R {
        FAULTCLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - 1- pwm timer extended counter (xcnt) reset to extended reload value (xrld) by synci is enabled"]
    #[inline(always)]
    pub fn xrldsyncen(&self) -> XRLDSYNCEN_R {
        XRLDSYNCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 1:2 - This bit field select the force effective time 00: force immediately 01: force at main counter reload time 10: force at FRCSYNCI 11: no force"]
    #[inline(always)]
    pub fn frctime(&self) -> FRCTIME_R {
        FRCTIME_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 0 - 1- write 1 to enable software force, if the frcsrcsel is set to 0, force will take effect"]
    #[inline(always)]
    pub fn swfrc(&self) -> SWFRC_R {
        SWFRC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 1- enable the internal fault input 3"]
    #[inline(always)]
    pub fn faulti3en(&mut self) -> FAULTI3EN_W<31> {
        FAULTI3EN_W::new(self)
    }
    #[doc = "Bit 30 - 1- enable the internal fault input 2"]
    #[inline(always)]
    pub fn faulti2en(&mut self) -> FAULTI2EN_W<30> {
        FAULTI2EN_W::new(self)
    }
    #[doc = "Bit 29 - 1- enable the internal fault input 1"]
    #[inline(always)]
    pub fn faulti1en(&mut self) -> FAULTI1EN_W<29> {
        FAULTI1EN_W::new(self)
    }
    #[doc = "Bit 28 - 1- enable the internal fault input 0"]
    #[inline(always)]
    pub fn faulti0en(&mut self) -> FAULTI0EN_W<28> {
        FAULTI0EN_W::new(self)
    }
    #[doc = "Bit 27 - 1- enable debug mode output protection"]
    #[inline(always)]
    pub fn debugfault(&mut self) -> DEBUGFAULT_W<27> {
        DEBUGFAULT_W::new(self)
    }
    #[doc = "Bit 26 - polarity of input pwm_force, 1- active low 0- active high"]
    #[inline(always)]
    pub fn frcpol(&mut self) -> FRCPOL_W<26> {
        FRCPOL_W::new(self)
    }
    #[doc = "Bit 24 - When hardware event is selected as shawdow register effective time and the select comparator is configured as input capture mode. This bit assign its which edge is used as shadow register hardware load event. 1- Falling edge 0- Rising edge"]
    #[inline(always)]
    pub fn hwshdwedg(&mut self) -> HWSHDWEDG_W<24> {
        HWSHDWEDG_W::new(self)
    }
    #[doc = "Bits 19:23 - This bitfield select one of the comparators as hardware event time to load comparator shadow registers"]
    #[inline(always)]
    pub fn cmpshdwsel(&mut self) -> CMPSHDWSEL_W<19> {
        CMPSHDWSEL_W::new(self)
    }
    #[doc = "Bit 18 - When hardware load is selected as output fault recover trigger and the selected channel is capture mode. This bit assign its effective edge of fault recover trigger. 1- Falling edge 0- Rising edge"]
    #[inline(always)]
    pub fn faultrecedg(&mut self) -> FAULTRECEDG_W<18> {
        FAULTRECEDG_W::new(self)
    }
    #[doc = "Bits 13:17 - Selec one of the 24 comparators as fault output recover trigger."]
    #[inline(always)]
    pub fn faultrechwsel(&mut self) -> FAULTRECHWSEL_W<13> {
        FAULTRECHWSEL_W::new(self)
    }
    #[doc = "Bit 12 - 1- enable the external fault input 1"]
    #[inline(always)]
    pub fn faulte1en(&mut self) -> FAULTE1EN_W<12> {
        FAULTE1EN_W::new(self)
    }
    #[doc = "Bit 11 - 1- enable the external fault input 0"]
    #[inline(always)]
    pub fn faulte0en(&mut self) -> FAULTE0EN_W<11> {
        FAULTE0EN_W::new(self)
    }
    #[doc = "Bits 9:10 - external fault polarity 1-active low 0-active high"]
    #[inline(always)]
    pub fn faultexpol(&mut self) -> FAULTEXPOL_W<9> {
        FAULTEXPOL_W::new(self)
    }
    #[doc = "Bit 8 - 1- pwm timer counter reset to reload value (rld) by synci is enabled"]
    #[inline(always)]
    pub fn rldsyncen(&mut self) -> RLDSYNCEN_W<8> {
        RLDSYNCEN_W::new(self)
    }
    #[doc = "Bit 7 - 1- enable the pwm timer counter 0- stop the pwm timer counter"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W<7> {
        CEN_W::new(self)
    }
    #[doc = "Bit 6 - 1- Write 1 to clear the fault condition. The output will recover if FAULTRECTIME is set to 2b'11. User should write 1 to this bit after the active FAULT signal de-assert and before it re-assert again."]
    #[inline(always)]
    pub fn faultclr(&mut self) -> FAULTCLR_W<6> {
        FAULTCLR_W::new(self)
    }
    #[doc = "Bit 5 - 1- pwm timer extended counter (xcnt) reset to extended reload value (xrld) by synci is enabled"]
    #[inline(always)]
    pub fn xrldsyncen(&mut self) -> XRLDSYNCEN_W<5> {
        XRLDSYNCEN_W::new(self)
    }
    #[doc = "Bits 1:2 - This bit field select the force effective time 00: force immediately 01: force at main counter reload time 10: force at FRCSYNCI 11: no force"]
    #[inline(always)]
    pub fn frctime(&mut self) -> FRCTIME_W<1> {
        FRCTIME_W::new(self)
    }
    #[doc = "Bit 0 - 1- write 1 to enable software force, if the frcsrcsel is set to 0, force will take effect"]
    #[inline(always)]
    pub fn swfrc(&mut self) -> SWFRC_W<0> {
        SWFRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcr](index.html) module"]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcr::R](R) reader structure"]
impl crate::Readable for GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcr::W](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCR to value 0"]
impl crate::Resettable for GCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
