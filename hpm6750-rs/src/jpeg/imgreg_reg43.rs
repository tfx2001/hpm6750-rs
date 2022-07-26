#[doc = "Register `IMGREG_REG43` reader"]
pub struct R(crate::R<IMGREG_REG43_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMGREG_REG43_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMGREG_REG43_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMGREG_REG43_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMGREG_REG43` writer"]
pub struct W(crate::W<IMGREG_REG43_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMGREG_REG43_SPEC>;
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
impl From<crate::W<IMGREG_REG43_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMGREG_REG43_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NBLOCK` reader - Encoder use only. The number of data units (8x8 blocks of data) of the color componet contained in the MCU minus 1."]
pub type NBLOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBLOCK` writer - Encoder use only. The number of data units (8x8 blocks of data) of the color componet contained in the MCU minus 1."]
pub type NBLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMGREG_REG43_SPEC, u8, u8, 4, O>;
#[doc = "Field `QT` reader - Encoder use only. The selection of the quantization table."]
pub type QT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QT` writer - Encoder use only. The selection of the quantization table."]
pub type QT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMGREG_REG43_SPEC, u8, u8, 2, O>;
#[doc = "Field `HA` reader - Encoder use only. The selection of the Huffman table for the encoding of the AC coefficients in the data units belonging to the color component."]
pub type HA_R = crate::BitReader<bool>;
#[doc = "Field `HA` writer - Encoder use only. The selection of the Huffman table for the encoding of the AC coefficients in the data units belonging to the color component."]
pub type HA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMGREG_REG43_SPEC, bool, O>;
#[doc = "Field `HD` reader - Encoder use only. The selection of the Huffman table for the encoding of the DC coefficients in the data units belonging to the color component."]
pub type HD_R = crate::BitReader<bool>;
#[doc = "Field `HD` writer - Encoder use only. The selection of the Huffman table for the encoding of the DC coefficients in the data units belonging to the color component."]
pub type HD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMGREG_REG43_SPEC, bool, O>;
impl R {
    #[doc = "Bits 4:7 - Encoder use only. The number of data units (8x8 blocks of data) of the color componet contained in the MCU minus 1."]
    #[inline(always)]
    pub fn nblock(&self) -> NBLOCK_R {
        NBLOCK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - Encoder use only. The selection of the quantization table."]
    #[inline(always)]
    pub fn qt(&self) -> QT_R {
        QT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 1 - Encoder use only. The selection of the Huffman table for the encoding of the AC coefficients in the data units belonging to the color component."]
    #[inline(always)]
    pub fn ha(&self) -> HA_R {
        HA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Encoder use only. The selection of the Huffman table for the encoding of the DC coefficients in the data units belonging to the color component."]
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Encoder use only. The number of data units (8x8 blocks of data) of the color componet contained in the MCU minus 1."]
    #[inline(always)]
    pub fn nblock(&mut self) -> NBLOCK_W<4> {
        NBLOCK_W::new(self)
    }
    #[doc = "Bits 2:3 - Encoder use only. The selection of the quantization table."]
    #[inline(always)]
    pub fn qt(&mut self) -> QT_W<2> {
        QT_W::new(self)
    }
    #[doc = "Bit 1 - Encoder use only. The selection of the Huffman table for the encoding of the AC coefficients in the data units belonging to the color component."]
    #[inline(always)]
    pub fn ha(&mut self) -> HA_W<1> {
        HA_W::new(self)
    }
    #[doc = "Bit 0 - Encoder use only. The selection of the Huffman table for the encoding of the DC coefficients in the data units belonging to the color component."]
    #[inline(always)]
    pub fn hd(&mut self) -> HD_W<0> {
        HD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Image Control Register 43\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imgreg_reg43](index.html) module"]
pub struct IMGREG_REG43_SPEC;
impl crate::RegisterSpec for IMGREG_REG43_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imgreg_reg43::R](R) reader structure"]
impl crate::Readable for IMGREG_REG43_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imgreg_reg43::W](W) writer structure"]
impl crate::Writable for IMGREG_REG43_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMGREG_REG43 to value 0"]
impl crate::Resettable for IMGREG_REG43_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
