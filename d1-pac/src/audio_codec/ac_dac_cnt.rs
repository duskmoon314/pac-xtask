#[doc = "Register `ac_dac_cnt` reader"]
pub struct R(crate::R<AC_DAC_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_DAC_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_DAC_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_DAC_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_dac_cnt` writer"]
pub struct W(crate::W<AC_DAC_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_DAC_CNT_SPEC>;
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
impl From<crate::W<AC_DAC_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_DAC_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_cnt` reader - TX Sample Counter\n\nThe audio sample number of sending into TXFIFO.\n\nWhen one sample is put into TXFIFO by DMA or by host IO, the TX sample counter register increases by one. The TX sample counter register can be set to any initial valve at any time. After being updated by the initial value, the counter register should count from this initial value."]
pub type TX_CNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `tx_cnt` writer - TX Sample Counter\n\nThe audio sample number of sending into TXFIFO.\n\nWhen one sample is put into TXFIFO by DMA or by host IO, the TX sample counter register increases by one. The TX sample counter register can be set to any initial valve at any time. After being updated by the initial value, the counter register should count from this initial value."]
pub type TX_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AC_DAC_CNT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - TX Sample Counter\n\nThe audio sample number of sending into TXFIFO.\n\nWhen one sample is put into TXFIFO by DMA or by host IO, the TX sample counter register increases by one. The TX sample counter register can be set to any initial valve at any time. After being updated by the initial value, the counter register should count from this initial value."]
    #[inline(always)]
    pub fn tx_cnt(&self) -> TX_CNT_R {
        TX_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TX Sample Counter\n\nThe audio sample number of sending into TXFIFO.\n\nWhen one sample is put into TXFIFO by DMA or by host IO, the TX sample counter register increases by one. The TX sample counter register can be set to any initial valve at any time. After being updated by the initial value, the counter register should count from this initial value."]
    #[inline(always)]
    pub fn tx_cnt(&mut self) -> TX_CNT_W<0> {
        TX_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC TX FIFO Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_dac_cnt](index.html) module"]
pub struct AC_DAC_CNT_SPEC;
impl crate::RegisterSpec for AC_DAC_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_dac_cnt::R](R) reader structure"]
impl crate::Readable for AC_DAC_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_dac_cnt::W](W) writer structure"]
impl crate::Writable for AC_DAC_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ac_dac_cnt to value 0"]
impl crate::Resettable for AC_DAC_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}