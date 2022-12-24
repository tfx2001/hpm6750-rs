#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SENSOR_BIT_WIDTH` reader - the bit width of the sensor 0: 8 bits 1: 10 bits Others: Undefined"]
pub type SENSOR_BIT_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SENSOR_BIT_WIDTH` writer - the bit width of the sensor 0: 8 bits 1: 10 bits Others: Undefined"]
pub type SENSOR_BIT_WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `COLOR_FORMATS` reader - input color formats: 0010b: 24bit: RGB888 0100b: 16bit: RGB565 0111b: 16bit: YCbCr422 (Y0 Cb Y1 Cr, each 8-bit) YUV YCrCb Note: YUV420 is not supported. 1000b: 24bit: YUV444"]
pub type COLOR_FORMATS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COLOR_FORMATS` writer - input color formats: 0010b: 24bit: RGB888 0100b: 16bit: RGB565 0111b: 16bit: YCbCr422 (Y0 Cb Y1 Cr, each 8-bit) YUV YCrCb Note: YUV420 is not supported. 1000b: 24bit: YUV444"]
pub type COLOR_FORMATS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `STORAGE_MODE` reader - 00: Normal Mode (one plane mode) 01: Two Plane Mode (Y, UV plane) 10: Y-only Mode, byte sequence as Y0,Y1,Y2,Y3 11: Binary Mode, bit sequence is from LSB to MSB when CR20\\[BIG_END\\]=0"]
pub type STORAGE_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STORAGE_MODE` writer - 00: Normal Mode (one plane mode) 01: Two Plane Mode (Y, UV plane) 10: Y-only Mode, byte sequence as Y0,Y1,Y2,Y3 11: Binary Mode, bit sequence is from LSB to MSB when CR20\\[BIG_END\\]=0"]
pub type STORAGE_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `INV_DATA` reader - Invert Data Input. This bit enables or disables internal inverters on the data lines. 0 CAM_D data lines are directly applied to internal circuitry 1 CAM_D data lines are inverted before applied to internal circuitry"]
pub type INV_DATA_R = crate::BitReader<bool>;
#[doc = "Field `INV_DATA` writer - Invert Data Input. This bit enables or disables internal inverters on the data lines. 0 CAM_D data lines are directly applied to internal circuitry 1 CAM_D data lines are inverted before applied to internal circuitry"]
pub type INV_DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SOF_INT_POL` reader - SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt. 0 SOF interrupt is generated on SOF falling edge 1 SOF interrupt is generated on SOF rising edge"]
pub type SOF_INT_POL_R = crate::BitReader<bool>;
#[doc = "Field `SOF_INT_POL` writer - SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt. 0 SOF interrupt is generated on SOF falling edge 1 SOF interrupt is generated on SOF rising edge"]
pub type SOF_INT_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SYNC_RXFIFO_CLR` reader - Synchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO on every SOF."]
pub type SYNC_RXFIFO_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SYNC_RXFIFO_CLR` writer - Synchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO on every SOF."]
pub type SYNC_RXFIFO_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ASYNC_RXFIFO_CLR` reader - ASynchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO immediately. It will be auto-cleared."]
pub type ASYNC_RXFIFO_CLR_R = crate::BitReader<bool>;
#[doc = "Field `ASYNC_RXFIFO_CLR` writer - ASynchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO immediately. It will be auto-cleared."]
pub type ASYNC_RXFIFO_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `RESTART_BUSPTR` reader - force to restart the bus pointer at the every end of the sof period, and at the same time, clr the fifo pointer"]
pub type RESTART_BUSPTR_R = crate::BitReader<bool>;
#[doc = "Field `RESTART_BUSPTR` writer - force to restart the bus pointer at the every end of the sof period, and at the same time, clr the fifo pointer"]
pub type RESTART_BUSPTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `PACK_DIR` reader - Data Packing Direction. This bit Controls how 8-bit/10-bit image data is packed into 32-bit RX FIFO. 0 Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. 1 Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO."]
pub type PACK_DIR_R = crate::BitReader<bool>;
#[doc = "Field `PACK_DIR` writer - Data Packing Direction. This bit Controls how 8-bit/10-bit image data is packed into 32-bit RX FIFO. 0 Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. 1 Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO."]
pub type PACK_DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SWAP16_EN` reader - SWAP 16-Bit Enable. This bit enables the swapping of 16-bit data. Data is packed from 8-bit or 10-bit to 32-bit first (according to the setting of PACK_DIR) and then swapped as 16-bit words before being put into the RX FIFO. The action of the bit only affects the RX FIFO. NOTE: Example of swapping enabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x 33441122 NOTE: Example of swapping disabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x11223344 0 Disable swapping 1 Enable swapping"]
pub type SWAP16_EN_R = crate::BitReader<bool>;
#[doc = "Field `SWAP16_EN` writer - SWAP 16-Bit Enable. This bit enables the swapping of 16-bit data. Data is packed from 8-bit or 10-bit to 32-bit first (according to the setting of PACK_DIR) and then swapped as 16-bit words before being put into the RX FIFO. The action of the bit only affects the RX FIFO. NOTE: Example of swapping enabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x 33441122 NOTE: Example of swapping disabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x11223344 0 Disable swapping 1 Enable swapping"]
pub type SWAP16_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `INV_VSYNC` reader - invert vsync pad input before it is used"]
pub type INV_VSYNC_R = crate::BitReader<bool>;
#[doc = "Field `INV_VSYNC` writer - invert vsync pad input before it is used"]
pub type INV_VSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `INV_HSYNC` reader - invert hsync pad input before it is used"]
pub type INV_HSYNC_R = crate::BitReader<bool>;
#[doc = "Field `INV_HSYNC` writer - invert hsync pad input before it is used"]
pub type INV_HSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `INV_PIXCLK` reader - invert pixclk pad input before it is used"]
pub type INV_PIXCLK_R = crate::BitReader<bool>;
#[doc = "Field `INV_PIXCLK` writer - invert pixclk pad input before it is used"]
pub type INV_PIXCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `COLOR_EXT` reader - If asserted, will change the output color to ARGB8888 mode. Used by input color as RGB565, RGB888, YUV888, etc. The byte sequence is B,G,R,A. Depends on correct CR2\\[ClrBitFormat\\]
configuration."]
pub type COLOR_EXT_R = crate::BitReader<bool>;
#[doc = "Field `COLOR_EXT` writer - If asserted, will change the output color to ARGB8888 mode. Used by input color as RGB565, RGB888, YUV888, etc. The byte sequence is B,G,R,A. Depends on correct CR2\\[ClrBitFormat\\]
configuration."]
pub type COLOR_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - the bit width of the sensor 0: 8 bits 1: 10 bits Others: Undefined"]
    #[inline(always)]
    pub fn sensor_bit_width(&self) -> SENSOR_BIT_WIDTH_R {
        SENSOR_BIT_WIDTH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - input color formats: 0010b: 24bit: RGB888 0100b: 16bit: RGB565 0111b: 16bit: YCbCr422 (Y0 Cb Y1 Cr, each 8-bit) YUV YCrCb Note: YUV420 is not supported. 1000b: 24bit: YUV444"]
    #[inline(always)]
    pub fn color_formats(&self) -> COLOR_FORMATS_R {
        COLOR_FORMATS_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - 00: Normal Mode (one plane mode) 01: Two Plane Mode (Y, UV plane) 10: Y-only Mode, byte sequence as Y0,Y1,Y2,Y3 11: Binary Mode, bit sequence is from LSB to MSB when CR20\\[BIG_END\\]=0"]
    #[inline(always)]
    pub fn storage_mode(&self) -> STORAGE_MODE_R {
        STORAGE_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 15 - Invert Data Input. This bit enables or disables internal inverters on the data lines. 0 CAM_D data lines are directly applied to internal circuitry 1 CAM_D data lines are inverted before applied to internal circuitry"]
    #[inline(always)]
    pub fn inv_data(&self) -> INV_DATA_R {
        INV_DATA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt. 0 SOF interrupt is generated on SOF falling edge 1 SOF interrupt is generated on SOF rising edge"]
    #[inline(always)]
    pub fn sof_int_pol(&self) -> SOF_INT_POL_R {
        SOF_INT_POL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Synchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO on every SOF."]
    #[inline(always)]
    pub fn sync_rxfifo_clr(&self) -> SYNC_RXFIFO_CLR_R {
        SYNC_RXFIFO_CLR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ASynchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO immediately. It will be auto-cleared."]
    #[inline(always)]
    pub fn async_rxfifo_clr(&self) -> ASYNC_RXFIFO_CLR_R {
        ASYNC_RXFIFO_CLR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - force to restart the bus pointer at the every end of the sof period, and at the same time, clr the fifo pointer"]
    #[inline(always)]
    pub fn restart_busptr(&self) -> RESTART_BUSPTR_R {
        RESTART_BUSPTR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Data Packing Direction. This bit Controls how 8-bit/10-bit image data is packed into 32-bit RX FIFO. 0 Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. 1 Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO."]
    #[inline(always)]
    pub fn pack_dir(&self) -> PACK_DIR_R {
        PACK_DIR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SWAP 16-Bit Enable. This bit enables the swapping of 16-bit data. Data is packed from 8-bit or 10-bit to 32-bit first (according to the setting of PACK_DIR) and then swapped as 16-bit words before being put into the RX FIFO. The action of the bit only affects the RX FIFO. NOTE: Example of swapping enabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x 33441122 NOTE: Example of swapping disabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x11223344 0 Disable swapping 1 Enable swapping"]
    #[inline(always)]
    pub fn swap16_en(&self) -> SWAP16_EN_R {
        SWAP16_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - invert vsync pad input before it is used"]
    #[inline(always)]
    pub fn inv_vsync(&self) -> INV_VSYNC_R {
        INV_VSYNC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - invert hsync pad input before it is used"]
    #[inline(always)]
    pub fn inv_hsync(&self) -> INV_HSYNC_R {
        INV_HSYNC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - invert pixclk pad input before it is used"]
    #[inline(always)]
    pub fn inv_pixclk(&self) -> INV_PIXCLK_R {
        INV_PIXCLK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - If asserted, will change the output color to ARGB8888 mode. Used by input color as RGB565, RGB888, YUV888, etc. The byte sequence is B,G,R,A. Depends on correct CR2\\[ClrBitFormat\\]
configuration."]
    #[inline(always)]
    pub fn color_ext(&self) -> COLOR_EXT_R {
        COLOR_EXT_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - the bit width of the sensor 0: 8 bits 1: 10 bits Others: Undefined"]
    #[inline(always)]
    #[must_use]
    pub fn sensor_bit_width(&mut self) -> SENSOR_BIT_WIDTH_W<0> {
        SENSOR_BIT_WIDTH_W::new(self)
    }
    #[doc = "Bits 3:6 - input color formats: 0010b: 24bit: RGB888 0100b: 16bit: RGB565 0111b: 16bit: YCbCr422 (Y0 Cb Y1 Cr, each 8-bit) YUV YCrCb Note: YUV420 is not supported. 1000b: 24bit: YUV444"]
    #[inline(always)]
    #[must_use]
    pub fn color_formats(&mut self) -> COLOR_FORMATS_W<3> {
        COLOR_FORMATS_W::new(self)
    }
    #[doc = "Bits 10:11 - 00: Normal Mode (one plane mode) 01: Two Plane Mode (Y, UV plane) 10: Y-only Mode, byte sequence as Y0,Y1,Y2,Y3 11: Binary Mode, bit sequence is from LSB to MSB when CR20\\[BIG_END\\]=0"]
    #[inline(always)]
    #[must_use]
    pub fn storage_mode(&mut self) -> STORAGE_MODE_W<10> {
        STORAGE_MODE_W::new(self)
    }
    #[doc = "Bit 15 - Invert Data Input. This bit enables or disables internal inverters on the data lines. 0 CAM_D data lines are directly applied to internal circuitry 1 CAM_D data lines are inverted before applied to internal circuitry"]
    #[inline(always)]
    #[must_use]
    pub fn inv_data(&mut self) -> INV_DATA_W<15> {
        INV_DATA_W::new(self)
    }
    #[doc = "Bit 17 - SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt. 0 SOF interrupt is generated on SOF falling edge 1 SOF interrupt is generated on SOF rising edge"]
    #[inline(always)]
    #[must_use]
    pub fn sof_int_pol(&mut self) -> SOF_INT_POL_W<17> {
        SOF_INT_POL_W::new(self)
    }
    #[doc = "Bit 19 - Synchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO on every SOF."]
    #[inline(always)]
    #[must_use]
    pub fn sync_rxfifo_clr(&mut self) -> SYNC_RXFIFO_CLR_W<19> {
        SYNC_RXFIFO_CLR_W::new(self)
    }
    #[doc = "Bit 20 - ASynchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO immediately. It will be auto-cleared."]
    #[inline(always)]
    #[must_use]
    pub fn async_rxfifo_clr(&mut self) -> ASYNC_RXFIFO_CLR_W<20> {
        ASYNC_RXFIFO_CLR_W::new(self)
    }
    #[doc = "Bit 23 - force to restart the bus pointer at the every end of the sof period, and at the same time, clr the fifo pointer"]
    #[inline(always)]
    #[must_use]
    pub fn restart_busptr(&mut self) -> RESTART_BUSPTR_W<23> {
        RESTART_BUSPTR_W::new(self)
    }
    #[doc = "Bit 24 - Data Packing Direction. This bit Controls how 8-bit/10-bit image data is packed into 32-bit RX FIFO. 0 Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. 1 Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn pack_dir(&mut self) -> PACK_DIR_W<24> {
        PACK_DIR_W::new(self)
    }
    #[doc = "Bit 25 - SWAP 16-Bit Enable. This bit enables the swapping of 16-bit data. Data is packed from 8-bit or 10-bit to 32-bit first (according to the setting of PACK_DIR) and then swapped as 16-bit words before being put into the RX FIFO. The action of the bit only affects the RX FIFO. NOTE: Example of swapping enabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x 33441122 NOTE: Example of swapping disabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x11223344 0 Disable swapping 1 Enable swapping"]
    #[inline(always)]
    #[must_use]
    pub fn swap16_en(&mut self) -> SWAP16_EN_W<25> {
        SWAP16_EN_W::new(self)
    }
    #[doc = "Bit 26 - invert vsync pad input before it is used"]
    #[inline(always)]
    #[must_use]
    pub fn inv_vsync(&mut self) -> INV_VSYNC_W<26> {
        INV_VSYNC_W::new(self)
    }
    #[doc = "Bit 27 - invert hsync pad input before it is used"]
    #[inline(always)]
    #[must_use]
    pub fn inv_hsync(&mut self) -> INV_HSYNC_W<27> {
        INV_HSYNC_W::new(self)
    }
    #[doc = "Bit 28 - invert pixclk pad input before it is used"]
    #[inline(always)]
    #[must_use]
    pub fn inv_pixclk(&mut self) -> INV_PIXCLK_W<28> {
        INV_PIXCLK_W::new(self)
    }
    #[doc = "Bit 29 - If asserted, will change the output color to ARGB8888 mode. Used by input color as RGB565, RGB888, YUV888, etc. The byte sequence is B,G,R,A. Depends on correct CR2\\[ClrBitFormat\\]
configuration."]
    #[inline(always)]
    #[must_use]
    pub fn color_ext(&mut self) -> COLOR_EXT_W<29> {
        COLOR_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
