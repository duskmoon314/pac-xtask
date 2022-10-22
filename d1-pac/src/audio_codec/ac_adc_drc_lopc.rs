#[doc = "Register `ac_adc_drc_lopc` reader"]
pub struct R(crate::R<AC_ADC_DRC_LOPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_ADC_DRC_LOPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_ADC_DRC_LOPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_ADC_DRC_LOPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_adc_drc_lopc` writer"]
pub struct W(crate::W<AC_ADC_DRC_LOPC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_ADC_DRC_LOPC_SPEC>;
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
impl From<crate::W<AC_ADC_DRC_LOPC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_ADC_DRC_LOPC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_drc_lopc` reader - The output of the compressor, which is determined by the equation OPC/6.0206. The format is 8.24. (The default value is -40 dB)"]
pub type ADC_DRC_LOPC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `adc_drc_lopc` writer - The output of the compressor, which is determined by the equation OPC/6.0206. The format is 8.24. (The default value is -40 dB)"]
pub type ADC_DRC_LOPC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AC_ADC_DRC_LOPC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The output of the compressor, which is determined by the equation OPC/6.0206. The format is 8.24. (The default value is -40 dB)"]
    #[inline(always)]
    pub fn adc_drc_lopc(&self) -> ADC_DRC_LOPC_R {
        ADC_DRC_LOPC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The output of the compressor, which is determined by the equation OPC/6.0206. The format is 8.24. (The default value is -40 dB)"]
    #[inline(always)]
    pub fn adc_drc_lopc(&mut self) -> ADC_DRC_LOPC_W<0> {
        ADC_DRC_LOPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC DRC Compressor Low Output at Compressor Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_adc_drc_lopc](index.html) module"]
pub struct AC_ADC_DRC_LOPC_SPEC;
impl crate::RegisterSpec for AC_ADC_DRC_LOPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_adc_drc_lopc::R](R) reader structure"]
impl crate::Readable for AC_ADC_DRC_LOPC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_adc_drc_lopc::W](W) writer structure"]
impl crate::Writable for AC_ADC_DRC_LOPC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ac_adc_drc_lopc to value 0x2c3f"]
impl crate::Resettable for AC_ADC_DRC_LOPC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2c3f
    }
}