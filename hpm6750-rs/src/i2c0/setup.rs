#[doc = "Register `SETUP` reader"]
pub struct R(crate::R<SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETUP` writer"]
pub struct W(crate::W<SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETUP_SPEC>;
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
impl From<crate::W<SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IICEN` reader - Enable the I2C controller. 1: Enable 0: Disable"]
pub type IICEN_R = crate::BitReader<bool>;
#[doc = "Field `IICEN` writer - Enable the I2C controller. 1: Enable 0: Disable"]
pub type IICEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETUP_SPEC, bool, O>;
#[doc = "Field `ADDRESSING` reader - I2C addressing mode: 1: 10-bit addressing mode 0: 7-bit addressing mode"]
pub type ADDRESSING_R = crate::BitReader<bool>;
#[doc = "Field `ADDRESSING` writer - I2C addressing mode: 1: 10-bit addressing mode 0: 7-bit addressing mode"]
pub type ADDRESSING_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETUP_SPEC, bool, O>;
#[doc = "Field `MASTER` reader - Configure this device as a master or a slave. 1: Master mode 0: Slave mode"]
pub type MASTER_R = crate::BitReader<bool>;
#[doc = "Field `MASTER` writer - Configure this device as a master or a slave. 1: Master mode 0: Slave mode"]
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETUP_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - Enable the direct memory access mode data transfer. 1: Enable 0: Disable"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - Enable the direct memory access mode data transfer. 1: Enable 0: Disable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETUP_SPEC, bool, O>;
#[doc = "Field `T_SCLHI` reader - The HIGH period of generated SCL clock is defined by T_SCLHi. SCL HIGH period = (2 * tpclk) + (2 + T_SP + T_SCLHi) * tpclk* (TPM+1) The T_SCLHi value must be greater than T_SP and T_HDDAT values. This field is only valid when the controller is in the master mode."]
pub type T_SCLHI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_SCLHI` writer - The HIGH period of generated SCL clock is defined by T_SCLHi. SCL HIGH period = (2 * tpclk) + (2 + T_SP + T_SCLHi) * tpclk* (TPM+1) The T_SCLHi value must be greater than T_SP and T_HDDAT values. This field is only valid when the controller is in the master mode."]
pub type T_SCLHI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP_SPEC, u16, u16, 9, O>;
#[doc = "Field `T_SCLRADIO` reader - The LOW period of the generated SCL clock is defined by the combination of T_SCLRatio and T_SCLHi values. When T_SCLRatio = 0, the LOW period is equal to HIGH period. When T_SCLRatio = 1, the LOW period is roughly two times of HIGH period. SCL LOW period = (2 * tpclk) + (2 + T_SP + T_SCLHi * ratio) * tpclk * (TPM+1) 1: ratio = 2 0: ratio = 1 This field is only valid when the controller is in the master mode."]
pub type T_SCLRADIO_R = crate::BitReader<bool>;
#[doc = "Field `T_SCLRADIO` writer - The LOW period of the generated SCL clock is defined by the combination of T_SCLRatio and T_SCLHi values. When T_SCLRatio = 0, the LOW period is equal to HIGH period. When T_SCLRatio = 1, the LOW period is roughly two times of HIGH period. SCL LOW period = (2 * tpclk) + (2 + T_SP + T_SCLHi * ratio) * tpclk * (TPM+1) 1: ratio = 2 0: ratio = 1 This field is only valid when the controller is in the master mode."]
pub type T_SCLRADIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETUP_SPEC, bool, O>;
#[doc = "Field `T_HDDAT` reader - T_HDDAT defines the data hold time after SCL goes LOW Hold time = (2 * tpclk) + (2 + T_SP + T_HDDAT) * tpclk* (TPM+1)"]
pub type T_HDDAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_HDDAT` writer - T_HDDAT defines the data hold time after SCL goes LOW Hold time = (2 * tpclk) + (2 + T_SP + T_HDDAT) * tpclk* (TPM+1)"]
pub type T_HDDAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_SP` reader - T_SP defines the pulse width of spikes that must be suppressed by the input filter. Pulse width = T_SP * tpclk* (TPM+1)"]
pub type T_SP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_SP` writer - T_SP defines the pulse width of spikes that must be suppressed by the input filter. Pulse width = T_SP * tpclk* (TPM+1)"]
pub type T_SP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP_SPEC, u8, u8, 3, O>;
#[doc = "Field `T_SUDAT` reader - T_SUDAT defines the data setup time before releasing the SCL. Setup time = (2 * tpclk) + (2 + T_SP + T_SUDAT) * tpclk* (TPM+1) tpclk = PCLK period TPM = The multiplier value in Timing Parameter Multiplier Register"]
pub type T_SUDAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_SUDAT` writer - T_SUDAT defines the data setup time before releasing the SCL. Setup time = (2 * tpclk) + (2 + T_SP + T_SUDAT) * tpclk* (TPM+1) tpclk = PCLK period TPM = The multiplier value in Timing Parameter Multiplier Register"]
pub type T_SUDAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Enable the I2C controller. 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn iicen(&self) -> IICEN_R {
        IICEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C addressing mode: 1: 10-bit addressing mode 0: 7-bit addressing mode"]
    #[inline(always)]
    pub fn addressing(&self) -> ADDRESSING_R {
        ADDRESSING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configure this device as a master or a slave. 1: Master mode 0: Slave mode"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable the direct memory access mode data transfer. 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:12 - The HIGH period of generated SCL clock is defined by T_SCLHi. SCL HIGH period = (2 * tpclk) + (2 + T_SP + T_SCLHi) * tpclk* (TPM+1) The T_SCLHi value must be greater than T_SP and T_HDDAT values. This field is only valid when the controller is in the master mode."]
    #[inline(always)]
    pub fn t_sclhi(&self) -> T_SCLHI_R {
        T_SCLHI_R::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - The LOW period of the generated SCL clock is defined by the combination of T_SCLRatio and T_SCLHi values. When T_SCLRatio = 0, the LOW period is equal to HIGH period. When T_SCLRatio = 1, the LOW period is roughly two times of HIGH period. SCL LOW period = (2 * tpclk) + (2 + T_SP + T_SCLHi * ratio) * tpclk * (TPM+1) 1: ratio = 2 0: ratio = 1 This field is only valid when the controller is in the master mode."]
    #[inline(always)]
    pub fn t_sclradio(&self) -> T_SCLRADIO_R {
        T_SCLRADIO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - T_HDDAT defines the data hold time after SCL goes LOW Hold time = (2 * tpclk) + (2 + T_SP + T_HDDAT) * tpclk* (TPM+1)"]
    #[inline(always)]
    pub fn t_hddat(&self) -> T_HDDAT_R {
        T_HDDAT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - T_SP defines the pulse width of spikes that must be suppressed by the input filter. Pulse width = T_SP * tpclk* (TPM+1)"]
    #[inline(always)]
    pub fn t_sp(&self) -> T_SP_R {
        T_SP_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - T_SUDAT defines the data setup time before releasing the SCL. Setup time = (2 * tpclk) + (2 + T_SP + T_SUDAT) * tpclk* (TPM+1) tpclk = PCLK period TPM = The multiplier value in Timing Parameter Multiplier Register"]
    #[inline(always)]
    pub fn t_sudat(&self) -> T_SUDAT_R {
        T_SUDAT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the I2C controller. 1: Enable 0: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn iicen(&mut self) -> IICEN_W<0> {
        IICEN_W::new(self)
    }
    #[doc = "Bit 1 - I2C addressing mode: 1: 10-bit addressing mode 0: 7-bit addressing mode"]
    #[inline(always)]
    #[must_use]
    pub fn addressing(&mut self) -> ADDRESSING_W<1> {
        ADDRESSING_W::new(self)
    }
    #[doc = "Bit 2 - Configure this device as a master or a slave. 1: Master mode 0: Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<2> {
        MASTER_W::new(self)
    }
    #[doc = "Bit 3 - Enable the direct memory access mode data transfer. 1: Enable 0: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<3> {
        DMAEN_W::new(self)
    }
    #[doc = "Bits 4:12 - The HIGH period of generated SCL clock is defined by T_SCLHi. SCL HIGH period = (2 * tpclk) + (2 + T_SP + T_SCLHi) * tpclk* (TPM+1) The T_SCLHi value must be greater than T_SP and T_HDDAT values. This field is only valid when the controller is in the master mode."]
    #[inline(always)]
    #[must_use]
    pub fn t_sclhi(&mut self) -> T_SCLHI_W<4> {
        T_SCLHI_W::new(self)
    }
    #[doc = "Bit 13 - The LOW period of the generated SCL clock is defined by the combination of T_SCLRatio and T_SCLHi values. When T_SCLRatio = 0, the LOW period is equal to HIGH period. When T_SCLRatio = 1, the LOW period is roughly two times of HIGH period. SCL LOW period = (2 * tpclk) + (2 + T_SP + T_SCLHi * ratio) * tpclk * (TPM+1) 1: ratio = 2 0: ratio = 1 This field is only valid when the controller is in the master mode."]
    #[inline(always)]
    #[must_use]
    pub fn t_sclradio(&mut self) -> T_SCLRADIO_W<13> {
        T_SCLRADIO_W::new(self)
    }
    #[doc = "Bits 16:20 - T_HDDAT defines the data hold time after SCL goes LOW Hold time = (2 * tpclk) + (2 + T_SP + T_HDDAT) * tpclk* (TPM+1)"]
    #[inline(always)]
    #[must_use]
    pub fn t_hddat(&mut self) -> T_HDDAT_W<16> {
        T_HDDAT_W::new(self)
    }
    #[doc = "Bits 21:23 - T_SP defines the pulse width of spikes that must be suppressed by the input filter. Pulse width = T_SP * tpclk* (TPM+1)"]
    #[inline(always)]
    #[must_use]
    pub fn t_sp(&mut self) -> T_SP_W<21> {
        T_SP_W::new(self)
    }
    #[doc = "Bits 24:28 - T_SUDAT defines the data setup time before releasing the SCL. Setup time = (2 * tpclk) + (2 + T_SP + T_SUDAT) * tpclk* (TPM+1) tpclk = PCLK period TPM = The multiplier value in Timing Parameter Multiplier Register"]
    #[inline(always)]
    #[must_use]
    pub fn t_sudat(&mut self) -> T_SUDAT_W<24> {
        T_SUDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Setup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup](index.html) module"]
pub struct SETUP_SPEC;
impl crate::RegisterSpec for SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setup::R](R) reader structure"]
impl crate::Readable for SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setup::W](W) writer structure"]
impl crate::Writable for SETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETUP to value 0x0525_2100"]
impl crate::Resettable for SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0x0525_2100;
}
