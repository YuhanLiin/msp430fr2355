#[doc = "Reader of register SYSCTL"]
pub type R = crate::R<u16, super::SYSCTL>;
#[doc = "Writer for register SYSCTL"]
pub type W = crate::W<u16, super::SYSCTL>;
#[doc = "Register SYSCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RAM-based interrupt vectors\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRIVECT_A {
    #[doc = "0: Interrupt vectors generated with end address TOP of lower 64K FRAM FFFFh"]
    FRAM,
    #[doc = "1: Interrupt vectors generated with end address TOP of RAM, when RAM available"]
    RAM,
}
impl From<SYSRIVECT_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRIVECT_A) -> Self {
        match variant {
            SYSRIVECT_A::FRAM => false,
            SYSRIVECT_A::RAM => true,
        }
    }
}
#[doc = "Reader of field `SYSRIVECT`"]
pub type SYSRIVECT_R = crate::R<bool, SYSRIVECT_A>;
impl SYSRIVECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRIVECT_A {
        match self.bits {
            false => SYSRIVECT_A::FRAM,
            true => SYSRIVECT_A::RAM,
        }
    }
    #[doc = "Checks if the value of the field is `FRAM`"]
    #[inline(always)]
    pub fn is_fram(&self) -> bool {
        *self == SYSRIVECT_A::FRAM
    }
    #[doc = "Checks if the value of the field is `RAM`"]
    #[inline(always)]
    pub fn is_ram(&self) -> bool {
        *self == SYSRIVECT_A::RAM
    }
}
#[doc = "Write proxy for field `SYSRIVECT`"]
pub struct SYSRIVECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRIVECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRIVECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt vectors generated with end address TOP of lower 64K FRAM FFFFh"]
    #[inline(always)]
    pub fn fram(self) -> &'a mut W {
        self.variant(SYSRIVECT_A::FRAM)
    }
    #[doc = "Interrupt vectors generated with end address TOP of RAM, when RAM available"]
    #[inline(always)]
    pub fn ram(self) -> &'a mut W {
        self.variant(SYSRIVECT_A::RAM)
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
#[doc = "PMM access protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSPMMPE_A {
    #[doc = "0: Access from anywhere in memory"]
    SYSPMMPE_0,
    #[doc = "1: Access only from the BSL segments"]
    SYSPMMPE_1,
}
impl From<SYSPMMPE_A> for bool {
    #[inline(always)]
    fn from(variant: SYSPMMPE_A) -> Self {
        match variant {
            SYSPMMPE_A::SYSPMMPE_0 => false,
            SYSPMMPE_A::SYSPMMPE_1 => true,
        }
    }
}
#[doc = "Reader of field `SYSPMMPE`"]
pub type SYSPMMPE_R = crate::R<bool, SYSPMMPE_A>;
impl SYSPMMPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSPMMPE_A {
        match self.bits {
            false => SYSPMMPE_A::SYSPMMPE_0,
            true => SYSPMMPE_A::SYSPMMPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYSPMMPE_0`"]
    #[inline(always)]
    pub fn is_syspmmpe_0(&self) -> bool {
        *self == SYSPMMPE_A::SYSPMMPE_0
    }
    #[doc = "Checks if the value of the field is `SYSPMMPE_1`"]
    #[inline(always)]
    pub fn is_syspmmpe_1(&self) -> bool {
        *self == SYSPMMPE_A::SYSPMMPE_1
    }
}
#[doc = "Write proxy for field `SYSPMMPE`"]
pub struct SYSPMMPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSPMMPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSPMMPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Access from anywhere in memory"]
    #[inline(always)]
    pub fn syspmmpe_0(self) -> &'a mut W {
        self.variant(SYSPMMPE_A::SYSPMMPE_0)
    }
    #[doc = "Access only from the BSL segments"]
    #[inline(always)]
    pub fn syspmmpe_1(self) -> &'a mut W {
        self.variant(SYSPMMPE_A::SYSPMMPE_1)
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
#[doc = "BSL entry indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSBSLIND_A {
    #[doc = "0: No BSL entry sequence detected"]
    SYSBSLIND_0,
    #[doc = "1: BSL entry sequence detected"]
    SYSBSLIND_1,
}
impl From<SYSBSLIND_A> for bool {
    #[inline(always)]
    fn from(variant: SYSBSLIND_A) -> Self {
        match variant {
            SYSBSLIND_A::SYSBSLIND_0 => false,
            SYSBSLIND_A::SYSBSLIND_1 => true,
        }
    }
}
#[doc = "Reader of field `SYSBSLIND`"]
pub type SYSBSLIND_R = crate::R<bool, SYSBSLIND_A>;
impl SYSBSLIND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSBSLIND_A {
        match self.bits {
            false => SYSBSLIND_A::SYSBSLIND_0,
            true => SYSBSLIND_A::SYSBSLIND_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYSBSLIND_0`"]
    #[inline(always)]
    pub fn is_sysbslind_0(&self) -> bool {
        *self == SYSBSLIND_A::SYSBSLIND_0
    }
    #[doc = "Checks if the value of the field is `SYSBSLIND_1`"]
    #[inline(always)]
    pub fn is_sysbslind_1(&self) -> bool {
        *self == SYSBSLIND_A::SYSBSLIND_1
    }
}
#[doc = "Dedicated JTAG pins enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSJTAGPIN_A {
    #[doc = "0: Shared JTAG pins (JTAG mode selectable using SBW sequence)"]
    SHARED,
    #[doc = "1: Dedicated JTAG pins (explicit 4-wire JTAG mode selection)"]
    DEDICATED,
}
impl From<SYSJTAGPIN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSJTAGPIN_A) -> Self {
        match variant {
            SYSJTAGPIN_A::SHARED => false,
            SYSJTAGPIN_A::DEDICATED => true,
        }
    }
}
#[doc = "Reader of field `SYSJTAGPIN`"]
pub type SYSJTAGPIN_R = crate::R<bool, SYSJTAGPIN_A>;
impl SYSJTAGPIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSJTAGPIN_A {
        match self.bits {
            false => SYSJTAGPIN_A::SHARED,
            true => SYSJTAGPIN_A::DEDICATED,
        }
    }
    #[doc = "Checks if the value of the field is `SHARED`"]
    #[inline(always)]
    pub fn is_shared(&self) -> bool {
        *self == SYSJTAGPIN_A::SHARED
    }
    #[doc = "Checks if the value of the field is `DEDICATED`"]
    #[inline(always)]
    pub fn is_dedicated(&self) -> bool {
        *self == SYSJTAGPIN_A::DEDICATED
    }
}
#[doc = "Write proxy for field `SYSJTAGPIN`"]
pub struct SYSJTAGPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSJTAGPIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSJTAGPIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shared JTAG pins (JTAG mode selectable using SBW sequence)"]
    #[inline(always)]
    pub fn shared(self) -> &'a mut W {
        self.variant(SYSJTAGPIN_A::SHARED)
    }
    #[doc = "Dedicated JTAG pins (explicit 4-wire JTAG mode selection)"]
    #[inline(always)]
    pub fn dedicated(self) -> &'a mut W {
        self.variant(SYSJTAGPIN_A::DEDICATED)
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
    #[doc = "Bit 0 - RAM-based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&self) -> SYSRIVECT_R {
        SYSRIVECT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&self) -> SYSPMMPE_R {
        SYSPMMPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BSL entry indication"]
    #[inline(always)]
    pub fn sysbslind(&self) -> SYSBSLIND_R {
        SYSBSLIND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Dedicated JTAG pins enable"]
    #[inline(always)]
    pub fn sysjtagpin(&self) -> SYSJTAGPIN_R {
        SYSJTAGPIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM-based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&mut self) -> SYSRIVECT_W {
        SYSRIVECT_W { w: self }
    }
    #[doc = "Bit 2 - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&mut self) -> SYSPMMPE_W {
        SYSPMMPE_W { w: self }
    }
    #[doc = "Bit 5 - Dedicated JTAG pins enable"]
    #[inline(always)]
    pub fn sysjtagpin(&mut self) -> SYSJTAGPIN_W {
        SYSJTAGPIN_W { w: self }
    }
}
