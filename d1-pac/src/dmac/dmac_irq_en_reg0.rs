#[doc = "Register `DMAC_IRQ_EN_REG0` reader"]
pub struct R(crate::R<DMAC_IRQ_EN_REG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_IRQ_EN_REG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_IRQ_EN_REG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_IRQ_EN_REG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_IRQ_EN_REG0` writer"]
pub struct W(crate::W<DMAC_IRQ_EN_REG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_IRQ_EN_REG0_SPEC>;
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
impl From<crate::W<DMAC_IRQ_EN_REG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_IRQ_EN_REG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable the half package interrupt of DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_HLAF_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<DMA_HLAF_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_HLAF_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `DMA(0-7)_HLAF_IRQ_EN` reader - Enable the half package interrupt of DMA"]
pub struct DMA_HLAF_IRQ_EN_R(crate::FieldReader<bool>);
impl DMA_HLAF_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_HLAF_IRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_HLAF_IRQ_EN_A {
        match self.bits {
            false => DMA_HLAF_IRQ_EN_A::DISABLED,
            true => DMA_HLAF_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMA_HLAF_IRQ_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMA_HLAF_IRQ_EN_A::ENABLED
    }
}
impl core::ops::Deref for DMA_HLAF_IRQ_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `DMA(0-7)_HLAF_IRQ_EN` const generic writer - Enable the half package interrupt of DMA"]
pub struct DMA_HLAF_IRQ_EN_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> DMA_HLAF_IRQ_EN_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_HLAF_IRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_HLAF_IRQ_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_HLAF_IRQ_EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << O)) | ((value as u32 & 1) << O);
        self.w
    }
}
#[doc = "Enable the package end interrupt of DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_PKG_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<DMA_PKG_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_PKG_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `DMA(0-7)_PKG_IRQ_EN` reader - Enable the package end interrupt of DMA"]
pub struct DMA_PKG_IRQ_EN_R(crate::FieldReader<bool>);
impl DMA_PKG_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_PKG_IRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_PKG_IRQ_EN_A {
        match self.bits {
            false => DMA_PKG_IRQ_EN_A::DISABLED,
            true => DMA_PKG_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMA_PKG_IRQ_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMA_PKG_IRQ_EN_A::ENABLED
    }
}
impl core::ops::Deref for DMA_PKG_IRQ_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `DMA(0-7)_PKG_IRQ_EN` const generic writer - Enable the package end interrupt of DMA"]
pub struct DMA_PKG_IRQ_EN_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> DMA_PKG_IRQ_EN_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_PKG_IRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_PKG_IRQ_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_PKG_IRQ_EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << O)) | ((value as u32 & 1) << O);
        self.w
    }
}
#[doc = "Enable the queue end interrupt of DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_QUEUE_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<DMA_QUEUE_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_QUEUE_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `DMA(0-7)_QUEUE_IRQ_EN` reader - Enable the queue end interrupt of DMA"]
pub struct DMA_QUEUE_IRQ_EN_R(crate::FieldReader<bool>);
impl DMA_QUEUE_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_QUEUE_IRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_QUEUE_IRQ_EN_A {
        match self.bits {
            false => DMA_QUEUE_IRQ_EN_A::DISABLED,
            true => DMA_QUEUE_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMA_QUEUE_IRQ_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMA_QUEUE_IRQ_EN_A::ENABLED
    }
}
impl core::ops::Deref for DMA_QUEUE_IRQ_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `DMA(0-7)_QUEUE_IRQ_EN` const generic writer - Enable the queue end interrupt of DMA"]
pub struct DMA_QUEUE_IRQ_EN_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> DMA_QUEUE_IRQ_EN_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_QUEUE_IRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_QUEUE_IRQ_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_QUEUE_IRQ_EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << O)) | ((value as u32 & 1) << O);
        self.w
    }
}
impl R {
    #[doc = "Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_hlaf_irq_en(&self, n: usize) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma0_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma1_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma2_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma3_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma4_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma5_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma6_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma7_hlaf_irq_en(&self) -> DMA_HLAF_IRQ_EN_R {
        DMA_HLAF_IRQ_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_pkg_irq_en(&self, n: usize) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma0_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma1_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma2_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma3_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma4_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma5_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma6_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma7_pkg_irq_en(&self) -> DMA_PKG_IRQ_EN_R {
        DMA_PKG_IRQ_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_queue_irq_en(&self, n: usize) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma0_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma1_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma2_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma3_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma4_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma5_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma6_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma7_queue_irq_en(&self) -> DMA_QUEUE_IRQ_EN_R {
        DMA_QUEUE_IRQ_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_hlaf_irq_en<const O: usize>(&mut self) -> DMA_HLAF_IRQ_EN_W<O> {
        DMA_HLAF_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 0 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma0_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<0> {
        DMA_HLAF_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 4 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma1_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<4> {
        DMA_HLAF_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 8 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma2_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<8> {
        DMA_HLAF_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 12 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma3_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<12> {
        DMA_HLAF_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 16 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma4_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<16> {
        DMA_HLAF_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 20 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma5_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<20> {
        DMA_HLAF_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 24 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma6_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<24> {
        DMA_HLAF_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 28 - Enable the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma7_hlaf_irq_en(&mut self) -> DMA_HLAF_IRQ_EN_W<28> {
        DMA_HLAF_IRQ_EN_W { w: self }
    }
    #[doc = "Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_pkg_irq_en<const O: usize>(&mut self) -> DMA_PKG_IRQ_EN_W<O> {
        DMA_PKG_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma0_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<1> {
        DMA_PKG_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 5 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma1_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<5> {
        DMA_PKG_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 9 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma2_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<9> {
        DMA_PKG_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 13 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma3_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<13> {
        DMA_PKG_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 17 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma4_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<17> {
        DMA_PKG_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 21 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma5_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<21> {
        DMA_PKG_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 25 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma6_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<25> {
        DMA_PKG_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 29 - Enable the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma7_pkg_irq_en(&mut self) -> DMA_PKG_IRQ_EN_W<29> {
        DMA_PKG_IRQ_EN_W { w: self }
    }
    #[doc = "Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_queue_irq_en<const O: usize>(&mut self) -> DMA_QUEUE_IRQ_EN_W<O> {
        DMA_QUEUE_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma0_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<2> {
        DMA_QUEUE_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 6 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma1_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<6> {
        DMA_QUEUE_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 10 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma2_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<10> {
        DMA_QUEUE_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 14 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma3_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<14> {
        DMA_QUEUE_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 18 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma4_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<18> {
        DMA_QUEUE_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 22 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma5_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<22> {
        DMA_QUEUE_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 26 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma6_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<26> {
        DMA_QUEUE_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 30 - Enable the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma7_queue_irq_en(&mut self) -> DMA_QUEUE_IRQ_EN_W<30> {
        DMA_QUEUE_IRQ_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC IRQ Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_irq_en_reg0](index.html) module"]
pub struct DMAC_IRQ_EN_REG0_SPEC;
impl crate::RegisterSpec for DMAC_IRQ_EN_REG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_irq_en_reg0::R](R) reader structure"]
impl crate::Readable for DMAC_IRQ_EN_REG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_irq_en_reg0::W](W) writer structure"]
impl crate::Writable for DMAC_IRQ_EN_REG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_IRQ_EN_REG0 to value 0"]
impl crate::Resettable for DMAC_IRQ_EN_REG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
