#[doc = "Register `ADCMCTL0` reader"]
pub struct R(crate::R<ADCMCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMCTL0` writer"]
pub struct W(crate::W<ADCMCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMCTL0_SPEC>;
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
impl From<crate::W<ADCMCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCINCH` reader - Input channel select"]
pub type ADCINCH_R = crate::FieldReader<u8, ADCINCH_A>;
#[doc = "Input channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCINCH_A {
    #[doc = "0: A0 - see device-specific data sheet"]
    ADCINCH_0 = 0,
    #[doc = "1: A1 - see device-specific data sheet"]
    ADCINCH_1 = 1,
    #[doc = "2: A2 - see device-specific data sheet"]
    ADCINCH_2 = 2,
    #[doc = "3: A3 - see device-specific data sheet"]
    ADCINCH_3 = 3,
    #[doc = "4: A4 - see device-specific data sheet"]
    ADCINCH_4 = 4,
    #[doc = "5: A5 - see device-specific data sheet"]
    ADCINCH_5 = 5,
    #[doc = "6: A2 - see device-specific data sheet"]
    ADCINCH_6 = 6,
    #[doc = "7: A7 - see device-specific data sheet"]
    ADCINCH_7 = 7,
    #[doc = "8: A8 - see device-specific data sheet"]
    ADCINCH_8 = 8,
    #[doc = "9: A9 - see device-specific data sheet"]
    ADCINCH_9 = 9,
    #[doc = "10: A10 - see device-specific data sheet"]
    ADCINCH_10 = 10,
    #[doc = "11: A11 - see device-specific data sheet"]
    ADCINCH_11 = 11,
    #[doc = "12: A12 - see device-specific data sheet"]
    ADCINCH_12 = 12,
    #[doc = "13: A13 - see device-specific data sheet"]
    ADCINCH_13 = 13,
    #[doc = "14: A14 - see device-specific data sheet"]
    ADCINCH_14 = 14,
    #[doc = "15: A15 - see device-specific data sheet"]
    ADCINCH_15 = 15,
}
impl From<ADCINCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCINCH_A) -> Self {
        variant as _
    }
}
impl ADCINCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCINCH_A {
        match self.bits {
            0 => ADCINCH_A::ADCINCH_0,
            1 => ADCINCH_A::ADCINCH_1,
            2 => ADCINCH_A::ADCINCH_2,
            3 => ADCINCH_A::ADCINCH_3,
            4 => ADCINCH_A::ADCINCH_4,
            5 => ADCINCH_A::ADCINCH_5,
            6 => ADCINCH_A::ADCINCH_6,
            7 => ADCINCH_A::ADCINCH_7,
            8 => ADCINCH_A::ADCINCH_8,
            9 => ADCINCH_A::ADCINCH_9,
            10 => ADCINCH_A::ADCINCH_10,
            11 => ADCINCH_A::ADCINCH_11,
            12 => ADCINCH_A::ADCINCH_12,
            13 => ADCINCH_A::ADCINCH_13,
            14 => ADCINCH_A::ADCINCH_14,
            15 => ADCINCH_A::ADCINCH_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCINCH_0`"]
    #[inline(always)]
    pub fn is_adcinch_0(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_0
    }
    #[doc = "Checks if the value of the field is `ADCINCH_1`"]
    #[inline(always)]
    pub fn is_adcinch_1(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_1
    }
    #[doc = "Checks if the value of the field is `ADCINCH_2`"]
    #[inline(always)]
    pub fn is_adcinch_2(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_2
    }
    #[doc = "Checks if the value of the field is `ADCINCH_3`"]
    #[inline(always)]
    pub fn is_adcinch_3(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_3
    }
    #[doc = "Checks if the value of the field is `ADCINCH_4`"]
    #[inline(always)]
    pub fn is_adcinch_4(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_4
    }
    #[doc = "Checks if the value of the field is `ADCINCH_5`"]
    #[inline(always)]
    pub fn is_adcinch_5(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_5
    }
    #[doc = "Checks if the value of the field is `ADCINCH_6`"]
    #[inline(always)]
    pub fn is_adcinch_6(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_6
    }
    #[doc = "Checks if the value of the field is `ADCINCH_7`"]
    #[inline(always)]
    pub fn is_adcinch_7(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_7
    }
    #[doc = "Checks if the value of the field is `ADCINCH_8`"]
    #[inline(always)]
    pub fn is_adcinch_8(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_8
    }
    #[doc = "Checks if the value of the field is `ADCINCH_9`"]
    #[inline(always)]
    pub fn is_adcinch_9(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_9
    }
    #[doc = "Checks if the value of the field is `ADCINCH_10`"]
    #[inline(always)]
    pub fn is_adcinch_10(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_10
    }
    #[doc = "Checks if the value of the field is `ADCINCH_11`"]
    #[inline(always)]
    pub fn is_adcinch_11(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_11
    }
    #[doc = "Checks if the value of the field is `ADCINCH_12`"]
    #[inline(always)]
    pub fn is_adcinch_12(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_12
    }
    #[doc = "Checks if the value of the field is `ADCINCH_13`"]
    #[inline(always)]
    pub fn is_adcinch_13(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_13
    }
    #[doc = "Checks if the value of the field is `ADCINCH_14`"]
    #[inline(always)]
    pub fn is_adcinch_14(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_14
    }
    #[doc = "Checks if the value of the field is `ADCINCH_15`"]
    #[inline(always)]
    pub fn is_adcinch_15(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_15
    }
}
#[doc = "Field `ADCINCH` writer - Input channel select"]
pub type ADCINCH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADCMCTL0_SPEC, u8, ADCINCH_A, 4, O>;
impl<'a, const O: u8> ADCINCH_W<'a, O> {
    #[doc = "A0 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_0(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_0)
    }
    #[doc = "A1 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_1(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_1)
    }
    #[doc = "A2 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_2(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_2)
    }
    #[doc = "A3 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_3(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_3)
    }
    #[doc = "A4 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_4(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_4)
    }
    #[doc = "A5 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_5(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_5)
    }
    #[doc = "A2 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_6(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_6)
    }
    #[doc = "A7 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_7(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_7)
    }
    #[doc = "A8 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_8(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_8)
    }
    #[doc = "A9 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_9(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_9)
    }
    #[doc = "A10 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_10(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_10)
    }
    #[doc = "A11 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_11(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_11)
    }
    #[doc = "A12 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_12(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_12)
    }
    #[doc = "A13 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_13(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_13)
    }
    #[doc = "A14 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_14(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_14)
    }
    #[doc = "A15 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_15(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_15)
    }
}
#[doc = "Field `ADCSREF` reader - Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active."]
pub type ADCSREF_R = crate::FieldReader<u8, ADCSREF_A>;
#[doc = "Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCSREF_A {
    #[doc = "0: 000b = V(R+) = AVCC and V(R-) = AVSS"]
    ADCSREF_0 = 0,
    #[doc = "1: 001b = V(R+) = VREF and V(R-) = AVSS"]
    ADCSREF_1 = 1,
    #[doc = "2: 010b = V(R+) = VEREF+ buffered and V(R-) = AVSS"]
    ADCSREF_2 = 2,
    #[doc = "3: 011b =V(R+) = VEREF+ and V(R-) = AVSS"]
    ADCSREF_3 = 3,
    #[doc = "4: 100b = V(R+) = AVCC and V(R-) = VEREF-"]
    ADCSREF_4 = 4,
    #[doc = "5: 101b = V(R+) = VREF and V(R-) = VEREF-"]
    ADCSREF_5 = 5,
    #[doc = "6: 110b = V(R+) = VEREF+ buffered and V(R-) = VEREF-"]
    ADCSREF_6 = 6,
    #[doc = "7: 111b = V(R+) = VEREF+ and V(R-) = VEREF-"]
    ADCSREF_7 = 7,
}
impl From<ADCSREF_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSREF_A) -> Self {
        variant as _
    }
}
impl ADCSREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCSREF_A {
        match self.bits {
            0 => ADCSREF_A::ADCSREF_0,
            1 => ADCSREF_A::ADCSREF_1,
            2 => ADCSREF_A::ADCSREF_2,
            3 => ADCSREF_A::ADCSREF_3,
            4 => ADCSREF_A::ADCSREF_4,
            5 => ADCSREF_A::ADCSREF_5,
            6 => ADCSREF_A::ADCSREF_6,
            7 => ADCSREF_A::ADCSREF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSREF_0`"]
    #[inline(always)]
    pub fn is_adcsref_0(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_0
    }
    #[doc = "Checks if the value of the field is `ADCSREF_1`"]
    #[inline(always)]
    pub fn is_adcsref_1(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_1
    }
    #[doc = "Checks if the value of the field is `ADCSREF_2`"]
    #[inline(always)]
    pub fn is_adcsref_2(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_2
    }
    #[doc = "Checks if the value of the field is `ADCSREF_3`"]
    #[inline(always)]
    pub fn is_adcsref_3(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_3
    }
    #[doc = "Checks if the value of the field is `ADCSREF_4`"]
    #[inline(always)]
    pub fn is_adcsref_4(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_4
    }
    #[doc = "Checks if the value of the field is `ADCSREF_5`"]
    #[inline(always)]
    pub fn is_adcsref_5(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_5
    }
    #[doc = "Checks if the value of the field is `ADCSREF_6`"]
    #[inline(always)]
    pub fn is_adcsref_6(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_6
    }
    #[doc = "Checks if the value of the field is `ADCSREF_7`"]
    #[inline(always)]
    pub fn is_adcsref_7(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_7
    }
}
#[doc = "Field `ADCSREF` writer - Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active."]
pub type ADCSREF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADCMCTL0_SPEC, u8, ADCSREF_A, 3, O>;
impl<'a, const O: u8> ADCSREF_W<'a, O> {
    #[doc = "000b = V(R+) = AVCC and V(R-) = AVSS"]
    #[inline(always)]
    pub fn adcsref_0(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_0)
    }
    #[doc = "001b = V(R+) = VREF and V(R-) = AVSS"]
    #[inline(always)]
    pub fn adcsref_1(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_1)
    }
    #[doc = "010b = V(R+) = VEREF+ buffered and V(R-) = AVSS"]
    #[inline(always)]
    pub fn adcsref_2(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_2)
    }
    #[doc = "011b =V(R+) = VEREF+ and V(R-) = AVSS"]
    #[inline(always)]
    pub fn adcsref_3(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_3)
    }
    #[doc = "100b = V(R+) = AVCC and V(R-) = VEREF-"]
    #[inline(always)]
    pub fn adcsref_4(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_4)
    }
    #[doc = "101b = V(R+) = VREF and V(R-) = VEREF-"]
    #[inline(always)]
    pub fn adcsref_5(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_5)
    }
    #[doc = "110b = V(R+) = VEREF+ buffered and V(R-) = VEREF-"]
    #[inline(always)]
    pub fn adcsref_6(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_6)
    }
    #[doc = "111b = V(R+) = VEREF+ and V(R-) = VEREF-"]
    #[inline(always)]
    pub fn adcsref_7(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_7)
    }
}
#[doc = "Field `EXPCHEN` reader - ADC input channels expanded"]
pub type EXPCHEN_R = crate::BitReader<EXPCHEN_A>;
#[doc = "ADC input channels expanded\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXPCHEN_A {
    #[doc = "0: ADC channel expanded disable"]
    EXPCHEN_0 = 0,
    #[doc = "1: ADC channel expanded enable"]
    EXPCHEN_1 = 1,
}
impl From<EXPCHEN_A> for bool {
    #[inline(always)]
    fn from(variant: EXPCHEN_A) -> Self {
        variant as u8 != 0
    }
}
impl EXPCHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXPCHEN_A {
        match self.bits {
            false => EXPCHEN_A::EXPCHEN_0,
            true => EXPCHEN_A::EXPCHEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXPCHEN_0`"]
    #[inline(always)]
    pub fn is_expchen_0(&self) -> bool {
        *self == EXPCHEN_A::EXPCHEN_0
    }
    #[doc = "Checks if the value of the field is `EXPCHEN_1`"]
    #[inline(always)]
    pub fn is_expchen_1(&self) -> bool {
        *self == EXPCHEN_A::EXPCHEN_1
    }
}
#[doc = "Field `EXPCHEN` writer - ADC input channels expanded"]
pub type EXPCHEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMCTL0_SPEC, EXPCHEN_A, O>;
impl<'a, const O: u8> EXPCHEN_W<'a, O> {
    #[doc = "ADC channel expanded disable"]
    #[inline(always)]
    pub fn expchen_0(self) -> &'a mut W {
        self.variant(EXPCHEN_A::EXPCHEN_0)
    }
    #[doc = "ADC channel expanded enable"]
    #[inline(always)]
    pub fn expchen_1(self) -> &'a mut W {
        self.variant(EXPCHEN_A::EXPCHEN_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Input channel select"]
    #[inline(always)]
    pub fn adcinch(&self) -> ADCINCH_R {
        ADCINCH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active."]
    #[inline(always)]
    pub fn adcsref(&self) -> ADCSREF_R {
        ADCSREF_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - ADC input channels expanded"]
    #[inline(always)]
    pub fn expchen(&self) -> EXPCHEN_R {
        EXPCHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input channel select"]
    #[inline(always)]
    pub fn adcinch(&mut self) -> ADCINCH_W<0> {
        ADCINCH_W::new(self)
    }
    #[doc = "Bits 4:6 - Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active."]
    #[inline(always)]
    pub fn adcsref(&mut self) -> ADCSREF_W<4> {
        ADCSREF_W::new(self)
    }
    #[doc = "Bit 8 - ADC input channels expanded"]
    #[inline(always)]
    pub fn expchen(&mut self) -> EXPCHEN_W<8> {
        EXPCHEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Conversion Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmctl0](index.html) module"]
pub struct ADCMCTL0_SPEC;
impl crate::RegisterSpec for ADCMCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmctl0::R](R) reader structure"]
impl crate::Readable for ADCMCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmctl0::W](W) writer structure"]
impl crate::Writable for ADCMCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCMCTL0 to value 0"]
impl crate::Resettable for ADCMCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
