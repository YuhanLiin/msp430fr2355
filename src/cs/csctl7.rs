#[doc = "Register `CSCTL7` reader"]
pub type R = crate::R<Csctl7Spec>;
#[doc = "Register `CSCTL7` writer"]
pub type W = crate::W<Csctl7Spec>;
#[doc = "DCO fault flag. If this bit is set, the OFIFG flag is also set. The DCOFFG bit is set if DCO = {0} or DCO = {511}. DCOFFG can be cleared by software. If the DCO fault condition still remains, DCOFFG is set. As long as DCOFFG is set, FLLUNLOCK shows the DCOERROR condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcoffg {
    #[doc = "0: No fault condition occurred after the last reset."]
    Dcoffg0 = 0,
    #[doc = "1: DCO fault. A DCO fault occurred after the last reset."]
    Dcoffg1 = 1,
}
impl From<Dcoffg> for bool {
    #[inline(always)]
    fn from(variant: Dcoffg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCOFFG` reader - DCO fault flag. If this bit is set, the OFIFG flag is also set. The DCOFFG bit is set if DCO = {0} or DCO = {511}. DCOFFG can be cleared by software. If the DCO fault condition still remains, DCOFFG is set. As long as DCOFFG is set, FLLUNLOCK shows the DCOERROR condition."]
pub type DcoffgR = crate::BitReader<Dcoffg>;
impl DcoffgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcoffg {
        match self.bits {
            false => Dcoffg::Dcoffg0,
            true => Dcoffg::Dcoffg1,
        }
    }
    #[doc = "No fault condition occurred after the last reset."]
    #[inline(always)]
    pub fn is_dcoffg_0(&self) -> bool {
        *self == Dcoffg::Dcoffg0
    }
    #[doc = "DCO fault. A DCO fault occurred after the last reset."]
    #[inline(always)]
    pub fn is_dcoffg_1(&self) -> bool {
        *self == Dcoffg::Dcoffg1
    }
}
#[doc = "Field `DCOFFG` writer - DCO fault flag. If this bit is set, the OFIFG flag is also set. The DCOFFG bit is set if DCO = {0} or DCO = {511}. DCOFFG can be cleared by software. If the DCO fault condition still remains, DCOFFG is set. As long as DCOFFG is set, FLLUNLOCK shows the DCOERROR condition."]
pub type DcoffgW<'a, REG> = crate::BitWriter<'a, REG, Dcoffg>;
impl<'a, REG> DcoffgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No fault condition occurred after the last reset."]
    #[inline(always)]
    pub fn dcoffg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcoffg::Dcoffg0)
    }
    #[doc = "DCO fault. A DCO fault occurred after the last reset."]
    #[inline(always)]
    pub fn dcoffg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcoffg::Dcoffg1)
    }
}
#[doc = "T1 oscillator fault flag. If this bit is set, the OFIFG flag is also set. XT1OFFG is set if a XT1 fault condition exists. XT1OFFG can be cleared by software. If the XT1 fault condition still remains, XT1OFFG is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xt1offg {
    #[doc = "0: No fault condition occurred after the last reset."]
    Xt1offg0 = 0,
    #[doc = "1: XT1 fault. An XT1 fault occurred after the last reset."]
    Xt1offg1 = 1,
}
impl From<Xt1offg> for bool {
    #[inline(always)]
    fn from(variant: Xt1offg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XT1OFFG` reader - T1 oscillator fault flag. If this bit is set, the OFIFG flag is also set. XT1OFFG is set if a XT1 fault condition exists. XT1OFFG can be cleared by software. If the XT1 fault condition still remains, XT1OFFG is set."]
pub type Xt1offgR = crate::BitReader<Xt1offg>;
impl Xt1offgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xt1offg {
        match self.bits {
            false => Xt1offg::Xt1offg0,
            true => Xt1offg::Xt1offg1,
        }
    }
    #[doc = "No fault condition occurred after the last reset."]
    #[inline(always)]
    pub fn is_xt1offg_0(&self) -> bool {
        *self == Xt1offg::Xt1offg0
    }
    #[doc = "XT1 fault. An XT1 fault occurred after the last reset."]
    #[inline(always)]
    pub fn is_xt1offg_1(&self) -> bool {
        *self == Xt1offg::Xt1offg1
    }
}
#[doc = "Field `XT1OFFG` writer - T1 oscillator fault flag. If this bit is set, the OFIFG flag is also set. XT1OFFG is set if a XT1 fault condition exists. XT1OFFG can be cleared by software. If the XT1 fault condition still remains, XT1OFFG is set."]
pub type Xt1offgW<'a, REG> = crate::BitWriter<'a, REG, Xt1offg>;
impl<'a, REG> Xt1offgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No fault condition occurred after the last reset."]
    #[inline(always)]
    pub fn xt1offg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1offg::Xt1offg0)
    }
    #[doc = "XT1 fault. An XT1 fault occurred after the last reset."]
    #[inline(always)]
    pub fn xt1offg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1offg::Xt1offg1)
    }
}
#[doc = "REFO ready flag. This bit reflects the REFO readiness whent REFO is good for operation (such as FLL reference)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refoready {
    #[doc = "0: REFO unstable"]
    Refoready0 = 0,
    #[doc = "1: REFO ready to go"]
    Refoready1 = 1,
}
impl From<Refoready> for bool {
    #[inline(always)]
    fn from(variant: Refoready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFOREADY` reader - REFO ready flag. This bit reflects the REFO readiness whent REFO is good for operation (such as FLL reference)"]
pub type ReforeadyR = crate::BitReader<Refoready>;
impl ReforeadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refoready {
        match self.bits {
            false => Refoready::Refoready0,
            true => Refoready::Refoready1,
        }
    }
    #[doc = "REFO unstable"]
    #[inline(always)]
    pub fn is_refoready_0(&self) -> bool {
        *self == Refoready::Refoready0
    }
    #[doc = "REFO ready to go"]
    #[inline(always)]
    pub fn is_refoready_1(&self) -> bool {
        *self == Refoready::Refoready1
    }
}
#[doc = "FLL unlock interrupt flag. This flag is set when FLLUNLOCK bits equal 10b (DCO too fast). If FLLULPUC is also set, a PUC is triggered when FLLUIFG is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fllulifg {
    #[doc = "0: FLLUNLOCK bits not equal to 10b"]
    Fllulifg0 = 0,
    #[doc = "1: FLLUNLOCK bits equal to 10b"]
    Fllulifg1 = 1,
}
impl From<Fllulifg> for bool {
    #[inline(always)]
    fn from(variant: Fllulifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLLULIFG` reader - FLL unlock interrupt flag. This flag is set when FLLUNLOCK bits equal 10b (DCO too fast). If FLLULPUC is also set, a PUC is triggered when FLLUIFG is set."]
pub type FllulifgR = crate::BitReader<Fllulifg>;
impl FllulifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fllulifg {
        match self.bits {
            false => Fllulifg::Fllulifg0,
            true => Fllulifg::Fllulifg1,
        }
    }
    #[doc = "FLLUNLOCK bits not equal to 10b"]
    #[inline(always)]
    pub fn is_fllulifg_0(&self) -> bool {
        *self == Fllulifg::Fllulifg0
    }
    #[doc = "FLLUNLOCK bits equal to 10b"]
    #[inline(always)]
    pub fn is_fllulifg_1(&self) -> bool {
        *self == Fllulifg::Fllulifg1
    }
}
#[doc = "Field `FLLULIFG` writer - FLL unlock interrupt flag. This flag is set when FLLUNLOCK bits equal 10b (DCO too fast). If FLLULPUC is also set, a PUC is triggered when FLLUIFG is set."]
pub type FllulifgW<'a, REG> = crate::BitWriter<'a, REG, Fllulifg>;
impl<'a, REG> FllulifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FLLUNLOCK bits not equal to 10b"]
    #[inline(always)]
    pub fn fllulifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fllulifg::Fllulifg0)
    }
    #[doc = "FLLUNLOCK bits equal to 10b"]
    #[inline(always)]
    pub fn fllulifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fllulifg::Fllulifg1)
    }
}
#[doc = "Enable start counter for XT1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enstfcnt1 {
    #[doc = "0: Startup fault counter disabled. Counter is cleared.."]
    Enstfcnt1_0 = 0,
    #[doc = "1: Startup fault counter enabled."]
    Enstfcnt1_1 = 1,
}
impl From<Enstfcnt1> for bool {
    #[inline(always)]
    fn from(variant: Enstfcnt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENSTFCNT1` reader - Enable start counter for XT1."]
pub type Enstfcnt1R = crate::BitReader<Enstfcnt1>;
impl Enstfcnt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enstfcnt1 {
        match self.bits {
            false => Enstfcnt1::Enstfcnt1_0,
            true => Enstfcnt1::Enstfcnt1_1,
        }
    }
    #[doc = "Startup fault counter disabled. Counter is cleared.."]
    #[inline(always)]
    pub fn is_enstfcnt1_0(&self) -> bool {
        *self == Enstfcnt1::Enstfcnt1_0
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn is_enstfcnt1_1(&self) -> bool {
        *self == Enstfcnt1::Enstfcnt1_1
    }
}
#[doc = "Field `ENSTFCNT1` writer - Enable start counter for XT1."]
pub type Enstfcnt1W<'a, REG> = crate::BitWriter<'a, REG, Enstfcnt1>;
impl<'a, REG> Enstfcnt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Startup fault counter disabled. Counter is cleared.."]
    #[inline(always)]
    pub fn enstfcnt1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enstfcnt1::Enstfcnt1_0)
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn enstfcnt1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enstfcnt1::Enstfcnt1_1)
    }
}
#[doc = "Unlock. These bits indicate the current FLL unlock condition. These bits are both set as long as the DCOFFG flag is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fllunlock {
    #[doc = "0: FLL is locked. No unlock condition currently active."]
    Fllunlock0 = 0,
    #[doc = "1: DCOCLK is currently too slow."]
    Fllunlock1 = 1,
    #[doc = "2: DCOCLK is currently too fast."]
    Fllunlock2 = 2,
    #[doc = "3: DCOERROR. DCO out of range."]
    Fllunlock3 = 3,
}
impl From<Fllunlock> for u8 {
    #[inline(always)]
    fn from(variant: Fllunlock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fllunlock {
    type Ux = u8;
}
impl crate::IsEnum for Fllunlock {}
#[doc = "Field `FLLUNLOCK` reader - Unlock. These bits indicate the current FLL unlock condition. These bits are both set as long as the DCOFFG flag is set."]
pub type FllunlockR = crate::FieldReader<Fllunlock>;
impl FllunlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fllunlock {
        match self.bits {
            0 => Fllunlock::Fllunlock0,
            1 => Fllunlock::Fllunlock1,
            2 => Fllunlock::Fllunlock2,
            3 => Fllunlock::Fllunlock3,
            _ => unreachable!(),
        }
    }
    #[doc = "FLL is locked. No unlock condition currently active."]
    #[inline(always)]
    pub fn is_fllunlock_0(&self) -> bool {
        *self == Fllunlock::Fllunlock0
    }
    #[doc = "DCOCLK is currently too slow."]
    #[inline(always)]
    pub fn is_fllunlock_1(&self) -> bool {
        *self == Fllunlock::Fllunlock1
    }
    #[doc = "DCOCLK is currently too fast."]
    #[inline(always)]
    pub fn is_fllunlock_2(&self) -> bool {
        *self == Fllunlock::Fllunlock2
    }
    #[doc = "DCOERROR. DCO out of range."]
    #[inline(always)]
    pub fn is_fllunlock_3(&self) -> bool {
        *self == Fllunlock::Fllunlock3
    }
}
#[doc = "Field `FLLUNLOCK` writer - Unlock. These bits indicate the current FLL unlock condition. These bits are both set as long as the DCOFFG flag is set."]
pub type FllunlockW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fllunlock, crate::Safe>;
impl<'a, REG> FllunlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FLL is locked. No unlock condition currently active."]
    #[inline(always)]
    pub fn fllunlock_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlock::Fllunlock0)
    }
    #[doc = "DCOCLK is currently too slow."]
    #[inline(always)]
    pub fn fllunlock_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlock::Fllunlock1)
    }
    #[doc = "DCOCLK is currently too fast."]
    #[inline(always)]
    pub fn fllunlock_2(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlock::Fllunlock2)
    }
    #[doc = "DCOERROR. DCO out of range."]
    #[inline(always)]
    pub fn fllunlock_3(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlock::Fllunlock3)
    }
}
#[doc = "Unlock history bits. These bits indicate the FLL unlock condition history. As soon as any unlock condition happens, the respective bits are set and remain set until cleared by software by writing 0 to it or by a POR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fllunlockhis {
    #[doc = "0: FLL is locked. No unlock situation has been detected since the last reset of these bits."]
    Fllunlockhis0 = 0,
    #[doc = "1: DCOCLK has been too slow since the bits were cleared."]
    Fllunlockhis1 = 1,
    #[doc = "2: DCOCLK has been too fast since the bits were cleared."]
    Fllunlockhis2 = 2,
    #[doc = "3: DCOCLK has been both too fast and too slow since the bits were cleared."]
    Fllunlockhis3 = 3,
}
impl From<Fllunlockhis> for u8 {
    #[inline(always)]
    fn from(variant: Fllunlockhis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fllunlockhis {
    type Ux = u8;
}
impl crate::IsEnum for Fllunlockhis {}
#[doc = "Field `FLLUNLOCKHIS` reader - Unlock history bits. These bits indicate the FLL unlock condition history. As soon as any unlock condition happens, the respective bits are set and remain set until cleared by software by writing 0 to it or by a POR."]
pub type FllunlockhisR = crate::FieldReader<Fllunlockhis>;
impl FllunlockhisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fllunlockhis {
        match self.bits {
            0 => Fllunlockhis::Fllunlockhis0,
            1 => Fllunlockhis::Fllunlockhis1,
            2 => Fllunlockhis::Fllunlockhis2,
            3 => Fllunlockhis::Fllunlockhis3,
            _ => unreachable!(),
        }
    }
    #[doc = "FLL is locked. No unlock situation has been detected since the last reset of these bits."]
    #[inline(always)]
    pub fn is_fllunlockhis_0(&self) -> bool {
        *self == Fllunlockhis::Fllunlockhis0
    }
    #[doc = "DCOCLK has been too slow since the bits were cleared."]
    #[inline(always)]
    pub fn is_fllunlockhis_1(&self) -> bool {
        *self == Fllunlockhis::Fllunlockhis1
    }
    #[doc = "DCOCLK has been too fast since the bits were cleared."]
    #[inline(always)]
    pub fn is_fllunlockhis_2(&self) -> bool {
        *self == Fllunlockhis::Fllunlockhis2
    }
    #[doc = "DCOCLK has been both too fast and too slow since the bits were cleared."]
    #[inline(always)]
    pub fn is_fllunlockhis_3(&self) -> bool {
        *self == Fllunlockhis::Fllunlockhis3
    }
}
#[doc = "Field `FLLUNLOCKHIS` writer - Unlock history bits. These bits indicate the FLL unlock condition history. As soon as any unlock condition happens, the respective bits are set and remain set until cleared by software by writing 0 to it or by a POR."]
pub type FllunlockhisW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fllunlockhis, crate::Safe>;
impl<'a, REG> FllunlockhisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FLL is locked. No unlock situation has been detected since the last reset of these bits."]
    #[inline(always)]
    pub fn fllunlockhis_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlockhis::Fllunlockhis0)
    }
    #[doc = "DCOCLK has been too slow since the bits were cleared."]
    #[inline(always)]
    pub fn fllunlockhis_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlockhis::Fllunlockhis1)
    }
    #[doc = "DCOCLK has been too fast since the bits were cleared."]
    #[inline(always)]
    pub fn fllunlockhis_2(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlockhis::Fllunlockhis2)
    }
    #[doc = "DCOCLK has been both too fast and too slow since the bits were cleared."]
    #[inline(always)]
    pub fn fllunlockhis_3(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlockhis::Fllunlockhis3)
    }
}
#[doc = "Field `FLLULPUC` reader - FLL unlock PUC enable. If the FLLULPUC bit is set, a reset (PUC) is triggered if FLLULIFG is set. FLLULIFG indicates when FLLUNLOCK bits equal 10 (too fast). FLLULPUC is automatically cleared upon servicing the event. If FLLULPUC is cleared (0), no PUC can be triggered by FLLULIFG."]
pub type FllulpucR = crate::BitReader;
#[doc = "Field `FLLULPUC` writer - FLL unlock PUC enable. If the FLLULPUC bit is set, a reset (PUC) is triggered if FLLULIFG is set. FLLULIFG indicates when FLLUNLOCK bits equal 10 (too fast). FLLULPUC is automatically cleared upon servicing the event. If FLLULPUC is cleared (0), no PUC can be triggered by FLLULIFG."]
pub type FllulpucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Warning enable. If this bit is set, an interrupt is generated based on the FLLUNLOCKHIS bits. If FLLUNLOCKHIS is not equal to 00, an OFIFG is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fllwarnen {
    #[doc = "0: FLLUNLOCKHIS status cannot set OFIFG."]
    Fllwarnen0 = 0,
    #[doc = "1: FLLUNLOCKHIS status can set OFIFG."]
    Fllwarnen1 = 1,
}
impl From<Fllwarnen> for bool {
    #[inline(always)]
    fn from(variant: Fllwarnen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLLWARNEN` reader - Warning enable. If this bit is set, an interrupt is generated based on the FLLUNLOCKHIS bits. If FLLUNLOCKHIS is not equal to 00, an OFIFG is generated."]
pub type FllwarnenR = crate::BitReader<Fllwarnen>;
impl FllwarnenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fllwarnen {
        match self.bits {
            false => Fllwarnen::Fllwarnen0,
            true => Fllwarnen::Fllwarnen1,
        }
    }
    #[doc = "FLLUNLOCKHIS status cannot set OFIFG."]
    #[inline(always)]
    pub fn is_fllwarnen_0(&self) -> bool {
        *self == Fllwarnen::Fllwarnen0
    }
    #[doc = "FLLUNLOCKHIS status can set OFIFG."]
    #[inline(always)]
    pub fn is_fllwarnen_1(&self) -> bool {
        *self == Fllwarnen::Fllwarnen1
    }
}
#[doc = "Field `FLLWARNEN` writer - Warning enable. If this bit is set, an interrupt is generated based on the FLLUNLOCKHIS bits. If FLLUNLOCKHIS is not equal to 00, an OFIFG is generated."]
pub type FllwarnenW<'a, REG> = crate::BitWriter<'a, REG, Fllwarnen>;
impl<'a, REG> FllwarnenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FLLUNLOCKHIS status cannot set OFIFG."]
    #[inline(always)]
    pub fn fllwarnen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fllwarnen::Fllwarnen0)
    }
    #[doc = "FLLUNLOCKHIS status can set OFIFG."]
    #[inline(always)]
    pub fn fllwarnen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fllwarnen::Fllwarnen1)
    }
}
impl R {
    #[doc = "Bit 0 - DCO fault flag. If this bit is set, the OFIFG flag is also set. The DCOFFG bit is set if DCO = {0} or DCO = {511}. DCOFFG can be cleared by software. If the DCO fault condition still remains, DCOFFG is set. As long as DCOFFG is set, FLLUNLOCK shows the DCOERROR condition."]
    #[inline(always)]
    pub fn dcoffg(&self) -> DcoffgR {
        DcoffgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - T1 oscillator fault flag. If this bit is set, the OFIFG flag is also set. XT1OFFG is set if a XT1 fault condition exists. XT1OFFG can be cleared by software. If the XT1 fault condition still remains, XT1OFFG is set."]
    #[inline(always)]
    pub fn xt1offg(&self) -> Xt1offgR {
        Xt1offgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REFO ready flag. This bit reflects the REFO readiness whent REFO is good for operation (such as FLL reference)"]
    #[inline(always)]
    pub fn refoready(&self) -> ReforeadyR {
        ReforeadyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLL unlock interrupt flag. This flag is set when FLLUNLOCK bits equal 10b (DCO too fast). If FLLULPUC is also set, a PUC is triggered when FLLUIFG is set."]
    #[inline(always)]
    pub fn fllulifg(&self) -> FllulifgR {
        FllulifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable start counter for XT1."]
    #[inline(always)]
    pub fn enstfcnt1(&self) -> Enstfcnt1R {
        Enstfcnt1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Unlock. These bits indicate the current FLL unlock condition. These bits are both set as long as the DCOFFG flag is set."]
    #[inline(always)]
    pub fn fllunlock(&self) -> FllunlockR {
        FllunlockR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Unlock history bits. These bits indicate the FLL unlock condition history. As soon as any unlock condition happens, the respective bits are set and remain set until cleared by software by writing 0 to it or by a POR."]
    #[inline(always)]
    pub fn fllunlockhis(&self) -> FllunlockhisR {
        FllunlockhisR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - FLL unlock PUC enable. If the FLLULPUC bit is set, a reset (PUC) is triggered if FLLULIFG is set. FLLULIFG indicates when FLLUNLOCK bits equal 10 (too fast). FLLULPUC is automatically cleared upon servicing the event. If FLLULPUC is cleared (0), no PUC can be triggered by FLLULIFG."]
    #[inline(always)]
    pub fn fllulpuc(&self) -> FllulpucR {
        FllulpucR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Warning enable. If this bit is set, an interrupt is generated based on the FLLUNLOCKHIS bits. If FLLUNLOCKHIS is not equal to 00, an OFIFG is generated."]
    #[inline(always)]
    pub fn fllwarnen(&self) -> FllwarnenR {
        FllwarnenR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCO fault flag. If this bit is set, the OFIFG flag is also set. The DCOFFG bit is set if DCO = {0} or DCO = {511}. DCOFFG can be cleared by software. If the DCO fault condition still remains, DCOFFG is set. As long as DCOFFG is set, FLLUNLOCK shows the DCOERROR condition."]
    #[inline(always)]
    pub fn dcoffg(&mut self) -> DcoffgW<'_, Csctl7Spec> {
        DcoffgW::new(self, 0)
    }
    #[doc = "Bit 1 - T1 oscillator fault flag. If this bit is set, the OFIFG flag is also set. XT1OFFG is set if a XT1 fault condition exists. XT1OFFG can be cleared by software. If the XT1 fault condition still remains, XT1OFFG is set."]
    #[inline(always)]
    pub fn xt1offg(&mut self) -> Xt1offgW<'_, Csctl7Spec> {
        Xt1offgW::new(self, 1)
    }
    #[doc = "Bit 4 - FLL unlock interrupt flag. This flag is set when FLLUNLOCK bits equal 10b (DCO too fast). If FLLULPUC is also set, a PUC is triggered when FLLUIFG is set."]
    #[inline(always)]
    pub fn fllulifg(&mut self) -> FllulifgW<'_, Csctl7Spec> {
        FllulifgW::new(self, 4)
    }
    #[doc = "Bit 6 - Enable start counter for XT1."]
    #[inline(always)]
    pub fn enstfcnt1(&mut self) -> Enstfcnt1W<'_, Csctl7Spec> {
        Enstfcnt1W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Unlock. These bits indicate the current FLL unlock condition. These bits are both set as long as the DCOFFG flag is set."]
    #[inline(always)]
    pub fn fllunlock(&mut self) -> FllunlockW<'_, Csctl7Spec> {
        FllunlockW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Unlock history bits. These bits indicate the FLL unlock condition history. As soon as any unlock condition happens, the respective bits are set and remain set until cleared by software by writing 0 to it or by a POR."]
    #[inline(always)]
    pub fn fllunlockhis(&mut self) -> FllunlockhisW<'_, Csctl7Spec> {
        FllunlockhisW::new(self, 10)
    }
    #[doc = "Bit 12 - FLL unlock PUC enable. If the FLLULPUC bit is set, a reset (PUC) is triggered if FLLULIFG is set. FLLULIFG indicates when FLLUNLOCK bits equal 10 (too fast). FLLULPUC is automatically cleared upon servicing the event. If FLLULPUC is cleared (0), no PUC can be triggered by FLLULIFG."]
    #[inline(always)]
    pub fn fllulpuc(&mut self) -> FllulpucW<'_, Csctl7Spec> {
        FllulpucW::new(self, 12)
    }
    #[doc = "Bit 13 - Warning enable. If this bit is set, an interrupt is generated based on the FLLUNLOCKHIS bits. If FLLUNLOCKHIS is not equal to 00, an OFIFG is generated."]
    #[inline(always)]
    pub fn fllwarnen(&mut self) -> FllwarnenW<'_, Csctl7Spec> {
        FllwarnenW::new(self, 13)
    }
}
#[doc = "Clock System Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl7Spec;
impl crate::RegisterSpec for Csctl7Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl7::R`](R) reader structure"]
impl crate::Readable for Csctl7Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl7::W`](W) writer structure"]
impl crate::Writable for Csctl7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL7 to value 0"]
impl crate::Resettable for Csctl7Spec {}
