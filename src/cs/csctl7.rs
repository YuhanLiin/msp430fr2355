#[doc = "Reader of register CSCTL7"]
pub type R = crate::R<u16, super::CSCTL7>;
#[doc = "Writer for register CSCTL7"]
pub type W = crate::W<u16, super::CSCTL7>;
#[doc = "Register CSCTL7 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL7 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "REFO ready flag. This bit reflects the REFO readiness whent REFO is good for operation (such as FLL reference)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFOREADY_A {
    #[doc = "0: REFO unstable"]
    REFOREADY_0,
    #[doc = "1: REFO ready to go"]
    REFOREADY_1,
}
impl From<REFOREADY_A> for bool {
    #[inline(always)]
    fn from(variant: REFOREADY_A) -> Self {
        match variant {
            REFOREADY_A::REFOREADY_0 => false,
            REFOREADY_A::REFOREADY_1 => true,
        }
    }
}
#[doc = "Reader of field `REFOREADY`"]
pub type REFOREADY_R = crate::R<bool, REFOREADY_A>;
impl REFOREADY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFOREADY_A {
        match self.bits {
            false => REFOREADY_A::REFOREADY_0,
            true => REFOREADY_A::REFOREADY_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFOREADY_0`"]
    #[inline(always)]
    pub fn is_refoready_0(&self) -> bool {
        *self == REFOREADY_A::REFOREADY_0
    }
    #[doc = "Checks if the value of the field is `REFOREADY_1`"]
    #[inline(always)]
    pub fn is_refoready_1(&self) -> bool {
        *self == REFOREADY_A::REFOREADY_1
    }
}
#[doc = "DCO fault flag. If this bit is set, the OFIFG flag is also set. The DCOFFG bit is set if DCO = {0} or DCO = {511}. DCOFFG can be cleared by software. If the DCO fault condition still remains, DCOFFG is set. As long as DCOFFG is set, FLLUNLOCK shows the DCOERROR condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOFFG_A {
    #[doc = "0: No fault condition occurred after the last reset."]
    DCOFFG_0,
    #[doc = "1: DCO fault. A DCO fault occurred after the last reset."]
    DCOFFG_1,
}
impl From<DCOFFG_A> for bool {
    #[inline(always)]
    fn from(variant: DCOFFG_A) -> Self {
        match variant {
            DCOFFG_A::DCOFFG_0 => false,
            DCOFFG_A::DCOFFG_1 => true,
        }
    }
}
#[doc = "Reader of field `DCOFFG`"]
pub type DCOFFG_R = crate::R<bool, DCOFFG_A>;
impl DCOFFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOFFG_A {
        match self.bits {
            false => DCOFFG_A::DCOFFG_0,
            true => DCOFFG_A::DCOFFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCOFFG_0`"]
    #[inline(always)]
    pub fn is_dcoffg_0(&self) -> bool {
        *self == DCOFFG_A::DCOFFG_0
    }
    #[doc = "Checks if the value of the field is `DCOFFG_1`"]
    #[inline(always)]
    pub fn is_dcoffg_1(&self) -> bool {
        *self == DCOFFG_A::DCOFFG_1
    }
}
#[doc = "Write proxy for field `DCOFFG`"]
pub struct DCOFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOFFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCOFFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No fault condition occurred after the last reset."]
    #[inline(always)]
    pub fn dcoffg_0(self) -> &'a mut W {
        self.variant(DCOFFG_A::DCOFFG_0)
    }
    #[doc = "DCO fault. A DCO fault occurred after the last reset."]
    #[inline(always)]
    pub fn dcoffg_1(self) -> &'a mut W {
        self.variant(DCOFFG_A::DCOFFG_1)
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
#[doc = "T1 oscillator fault flag. If this bit is set, the OFIFG flag is also set. XT1OFFG is set if a XT1 fault condition exists. XT1OFFG can be cleared by software. If the XT1 fault condition still remains, XT1OFFG is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XT1OFFG_A {
    #[doc = "0: No fault condition occurred after the last reset."]
    XT1OFFG_0,
    #[doc = "1: XT1 fault. An XT1 fault occurred after the last reset."]
    XT1OFFG_1,
}
impl From<XT1OFFG_A> for bool {
    #[inline(always)]
    fn from(variant: XT1OFFG_A) -> Self {
        match variant {
            XT1OFFG_A::XT1OFFG_0 => false,
            XT1OFFG_A::XT1OFFG_1 => true,
        }
    }
}
#[doc = "Reader of field `XT1OFFG`"]
pub type XT1OFFG_R = crate::R<bool, XT1OFFG_A>;
impl XT1OFFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XT1OFFG_A {
        match self.bits {
            false => XT1OFFG_A::XT1OFFG_0,
            true => XT1OFFG_A::XT1OFFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `XT1OFFG_0`"]
    #[inline(always)]
    pub fn is_xt1offg_0(&self) -> bool {
        *self == XT1OFFG_A::XT1OFFG_0
    }
    #[doc = "Checks if the value of the field is `XT1OFFG_1`"]
    #[inline(always)]
    pub fn is_xt1offg_1(&self) -> bool {
        *self == XT1OFFG_A::XT1OFFG_1
    }
}
#[doc = "Write proxy for field `XT1OFFG`"]
pub struct XT1OFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> XT1OFFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XT1OFFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No fault condition occurred after the last reset."]
    #[inline(always)]
    pub fn xt1offg_0(self) -> &'a mut W {
        self.variant(XT1OFFG_A::XT1OFFG_0)
    }
    #[doc = "XT1 fault. An XT1 fault occurred after the last reset."]
    #[inline(always)]
    pub fn xt1offg_1(self) -> &'a mut W {
        self.variant(XT1OFFG_A::XT1OFFG_1)
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
#[doc = "FLL unlock interrupt flag. This flag is set when FLLUNLOCK bits equal 10b (DCO too fast). If FLLULPUC is also set, a PUC is triggered when FLLUIFG is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLULIFG_A {
    #[doc = "0: FLLUNLOCK bits not equal to 10b"]
    FLLULIFG_0,
    #[doc = "1: FLLUNLOCK bits equal to 10b"]
    FLLULIFG_1,
}
impl From<FLLULIFG_A> for bool {
    #[inline(always)]
    fn from(variant: FLLULIFG_A) -> Self {
        match variant {
            FLLULIFG_A::FLLULIFG_0 => false,
            FLLULIFG_A::FLLULIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `FLLULIFG`"]
pub type FLLULIFG_R = crate::R<bool, FLLULIFG_A>;
impl FLLULIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLULIFG_A {
        match self.bits {
            false => FLLULIFG_A::FLLULIFG_0,
            true => FLLULIFG_A::FLLULIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLLULIFG_0`"]
    #[inline(always)]
    pub fn is_fllulifg_0(&self) -> bool {
        *self == FLLULIFG_A::FLLULIFG_0
    }
    #[doc = "Checks if the value of the field is `FLLULIFG_1`"]
    #[inline(always)]
    pub fn is_fllulifg_1(&self) -> bool {
        *self == FLLULIFG_A::FLLULIFG_1
    }
}
#[doc = "Write proxy for field `FLLULIFG`"]
pub struct FLLULIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLULIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLLULIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FLLUNLOCK bits not equal to 10b"]
    #[inline(always)]
    pub fn fllulifg_0(self) -> &'a mut W {
        self.variant(FLLULIFG_A::FLLULIFG_0)
    }
    #[doc = "FLLUNLOCK bits equal to 10b"]
    #[inline(always)]
    pub fn fllulifg_1(self) -> &'a mut W {
        self.variant(FLLULIFG_A::FLLULIFG_1)
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
#[doc = "Enable start counter for XT1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENSTFCNT1_A {
    #[doc = "0: Startup fault counter disabled. Counter is cleared.."]
    ENSTFCNT1_0,
    #[doc = "1: Startup fault counter enabled."]
    ENSTFCNT1_1,
}
impl From<ENSTFCNT1_A> for bool {
    #[inline(always)]
    fn from(variant: ENSTFCNT1_A) -> Self {
        match variant {
            ENSTFCNT1_A::ENSTFCNT1_0 => false,
            ENSTFCNT1_A::ENSTFCNT1_1 => true,
        }
    }
}
#[doc = "Reader of field `ENSTFCNT1`"]
pub type ENSTFCNT1_R = crate::R<bool, ENSTFCNT1_A>;
impl ENSTFCNT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENSTFCNT1_A {
        match self.bits {
            false => ENSTFCNT1_A::ENSTFCNT1_0,
            true => ENSTFCNT1_A::ENSTFCNT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENSTFCNT1_0`"]
    #[inline(always)]
    pub fn is_enstfcnt1_0(&self) -> bool {
        *self == ENSTFCNT1_A::ENSTFCNT1_0
    }
    #[doc = "Checks if the value of the field is `ENSTFCNT1_1`"]
    #[inline(always)]
    pub fn is_enstfcnt1_1(&self) -> bool {
        *self == ENSTFCNT1_A::ENSTFCNT1_1
    }
}
#[doc = "Write proxy for field `ENSTFCNT1`"]
pub struct ENSTFCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSTFCNT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENSTFCNT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Startup fault counter disabled. Counter is cleared.."]
    #[inline(always)]
    pub fn enstfcnt1_0(self) -> &'a mut W {
        self.variant(ENSTFCNT1_A::ENSTFCNT1_0)
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn enstfcnt1_1(self) -> &'a mut W {
        self.variant(ENSTFCNT1_A::ENSTFCNT1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Unlock. These bits indicate the current FLL unlock condition. These bits are both set as long as the DCOFFG flag is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLUNLOCK_A {
    #[doc = "0: FLL is locked. No unlock condition currently active."]
    FLLUNLOCK_0,
    #[doc = "1: DCOCLK is currently too slow."]
    FLLUNLOCK_1,
    #[doc = "2: DCOCLK is currently too fast."]
    FLLUNLOCK_2,
    #[doc = "3: DCOERROR. DCO out of range."]
    FLLUNLOCK_3,
}
impl From<FLLUNLOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLUNLOCK_A) -> Self {
        match variant {
            FLLUNLOCK_A::FLLUNLOCK_0 => 0,
            FLLUNLOCK_A::FLLUNLOCK_1 => 1,
            FLLUNLOCK_A::FLLUNLOCK_2 => 2,
            FLLUNLOCK_A::FLLUNLOCK_3 => 3,
        }
    }
}
#[doc = "Reader of field `FLLUNLOCK`"]
pub type FLLUNLOCK_R = crate::R<u8, FLLUNLOCK_A>;
impl FLLUNLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLUNLOCK_A {
        match self.bits {
            0 => FLLUNLOCK_A::FLLUNLOCK_0,
            1 => FLLUNLOCK_A::FLLUNLOCK_1,
            2 => FLLUNLOCK_A::FLLUNLOCK_2,
            3 => FLLUNLOCK_A::FLLUNLOCK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCK_0`"]
    #[inline(always)]
    pub fn is_fllunlock_0(&self) -> bool {
        *self == FLLUNLOCK_A::FLLUNLOCK_0
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCK_1`"]
    #[inline(always)]
    pub fn is_fllunlock_1(&self) -> bool {
        *self == FLLUNLOCK_A::FLLUNLOCK_1
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCK_2`"]
    #[inline(always)]
    pub fn is_fllunlock_2(&self) -> bool {
        *self == FLLUNLOCK_A::FLLUNLOCK_2
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCK_3`"]
    #[inline(always)]
    pub fn is_fllunlock_3(&self) -> bool {
        *self == FLLUNLOCK_A::FLLUNLOCK_3
    }
}
#[doc = "Write proxy for field `FLLUNLOCK`"]
pub struct FLLUNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLUNLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLLUNLOCK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FLL is locked. No unlock condition currently active."]
    #[inline(always)]
    pub fn fllunlock_0(self) -> &'a mut W {
        self.variant(FLLUNLOCK_A::FLLUNLOCK_0)
    }
    #[doc = "DCOCLK is currently too slow."]
    #[inline(always)]
    pub fn fllunlock_1(self) -> &'a mut W {
        self.variant(FLLUNLOCK_A::FLLUNLOCK_1)
    }
    #[doc = "DCOCLK is currently too fast."]
    #[inline(always)]
    pub fn fllunlock_2(self) -> &'a mut W {
        self.variant(FLLUNLOCK_A::FLLUNLOCK_2)
    }
    #[doc = "DCOERROR. DCO out of range."]
    #[inline(always)]
    pub fn fllunlock_3(self) -> &'a mut W {
        self.variant(FLLUNLOCK_A::FLLUNLOCK_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Unlock history bits. These bits indicate the FLL unlock condition history. As soon as any unlock condition happens, the respective bits are set and remain set until cleared by software by writing 0 to it or by a POR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLUNLOCKHIS_A {
    #[doc = "0: FLL is locked. No unlock situation has been detected since the last reset of these bits."]
    FLLUNLOCKHIS_0,
    #[doc = "1: DCOCLK has been too slow since the bits were cleared."]
    FLLUNLOCKHIS_1,
    #[doc = "2: DCOCLK has been too fast since the bits were cleared."]
    FLLUNLOCKHIS_2,
    #[doc = "3: DCOCLK has been both too fast and too slow since the bits were cleared."]
    FLLUNLOCKHIS_3,
}
impl From<FLLUNLOCKHIS_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLUNLOCKHIS_A) -> Self {
        match variant {
            FLLUNLOCKHIS_A::FLLUNLOCKHIS_0 => 0,
            FLLUNLOCKHIS_A::FLLUNLOCKHIS_1 => 1,
            FLLUNLOCKHIS_A::FLLUNLOCKHIS_2 => 2,
            FLLUNLOCKHIS_A::FLLUNLOCKHIS_3 => 3,
        }
    }
}
#[doc = "Reader of field `FLLUNLOCKHIS`"]
pub type FLLUNLOCKHIS_R = crate::R<u8, FLLUNLOCKHIS_A>;
impl FLLUNLOCKHIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLUNLOCKHIS_A {
        match self.bits {
            0 => FLLUNLOCKHIS_A::FLLUNLOCKHIS_0,
            1 => FLLUNLOCKHIS_A::FLLUNLOCKHIS_1,
            2 => FLLUNLOCKHIS_A::FLLUNLOCKHIS_2,
            3 => FLLUNLOCKHIS_A::FLLUNLOCKHIS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCKHIS_0`"]
    #[inline(always)]
    pub fn is_fllunlockhis_0(&self) -> bool {
        *self == FLLUNLOCKHIS_A::FLLUNLOCKHIS_0
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCKHIS_1`"]
    #[inline(always)]
    pub fn is_fllunlockhis_1(&self) -> bool {
        *self == FLLUNLOCKHIS_A::FLLUNLOCKHIS_1
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCKHIS_2`"]
    #[inline(always)]
    pub fn is_fllunlockhis_2(&self) -> bool {
        *self == FLLUNLOCKHIS_A::FLLUNLOCKHIS_2
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCKHIS_3`"]
    #[inline(always)]
    pub fn is_fllunlockhis_3(&self) -> bool {
        *self == FLLUNLOCKHIS_A::FLLUNLOCKHIS_3
    }
}
#[doc = "Write proxy for field `FLLUNLOCKHIS`"]
pub struct FLLUNLOCKHIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLUNLOCKHIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLLUNLOCKHIS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FLL is locked. No unlock situation has been detected since the last reset of these bits."]
    #[inline(always)]
    pub fn fllunlockhis_0(self) -> &'a mut W {
        self.variant(FLLUNLOCKHIS_A::FLLUNLOCKHIS_0)
    }
    #[doc = "DCOCLK has been too slow since the bits were cleared."]
    #[inline(always)]
    pub fn fllunlockhis_1(self) -> &'a mut W {
        self.variant(FLLUNLOCKHIS_A::FLLUNLOCKHIS_1)
    }
    #[doc = "DCOCLK has been too fast since the bits were cleared."]
    #[inline(always)]
    pub fn fllunlockhis_2(self) -> &'a mut W {
        self.variant(FLLUNLOCKHIS_A::FLLUNLOCKHIS_2)
    }
    #[doc = "DCOCLK has been both too fast and too slow since the bits were cleared."]
    #[inline(always)]
    pub fn fllunlockhis_3(self) -> &'a mut W {
        self.variant(FLLUNLOCKHIS_A::FLLUNLOCKHIS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "FLL unlock PUC enable. If the FLLULPUC bit is set, a reset (PUC) is triggered if FLLULIFG is set. FLLULIFG indicates when FLLUNLOCK bits equal 10 (too fast). FLLULPUC is automatically cleared upon servicing the event. If FLLULPUC is cleared (0), no PUC can be triggered by FLLULIFG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLULPUC_A {}
impl From<FLLULPUC_A> for bool {
    #[inline(always)]
    fn from(variant: FLLULPUC_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `FLLULPUC`"]
pub type FLLULPUC_R = crate::R<bool, FLLULPUC_A>;
impl FLLULPUC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FLLULPUC_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `FLLULPUC`"]
pub struct FLLULPUC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLULPUC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLLULPUC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Warning enable. If this bit is set, an interrupt is generated based on the FLLUNLOCKHIS bits. If FLLUNLOCKHIS is not equal to 00, an OFIFG is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLWARNEN_A {
    #[doc = "0: FLLUNLOCKHIS status cannot set OFIFG."]
    FLLWARNEN_0,
    #[doc = "1: FLLUNLOCKHIS status can set OFIFG."]
    FLLWARNEN_1,
}
impl From<FLLWARNEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLLWARNEN_A) -> Self {
        match variant {
            FLLWARNEN_A::FLLWARNEN_0 => false,
            FLLWARNEN_A::FLLWARNEN_1 => true,
        }
    }
}
#[doc = "Reader of field `FLLWARNEN`"]
pub type FLLWARNEN_R = crate::R<bool, FLLWARNEN_A>;
impl FLLWARNEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLWARNEN_A {
        match self.bits {
            false => FLLWARNEN_A::FLLWARNEN_0,
            true => FLLWARNEN_A::FLLWARNEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLLWARNEN_0`"]
    #[inline(always)]
    pub fn is_fllwarnen_0(&self) -> bool {
        *self == FLLWARNEN_A::FLLWARNEN_0
    }
    #[doc = "Checks if the value of the field is `FLLWARNEN_1`"]
    #[inline(always)]
    pub fn is_fllwarnen_1(&self) -> bool {
        *self == FLLWARNEN_A::FLLWARNEN_1
    }
}
#[doc = "Write proxy for field `FLLWARNEN`"]
pub struct FLLWARNEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLWARNEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLLWARNEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FLLUNLOCKHIS status cannot set OFIFG."]
    #[inline(always)]
    pub fn fllwarnen_0(self) -> &'a mut W {
        self.variant(FLLWARNEN_A::FLLWARNEN_0)
    }
    #[doc = "FLLUNLOCKHIS status can set OFIFG."]
    #[inline(always)]
    pub fn fllwarnen_1(self) -> &'a mut W {
        self.variant(FLLWARNEN_A::FLLWARNEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - REFO ready flag. This bit reflects the REFO readiness whent REFO is good for operation (such as FLL reference)"]
    #[inline(always)]
    pub fn refoready(&self) -> REFOREADY_R {
        REFOREADY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DCO fault flag. If this bit is set, the OFIFG flag is also set. The DCOFFG bit is set if DCO = {0} or DCO = {511}. DCOFFG can be cleared by software. If the DCO fault condition still remains, DCOFFG is set. As long as DCOFFG is set, FLLUNLOCK shows the DCOERROR condition."]
    #[inline(always)]
    pub fn dcoffg(&self) -> DCOFFG_R {
        DCOFFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - T1 oscillator fault flag. If this bit is set, the OFIFG flag is also set. XT1OFFG is set if a XT1 fault condition exists. XT1OFFG can be cleared by software. If the XT1 fault condition still remains, XT1OFFG is set."]
    #[inline(always)]
    pub fn xt1offg(&self) -> XT1OFFG_R {
        XT1OFFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FLL unlock interrupt flag. This flag is set when FLLUNLOCK bits equal 10b (DCO too fast). If FLLULPUC is also set, a PUC is triggered when FLLUIFG is set."]
    #[inline(always)]
    pub fn fllulifg(&self) -> FLLULIFG_R {
        FLLULIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable start counter for XT1."]
    #[inline(always)]
    pub fn enstfcnt1(&self) -> ENSTFCNT1_R {
        ENSTFCNT1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Unlock. These bits indicate the current FLL unlock condition. These bits are both set as long as the DCOFFG flag is set."]
    #[inline(always)]
    pub fn fllunlock(&self) -> FLLUNLOCK_R {
        FLLUNLOCK_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Unlock history bits. These bits indicate the FLL unlock condition history. As soon as any unlock condition happens, the respective bits are set and remain set until cleared by software by writing 0 to it or by a POR."]
    #[inline(always)]
    pub fn fllunlockhis(&self) -> FLLUNLOCKHIS_R {
        FLLUNLOCKHIS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - FLL unlock PUC enable. If the FLLULPUC bit is set, a reset (PUC) is triggered if FLLULIFG is set. FLLULIFG indicates when FLLUNLOCK bits equal 10 (too fast). FLLULPUC is automatically cleared upon servicing the event. If FLLULPUC is cleared (0), no PUC can be triggered by FLLULIFG."]
    #[inline(always)]
    pub fn fllulpuc(&self) -> FLLULPUC_R {
        FLLULPUC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Warning enable. If this bit is set, an interrupt is generated based on the FLLUNLOCKHIS bits. If FLLUNLOCKHIS is not equal to 00, an OFIFG is generated."]
    #[inline(always)]
    pub fn fllwarnen(&self) -> FLLWARNEN_R {
        FLLWARNEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCO fault flag. If this bit is set, the OFIFG flag is also set. The DCOFFG bit is set if DCO = {0} or DCO = {511}. DCOFFG can be cleared by software. If the DCO fault condition still remains, DCOFFG is set. As long as DCOFFG is set, FLLUNLOCK shows the DCOERROR condition."]
    #[inline(always)]
    pub fn dcoffg(&mut self) -> DCOFFG_W {
        DCOFFG_W { w: self }
    }
    #[doc = "Bit 1 - T1 oscillator fault flag. If this bit is set, the OFIFG flag is also set. XT1OFFG is set if a XT1 fault condition exists. XT1OFFG can be cleared by software. If the XT1 fault condition still remains, XT1OFFG is set."]
    #[inline(always)]
    pub fn xt1offg(&mut self) -> XT1OFFG_W {
        XT1OFFG_W { w: self }
    }
    #[doc = "Bit 4 - FLL unlock interrupt flag. This flag is set when FLLUNLOCK bits equal 10b (DCO too fast). If FLLULPUC is also set, a PUC is triggered when FLLUIFG is set."]
    #[inline(always)]
    pub fn fllulifg(&mut self) -> FLLULIFG_W {
        FLLULIFG_W { w: self }
    }
    #[doc = "Bit 6 - Enable start counter for XT1."]
    #[inline(always)]
    pub fn enstfcnt1(&mut self) -> ENSTFCNT1_W {
        ENSTFCNT1_W { w: self }
    }
    #[doc = "Bits 8:9 - Unlock. These bits indicate the current FLL unlock condition. These bits are both set as long as the DCOFFG flag is set."]
    #[inline(always)]
    pub fn fllunlock(&mut self) -> FLLUNLOCK_W {
        FLLUNLOCK_W { w: self }
    }
    #[doc = "Bits 10:11 - Unlock history bits. These bits indicate the FLL unlock condition history. As soon as any unlock condition happens, the respective bits are set and remain set until cleared by software by writing 0 to it or by a POR."]
    #[inline(always)]
    pub fn fllunlockhis(&mut self) -> FLLUNLOCKHIS_W {
        FLLUNLOCKHIS_W { w: self }
    }
    #[doc = "Bit 12 - FLL unlock PUC enable. If the FLLULPUC bit is set, a reset (PUC) is triggered if FLLULIFG is set. FLLULIFG indicates when FLLUNLOCK bits equal 10 (too fast). FLLULPUC is automatically cleared upon servicing the event. If FLLULPUC is cleared (0), no PUC can be triggered by FLLULIFG."]
    #[inline(always)]
    pub fn fllulpuc(&mut self) -> FLLULPUC_W {
        FLLULPUC_W { w: self }
    }
    #[doc = "Bit 13 - Warning enable. If this bit is set, an interrupt is generated based on the FLLUNLOCKHIS bits. If FLLUNLOCKHIS is not equal to 00, an OFIFG is generated."]
    #[inline(always)]
    pub fn fllwarnen(&mut self) -> FLLWARNEN_W {
        FLLWARNEN_W { w: self }
    }
}
