#[doc = "Reader of register PM5CTL0"]
pub type R = crate::R<u16, super::PM5CTL0>;
#[doc = "Writer for register PM5CTL0"]
pub type W = crate::W<u16, super::PM5CTL0>;
#[doc = "Register PM5CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PM5CTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LPMx.5 Lock Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKLPM5_A {
    #[doc = "0: LPMx.5 configuration is not locked and defaults to its reset condition."]
    LOCKLPM5_0 = 0,
    #[doc = "1: LPMx.5 configuration remains locked. Pin state is held during LPMx.5 entry and exit."]
    LOCKLPM5_1 = 1,
}
impl From<LOCKLPM5_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKLPM5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCKLPM5`"]
pub type LOCKLPM5_R = crate::R<bool, LOCKLPM5_A>;
impl LOCKLPM5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKLPM5_A {
        match self.bits {
            false => LOCKLPM5_A::LOCKLPM5_0,
            true => LOCKLPM5_A::LOCKLPM5_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKLPM5_0`"]
    #[inline(always)]
    pub fn is_locklpm5_0(&self) -> bool {
        *self == LOCKLPM5_A::LOCKLPM5_0
    }
    #[doc = "Checks if the value of the field is `LOCKLPM5_1`"]
    #[inline(always)]
    pub fn is_locklpm5_1(&self) -> bool {
        *self == LOCKLPM5_A::LOCKLPM5_1
    }
}
#[doc = "Write proxy for field `LOCKLPM5`"]
pub struct LOCKLPM5_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKLPM5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKLPM5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPMx.5 configuration is not locked and defaults to its reset condition."]
    #[inline(always)]
    pub fn locklpm5_0(self) -> &'a mut W {
        self.variant(LOCKLPM5_A::LOCKLPM5_0)
    }
    #[doc = "LPMx.5 configuration remains locked. Pin state is held during LPMx.5 entry and exit."]
    #[inline(always)]
    pub fn locklpm5_1(self) -> &'a mut W {
        self.variant(LOCKLPM5_A::LOCKLPM5_1)
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
#[doc = "Reports or sets the LPM3.5 switch connection upon the switch mode set by LPM5SM. When this bit is set, the VLPM3.5 domain can accept full-speed read and write operation by CPU MCLK. If the switch is disconnected, all peripherals within this domain can only accept the operation no more than 40 kHz. In automatic mode (LPM5SM = 0), this bit represents the switch connection between Vcore and VLPM3.5. Any write to this bit has no effect. In manual mode (LPM5SM = 1), this bit can be fully read and written by software. When this bit is set, the switch connection between Vcore and VLPM3.5 is connected. Otherwise, the switch is disconnected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM5SW_A {
    #[doc = "0: LPMx.5 switch disconnected"]
    LPM5SW_0 = 0,
    #[doc = "1: LPMx.5 switch connected"]
    LPM5SW_1 = 1,
}
impl From<LPM5SW_A> for bool {
    #[inline(always)]
    fn from(variant: LPM5SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPM5SW`"]
pub type LPM5SW_R = crate::R<bool, LPM5SW_A>;
impl LPM5SW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPM5SW_A {
        match self.bits {
            false => LPM5SW_A::LPM5SW_0,
            true => LPM5SW_A::LPM5SW_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPM5SW_0`"]
    #[inline(always)]
    pub fn is_lpm5sw_0(&self) -> bool {
        *self == LPM5SW_A::LPM5SW_0
    }
    #[doc = "Checks if the value of the field is `LPM5SW_1`"]
    #[inline(always)]
    pub fn is_lpm5sw_1(&self) -> bool {
        *self == LPM5SW_A::LPM5SW_1
    }
}
#[doc = "Write proxy for field `LPM5SW`"]
pub struct LPM5SW_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM5SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPM5SW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPMx.5 switch disconnected"]
    #[inline(always)]
    pub fn lpm5sw_0(self) -> &'a mut W {
        self.variant(LPM5SW_A::LPM5SW_0)
    }
    #[doc = "LPMx.5 switch connected"]
    #[inline(always)]
    pub fn lpm5sw_1(self) -> &'a mut W {
        self.variant(LPM5SW_A::LPM5SW_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Specifies the operation mode of the LPM3.5 switch.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM5SM_A {
    #[doc = "0: Automatic mode for LPM3.5 switch that the switch is fully handled by the circuitry during mode switch."]
    LPM5SM_0 = 0,
    #[doc = "1: Manual mode for LPM3.5 switch that the switch is specified by LPM5SW bit setting in software."]
    LPM5SM_1 = 1,
}
impl From<LPM5SM_A> for bool {
    #[inline(always)]
    fn from(variant: LPM5SM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPM5SM`"]
pub type LPM5SM_R = crate::R<bool, LPM5SM_A>;
impl LPM5SM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPM5SM_A {
        match self.bits {
            false => LPM5SM_A::LPM5SM_0,
            true => LPM5SM_A::LPM5SM_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPM5SM_0`"]
    #[inline(always)]
    pub fn is_lpm5sm_0(&self) -> bool {
        *self == LPM5SM_A::LPM5SM_0
    }
    #[doc = "Checks if the value of the field is `LPM5SM_1`"]
    #[inline(always)]
    pub fn is_lpm5sm_1(&self) -> bool {
        *self == LPM5SM_A::LPM5SM_1
    }
}
#[doc = "Write proxy for field `LPM5SM`"]
pub struct LPM5SM_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM5SM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPM5SM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic mode for LPM3.5 switch that the switch is fully handled by the circuitry during mode switch."]
    #[inline(always)]
    pub fn lpm5sm_0(self) -> &'a mut W {
        self.variant(LPM5SM_A::LPM5SM_0)
    }
    #[doc = "Manual mode for LPM3.5 switch that the switch is specified by LPM5SW bit setting in software."]
    #[inline(always)]
    pub fn lpm5sm_1(self) -> &'a mut W {
        self.variant(LPM5SM_A::LPM5SM_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LPMx.5 Lock Bit"]
    #[inline(always)]
    pub fn locklpm5(&self) -> LOCKLPM5_R {
        LOCKLPM5_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reports or sets the LPM3.5 switch connection upon the switch mode set by LPM5SM. When this bit is set, the VLPM3.5 domain can accept full-speed read and write operation by CPU MCLK. If the switch is disconnected, all peripherals within this domain can only accept the operation no more than 40 kHz. In automatic mode (LPM5SM = 0), this bit represents the switch connection between Vcore and VLPM3.5. Any write to this bit has no effect. In manual mode (LPM5SM = 1), this bit can be fully read and written by software. When this bit is set, the switch connection between Vcore and VLPM3.5 is connected. Otherwise, the switch is disconnected."]
    #[inline(always)]
    pub fn lpm5sw(&self) -> LPM5SW_R {
        LPM5SW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Specifies the operation mode of the LPM3.5 switch."]
    #[inline(always)]
    pub fn lpm5sm(&self) -> LPM5SM_R {
        LPM5SM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPMx.5 Lock Bit"]
    #[inline(always)]
    pub fn locklpm5(&mut self) -> LOCKLPM5_W {
        LOCKLPM5_W { w: self }
    }
    #[doc = "Bit 4 - Reports or sets the LPM3.5 switch connection upon the switch mode set by LPM5SM. When this bit is set, the VLPM3.5 domain can accept full-speed read and write operation by CPU MCLK. If the switch is disconnected, all peripherals within this domain can only accept the operation no more than 40 kHz. In automatic mode (LPM5SM = 0), this bit represents the switch connection between Vcore and VLPM3.5. Any write to this bit has no effect. In manual mode (LPM5SM = 1), this bit can be fully read and written by software. When this bit is set, the switch connection between Vcore and VLPM3.5 is connected. Otherwise, the switch is disconnected."]
    #[inline(always)]
    pub fn lpm5sw(&mut self) -> LPM5SW_W {
        LPM5SW_W { w: self }
    }
    #[doc = "Bit 5 - Specifies the operation mode of the LPM3.5 switch."]
    #[inline(always)]
    pub fn lpm5sm(&mut self) -> LPM5SM_W {
        LPM5SM_W { w: self }
    }
}
