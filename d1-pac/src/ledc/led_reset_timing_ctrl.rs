#[doc = "Register `LED_RESET_TIMING_CTRL` reader"]
pub struct R(crate::R<LED_RESET_TIMING_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LED_RESET_TIMING_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LED_RESET_TIMING_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LED_RESET_TIMING_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LED_RESET_TIMING_CTRL` writer"]
pub struct W(crate::W<LED_RESET_TIMING_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LED_RESET_TIMING_CTRL_SPEC>;
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
impl From<crate::W<LED_RESET_TIMING_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LED_RESET_TIMING_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TR_TIME` reader - "]
pub struct TR_TIME_R(crate::FieldReader<u16>);
impl TR_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TR_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR_TIME_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR_TIME` writer - "]
pub struct TR_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 16)) | ((value as u32 & 0x1fff) << 16);
        self.w
    }
}
#[doc = "Field `LED_NUM` reader - "]
pub struct LED_NUM_R(crate::FieldReader<u16>);
impl LED_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LED_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LED_NUM_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LED_NUM` writer - "]
pub struct LED_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn tr_time(&self) -> TR_TIME_R {
        TR_TIME_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn led_num(&self) -> LED_NUM_R {
        LED_NUM_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn tr_time(&mut self) -> TR_TIME_W {
        TR_TIME_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn led_num(&mut self) -> LED_NUM_W {
        LED_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC Reset Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [led_reset_timing_ctrl](index.html) module"]
pub struct LED_RESET_TIMING_CTRL_SPEC;
impl crate::RegisterSpec for LED_RESET_TIMING_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [led_reset_timing_ctrl::R](R) reader structure"]
impl crate::Readable for LED_RESET_TIMING_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [led_reset_timing_ctrl::W](W) writer structure"]
impl crate::Writable for LED_RESET_TIMING_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LED_RESET_TIMING_CTRL to value 0"]
impl crate::Resettable for LED_RESET_TIMING_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
