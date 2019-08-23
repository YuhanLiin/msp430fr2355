#[doc = "Reader of register CSCTL4"]
pub type R = crate::R<u16, super::CSCTL4>;
#[doc = "Writer for register CSCTL4"]
pub type W = crate::W<u16, super::CSCTL4>;
#[doc = "Register CSCTL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL4 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects the MCLK and SMCLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELMS_A {
    #[doc = "0: DCOCLKDIV"]
    DCOCLKDIV,
    #[doc = "1: REFOCLK"]
    REFOCLK,
    #[doc = "2: XT1CLK"]
    XT1CLK,
    #[doc = "3: VLOCLK"]
    VLOCLK,
    #[doc = "4: Reserved for future use"]
    SELMS_4,
    #[doc = "5: Reserved for future use"]
    SELMS_5,
    #[doc = "6: Reserved for future use"]
    SELMS_6,
    #[doc = "7: Reserved for future use"]
    SELMS_7,
}
impl From<SELMS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELMS_A) -> Self {
        match variant {
            SELMS_A::DCOCLKDIV => 0,
            SELMS_A::REFOCLK => 1,
            SELMS_A::XT1CLK => 2,
            SELMS_A::VLOCLK => 3,
            SELMS_A::SELMS_4 => 4,
            SELMS_A::SELMS_5 => 5,
            SELMS_A::SELMS_6 => 6,
            SELMS_A::SELMS_7 => 7,
        }
    }
}
#[doc = "Reader of field `SELMS`"]
pub type SELMS_R = crate::R<u8, SELMS_A>;
impl SELMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELMS_A {
        match self.bits {
            0 => SELMS_A::DCOCLKDIV,
            1 => SELMS_A::REFOCLK,
            2 => SELMS_A::XT1CLK,
            3 => SELMS_A::VLOCLK,
            4 => SELMS_A::SELMS_4,
            5 => SELMS_A::SELMS_5,
            6 => SELMS_A::SELMS_6,
            7 => SELMS_A::SELMS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DCOCLKDIV`"]
    #[inline(always)]
    pub fn is_dcoclkdiv(&self) -> bool {
        *self == SELMS_A::DCOCLKDIV
    }
    #[doc = "Checks if the value of the field is `REFOCLK`"]
    #[inline(always)]
    pub fn is_refoclk(&self) -> bool {
        *self == SELMS_A::REFOCLK
    }
    #[doc = "Checks if the value of the field is `XT1CLK`"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == SELMS_A::XT1CLK
    }
    #[doc = "Checks if the value of the field is `VLOCLK`"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == SELMS_A::VLOCLK
    }
    #[doc = "Checks if the value of the field is `SELMS_4`"]
    #[inline(always)]
    pub fn is_selms_4(&self) -> bool {
        *self == SELMS_A::SELMS_4
    }
    #[doc = "Checks if the value of the field is `SELMS_5`"]
    #[inline(always)]
    pub fn is_selms_5(&self) -> bool {
        *self == SELMS_A::SELMS_5
    }
    #[doc = "Checks if the value of the field is `SELMS_6`"]
    #[inline(always)]
    pub fn is_selms_6(&self) -> bool {
        *self == SELMS_A::SELMS_6
    }
    #[doc = "Checks if the value of the field is `SELMS_7`"]
    #[inline(always)]
    pub fn is_selms_7(&self) -> bool {
        *self == SELMS_A::SELMS_7
    }
}
#[doc = "Write proxy for field `SELMS`"]
pub struct SELMS_W<'a> {
    w: &'a mut W,
}
impl<'a> SELMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELMS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DCOCLKDIV"]
    #[inline(always)]
    pub fn dcoclkdiv(self) -> &'a mut W {
        self.variant(SELMS_A::DCOCLKDIV)
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn refoclk(self) -> &'a mut W {
        self.variant(SELMS_A::REFOCLK)
    }
    #[doc = "XT1CLK"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut W {
        self.variant(SELMS_A::XT1CLK)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut W {
        self.variant(SELMS_A::VLOCLK)
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn selms_4(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_4)
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn selms_5(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_5)
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn selms_6(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_6)
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn selms_7(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Selects the ACLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELA_A {
    #[doc = "0: XT1CLK with divider (must be no more than 40 kHz)"]
    XT1CLK,
    #[doc = "1: REFO (internal 32-kHz clock source)"]
    REFOCLK,
    #[doc = "2: VLO (internal 10-kHz clock source)"]
    VLOCLK,
}
impl From<SELA_A> for u8 {
    #[inline(always)]
    fn from(variant: SELA_A) -> Self {
        match variant {
            SELA_A::XT1CLK => 0,
            SELA_A::REFOCLK => 1,
            SELA_A::VLOCLK => 2,
        }
    }
}
#[doc = "Reader of field `SELA`"]
pub type SELA_R = crate::R<u8, SELA_A>;
impl SELA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELA_A {
        match self.bits {
            0 => SELA_A::XT1CLK,
            1 => SELA_A::REFOCLK,
            2 => SELA_A::VLOCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XT1CLK`"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == SELA_A::XT1CLK
    }
    #[doc = "Checks if the value of the field is `REFOCLK`"]
    #[inline(always)]
    pub fn is_refoclk(&self) -> bool {
        *self == SELA_A::REFOCLK
    }
    #[doc = "Checks if the value of the field is `VLOCLK`"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == SELA_A::VLOCLK
    }
}
#[doc = "Write proxy for field `SELA`"]
pub struct SELA_W<'a> {
    w: &'a mut W,
}
impl<'a> SELA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "XT1CLK with divider (must be no more than 40 kHz)"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut W {
        self.variant(SELA_A::XT1CLK)
    }
    #[doc = "REFO (internal 32-kHz clock source)"]
    #[inline(always)]
    pub fn refoclk(self) -> &'a mut W {
        self.variant(SELA_A::REFOCLK)
    }
    #[doc = "VLO (internal 10-kHz clock source)"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut W {
        self.variant(SELA_A::VLOCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects the MCLK and SMCLK source"]
    #[inline(always)]
    pub fn selms(&self) -> SELMS_R {
        SELMS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Selects the ACLK source"]
    #[inline(always)]
    pub fn sela(&self) -> SELA_R {
        SELA_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the MCLK and SMCLK source"]
    #[inline(always)]
    pub fn selms(&mut self) -> SELMS_W {
        SELMS_W { w: self }
    }
    #[doc = "Bits 8:9 - Selects the ACLK source"]
    #[inline(always)]
    pub fn sela(&mut self) -> SELA_W {
        SELA_W { w: self }
    }
}
