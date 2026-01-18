#[doc = "Register `GCCTL0` reader"]
pub type R = crate::R<Gcctl0Spec>;
#[doc = "Register `GCCTL0` writer"]
pub type W = crate::W<Gcctl0Spec>;
#[doc = "Enables FRAM auto power up after LPM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frlpmpwr {
    #[doc = "0: FRAM startup is delayed to the first FRAM access after exit from LPM"]
    Frlpmpwr0 = 0,
    #[doc = "1: FRAM is powered up immediately on exit from LPM"]
    Frlpmpwr1 = 1,
}
impl From<Frlpmpwr> for bool {
    #[inline(always)]
    fn from(variant: Frlpmpwr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRLPMPWR` reader - Enables FRAM auto power up after LPM"]
pub type FrlpmpwrR = crate::BitReader<Frlpmpwr>;
impl FrlpmpwrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frlpmpwr {
        match self.bits {
            false => Frlpmpwr::Frlpmpwr0,
            true => Frlpmpwr::Frlpmpwr1,
        }
    }
    #[doc = "FRAM startup is delayed to the first FRAM access after exit from LPM"]
    #[inline(always)]
    pub fn is_frlpmpwr_0(&self) -> bool {
        *self == Frlpmpwr::Frlpmpwr0
    }
    #[doc = "FRAM is powered up immediately on exit from LPM"]
    #[inline(always)]
    pub fn is_frlpmpwr_1(&self) -> bool {
        *self == Frlpmpwr::Frlpmpwr1
    }
}
#[doc = "Field `FRLPMPWR` writer - Enables FRAM auto power up after LPM"]
pub type FrlpmpwrW<'a, REG> = crate::BitWriter<'a, REG, Frlpmpwr>;
impl<'a, REG> FrlpmpwrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FRAM startup is delayed to the first FRAM access after exit from LPM"]
    #[inline(always)]
    pub fn frlpmpwr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Frlpmpwr::Frlpmpwr0)
    }
    #[doc = "FRAM is powered up immediately on exit from LPM"]
    #[inline(always)]
    pub fn frlpmpwr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Frlpmpwr::Frlpmpwr1)
    }
}
#[doc = "FRAM Memory Power Control Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frpwr {
    #[doc = "0: Enable INACTIVE mode."]
    Frpwr0 = 0,
    #[doc = "1: Enable ACTIVE mode."]
    Frpwr1 = 1,
}
impl From<Frpwr> for bool {
    #[inline(always)]
    fn from(variant: Frpwr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRPWR` reader - FRAM Memory Power Control Request"]
pub type FrpwrR = crate::BitReader<Frpwr>;
impl FrpwrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frpwr {
        match self.bits {
            false => Frpwr::Frpwr0,
            true => Frpwr::Frpwr1,
        }
    }
    #[doc = "Enable INACTIVE mode."]
    #[inline(always)]
    pub fn is_frpwr_0(&self) -> bool {
        *self == Frpwr::Frpwr0
    }
    #[doc = "Enable ACTIVE mode."]
    #[inline(always)]
    pub fn is_frpwr_1(&self) -> bool {
        *self == Frpwr::Frpwr1
    }
}
#[doc = "Field `FRPWR` writer - FRAM Memory Power Control Request"]
pub type FrpwrW<'a, REG> = crate::BitWriter<'a, REG, Frpwr>;
impl<'a, REG> FrpwrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable INACTIVE mode."]
    #[inline(always)]
    pub fn frpwr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Frpwr::Frpwr0)
    }
    #[doc = "Enable ACTIVE mode."]
    #[inline(always)]
    pub fn frpwr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Frpwr::Frpwr1)
    }
}
#[doc = "Enable NMI event for the correctable bit error detection flag (CBDIFG)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cbdie {
    #[doc = "0: Disable NMI for the correctable bit error detection flag (CBDIFG)."]
    Cbdie0 = 0,
    #[doc = "1: Disable NMI for the correctable bit error detection flag (CBDIFG). Generates vector in SYSSNIV."]
    Cbdie1 = 1,
}
impl From<Cbdie> for bool {
    #[inline(always)]
    fn from(variant: Cbdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBDIE` reader - Enable NMI event for the correctable bit error detection flag (CBDIFG)"]
pub type CbdieR = crate::BitReader<Cbdie>;
impl CbdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cbdie {
        match self.bits {
            false => Cbdie::Cbdie0,
            true => Cbdie::Cbdie1,
        }
    }
    #[doc = "Disable NMI for the correctable bit error detection flag (CBDIFG)."]
    #[inline(always)]
    pub fn is_cbdie_0(&self) -> bool {
        *self == Cbdie::Cbdie0
    }
    #[doc = "Disable NMI for the correctable bit error detection flag (CBDIFG). Generates vector in SYSSNIV."]
    #[inline(always)]
    pub fn is_cbdie_1(&self) -> bool {
        *self == Cbdie::Cbdie1
    }
}
#[doc = "Field `CBDIE` writer - Enable NMI event for the correctable bit error detection flag (CBDIFG)"]
pub type CbdieW<'a, REG> = crate::BitWriter<'a, REG, Cbdie>;
impl<'a, REG> CbdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable NMI for the correctable bit error detection flag (CBDIFG)."]
    #[inline(always)]
    pub fn cbdie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cbdie::Cbdie0)
    }
    #[doc = "Disable NMI for the correctable bit error detection flag (CBDIFG). Generates vector in SYSSNIV."]
    #[inline(always)]
    pub fn cbdie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cbdie::Cbdie1)
    }
}
#[doc = "Enable NMI event for the uncorrectable bit error detection flag (UBDIFG)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ubdie {
    #[doc = "0: Disable NMI for the uncorrectable bit error detection flag (UBDIFG)."]
    Ubdie0 = 0,
    #[doc = "1: Enable NMI for the uncorrectable bit error detection flag (UBDIFG). Generates vector in SYSSNIV. Clear the UBDRSTEN bit."]
    Ubdie1 = 1,
}
impl From<Ubdie> for bool {
    #[inline(always)]
    fn from(variant: Ubdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UBDIE` reader - Enable NMI event for the uncorrectable bit error detection flag (UBDIFG)"]
pub type UbdieR = crate::BitReader<Ubdie>;
impl UbdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ubdie {
        match self.bits {
            false => Ubdie::Ubdie0,
            true => Ubdie::Ubdie1,
        }
    }
    #[doc = "Disable NMI for the uncorrectable bit error detection flag (UBDIFG)."]
    #[inline(always)]
    pub fn is_ubdie_0(&self) -> bool {
        *self == Ubdie::Ubdie0
    }
    #[doc = "Enable NMI for the uncorrectable bit error detection flag (UBDIFG). Generates vector in SYSSNIV. Clear the UBDRSTEN bit."]
    #[inline(always)]
    pub fn is_ubdie_1(&self) -> bool {
        *self == Ubdie::Ubdie1
    }
}
#[doc = "Field `UBDIE` writer - Enable NMI event for the uncorrectable bit error detection flag (UBDIFG)"]
pub type UbdieW<'a, REG> = crate::BitWriter<'a, REG, Ubdie>;
impl<'a, REG> UbdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable NMI for the uncorrectable bit error detection flag (UBDIFG)."]
    #[inline(always)]
    pub fn ubdie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ubdie::Ubdie0)
    }
    #[doc = "Enable NMI for the uncorrectable bit error detection flag (UBDIFG). Generates vector in SYSSNIV. Clear the UBDRSTEN bit."]
    #[inline(always)]
    pub fn ubdie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ubdie::Ubdie1)
    }
}
#[doc = "Enable Power Up Clear (PUC) reset for the uncorrectable bit error detection flag (UBDIFG)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ubdrsten {
    #[doc = "0: PUC not initiated on uncorrectable bit error detection flag."]
    Ubdrsten0 = 0,
    #[doc = "1: PUC initiated on uncorrectable bit error detection flag. Generates vector in SYSRSTIV. Clear the UBDIE bit."]
    Ubdrsten1 = 1,
}
impl From<Ubdrsten> for bool {
    #[inline(always)]
    fn from(variant: Ubdrsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UBDRSTEN` reader - Enable Power Up Clear (PUC) reset for the uncorrectable bit error detection flag (UBDIFG)"]
pub type UbdrstenR = crate::BitReader<Ubdrsten>;
impl UbdrstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ubdrsten {
        match self.bits {
            false => Ubdrsten::Ubdrsten0,
            true => Ubdrsten::Ubdrsten1,
        }
    }
    #[doc = "PUC not initiated on uncorrectable bit error detection flag."]
    #[inline(always)]
    pub fn is_ubdrsten_0(&self) -> bool {
        *self == Ubdrsten::Ubdrsten0
    }
    #[doc = "PUC initiated on uncorrectable bit error detection flag. Generates vector in SYSRSTIV. Clear the UBDIE bit."]
    #[inline(always)]
    pub fn is_ubdrsten_1(&self) -> bool {
        *self == Ubdrsten::Ubdrsten1
    }
}
#[doc = "Field `UBDRSTEN` writer - Enable Power Up Clear (PUC) reset for the uncorrectable bit error detection flag (UBDIFG)"]
pub type UbdrstenW<'a, REG> = crate::BitWriter<'a, REG, Ubdrsten>;
impl<'a, REG> UbdrstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PUC not initiated on uncorrectable bit error detection flag."]
    #[inline(always)]
    pub fn ubdrsten_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ubdrsten::Ubdrsten0)
    }
    #[doc = "PUC initiated on uncorrectable bit error detection flag. Generates vector in SYSRSTIV. Clear the UBDIE bit."]
    #[inline(always)]
    pub fn ubdrsten_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ubdrsten::Ubdrsten1)
    }
}
impl R {
    #[doc = "Bit 1 - Enables FRAM auto power up after LPM"]
    #[inline(always)]
    pub fn frlpmpwr(&self) -> FrlpmpwrR {
        FrlpmpwrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FRAM Memory Power Control Request"]
    #[inline(always)]
    pub fn frpwr(&self) -> FrpwrR {
        FrpwrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable NMI event for the correctable bit error detection flag (CBDIFG)"]
    #[inline(always)]
    pub fn cbdie(&self) -> CbdieR {
        CbdieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable NMI event for the uncorrectable bit error detection flag (UBDIFG)"]
    #[inline(always)]
    pub fn ubdie(&self) -> UbdieR {
        UbdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Power Up Clear (PUC) reset for the uncorrectable bit error detection flag (UBDIFG)"]
    #[inline(always)]
    pub fn ubdrsten(&self) -> UbdrstenR {
        UbdrstenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enables FRAM auto power up after LPM"]
    #[inline(always)]
    pub fn frlpmpwr(&mut self) -> FrlpmpwrW<'_, Gcctl0Spec> {
        FrlpmpwrW::new(self, 1)
    }
    #[doc = "Bit 2 - FRAM Memory Power Control Request"]
    #[inline(always)]
    pub fn frpwr(&mut self) -> FrpwrW<'_, Gcctl0Spec> {
        FrpwrW::new(self, 2)
    }
    #[doc = "Bit 5 - Enable NMI event for the correctable bit error detection flag (CBDIFG)"]
    #[inline(always)]
    pub fn cbdie(&mut self) -> CbdieW<'_, Gcctl0Spec> {
        CbdieW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable NMI event for the uncorrectable bit error detection flag (UBDIFG)"]
    #[inline(always)]
    pub fn ubdie(&mut self) -> UbdieW<'_, Gcctl0Spec> {
        UbdieW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Power Up Clear (PUC) reset for the uncorrectable bit error detection flag (UBDIFG)"]
    #[inline(always)]
    pub fn ubdrsten(&mut self) -> UbdrstenW<'_, Gcctl0Spec> {
        UbdrstenW::new(self, 7)
    }
}
#[doc = "General Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gcctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gcctl0Spec;
impl crate::RegisterSpec for Gcctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`gcctl0::R`](R) reader structure"]
impl crate::Readable for Gcctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`gcctl0::W`](W) writer structure"]
impl crate::Writable for Gcctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCCTL0 to value 0"]
impl crate::Resettable for Gcctl0Spec {}
