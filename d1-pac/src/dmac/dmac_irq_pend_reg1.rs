#[doc = "Register `DMAC_IRQ_PEND_REG1` reader"]
pub struct R(crate::R<DMAC_IRQ_PEND_REG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_IRQ_PEND_REG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_IRQ_PEND_REG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_IRQ_PEND_REG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_IRQ_PEND_REG1` writer"]
pub struct W(crate::W<DMAC_IRQ_PEND_REG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_IRQ_PEND_REG1_SPEC>;
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
impl From<crate::W<DMAC_IRQ_PEND_REG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_IRQ_PEND_REG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "The IRQ pending bit for the half package interrupt of DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_HLAF_IRQ_PEND_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<DMA_HLAF_IRQ_PEND_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_HLAF_IRQ_PEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `DMA(8-15)_HLAF_IRQ_PEND` reader - The IRQ pending bit for the half package interrupt of DMA"]
pub struct DMA_HLAF_IRQ_PEND_R(crate::FieldReader<bool>);
impl DMA_HLAF_IRQ_PEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_HLAF_IRQ_PEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_HLAF_IRQ_PEND_A {
        match self.bits {
            false => DMA_HLAF_IRQ_PEND_A::NO_EFFECT,
            true => DMA_HLAF_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == DMA_HLAF_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == DMA_HLAF_IRQ_PEND_A::PENDING
    }
}
impl core::ops::Deref for DMA_HLAF_IRQ_PEND_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `DMA(8-15)_HLAF_IRQ_PEND` const generic writer - The IRQ pending bit for the half package interrupt of DMA"]
pub struct DMA_HLAF_IRQ_PEND_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> DMA_HLAF_IRQ_PEND_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_HLAF_IRQ_PEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMA_HLAF_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(DMA_HLAF_IRQ_PEND_A::PENDING)
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
#[doc = "The IRQ pending bit for the package end interrupt of DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_PKG_IRQ_PEND_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<DMA_PKG_IRQ_PEND_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_PKG_IRQ_PEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `DMA(8-15)_PKG_IRQ_PEND` reader - The IRQ pending bit for the package end interrupt of DMA"]
pub struct DMA_PKG_IRQ_PEND_R(crate::FieldReader<bool>);
impl DMA_PKG_IRQ_PEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_PKG_IRQ_PEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_PKG_IRQ_PEND_A {
        match self.bits {
            false => DMA_PKG_IRQ_PEND_A::NO_EFFECT,
            true => DMA_PKG_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == DMA_PKG_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == DMA_PKG_IRQ_PEND_A::PENDING
    }
}
impl core::ops::Deref for DMA_PKG_IRQ_PEND_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `DMA(8-15)_PKG_IRQ_PEND` const generic writer - The IRQ pending bit for the package end interrupt of DMA"]
pub struct DMA_PKG_IRQ_PEND_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> DMA_PKG_IRQ_PEND_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_PKG_IRQ_PEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMA_PKG_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(DMA_PKG_IRQ_PEND_A::PENDING)
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
#[doc = "The IRQ pending bit for the queue end interrupt of DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_QUEUE_IRQ_PEND_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<DMA_QUEUE_IRQ_PEND_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_QUEUE_IRQ_PEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `DMA(8-15)_QUEUE_IRQ_PEND` reader - The IRQ pending bit for the queue end interrupt of DMA"]
pub struct DMA_QUEUE_IRQ_PEND_R(crate::FieldReader<bool>);
impl DMA_QUEUE_IRQ_PEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_QUEUE_IRQ_PEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_QUEUE_IRQ_PEND_A {
        match self.bits {
            false => DMA_QUEUE_IRQ_PEND_A::NO_EFFECT,
            true => DMA_QUEUE_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == DMA_QUEUE_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == DMA_QUEUE_IRQ_PEND_A::PENDING
    }
}
impl core::ops::Deref for DMA_QUEUE_IRQ_PEND_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `DMA(8-15)_QUEUE_IRQ_PEND` const generic writer - The IRQ pending bit for the queue end interrupt of DMA"]
pub struct DMA_QUEUE_IRQ_PEND_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> DMA_QUEUE_IRQ_PEND_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_QUEUE_IRQ_PEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMA_QUEUE_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(DMA_QUEUE_IRQ_PEND_A::PENDING)
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
    #[doc = "The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_hlaf_irq_pend(&self, n: usize) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> ((n - 8) * 4)) & 1) != 0)
    }
    #[doc = "Bit 0 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma8_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma9_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma10_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma11_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma12_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma13_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma14_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma15_hlaf_irq_pend(&self) -> DMA_HLAF_IRQ_PEND_R {
        DMA_HLAF_IRQ_PEND_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_pkg_irq_pend(&self, n: usize) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> ((n - 8) * 4 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma8_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma9_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma10_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma11_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma12_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma13_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma14_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma15_pkg_irq_pend(&self) -> DMA_PKG_IRQ_PEND_R {
        DMA_PKG_IRQ_PEND_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_queue_irq_pend(&self, n: usize) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> ((n - 8) * 4 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma8_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma9_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma10_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma11_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma12_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma13_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma14_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma15_queue_irq_pend(&self) -> DMA_QUEUE_IRQ_PEND_R {
        DMA_QUEUE_IRQ_PEND_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_hlaf_irq_pend<const O: usize>(&mut self) -> DMA_HLAF_IRQ_PEND_W<O> {
        DMA_HLAF_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 0 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma8_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<0> {
        DMA_HLAF_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 4 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma9_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<4> {
        DMA_HLAF_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 8 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma10_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<8> {
        DMA_HLAF_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 12 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma11_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<12> {
        DMA_HLAF_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 16 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma12_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<16> {
        DMA_HLAF_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 20 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma13_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<20> {
        DMA_HLAF_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 24 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma14_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<24> {
        DMA_HLAF_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 28 - The IRQ pending bit for the half package interrupt of DMA"]
    #[inline(always)]
    pub fn dma15_hlaf_irq_pend(&mut self) -> DMA_HLAF_IRQ_PEND_W<28> {
        DMA_HLAF_IRQ_PEND_W { w: self }
    }
    #[doc = "The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_pkg_irq_pend<const O: usize>(&mut self) -> DMA_PKG_IRQ_PEND_W<O> {
        DMA_PKG_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 1 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma8_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<1> {
        DMA_PKG_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 5 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma9_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<5> {
        DMA_PKG_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 9 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma10_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<9> {
        DMA_PKG_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 13 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma11_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<13> {
        DMA_PKG_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 17 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma12_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<17> {
        DMA_PKG_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 21 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma13_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<21> {
        DMA_PKG_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 25 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma14_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<25> {
        DMA_PKG_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 29 - The IRQ pending bit for the package end interrupt of DMA"]
    #[inline(always)]
    pub fn dma15_pkg_irq_pend(&mut self) -> DMA_PKG_IRQ_PEND_W<29> {
        DMA_PKG_IRQ_PEND_W { w: self }
    }
    #[doc = "The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub unsafe fn dma_queue_irq_pend<const O: usize>(&mut self) -> DMA_QUEUE_IRQ_PEND_W<O> {
        DMA_QUEUE_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 2 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma8_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<2> {
        DMA_QUEUE_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 6 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma9_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<6> {
        DMA_QUEUE_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 10 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma10_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<10> {
        DMA_QUEUE_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 14 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma11_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<14> {
        DMA_QUEUE_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 18 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma12_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<18> {
        DMA_QUEUE_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 22 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma13_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<22> {
        DMA_QUEUE_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 26 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma14_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<26> {
        DMA_QUEUE_IRQ_PEND_W { w: self }
    }
    #[doc = "Bit 30 - The IRQ pending bit for the queue end interrupt of DMA"]
    #[inline(always)]
    pub fn dma15_queue_irq_pend(&mut self) -> DMA_QUEUE_IRQ_PEND_W<30> {
        DMA_QUEUE_IRQ_PEND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC IRQ Pending Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_irq_pend_reg1](index.html) module"]
pub struct DMAC_IRQ_PEND_REG1_SPEC;
impl crate::RegisterSpec for DMAC_IRQ_PEND_REG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_irq_pend_reg1::R](R) reader structure"]
impl crate::Readable for DMAC_IRQ_PEND_REG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_irq_pend_reg1::W](W) writer structure"]
impl crate::Writable for DMAC_IRQ_PEND_REG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_IRQ_PEND_REG1 to value 0"]
impl crate::Resettable for DMAC_IRQ_PEND_REG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
