#[doc = "Register `CSCTL7` reader"]
pub struct R(crate::R<CSCTL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL7` writer"]
pub struct W(crate::W<CSCTL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL7_SPEC>;
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
impl From<crate::W<CSCTL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCOFFG` reader - DCO fault flag. If this bit is set, the OFIFG flag is also set. The DCOFFG bit is set if DCO = {0} or DCO = {511}. DCOFFG can be cleared by software. If the DCO fault condition still remains, DCOFFG is set. As long as DCOFFG is set, FLLUNLOCK shows the DCOERROR condition."]
pub type DCOFFG_R = crate::BitReader<DCOFFG_A>;
#[doc = "DCO fault flag. If this bit is set, the OFIFG flag is also set. The DCOFFG bit is set if DCO = {0} or DCO = {511}. DCOFFG can be cleared by software. If the DCO fault condition still remains, DCOFFG is set. As long as DCOFFG is set, FLLUNLOCK shows the DCOERROR condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCOFFG_A {
    #[doc = "0: No fault condition occurred after the last reset."]
    DCOFFG_0 = 0,
    #[doc = "1: DCO fault. A DCO fault occurred after the last reset."]
    DCOFFG_1 = 1,
}
impl From<DCOFFG_A> for bool {
    #[inline(always)]
    fn from(variant: DCOFFG_A) -> Self {
        variant as u8 != 0
    }
}
impl DCOFFG_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `DCOFFG` writer - DCO fault flag. If this bit is set, the OFIFG flag is also set. The DCOFFG bit is set if DCO = {0} or DCO = {511}. DCOFFG can be cleared by software. If the DCO fault condition still remains, DCOFFG is set. As long as DCOFFG is set, FLLUNLOCK shows the DCOERROR condition."]
pub type DCOFFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL7_SPEC, DCOFFG_A, O>;
impl<'a, const O: u8> DCOFFG_W<'a, O> {
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
}
#[doc = "Field `XT1OFFG` reader - T1 oscillator fault flag. If this bit is set, the OFIFG flag is also set. XT1OFFG is set if a XT1 fault condition exists. XT1OFFG can be cleared by software. If the XT1 fault condition still remains, XT1OFFG is set."]
pub type XT1OFFG_R = crate::BitReader<XT1OFFG_A>;
#[doc = "T1 oscillator fault flag. If this bit is set, the OFIFG flag is also set. XT1OFFG is set if a XT1 fault condition exists. XT1OFFG can be cleared by software. If the XT1 fault condition still remains, XT1OFFG is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XT1OFFG_A {
    #[doc = "0: No fault condition occurred after the last reset."]
    XT1OFFG_0 = 0,
    #[doc = "1: XT1 fault. An XT1 fault occurred after the last reset."]
    XT1OFFG_1 = 1,
}
impl From<XT1OFFG_A> for bool {
    #[inline(always)]
    fn from(variant: XT1OFFG_A) -> Self {
        variant as u8 != 0
    }
}
impl XT1OFFG_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `XT1OFFG` writer - T1 oscillator fault flag. If this bit is set, the OFIFG flag is also set. XT1OFFG is set if a XT1 fault condition exists. XT1OFFG can be cleared by software. If the XT1 fault condition still remains, XT1OFFG is set."]
pub type XT1OFFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL7_SPEC, XT1OFFG_A, O>;
impl<'a, const O: u8> XT1OFFG_W<'a, O> {
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
}
#[doc = "Field `REFOREADY` reader - REFO ready flag. This bit reflects the REFO readiness whent REFO is good for operation (such as FLL reference)"]
pub type REFOREADY_R = crate::BitReader<REFOREADY_A>;
#[doc = "REFO ready flag. This bit reflects the REFO readiness whent REFO is good for operation (such as FLL reference)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFOREADY_A {
    #[doc = "0: REFO unstable"]
    REFOREADY_0 = 0,
    #[doc = "1: REFO ready to go"]
    REFOREADY_1 = 1,
}
impl From<REFOREADY_A> for bool {
    #[inline(always)]
    fn from(variant: REFOREADY_A) -> Self {
        variant as u8 != 0
    }
}
impl REFOREADY_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `FLLULIFG` reader - FLL unlock interrupt flag. This flag is set when FLLUNLOCK bits equal 10b (DCO too fast). If FLLULPUC is also set, a PUC is triggered when FLLUIFG is set."]
pub type FLLULIFG_R = crate::BitReader<FLLULIFG_A>;
#[doc = "FLL unlock interrupt flag. This flag is set when FLLUNLOCK bits equal 10b (DCO too fast). If FLLULPUC is also set, a PUC is triggered when FLLUIFG is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLLULIFG_A {
    #[doc = "0: FLLUNLOCK bits not equal to 10b"]
    FLLULIFG_0 = 0,
    #[doc = "1: FLLUNLOCK bits equal to 10b"]
    FLLULIFG_1 = 1,
}
impl From<FLLULIFG_A> for bool {
    #[inline(always)]
    fn from(variant: FLLULIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl FLLULIFG_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `FLLULIFG` writer - FLL unlock interrupt flag. This flag is set when FLLUNLOCK bits equal 10b (DCO too fast). If FLLULPUC is also set, a PUC is triggered when FLLUIFG is set."]
pub type FLLULIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL7_SPEC, FLLULIFG_A, O>;
impl<'a, const O: u8> FLLULIFG_W<'a, O> {
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
}
#[doc = "Field `ENSTFCNT1` reader - Enable start counter for XT1."]
pub type ENSTFCNT1_R = crate::BitReader<ENSTFCNT1_A>;
#[doc = "Enable start counter for XT1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENSTFCNT1_A {
    #[doc = "0: Startup fault counter disabled. Counter is cleared.."]
    ENSTFCNT1_0 = 0,
    #[doc = "1: Startup fault counter enabled."]
    ENSTFCNT1_1 = 1,
}
impl From<ENSTFCNT1_A> for bool {
    #[inline(always)]
    fn from(variant: ENSTFCNT1_A) -> Self {
        variant as u8 != 0
    }
}
impl ENSTFCNT1_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `ENSTFCNT1` writer - Enable start counter for XT1."]
pub type ENSTFCNT1_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL7_SPEC, ENSTFCNT1_A, O>;
impl<'a, const O: u8> ENSTFCNT1_W<'a, O> {
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
}
#[doc = "Field `FLLUNLOCK` reader - Unlock. These bits indicate the current FLL unlock condition. These bits are both set as long as the DCOFFG flag is set."]
pub type FLLUNLOCK_R = crate::FieldReader<u8, FLLUNLOCK_A>;
#[doc = "Unlock. These bits indicate the current FLL unlock condition. These bits are both set as long as the DCOFFG flag is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLLUNLOCK_A {
    #[doc = "0: FLL is locked. No unlock condition currently active."]
    FLLUNLOCK_0 = 0,
    #[doc = "1: DCOCLK is currently too slow."]
    FLLUNLOCK_1 = 1,
    #[doc = "2: DCOCLK is currently too fast."]
    FLLUNLOCK_2 = 2,
    #[doc = "3: DCOERROR. DCO out of range."]
    FLLUNLOCK_3 = 3,
}
impl From<FLLUNLOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLUNLOCK_A) -> Self {
        variant as _
    }
}
impl FLLUNLOCK_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `FLLUNLOCK` writer - Unlock. These bits indicate the current FLL unlock condition. These bits are both set as long as the DCOFFG flag is set."]
pub type FLLUNLOCK_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CSCTL7_SPEC, u8, FLLUNLOCK_A, 2, O>;
impl<'a, const O: u8> FLLUNLOCK_W<'a, O> {
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
}
#[doc = "Field `FLLUNLOCKHIS` reader - Unlock history bits. These bits indicate the FLL unlock condition history. As soon as any unlock condition happens, the respective bits are set and remain set until cleared by software by writing 0 to it or by a POR."]
pub type FLLUNLOCKHIS_R = crate::FieldReader<u8, FLLUNLOCKHIS_A>;
#[doc = "Unlock history bits. These bits indicate the FLL unlock condition history. As soon as any unlock condition happens, the respective bits are set and remain set until cleared by software by writing 0 to it or by a POR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLLUNLOCKHIS_A {
    #[doc = "0: FLL is locked. No unlock situation has been detected since the last reset of these bits."]
    FLLUNLOCKHIS_0 = 0,
    #[doc = "1: DCOCLK has been too slow since the bits were cleared."]
    FLLUNLOCKHIS_1 = 1,
    #[doc = "2: DCOCLK has been too fast since the bits were cleared."]
    FLLUNLOCKHIS_2 = 2,
    #[doc = "3: DCOCLK has been both too fast and too slow since the bits were cleared."]
    FLLUNLOCKHIS_3 = 3,
}
impl From<FLLUNLOCKHIS_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLUNLOCKHIS_A) -> Self {
        variant as _
    }
}
impl FLLUNLOCKHIS_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `FLLUNLOCKHIS` writer - Unlock history bits. These bits indicate the FLL unlock condition history. As soon as any unlock condition happens, the respective bits are set and remain set until cleared by software by writing 0 to it or by a POR."]
pub type FLLUNLOCKHIS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CSCTL7_SPEC, u8, FLLUNLOCKHIS_A, 2, O>;
impl<'a, const O: u8> FLLUNLOCKHIS_W<'a, O> {
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
}
#[doc = "Field `FLLULPUC` reader - FLL unlock PUC enable. If the FLLULPUC bit is set, a reset (PUC) is triggered if FLLULIFG is set. FLLULIFG indicates when FLLUNLOCK bits equal 10 (too fast). FLLULPUC is automatically cleared upon servicing the event. If FLLULPUC is cleared (0), no PUC can be triggered by FLLULIFG."]
pub type FLLULPUC_R = crate::BitReader<bool>;
#[doc = "Field `FLLULPUC` writer - FLL unlock PUC enable. If the FLLULPUC bit is set, a reset (PUC) is triggered if FLLULIFG is set. FLLULIFG indicates when FLLUNLOCK bits equal 10 (too fast). FLLULPUC is automatically cleared upon servicing the event. If FLLULPUC is cleared (0), no PUC can be triggered by FLLULIFG."]
pub type FLLULPUC_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL7_SPEC, bool, O>;
#[doc = "Field `FLLWARNEN` reader - Warning enable. If this bit is set, an interrupt is generated based on the FLLUNLOCKHIS bits. If FLLUNLOCKHIS is not equal to 00, an OFIFG is generated."]
pub type FLLWARNEN_R = crate::BitReader<FLLWARNEN_A>;
#[doc = "Warning enable. If this bit is set, an interrupt is generated based on the FLLUNLOCKHIS bits. If FLLUNLOCKHIS is not equal to 00, an OFIFG is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLLWARNEN_A {
    #[doc = "0: FLLUNLOCKHIS status cannot set OFIFG."]
    FLLWARNEN_0 = 0,
    #[doc = "1: FLLUNLOCKHIS status can set OFIFG."]
    FLLWARNEN_1 = 1,
}
impl From<FLLWARNEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLLWARNEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FLLWARNEN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `FLLWARNEN` writer - Warning enable. If this bit is set, an interrupt is generated based on the FLLUNLOCKHIS bits. If FLLUNLOCKHIS is not equal to 00, an OFIFG is generated."]
pub type FLLWARNEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL7_SPEC, FLLWARNEN_A, O>;
impl<'a, const O: u8> FLLWARNEN_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0 - DCO fault flag. If this bit is set, the OFIFG flag is also set. The DCOFFG bit is set if DCO = {0} or DCO = {511}. DCOFFG can be cleared by software. If the DCO fault condition still remains, DCOFFG is set. As long as DCOFFG is set, FLLUNLOCK shows the DCOERROR condition."]
    #[inline(always)]
    pub fn dcoffg(&self) -> DCOFFG_R {
        DCOFFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - T1 oscillator fault flag. If this bit is set, the OFIFG flag is also set. XT1OFFG is set if a XT1 fault condition exists. XT1OFFG can be cleared by software. If the XT1 fault condition still remains, XT1OFFG is set."]
    #[inline(always)]
    pub fn xt1offg(&self) -> XT1OFFG_R {
        XT1OFFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REFO ready flag. This bit reflects the REFO readiness whent REFO is good for operation (such as FLL reference)"]
    #[inline(always)]
    pub fn refoready(&self) -> REFOREADY_R {
        REFOREADY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLL unlock interrupt flag. This flag is set when FLLUNLOCK bits equal 10b (DCO too fast). If FLLULPUC is also set, a PUC is triggered when FLLUIFG is set."]
    #[inline(always)]
    pub fn fllulifg(&self) -> FLLULIFG_R {
        FLLULIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable start counter for XT1."]
    #[inline(always)]
    pub fn enstfcnt1(&self) -> ENSTFCNT1_R {
        ENSTFCNT1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Unlock. These bits indicate the current FLL unlock condition. These bits are both set as long as the DCOFFG flag is set."]
    #[inline(always)]
    pub fn fllunlock(&self) -> FLLUNLOCK_R {
        FLLUNLOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Unlock history bits. These bits indicate the FLL unlock condition history. As soon as any unlock condition happens, the respective bits are set and remain set until cleared by software by writing 0 to it or by a POR."]
    #[inline(always)]
    pub fn fllunlockhis(&self) -> FLLUNLOCKHIS_R {
        FLLUNLOCKHIS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - FLL unlock PUC enable. If the FLLULPUC bit is set, a reset (PUC) is triggered if FLLULIFG is set. FLLULIFG indicates when FLLUNLOCK bits equal 10 (too fast). FLLULPUC is automatically cleared upon servicing the event. If FLLULPUC is cleared (0), no PUC can be triggered by FLLULIFG."]
    #[inline(always)]
    pub fn fllulpuc(&self) -> FLLULPUC_R {
        FLLULPUC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Warning enable. If this bit is set, an interrupt is generated based on the FLLUNLOCKHIS bits. If FLLUNLOCKHIS is not equal to 00, an OFIFG is generated."]
    #[inline(always)]
    pub fn fllwarnen(&self) -> FLLWARNEN_R {
        FLLWARNEN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCO fault flag. If this bit is set, the OFIFG flag is also set. The DCOFFG bit is set if DCO = {0} or DCO = {511}. DCOFFG can be cleared by software. If the DCO fault condition still remains, DCOFFG is set. As long as DCOFFG is set, FLLUNLOCK shows the DCOERROR condition."]
    #[inline(always)]
    pub fn dcoffg(&mut self) -> DCOFFG_W<0> {
        DCOFFG_W::new(self)
    }
    #[doc = "Bit 1 - T1 oscillator fault flag. If this bit is set, the OFIFG flag is also set. XT1OFFG is set if a XT1 fault condition exists. XT1OFFG can be cleared by software. If the XT1 fault condition still remains, XT1OFFG is set."]
    #[inline(always)]
    pub fn xt1offg(&mut self) -> XT1OFFG_W<1> {
        XT1OFFG_W::new(self)
    }
    #[doc = "Bit 4 - FLL unlock interrupt flag. This flag is set when FLLUNLOCK bits equal 10b (DCO too fast). If FLLULPUC is also set, a PUC is triggered when FLLUIFG is set."]
    #[inline(always)]
    pub fn fllulifg(&mut self) -> FLLULIFG_W<4> {
        FLLULIFG_W::new(self)
    }
    #[doc = "Bit 6 - Enable start counter for XT1."]
    #[inline(always)]
    pub fn enstfcnt1(&mut self) -> ENSTFCNT1_W<6> {
        ENSTFCNT1_W::new(self)
    }
    #[doc = "Bits 8:9 - Unlock. These bits indicate the current FLL unlock condition. These bits are both set as long as the DCOFFG flag is set."]
    #[inline(always)]
    pub fn fllunlock(&mut self) -> FLLUNLOCK_W<8> {
        FLLUNLOCK_W::new(self)
    }
    #[doc = "Bits 10:11 - Unlock history bits. These bits indicate the FLL unlock condition history. As soon as any unlock condition happens, the respective bits are set and remain set until cleared by software by writing 0 to it or by a POR."]
    #[inline(always)]
    pub fn fllunlockhis(&mut self) -> FLLUNLOCKHIS_W<10> {
        FLLUNLOCKHIS_W::new(self)
    }
    #[doc = "Bit 12 - FLL unlock PUC enable. If the FLLULPUC bit is set, a reset (PUC) is triggered if FLLULIFG is set. FLLULIFG indicates when FLLUNLOCK bits equal 10 (too fast). FLLULPUC is automatically cleared upon servicing the event. If FLLULPUC is cleared (0), no PUC can be triggered by FLLULIFG."]
    #[inline(always)]
    pub fn fllulpuc(&mut self) -> FLLULPUC_W<12> {
        FLLULPUC_W::new(self)
    }
    #[doc = "Bit 13 - Warning enable. If this bit is set, an interrupt is generated based on the FLLUNLOCKHIS bits. If FLLUNLOCKHIS is not equal to 00, an OFIFG is generated."]
    #[inline(always)]
    pub fn fllwarnen(&mut self) -> FLLWARNEN_W<13> {
        FLLWARNEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock System Control Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl7](index.html) module"]
pub struct CSCTL7_SPEC;
impl crate::RegisterSpec for CSCTL7_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl7::R](R) reader structure"]
impl crate::Readable for CSCTL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl7::W](W) writer structure"]
impl crate::Writable for CSCTL7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL7 to value 0"]
impl crate::Resettable for CSCTL7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
