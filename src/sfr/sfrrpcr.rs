#[doc = "Reader of register SFRRPCR"]
pub type R = crate::R<u16, super::SFRRPCR>;
#[doc = "Writer for register SFRRPCR"]
pub type W = crate::W<u16, super::SFRRPCR>;
#[doc = "Register SFRRPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SFRRPCR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "NMI select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSNMI_A {
    #[doc = "0: Reset function"]
    RESET,
    #[doc = "1: NMI function"]
    NMI,
}
impl From<SYSNMI_A> for bool {
    #[inline(always)]
    fn from(variant: SYSNMI_A) -> Self {
        match variant {
            SYSNMI_A::RESET => false,
            SYSNMI_A::NMI => true,
        }
    }
}
#[doc = "Reader of field `SYSNMI`"]
pub type SYSNMI_R = crate::R<bool, SYSNMI_A>;
impl SYSNMI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSNMI_A {
        match self.bits {
            false => SYSNMI_A::RESET,
            true => SYSNMI_A::NMI,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSNMI_A::RESET
    }
    #[doc = "Checks if the value of the field is `NMI`"]
    #[inline(always)]
    pub fn is_nmi(&self) -> bool {
        *self == SYSNMI_A::NMI
    }
}
#[doc = "Write proxy for field `SYSNMI`"]
pub struct SYSNMI_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSNMI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSNMI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset function"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYSNMI_A::RESET)
    }
    #[doc = "NMI function"]
    #[inline(always)]
    pub fn nmi(self) -> &'a mut W {
        self.variant(SYSNMI_A::NMI)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "NMI edge select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSNMIIES_A {
    #[doc = "0: NMI on rising edge"]
    RISING,
    #[doc = "1: NMI on falling edge"]
    FALLING,
}
impl From<SYSNMIIES_A> for bool {
    #[inline(always)]
    fn from(variant: SYSNMIIES_A) -> Self {
        match variant {
            SYSNMIIES_A::RISING => false,
            SYSNMIIES_A::FALLING => true,
        }
    }
}
#[doc = "Reader of field `SYSNMIIES`"]
pub type SYSNMIIES_R = crate::R<bool, SYSNMIIES_A>;
impl SYSNMIIES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSNMIIES_A {
        match self.bits {
            false => SYSNMIIES_A::RISING,
            true => SYSNMIIES_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == SYSNMIIES_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == SYSNMIIES_A::FALLING
    }
}
#[doc = "Write proxy for field `SYSNMIIES`"]
pub struct SYSNMIIES_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSNMIIES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSNMIIES_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NMI on rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(SYSNMIIES_A::RISING)
    }
    #[doc = "NMI on falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(SYSNMIIES_A::FALLING)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reset resistor pin pullup or pulldown\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRSTUP_A {
    #[doc = "0: Pulldown is selected"]
    PULLDOWN,
    #[doc = "1: Pullup is selected"]
    PULLUP,
}
impl From<SYSRSTUP_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRSTUP_A) -> Self {
        match variant {
            SYSRSTUP_A::PULLDOWN => false,
            SYSRSTUP_A::PULLUP => true,
        }
    }
}
#[doc = "Reader of field `SYSRSTUP`"]
pub type SYSRSTUP_R = crate::R<bool, SYSRSTUP_A>;
impl SYSRSTUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRSTUP_A {
        match self.bits {
            false => SYSRSTUP_A::PULLDOWN,
            true => SYSRSTUP_A::PULLUP,
        }
    }
    #[doc = "Checks if the value of the field is `PULLDOWN`"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == SYSRSTUP_A::PULLDOWN
    }
    #[doc = "Checks if the value of the field is `PULLUP`"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == SYSRSTUP_A::PULLUP
    }
}
#[doc = "Write proxy for field `SYSRSTUP`"]
pub struct SYSRSTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRSTUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRSTUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pulldown is selected"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(SYSRSTUP_A::PULLDOWN)
    }
    #[doc = "Pullup is selected"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(SYSRSTUP_A::PULLUP)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reset pin resistor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRSTRE_A {
    #[doc = "0: Pullup or pulldown resistor at the RST/NMI pin is disabled"]
    DISABLE,
    #[doc = "1: Pullup or pulldown resistor at the RST/NMI pin is enabled"]
    ENABLE,
}
impl From<SYSRSTRE_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRSTRE_A) -> Self {
        match variant {
            SYSRSTRE_A::DISABLE => false,
            SYSRSTRE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SYSRSTRE`"]
pub type SYSRSTRE_R = crate::R<bool, SYSRSTRE_A>;
impl SYSRSTRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRSTRE_A {
        match self.bits {
            false => SYSRSTRE_A::DISABLE,
            true => SYSRSTRE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SYSRSTRE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SYSRSTRE_A::ENABLE
    }
}
#[doc = "Write proxy for field `SYSRSTRE`"]
pub struct SYSRSTRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRSTRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRSTRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup or pulldown resistor at the RST/NMI pin is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYSRSTRE_A::DISABLE)
    }
    #[doc = "Pullup or pulldown resistor at the RST/NMI pin is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYSRSTRE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    pub fn sysnmi(&self) -> SYSNMI_R {
        SYSNMI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    pub fn sysnmiies(&self) -> SYSNMIIES_R {
        SYSNMIIES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset resistor pin pullup or pulldown"]
    #[inline(always)]
    pub fn sysrstup(&self) -> SYSRSTUP_R {
        SYSRSTUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset pin resistor enable"]
    #[inline(always)]
    pub fn sysrstre(&self) -> SYSRSTRE_R {
        SYSRSTRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    pub fn sysnmi(&mut self) -> SYSNMI_W {
        SYSNMI_W { w: self }
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    pub fn sysnmiies(&mut self) -> SYSNMIIES_W {
        SYSNMIIES_W { w: self }
    }
    #[doc = "Bit 2 - Reset resistor pin pullup or pulldown"]
    #[inline(always)]
    pub fn sysrstup(&mut self) -> SYSRSTUP_W {
        SYSRSTUP_W { w: self }
    }
    #[doc = "Bit 3 - Reset pin resistor enable"]
    #[inline(always)]
    pub fn sysrstre(&mut self) -> SYSRSTRE_W {
        SYSRSTRE_W { w: self }
    }
}
