#[doc = "Register `CPU_CPU1_LP` reader"]
pub struct R(crate::R<CPU_CPU1_LP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CPU1_LP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CPU1_LP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CPU1_LP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_CPU1_LP` writer"]
pub struct W(crate::W<CPU_CPU1_LP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CPU1_LP_SPEC>;
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
impl From<crate::W<CPU_CPU1_LP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CPU1_LP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKE_CNT` reader - CPU0 wake up counter, counter saturated at 255, write 0x00 to clear"]
pub type WAKE_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAKE_CNT` writer - CPU0 wake up counter, counter saturated at 255, write 0x00 to clear"]
pub type WAKE_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPU_CPU1_LP_SPEC, u8, u8, 8, O>;
#[doc = "Field `HALT` reader - halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI"]
pub type HALT_R = crate::BitReader<bool>;
#[doc = "Field `HALT` writer - halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI"]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_CPU1_LP_SPEC, bool, O>;
#[doc = "Field `WAKE` reader - CPU0 is waking up 0: CPU0 wake up not asserted 1: CPU0 wake up asserted"]
pub type WAKE_R = crate::BitReader<bool>;
#[doc = "Field `EXEC` reader - CPU0 is executing 0: CPU0 is not executing 1: CPU0 is executing"]
pub type EXEC_R = crate::BitReader<bool>;
#[doc = "Field `WAKE_FLAG` reader - CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wakeup happened"]
pub type WAKE_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `WAKE_FLAG` writer - CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wakeup happened"]
pub type WAKE_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_CPU1_LP_SPEC, bool, O>;
#[doc = "Field `SLEEP_FLAG` reader - CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
pub type SLEEP_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP_FLAG` writer - CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
pub type SLEEP_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_CPU1_LP_SPEC, bool, O>;
#[doc = "Field `RESET_FLAG` reader - CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
pub type RESET_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `RESET_FLAG` writer - CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
pub type RESET_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_CPU1_LP_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPU_CPU1_LP_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 24:31 - CPU0 wake up counter, counter saturated at 255, write 0x00 to clear"]
    #[inline(always)]
    pub fn wake_cnt(&self) -> WAKE_CNT_R {
        WAKE_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 16 - halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU0 is waking up 0: CPU0 wake up not asserted 1: CPU0 wake up asserted"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU0 is executing 0: CPU0 is not executing 1: CPU0 is executing"]
    #[inline(always)]
    pub fn exec(&self) -> EXEC_R {
        EXEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wakeup happened"]
    #[inline(always)]
    pub fn wake_flag(&self) -> WAKE_FLAG_R {
        WAKE_FLAG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
    #[inline(always)]
    pub fn sleep_flag(&self) -> SLEEP_FLAG_R {
        SLEEP_FLAG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
    #[inline(always)]
    pub fn reset_flag(&self) -> RESET_FLAG_R {
        RESET_FLAG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - CPU0 wake up counter, counter saturated at 255, write 0x00 to clear"]
    #[inline(always)]
    pub fn wake_cnt(&mut self) -> WAKE_CNT_W<24> {
        WAKE_CNT_W::new(self)
    }
    #[doc = "Bit 16 - halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI"]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W<16> {
        HALT_W::new(self)
    }
    #[doc = "Bit 10 - CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wakeup happened"]
    #[inline(always)]
    pub fn wake_flag(&mut self) -> WAKE_FLAG_W<10> {
        WAKE_FLAG_W::new(self)
    }
    #[doc = "Bit 9 - CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
    #[inline(always)]
    pub fn sleep_flag(&mut self) -> SLEEP_FLAG_W<9> {
        SLEEP_FLAG_W::new(self)
    }
    #[doc = "Bit 8 - CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
    #[inline(always)]
    pub fn reset_flag(&mut self) -> RESET_FLAG_W<8> {
        RESET_FLAG_W::new(self)
    }
    #[doc = "Bits 0:1 - Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_cpu1_lp](index.html) module"]
pub struct CPU_CPU1_LP_SPEC;
impl crate::RegisterSpec for CPU_CPU1_LP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_cpu1_lp::R](R) reader structure"]
impl crate::Readable for CPU_CPU1_LP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_cpu1_lp::W](W) writer structure"]
impl crate::Writable for CPU_CPU1_LP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_CPU1_LP to value 0x1200"]
impl crate::Resettable for CPU_CPU1_LP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1200
    }
}
