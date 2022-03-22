#[doc = "Register `LEDC_CTRL` reader"]
pub struct R(crate::R<LEDC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEDC_CTRL` writer"]
pub struct W(crate::W<LEDC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEDC_CTRL_SPEC>;
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
impl From<crate::W<LEDC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEDC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOTAL_DATA_LENGTH` reader - "]
pub struct TOTAL_DATA_LENGTH_R(crate::FieldReader<u16, u16>);
impl TOTAL_DATA_LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOTAL_DATA_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOTAL_DATA_LENGTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOTAL_DATA_LENGTH` writer - "]
pub struct TOTAL_DATA_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTAL_DATA_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 16)) | ((value as u32 & 0x1fff) << 16);
        self.w
    }
}
#[doc = "Field `RESET_LED_EN` reader - "]
pub struct RESET_LED_EN_R(crate::FieldReader<bool, bool>);
impl RESET_LED_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_LED_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_LED_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_LED_EN` writer - "]
pub struct RESET_LED_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_LED_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LED_RGB_MODE_A {
    #[doc = "0: `0`"]
    GRB = 0,
    #[doc = "1: `1`"]
    GBR = 1,
    #[doc = "2: `10`"]
    RGB = 2,
    #[doc = "3: `11`"]
    RBG = 3,
    #[doc = "4: `100`"]
    BGR = 4,
    #[doc = "5: `101`"]
    BRG = 5,
}
impl From<LED_RGB_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LED_RGB_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LED_RGB_MODE` reader - "]
pub struct LED_RGB_MODE_R(crate::FieldReader<u8, LED_RGB_MODE_A>);
impl LED_RGB_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LED_RGB_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LED_RGB_MODE_A> {
        match self.bits {
            0 => Some(LED_RGB_MODE_A::GRB),
            1 => Some(LED_RGB_MODE_A::GBR),
            2 => Some(LED_RGB_MODE_A::RGB),
            3 => Some(LED_RGB_MODE_A::RBG),
            4 => Some(LED_RGB_MODE_A::BGR),
            5 => Some(LED_RGB_MODE_A::BRG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GRB`"]
    #[inline(always)]
    pub fn is_grb(&self) -> bool {
        **self == LED_RGB_MODE_A::GRB
    }
    #[doc = "Checks if the value of the field is `GBR`"]
    #[inline(always)]
    pub fn is_gbr(&self) -> bool {
        **self == LED_RGB_MODE_A::GBR
    }
    #[doc = "Checks if the value of the field is `RGB`"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        **self == LED_RGB_MODE_A::RGB
    }
    #[doc = "Checks if the value of the field is `RBG`"]
    #[inline(always)]
    pub fn is_rbg(&self) -> bool {
        **self == LED_RGB_MODE_A::RBG
    }
    #[doc = "Checks if the value of the field is `BGR`"]
    #[inline(always)]
    pub fn is_bgr(&self) -> bool {
        **self == LED_RGB_MODE_A::BGR
    }
    #[doc = "Checks if the value of the field is `BRG`"]
    #[inline(always)]
    pub fn is_brg(&self) -> bool {
        **self == LED_RGB_MODE_A::BRG
    }
}
impl core::ops::Deref for LED_RGB_MODE_R {
    type Target = crate::FieldReader<u8, LED_RGB_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LED_RGB_MODE` writer - "]
pub struct LED_RGB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_RGB_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LED_RGB_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn grb(self) -> &'a mut W {
        self.variant(LED_RGB_MODE_A::GRB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn gbr(self) -> &'a mut W {
        self.variant(LED_RGB_MODE_A::GBR)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut W {
        self.variant(LED_RGB_MODE_A::RGB)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rbg(self) -> &'a mut W {
        self.variant(LED_RGB_MODE_A::RBG)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn bgr(self) -> &'a mut W {
        self.variant(LED_RGB_MODE_A::BGR)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn brg(self) -> &'a mut W {
        self.variant(LED_RGB_MODE_A::BRG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LED_MSB__A {
    #[doc = "0: `0`"]
    LSB = 0,
    #[doc = "1: `1`"]
    MSB = 1,
}
impl From<LED_MSB__A> for bool {
    #[inline(always)]
    fn from(variant: LED_MSB__A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `LED_MSB_(0-3)` reader - "]
pub struct LED_MSB__R(crate::FieldReader<bool, LED_MSB__A>);
impl LED_MSB__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LED_MSB__R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LED_MSB__A {
        match self.bits {
            false => LED_MSB__A::LSB,
            true => LED_MSB__A::MSB,
        }
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        **self == LED_MSB__A::LSB
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        **self == LED_MSB__A::MSB
    }
}
impl core::ops::Deref for LED_MSB__R {
    type Target = crate::FieldReader<bool, LED_MSB__A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `LED_MSB_(0-3)` writer - "]
pub struct LED_MSB__W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> LED_MSB__W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LED_MSB__A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(LED_MSB__A::LSB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(LED_MSB__A::MSB)
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
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | ((value as u32 & 0x01) << self.offset);
        self.w
    }
}
#[doc = "Fields `LED_MSB_(0-3)` const generic writer - "]
pub struct LED_MSB__CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> LED_MSB__CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LED_MSB__A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(LED_MSB__A::LSB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(LED_MSB__A::MSB)
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
        self.w.bits = (self.w.bits & !(0x01 << O)) | ((value as u32 & 0x01) << O);
        self.w
    }
}
#[doc = "Field `LEDC_SOFT_RESET` reader - "]
pub struct LEDC_SOFT_RESET_R(crate::FieldReader<bool, bool>);
impl LEDC_SOFT_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LEDC_SOFT_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEDC_SOFT_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEDC_SOFT_RESET` writer - "]
pub struct LEDC_SOFT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_SOFT_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEDC_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<LEDC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LEDC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDC_EN` reader - "]
pub struct LEDC_EN_R(crate::FieldReader<bool, LEDC_EN_A>);
impl LEDC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LEDC_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEDC_EN_A {
        match self.bits {
            false => LEDC_EN_A::DISABLE,
            true => LEDC_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == LEDC_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == LEDC_EN_A::ENABLE
    }
}
impl core::ops::Deref for LEDC_EN_R {
    type Target = crate::FieldReader<bool, LEDC_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEDC_EN` writer - "]
pub struct LEDC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEDC_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LEDC_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LEDC_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn total_data_length(&self) -> TOTAL_DATA_LENGTH_R {
        TOTAL_DATA_LENGTH_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reset_led_en(&self) -> RESET_LED_EN_R {
        RESET_LED_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn led_rgb_mode(&self) -> LED_RGB_MODE_R {
        LED_RGB_MODE_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = ""]
    #[inline(always)]
    pub unsafe fn led_msb_(&self, n: usize) -> LED_MSB__R {
        LED_MSB__R::new(((self.bits >> (n + 2)) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn led_msb_b(&self) -> LED_MSB__R {
        LED_MSB__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn led_msb_r(&self) -> LED_MSB__R {
        LED_MSB__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn led_msb_g(&self) -> LED_MSB__R {
        LED_MSB__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn led_msb_top(&self) -> LED_MSB__R {
        LED_MSB__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ledc_soft_reset(&self) -> LEDC_SOFT_RESET_R {
        LEDC_SOFT_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ledc_en(&self) -> LEDC_EN_R {
        LEDC_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn total_data_length(&mut self) -> TOTAL_DATA_LENGTH_W {
        TOTAL_DATA_LENGTH_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reset_led_en(&mut self) -> RESET_LED_EN_W {
        RESET_LED_EN_W { w: self }
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn led_rgb_mode(&mut self) -> LED_RGB_MODE_W {
        LED_RGB_MODE_W { w: self }
    }
    #[doc = ""]
    #[inline(always)]
    pub unsafe fn led_msb_(&mut self, n: usize) -> LED_MSB__W {
        LED_MSB__W {
            w: self,
            offset: n + 2,
        }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn led_msb_b(&mut self) -> LED_MSB__CGW<2> {
        LED_MSB__CGW { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn led_msb_r(&mut self) -> LED_MSB__CGW<3> {
        LED_MSB__CGW { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn led_msb_g(&mut self) -> LED_MSB__CGW<4> {
        LED_MSB__CGW { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn led_msb_top(&mut self) -> LED_MSB__CGW<5> {
        LED_MSB__CGW { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ledc_soft_reset(&mut self) -> LEDC_SOFT_RESET_W {
        LEDC_SOFT_RESET_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ledc_en(&mut self) -> LEDC_EN_W {
        LEDC_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_ctrl](index.html) module"]
pub struct LEDC_CTRL_SPEC;
impl crate::RegisterSpec for LEDC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledc_ctrl::R](R) reader structure"]
impl crate::Readable for LEDC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ledc_ctrl::W](W) writer structure"]
impl crate::Writable for LEDC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LEDC_CTRL to value 0"]
impl crate::Resettable for LEDC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
