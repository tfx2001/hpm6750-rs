#[doc = "Register `PMT_CSR` reader"]
pub struct R(crate::R<PMT_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMT_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMT_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMT_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMT_CSR` writer"]
pub struct W(crate::W<PMT_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMT_CSR_SPEC>;
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
impl From<crate::W<PMT_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMT_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWKFILTRST` reader - Remote Wake-Up Frame Filter Register Pointer Reset When this bit is set, it resets the remote wake-up frame filter register pointer to 3’b000. It is automatically cleared after 1 clock cycle."]
pub type RWKFILTRST_R = crate::BitReader<bool>;
#[doc = "Field `RWKFILTRST` writer - Remote Wake-Up Frame Filter Register Pointer Reset When this bit is set, it resets the remote wake-up frame filter register pointer to 3’b000. It is automatically cleared after 1 clock cycle."]
pub type RWKFILTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMT_CSR_SPEC, bool, O>;
#[doc = "Field `RWKPTR` reader - Remote Wake-up FIFO Pointer This field gives the current value (0 to 31) of the Remote Wake-up Frame filter register pointer. When the value of this pointer is equal to 7, 15, 23 or 31, the contents of the Remote Wake-up Frame Filter Register are transferred to the clk_rx_i domain when a write occurs to that register. The maximum value of the pointer is 7, 15, 23 and 31 respectively depending on the number of Remote Wakeup Filters selected during configuration."]
pub type RWKPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWKPTR` writer - Remote Wake-up FIFO Pointer This field gives the current value (0 to 31) of the Remote Wake-up Frame filter register pointer. When the value of this pointer is equal to 7, 15, 23 or 31, the contents of the Remote Wake-up Frame Filter Register are transferred to the clk_rx_i domain when a write occurs to that register. The maximum value of the pointer is 7, 15, 23 and 31 respectively depending on the number of Remote Wakeup Filters selected during configuration."]
pub type RWKPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMT_CSR_SPEC, u8, u8, 5, O>;
#[doc = "Field `GLBLUCAST` reader - Global Unicast When set, enables any unicast packet filtered by the MAC (DAF) address recognition to be a remote wake-up frame."]
pub type GLBLUCAST_R = crate::BitReader<bool>;
#[doc = "Field `GLBLUCAST` writer - Global Unicast When set, enables any unicast packet filtered by the MAC (DAF) address recognition to be a remote wake-up frame."]
pub type GLBLUCAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMT_CSR_SPEC, bool, O>;
#[doc = "Field `RWKPRCVD` reader - Remote Wake-Up Frame Received When set, this bit indicates the power management event is generated because of the reception of a remote wake-up frame. This bit is cleared by a Read into this register."]
pub type RWKPRCVD_R = crate::BitReader<bool>;
#[doc = "Field `RWKPRCVD` writer - Remote Wake-Up Frame Received When set, this bit indicates the power management event is generated because of the reception of a remote wake-up frame. This bit is cleared by a Read into this register."]
pub type RWKPRCVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMT_CSR_SPEC, bool, O>;
#[doc = "Field `MGKPRCVD` reader - Magic Packet Received When set, this bit indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared by a Read into this register."]
pub type MGKPRCVD_R = crate::BitReader<bool>;
#[doc = "Field `MGKPRCVD` writer - Magic Packet Received When set, this bit indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared by a Read into this register."]
pub type MGKPRCVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMT_CSR_SPEC, bool, O>;
#[doc = "Field `RWKPKTEN` reader - Remote Wake-Up Frame Enable When set, enables generation of a power management event because of remote wake-up frame reception."]
pub type RWKPKTEN_R = crate::BitReader<bool>;
#[doc = "Field `RWKPKTEN` writer - Remote Wake-Up Frame Enable When set, enables generation of a power management event because of remote wake-up frame reception."]
pub type RWKPKTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMT_CSR_SPEC, bool, O>;
#[doc = "Field `MGKPKTEN` reader - Magic Packet Enable When set, enables generation of a power management event because of magic packet reception."]
pub type MGKPKTEN_R = crate::BitReader<bool>;
#[doc = "Field `MGKPKTEN` writer - Magic Packet Enable When set, enables generation of a power management event because of magic packet reception."]
pub type MGKPKTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMT_CSR_SPEC, bool, O>;
#[doc = "Field `PWRDWN` reader - Power Down When set, the MAC receiver drops all received frames until it receives the expected magic packet or remote wake-up frame. This bit is then self-cleared and the power-down mode is disabled. The Software can also clear this bit before the expected magic packet or remote wake-up frame is received. The frames, received by the MAC after this bit is cleared, are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote Wake-Up Frame Enable bit is set high. Note: You can gate-off the CSR clock during the power-down mode. However, when the CSR clock is gated-off, you cannot perform any read or write operations on this register. Therefore, the Software cannot clear this bit."]
pub type PWRDWN_R = crate::BitReader<bool>;
#[doc = "Field `PWRDWN` writer - Power Down When set, the MAC receiver drops all received frames until it receives the expected magic packet or remote wake-up frame. This bit is then self-cleared and the power-down mode is disabled. The Software can also clear this bit before the expected magic packet or remote wake-up frame is received. The frames, received by the MAC after this bit is cleared, are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote Wake-Up Frame Enable bit is set high. Note: You can gate-off the CSR clock during the power-down mode. However, when the CSR clock is gated-off, you cannot perform any read or write operations on this register. Therefore, the Software cannot clear this bit."]
pub type PWRDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMT_CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Remote Wake-Up Frame Filter Register Pointer Reset When this bit is set, it resets the remote wake-up frame filter register pointer to 3’b000. It is automatically cleared after 1 clock cycle."]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Remote Wake-up FIFO Pointer This field gives the current value (0 to 31) of the Remote Wake-up Frame filter register pointer. When the value of this pointer is equal to 7, 15, 23 or 31, the contents of the Remote Wake-up Frame Filter Register are transferred to the clk_rx_i domain when a write occurs to that register. The maximum value of the pointer is 7, 15, 23 and 31 respectively depending on the number of Remote Wakeup Filters selected during configuration."]
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - Global Unicast When set, enables any unicast packet filtered by the MAC (DAF) address recognition to be a remote wake-up frame."]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 6 - Remote Wake-Up Frame Received When set, this bit indicates the power management event is generated because of the reception of a remote wake-up frame. This bit is cleared by a Read into this register."]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received When set, this bit indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared by a Read into this register."]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 2 - Remote Wake-Up Frame Enable When set, enables generation of a power management event because of remote wake-up frame reception."]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable When set, enables generation of a power management event because of magic packet reception."]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Power Down When set, the MAC receiver drops all received frames until it receives the expected magic packet or remote wake-up frame. This bit is then self-cleared and the power-down mode is disabled. The Software can also clear this bit before the expected magic packet or remote wake-up frame is received. The frames, received by the MAC after this bit is cleared, are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote Wake-Up Frame Enable bit is set high. Note: You can gate-off the CSR clock during the power-down mode. However, when the CSR clock is gated-off, you cannot perform any read or write operations on this register. Therefore, the Software cannot clear this bit."]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Remote Wake-Up Frame Filter Register Pointer Reset When this bit is set, it resets the remote wake-up frame filter register pointer to 3’b000. It is automatically cleared after 1 clock cycle."]
    #[inline(always)]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W<31> {
        RWKFILTRST_W::new(self)
    }
    #[doc = "Bits 24:28 - Remote Wake-up FIFO Pointer This field gives the current value (0 to 31) of the Remote Wake-up Frame filter register pointer. When the value of this pointer is equal to 7, 15, 23 or 31, the contents of the Remote Wake-up Frame Filter Register are transferred to the clk_rx_i domain when a write occurs to that register. The maximum value of the pointer is 7, 15, 23 and 31 respectively depending on the number of Remote Wakeup Filters selected during configuration."]
    #[inline(always)]
    pub fn rwkptr(&mut self) -> RWKPTR_W<24> {
        RWKPTR_W::new(self)
    }
    #[doc = "Bit 9 - Global Unicast When set, enables any unicast packet filtered by the MAC (DAF) address recognition to be a remote wake-up frame."]
    #[inline(always)]
    pub fn glblucast(&mut self) -> GLBLUCAST_W<9> {
        GLBLUCAST_W::new(self)
    }
    #[doc = "Bit 6 - Remote Wake-Up Frame Received When set, this bit indicates the power management event is generated because of the reception of a remote wake-up frame. This bit is cleared by a Read into this register."]
    #[inline(always)]
    pub fn rwkprcvd(&mut self) -> RWKPRCVD_W<6> {
        RWKPRCVD_W::new(self)
    }
    #[doc = "Bit 5 - Magic Packet Received When set, this bit indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared by a Read into this register."]
    #[inline(always)]
    pub fn mgkprcvd(&mut self) -> MGKPRCVD_W<5> {
        MGKPRCVD_W::new(self)
    }
    #[doc = "Bit 2 - Remote Wake-Up Frame Enable When set, enables generation of a power management event because of remote wake-up frame reception."]
    #[inline(always)]
    pub fn rwkpkten(&mut self) -> RWKPKTEN_W<2> {
        RWKPKTEN_W::new(self)
    }
    #[doc = "Bit 1 - Magic Packet Enable When set, enables generation of a power management event because of magic packet reception."]
    #[inline(always)]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W<1> {
        MGKPKTEN_W::new(self)
    }
    #[doc = "Bit 0 - Power Down When set, the MAC receiver drops all received frames until it receives the expected magic packet or remote wake-up frame. This bit is then self-cleared and the power-down mode is disabled. The Software can also clear this bit before the expected magic packet or remote wake-up frame is received. The frames, received by the MAC after this bit is cleared, are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote Wake-Up Frame Enable bit is set high. Note: You can gate-off the CSR clock during the power-down mode. However, when the CSR clock is gated-off, you cannot perform any read or write operations on this register. Therefore, the Software cannot clear this bit."]
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<0> {
        PWRDWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMT Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmt_csr](index.html) module"]
pub struct PMT_CSR_SPEC;
impl crate::RegisterSpec for PMT_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmt_csr::R](R) reader structure"]
impl crate::Readable for PMT_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmt_csr::W](W) writer structure"]
impl crate::Writable for PMT_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMT_CSR to value 0"]
impl crate::Resettable for PMT_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
