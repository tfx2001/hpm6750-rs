#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_RST` reader - Software reset, high active. When write 1 ,all internal logical will be reset. 0b - No action 1b - All LCDC internal registers are forced into their reset state. Interface registers are not affected."]
pub type SW_RST_R = crate::BitReader<bool>;
#[doc = "Field `SW_RST` writer - Software reset, high active. When write 1 ,all internal logical will be reset. 0b - No action 1b - All LCDC internal registers are forced into their reset state. Interface registers are not affected."]
pub type SW_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DISP_ON` reader - Display panel On/Off mode. 0b - Display Off. 1b - Display On. Display can be set off at any time, but it can only be set on after VS_BLANK status is asserted. So a good procedure to stop and turn on the display is: 1) clr VS_BLANK status 2) assert software reset 3) de-assert software reset 4) set display off 5) check VS_BLANK status until it is asserted, 6)reset the module, change settings 7) set display on"]
pub type DISP_ON_R = crate::BitReader<bool>;
#[doc = "Field `DISP_ON` writer - Display panel On/Off mode. 0b - Display Off. 1b - Display On. Display can be set off at any time, but it can only be set on after VS_BLANK status is asserted. So a good procedure to stop and turn on the display is: 1) clr VS_BLANK status 2) assert software reset 3) de-assert software reset 4) set display off 5) check VS_BLANK status until it is asserted, 6)reset the module, change settings 7) set display on"]
pub type DISP_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LINE_PATTERN` reader - LCDIF line output order. 000b - RGB. 001b - RBG. 010b - GBR. 011b - GRB. 100b - BRG. 101b - BGR."]
pub type LINE_PATTERN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINE_PATTERN` writer - LCDIF line output order. 000b - RGB. 001b - RBG. 010b - GBR. 011b - GRB. 100b - BRG. 101b - BGR."]
pub type LINE_PATTERN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DISP_MODE` reader - LCDIF operating mode. 00b - Normal mode. Panel content controlled by layer configuration. 01b - Test Mode1.(BGND Color Display) 10b - Test Mode2.(Column Color Bar) 11b - Test Mode3.(Row Color Bar)"]
pub type DISP_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DISP_MODE` writer - LCDIF operating mode. 00b - Normal mode. Panel content controlled by layer configuration. 01b - Test Mode1.(BGND Color Display) 10b - Test Mode2.(Column Color Bar) 11b - Test Mode3.(Row Color Bar)"]
pub type DISP_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `BGDCL4CLR` reader - background color for clear mode when the alpha channel is 0"]
pub type BGDCL4CLR_R = crate::BitReader<bool>;
#[doc = "Field `BGDCL4CLR` writer - background color for clear mode when the alpha channel is 0"]
pub type BGDCL4CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ARQOS` reader - ARQOS for bus fabric arbitration"]
pub type ARQOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARQOS` writer - ARQOS for bus fabric arbitration"]
pub type ARQOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `INV_PXDATA` reader - Indicates if value at the output (pixel data output) needs to be negated. 0b - Output is to remain same as the data inside memory 1b - Output to be negated from the data inside memory"]
pub type INV_PXDATA_R = crate::BitReader<bool>;
#[doc = "Field `INV_PXDATA` writer - Indicates if value at the output (pixel data output) needs to be negated. 0b - Output is to remain same as the data inside memory 1b - Output to be negated from the data inside memory"]
pub type INV_PXDATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `INV_PXCLK` reader - Polarity change of Pixel Clock. 0b - LCDC outputs data on the rising edge, and Display samples data on the falling edge 1b - LCDC outputs data on the falling edge, Display samples data on the rising edge"]
pub type INV_PXCLK_R = crate::BitReader<bool>;
#[doc = "Field `INV_PXCLK` writer - Polarity change of Pixel Clock. 0b - LCDC outputs data on the rising edge, and Display samples data on the falling edge 1b - LCDC outputs data on the falling edge, Display samples data on the rising edge"]
pub type INV_PXCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `INV_HREF` reader - Polarity of HREF 0b - HREF signal active HIGH, indicating active pixel data 1b - HREF signal active LOW"]
pub type INV_HREF_R = crate::BitReader<bool>;
#[doc = "Field `INV_HREF` writer - Polarity of HREF 0b - HREF signal active HIGH, indicating active pixel data 1b - HREF signal active LOW"]
pub type INV_HREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `INV_VSYNC` reader - Polarity of VSYNC 0b - VSYNC signal active HIGH 1b - VSYNC signal active LOW"]
pub type INV_VSYNC_R = crate::BitReader<bool>;
#[doc = "Field `INV_VSYNC` writer - Polarity of VSYNC 0b - VSYNC signal active HIGH 1b - VSYNC signal active LOW"]
pub type INV_VSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `INV_HSYNC` reader - Polarity of HSYNC 0b - HSYNC signal active HIGH 1b - HSYNC signal active LOW"]
pub type INV_HSYNC_R = crate::BitReader<bool>;
#[doc = "Field `INV_HSYNC` writer - Polarity of HSYNC 0b - HSYNC signal active HIGH 1b - HSYNC signal active LOW"]
pub type INV_HSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Software reset, high active. When write 1 ,all internal logical will be reset. 0b - No action 1b - All LCDC internal registers are forced into their reset state. Interface registers are not affected."]
    #[inline(always)]
    pub fn sw_rst(&self) -> SW_RST_R {
        SW_RST_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Display panel On/Off mode. 0b - Display Off. 1b - Display On. Display can be set off at any time, but it can only be set on after VS_BLANK status is asserted. So a good procedure to stop and turn on the display is: 1) clr VS_BLANK status 2) assert software reset 3) de-assert software reset 4) set display off 5) check VS_BLANK status until it is asserted, 6)reset the module, change settings 7) set display on"]
    #[inline(always)]
    pub fn disp_on(&self) -> DISP_ON_R {
        DISP_ON_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 27:29 - LCDIF line output order. 000b - RGB. 001b - RBG. 010b - GBR. 011b - GRB. 100b - BRG. 101b - BGR."]
    #[inline(always)]
    pub fn line_pattern(&self) -> LINE_PATTERN_R {
        LINE_PATTERN_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 25:26 - LCDIF operating mode. 00b - Normal mode. Panel content controlled by layer configuration. 01b - Test Mode1.(BGND Color Display) 10b - Test Mode2.(Column Color Bar) 11b - Test Mode3.(Row Color Bar)"]
    #[inline(always)]
    pub fn disp_mode(&self) -> DISP_MODE_R {
        DISP_MODE_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 24 - background color for clear mode when the alpha channel is 0"]
    #[inline(always)]
    pub fn bgdcl4clr(&self) -> BGDCL4CLR_R {
        BGDCL4CLR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 20:23 - ARQOS for bus fabric arbitration"]
    #[inline(always)]
    pub fn arqos(&self) -> ARQOS_R {
        ARQOS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Indicates if value at the output (pixel data output) needs to be negated. 0b - Output is to remain same as the data inside memory 1b - Output to be negated from the data inside memory"]
    #[inline(always)]
    pub fn inv_pxdata(&self) -> INV_PXDATA_R {
        INV_PXDATA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Polarity change of Pixel Clock. 0b - LCDC outputs data on the rising edge, and Display samples data on the falling edge 1b - LCDC outputs data on the falling edge, Display samples data on the rising edge"]
    #[inline(always)]
    pub fn inv_pxclk(&self) -> INV_PXCLK_R {
        INV_PXCLK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Polarity of HREF 0b - HREF signal active HIGH, indicating active pixel data 1b - HREF signal active LOW"]
    #[inline(always)]
    pub fn inv_href(&self) -> INV_HREF_R {
        INV_HREF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Polarity of VSYNC 0b - VSYNC signal active HIGH 1b - VSYNC signal active LOW"]
    #[inline(always)]
    pub fn inv_vsync(&self) -> INV_VSYNC_R {
        INV_VSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Polarity of HSYNC 0b - HSYNC signal active HIGH 1b - HSYNC signal active LOW"]
    #[inline(always)]
    pub fn inv_hsync(&self) -> INV_HSYNC_R {
        INV_HSYNC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Software reset, high active. When write 1 ,all internal logical will be reset. 0b - No action 1b - All LCDC internal registers are forced into their reset state. Interface registers are not affected."]
    #[inline(always)]
    pub fn sw_rst(&mut self) -> SW_RST_W<31> {
        SW_RST_W::new(self)
    }
    #[doc = "Bit 30 - Display panel On/Off mode. 0b - Display Off. 1b - Display On. Display can be set off at any time, but it can only be set on after VS_BLANK status is asserted. So a good procedure to stop and turn on the display is: 1) clr VS_BLANK status 2) assert software reset 3) de-assert software reset 4) set display off 5) check VS_BLANK status until it is asserted, 6)reset the module, change settings 7) set display on"]
    #[inline(always)]
    pub fn disp_on(&mut self) -> DISP_ON_W<30> {
        DISP_ON_W::new(self)
    }
    #[doc = "Bits 27:29 - LCDIF line output order. 000b - RGB. 001b - RBG. 010b - GBR. 011b - GRB. 100b - BRG. 101b - BGR."]
    #[inline(always)]
    pub fn line_pattern(&mut self) -> LINE_PATTERN_W<27> {
        LINE_PATTERN_W::new(self)
    }
    #[doc = "Bits 25:26 - LCDIF operating mode. 00b - Normal mode. Panel content controlled by layer configuration. 01b - Test Mode1.(BGND Color Display) 10b - Test Mode2.(Column Color Bar) 11b - Test Mode3.(Row Color Bar)"]
    #[inline(always)]
    pub fn disp_mode(&mut self) -> DISP_MODE_W<25> {
        DISP_MODE_W::new(self)
    }
    #[doc = "Bit 24 - background color for clear mode when the alpha channel is 0"]
    #[inline(always)]
    pub fn bgdcl4clr(&mut self) -> BGDCL4CLR_W<24> {
        BGDCL4CLR_W::new(self)
    }
    #[doc = "Bits 20:23 - ARQOS for bus fabric arbitration"]
    #[inline(always)]
    pub fn arqos(&mut self) -> ARQOS_W<20> {
        ARQOS_W::new(self)
    }
    #[doc = "Bit 4 - Indicates if value at the output (pixel data output) needs to be negated. 0b - Output is to remain same as the data inside memory 1b - Output to be negated from the data inside memory"]
    #[inline(always)]
    pub fn inv_pxdata(&mut self) -> INV_PXDATA_W<4> {
        INV_PXDATA_W::new(self)
    }
    #[doc = "Bit 3 - Polarity change of Pixel Clock. 0b - LCDC outputs data on the rising edge, and Display samples data on the falling edge 1b - LCDC outputs data on the falling edge, Display samples data on the rising edge"]
    #[inline(always)]
    pub fn inv_pxclk(&mut self) -> INV_PXCLK_W<3> {
        INV_PXCLK_W::new(self)
    }
    #[doc = "Bit 2 - Polarity of HREF 0b - HREF signal active HIGH, indicating active pixel data 1b - HREF signal active LOW"]
    #[inline(always)]
    pub fn inv_href(&mut self) -> INV_HREF_W<2> {
        INV_HREF_W::new(self)
    }
    #[doc = "Bit 1 - Polarity of VSYNC 0b - VSYNC signal active HIGH 1b - VSYNC signal active LOW"]
    #[inline(always)]
    pub fn inv_vsync(&mut self) -> INV_VSYNC_W<1> {
        INV_VSYNC_W::new(self)
    }
    #[doc = "Bit 0 - Polarity of HSYNC 0b - HSYNC signal active HIGH 1b - HSYNC signal active LOW"]
    #[inline(always)]
    pub fn inv_hsync(&mut self) -> INV_HSYNC_W<0> {
        INV_HSYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
