#[doc = "Register `PMMCTL2` reader"]
pub type R = crate::R<Pmmctl2Spec>;
#[doc = "Register `PMMCTL2` writer"]
pub type W = crate::W<Pmmctl2Spec>;
#[doc = "Internal reference enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intrefen {
    #[doc = "0: Disable internal reference"]
    Intrefen0 = 0,
    #[doc = "1: Enable internal reference"]
    Intrefen1 = 1,
}
impl From<Intrefen> for bool {
    #[inline(always)]
    fn from(variant: Intrefen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTREFEN` reader - Internal reference enable"]
pub type IntrefenR = crate::BitReader<Intrefen>;
impl IntrefenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intrefen {
        match self.bits {
            false => Intrefen::Intrefen0,
            true => Intrefen::Intrefen1,
        }
    }
    #[doc = "Disable internal reference"]
    #[inline(always)]
    pub fn is_intrefen_0(&self) -> bool {
        *self == Intrefen::Intrefen0
    }
    #[doc = "Enable internal reference"]
    #[inline(always)]
    pub fn is_intrefen_1(&self) -> bool {
        *self == Intrefen::Intrefen1
    }
}
#[doc = "Field `INTREFEN` writer - Internal reference enable"]
pub type IntrefenW<'a, REG> = crate::BitWriter<'a, REG, Intrefen>;
impl<'a, REG> IntrefenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable internal reference"]
    #[inline(always)]
    pub fn intrefen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Intrefen::Intrefen0)
    }
    #[doc = "Enable internal reference"]
    #[inline(always)]
    pub fn intrefen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Intrefen::Intrefen1)
    }
}
#[doc = "External reference output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extrefen {
    #[doc = "0: Disable external reference output"]
    Extrefen0 = 0,
    #[doc = "1: Enable internal reference output"]
    Extrefen1 = 1,
}
impl From<Extrefen> for bool {
    #[inline(always)]
    fn from(variant: Extrefen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTREFEN` reader - External reference output enable"]
pub type ExtrefenR = crate::BitReader<Extrefen>;
impl ExtrefenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extrefen {
        match self.bits {
            false => Extrefen::Extrefen0,
            true => Extrefen::Extrefen1,
        }
    }
    #[doc = "Disable external reference output"]
    #[inline(always)]
    pub fn is_extrefen_0(&self) -> bool {
        *self == Extrefen::Extrefen0
    }
    #[doc = "Enable internal reference output"]
    #[inline(always)]
    pub fn is_extrefen_1(&self) -> bool {
        *self == Extrefen::Extrefen1
    }
}
#[doc = "Field `EXTREFEN` writer - External reference output enable"]
pub type ExtrefenW<'a, REG> = crate::BitWriter<'a, REG, Extrefen>;
impl<'a, REG> ExtrefenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable external reference output"]
    #[inline(always)]
    pub fn extrefen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Extrefen::Extrefen0)
    }
    #[doc = "Enable internal reference output"]
    #[inline(always)]
    pub fn extrefen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Extrefen::Extrefen1)
    }
}
#[doc = "Temperature sensor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsensoren {
    #[doc = "0: Disable temperature sensor"]
    Tsensoren0 = 0,
    #[doc = "1: Enable temperature sensor"]
    Tsensoren1 = 1,
}
impl From<Tsensoren> for bool {
    #[inline(always)]
    fn from(variant: Tsensoren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSENSOREN` reader - Temperature sensor enable"]
pub type TsensorenR = crate::BitReader<Tsensoren>;
impl TsensorenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsensoren {
        match self.bits {
            false => Tsensoren::Tsensoren0,
            true => Tsensoren::Tsensoren1,
        }
    }
    #[doc = "Disable temperature sensor"]
    #[inline(always)]
    pub fn is_tsensoren_0(&self) -> bool {
        *self == Tsensoren::Tsensoren0
    }
    #[doc = "Enable temperature sensor"]
    #[inline(always)]
    pub fn is_tsensoren_1(&self) -> bool {
        *self == Tsensoren::Tsensoren1
    }
}
#[doc = "Field `TSENSOREN` writer - Temperature sensor enable"]
pub type TsensorenW<'a, REG> = crate::BitWriter<'a, REG, Tsensoren>;
impl<'a, REG> TsensorenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable temperature sensor"]
    #[inline(always)]
    pub fn tsensoren_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsensoren::Tsensoren0)
    }
    #[doc = "Enable temperature sensor"]
    #[inline(always)]
    pub fn tsensoren_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsensoren::Tsensoren1)
    }
}
#[doc = "Reference voltage level select. Can be modified only when REFGENBUSY = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refvsel {
    #[doc = "0: 00b = 1.5V"]
    Refvsel0 = 0,
    #[doc = "1: 01b = 2.0V"]
    Refvsel1 = 1,
    #[doc = "2: 10b = 2.5V"]
    Refvsel2 = 2,
    #[doc = "3: 11b = Reserved"]
    Refvsel3 = 3,
}
impl From<Refvsel> for u8 {
    #[inline(always)]
    fn from(variant: Refvsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refvsel {
    type Ux = u8;
}
impl crate::IsEnum for Refvsel {}
#[doc = "Field `REFVSEL` reader - Reference voltage level select. Can be modified only when REFGENBUSY = 0."]
pub type RefvselR = crate::FieldReader<Refvsel>;
impl RefvselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refvsel {
        match self.bits {
            0 => Refvsel::Refvsel0,
            1 => Refvsel::Refvsel1,
            2 => Refvsel::Refvsel2,
            3 => Refvsel::Refvsel3,
            _ => unreachable!(),
        }
    }
    #[doc = "00b = 1.5V"]
    #[inline(always)]
    pub fn is_refvsel_0(&self) -> bool {
        *self == Refvsel::Refvsel0
    }
    #[doc = "01b = 2.0V"]
    #[inline(always)]
    pub fn is_refvsel_1(&self) -> bool {
        *self == Refvsel::Refvsel1
    }
    #[doc = "10b = 2.5V"]
    #[inline(always)]
    pub fn is_refvsel_2(&self) -> bool {
        *self == Refvsel::Refvsel2
    }
    #[doc = "11b = Reserved"]
    #[inline(always)]
    pub fn is_refvsel_3(&self) -> bool {
        *self == Refvsel::Refvsel3
    }
}
#[doc = "Field `REFVSEL` writer - Reference voltage level select. Can be modified only when REFGENBUSY = 0."]
pub type RefvselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refvsel, crate::Safe>;
impl<'a, REG> RefvselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "00b = 1.5V"]
    #[inline(always)]
    pub fn refvsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refvsel::Refvsel0)
    }
    #[doc = "01b = 2.0V"]
    #[inline(always)]
    pub fn refvsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refvsel::Refvsel1)
    }
    #[doc = "10b = 2.5V"]
    #[inline(always)]
    pub fn refvsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Refvsel::Refvsel2)
    }
    #[doc = "11b = Reserved"]
    #[inline(always)]
    pub fn refvsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Refvsel::Refvsel3)
    }
}
#[doc = "Reference generator one-time trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refgen {
    #[doc = "0: No trigger"]
    Refgen0 = 0,
    #[doc = "1: Generation of the reference voltage is started by writing 1 or by a hardware trigger"]
    Refgen1 = 1,
}
impl From<Refgen> for bool {
    #[inline(always)]
    fn from(variant: Refgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFGEN` reader - Reference generator one-time trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware."]
pub type RefgenR = crate::BitReader<Refgen>;
impl RefgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refgen {
        match self.bits {
            false => Refgen::Refgen0,
            true => Refgen::Refgen1,
        }
    }
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn is_refgen_0(&self) -> bool {
        *self == Refgen::Refgen0
    }
    #[doc = "Generation of the reference voltage is started by writing 1 or by a hardware trigger"]
    #[inline(always)]
    pub fn is_refgen_1(&self) -> bool {
        *self == Refgen::Refgen1
    }
}
#[doc = "Field `REFGEN` writer - Reference generator one-time trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware."]
pub type RefgenW<'a, REG> = crate::BitWriter<'a, REG, Refgen>;
impl<'a, REG> RefgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn refgen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refgen::Refgen0)
    }
    #[doc = "Generation of the reference voltage is started by writing 1 or by a hardware trigger"]
    #[inline(always)]
    pub fn refgen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refgen::Refgen1)
    }
}
#[doc = "Bandgap and bandgap buffer one-time trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refbgen {
    #[doc = "0: No trigger"]
    Refbg0 = 0,
    #[doc = "1: Generation of the bandgap voltage is started by writing 1 or by a hardware trigger"]
    Refbg1 = 1,
}
impl From<Refbgen> for bool {
    #[inline(always)]
    fn from(variant: Refbgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFBGEN` reader - Bandgap and bandgap buffer one-time trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware."]
pub type RefbgenR = crate::BitReader<Refbgen>;
impl RefbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refbgen {
        match self.bits {
            false => Refbgen::Refbg0,
            true => Refbgen::Refbg1,
        }
    }
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn is_refbg_0(&self) -> bool {
        *self == Refbgen::Refbg0
    }
    #[doc = "Generation of the bandgap voltage is started by writing 1 or by a hardware trigger"]
    #[inline(always)]
    pub fn is_refbg_1(&self) -> bool {
        *self == Refbgen::Refbg1
    }
}
#[doc = "Field `REFBGEN` writer - Bandgap and bandgap buffer one-time trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware."]
pub type RefbgenW<'a, REG> = crate::BitWriter<'a, REG, Refbgen>;
impl<'a, REG> RefbgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn refbg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refbgen::Refbg0)
    }
    #[doc = "Generation of the bandgap voltage is started by writing 1 or by a hardware trigger"]
    #[inline(always)]
    pub fn refbg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refbgen::Refbg1)
    }
}
#[doc = "Reference generator active. Read only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refgenact {
    #[doc = "0: Reference generator not active"]
    Refgenact0 = 0,
    #[doc = "1: Reference generator active"]
    Refgenact1 = 1,
}
impl From<Refgenact> for bool {
    #[inline(always)]
    fn from(variant: Refgenact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFGENACT` reader - Reference generator active. Read only."]
pub type RefgenactR = crate::BitReader<Refgenact>;
impl RefgenactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refgenact {
        match self.bits {
            false => Refgenact::Refgenact0,
            true => Refgenact::Refgenact1,
        }
    }
    #[doc = "Reference generator not active"]
    #[inline(always)]
    pub fn is_refgenact_0(&self) -> bool {
        *self == Refgenact::Refgenact0
    }
    #[doc = "Reference generator active"]
    #[inline(always)]
    pub fn is_refgenact_1(&self) -> bool {
        *self == Refgenact::Refgenact1
    }
}
#[doc = "Reference bandgap active. Ready only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refbgact {
    #[doc = "0: Reference bandgap buffer not active"]
    Refbgact0 = 0,
    #[doc = "1: Reference bandgap buffer active"]
    Refbgact1 = 1,
}
impl From<Refbgact> for bool {
    #[inline(always)]
    fn from(variant: Refbgact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFBGACT` reader - Reference bandgap active. Ready only."]
pub type RefbgactR = crate::BitReader<Refbgact>;
impl RefbgactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refbgact {
        match self.bits {
            false => Refbgact::Refbgact0,
            true => Refbgact::Refbgact1,
        }
    }
    #[doc = "Reference bandgap buffer not active"]
    #[inline(always)]
    pub fn is_refbgact_0(&self) -> bool {
        *self == Refbgact::Refbgact0
    }
    #[doc = "Reference bandgap buffer active"]
    #[inline(always)]
    pub fn is_refbgact_1(&self) -> bool {
        *self == Refbgact::Refbgact1
    }
}
#[doc = "Bandgap mode. Ready only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bgmode {
    #[doc = "0: Static mode (higher precision)"]
    Bgmode0 = 0,
    #[doc = "1: Sampled mode (lower power consumption)"]
    Bgmode1 = 1,
}
impl From<Bgmode> for bool {
    #[inline(always)]
    fn from(variant: Bgmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGMODE` reader - Bandgap mode. Ready only."]
pub type BgmodeR = crate::BitReader<Bgmode>;
impl BgmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bgmode {
        match self.bits {
            false => Bgmode::Bgmode0,
            true => Bgmode::Bgmode1,
        }
    }
    #[doc = "Static mode (higher precision)"]
    #[inline(always)]
    pub fn is_bgmode_0(&self) -> bool {
        *self == Bgmode::Bgmode0
    }
    #[doc = "Sampled mode (lower power consumption)"]
    #[inline(always)]
    pub fn is_bgmode_1(&self) -> bool {
        *self == Bgmode::Bgmode1
    }
}
#[doc = "Field `BGMODE` writer - Bandgap mode. Ready only."]
pub type BgmodeW<'a, REG> = crate::BitWriter<'a, REG, Bgmode>;
impl<'a, REG> BgmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Static mode (higher precision)"]
    #[inline(always)]
    pub fn bgmode_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bgmode::Bgmode0)
    }
    #[doc = "Sampled mode (lower power consumption)"]
    #[inline(always)]
    pub fn bgmode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bgmode::Bgmode1)
    }
}
#[doc = "Variable reference voltage ready status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refgenrdy {
    #[doc = "0: Reference voltage output is not ready to be used."]
    Refgenrdy0 = 0,
    #[doc = "1: Reference voltage output is ready to be used"]
    Refgenrdy1 = 1,
}
impl From<Refgenrdy> for bool {
    #[inline(always)]
    fn from(variant: Refgenrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFGENRDY` reader - Variable reference voltage ready status."]
pub type RefgenrdyR = crate::BitReader<Refgenrdy>;
impl RefgenrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refgenrdy {
        match self.bits {
            false => Refgenrdy::Refgenrdy0,
            true => Refgenrdy::Refgenrdy1,
        }
    }
    #[doc = "Reference voltage output is not ready to be used."]
    #[inline(always)]
    pub fn is_refgenrdy_0(&self) -> bool {
        *self == Refgenrdy::Refgenrdy0
    }
    #[doc = "Reference voltage output is ready to be used"]
    #[inline(always)]
    pub fn is_refgenrdy_1(&self) -> bool {
        *self == Refgenrdy::Refgenrdy1
    }
}
#[doc = "Field `REFGENRDY` writer - Variable reference voltage ready status."]
pub type RefgenrdyW<'a, REG> = crate::BitWriter<'a, REG, Refgenrdy>;
impl<'a, REG> RefgenrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reference voltage output is not ready to be used."]
    #[inline(always)]
    pub fn refgenrdy_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refgenrdy::Refgenrdy0)
    }
    #[doc = "Reference voltage output is ready to be used"]
    #[inline(always)]
    pub fn refgenrdy_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refgenrdy::Refgenrdy1)
    }
}
#[doc = "Buffered bandgap voltage ready status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refbgrdy {
    #[doc = "0: Buffered bandgap voltage is not ready to be used"]
    Refbgrdy0 = 0,
    #[doc = "1: Buffered bandgap voltage is ready to be used"]
    Refbgrdy1 = 1,
}
impl From<Refbgrdy> for bool {
    #[inline(always)]
    fn from(variant: Refbgrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFBGRDY` reader - Buffered bandgap voltage ready status."]
pub type RefbgrdyR = crate::BitReader<Refbgrdy>;
impl RefbgrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refbgrdy {
        match self.bits {
            false => Refbgrdy::Refbgrdy0,
            true => Refbgrdy::Refbgrdy1,
        }
    }
    #[doc = "Buffered bandgap voltage is not ready to be used"]
    #[inline(always)]
    pub fn is_refbgrdy_0(&self) -> bool {
        *self == Refbgrdy::Refbgrdy0
    }
    #[doc = "Buffered bandgap voltage is ready to be used"]
    #[inline(always)]
    pub fn is_refbgrdy_1(&self) -> bool {
        *self == Refbgrdy::Refbgrdy1
    }
}
#[doc = "Field `REFBGRDY` writer - Buffered bandgap voltage ready status."]
pub type RefbgrdyW<'a, REG> = crate::BitWriter<'a, REG, Refbgrdy>;
impl<'a, REG> RefbgrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Buffered bandgap voltage is not ready to be used"]
    #[inline(always)]
    pub fn refbgrdy_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refbgrdy::Refbgrdy0)
    }
    #[doc = "Buffered bandgap voltage is ready to be used"]
    #[inline(always)]
    pub fn refbgrdy_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refbgrdy::Refbgrdy1)
    }
}
#[doc = "Field `PWRMODE` reader - Power Mode Selection. The two bits are used to select the power supply in multi power supply systems. A single power supply system is not affected by the bits. Reserved for future use."]
pub type PwrmodeR = crate::FieldReader;
#[doc = "Field `PWRMODE` writer - Power Mode Selection. The two bits are used to select the power supply in multi power supply systems. A single power supply system is not affected by the bits. Reserved for future use."]
pub type PwrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Internal reference enable"]
    #[inline(always)]
    pub fn intrefen(&self) -> IntrefenR {
        IntrefenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External reference output enable"]
    #[inline(always)]
    pub fn extrefen(&self) -> ExtrefenR {
        ExtrefenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tsensoren(&self) -> TsensorenR {
        TsensorenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reference voltage level select. Can be modified only when REFGENBUSY = 0."]
    #[inline(always)]
    pub fn refvsel(&self) -> RefvselR {
        RefvselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Reference generator one-time trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn refgen(&self) -> RefgenR {
        RefgenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer one-time trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn refbgen(&self) -> RefbgenR {
        RefbgenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reference generator active. Read only."]
    #[inline(always)]
    pub fn refgenact(&self) -> RefgenactR {
        RefgenactR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reference bandgap active. Ready only."]
    #[inline(always)]
    pub fn refbgact(&self) -> RefbgactR {
        RefbgactR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Bandgap mode. Ready only."]
    #[inline(always)]
    pub fn bgmode(&self) -> BgmodeR {
        BgmodeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Variable reference voltage ready status."]
    #[inline(always)]
    pub fn refgenrdy(&self) -> RefgenrdyR {
        RefgenrdyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Buffered bandgap voltage ready status."]
    #[inline(always)]
    pub fn refbgrdy(&self) -> RefbgrdyR {
        RefbgrdyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Power Mode Selection. The two bits are used to select the power supply in multi power supply systems. A single power supply system is not affected by the bits. Reserved for future use."]
    #[inline(always)]
    pub fn pwrmode(&self) -> PwrmodeR {
        PwrmodeR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal reference enable"]
    #[inline(always)]
    pub fn intrefen(&mut self) -> IntrefenW<'_, Pmmctl2Spec> {
        IntrefenW::new(self, 0)
    }
    #[doc = "Bit 1 - External reference output enable"]
    #[inline(always)]
    pub fn extrefen(&mut self) -> ExtrefenW<'_, Pmmctl2Spec> {
        ExtrefenW::new(self, 1)
    }
    #[doc = "Bit 3 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tsensoren(&mut self) -> TsensorenW<'_, Pmmctl2Spec> {
        TsensorenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Reference voltage level select. Can be modified only when REFGENBUSY = 0."]
    #[inline(always)]
    pub fn refvsel(&mut self) -> RefvselW<'_, Pmmctl2Spec> {
        RefvselW::new(self, 4)
    }
    #[doc = "Bit 6 - Reference generator one-time trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn refgen(&mut self) -> RefgenW<'_, Pmmctl2Spec> {
        RefgenW::new(self, 6)
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer one-time trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn refbgen(&mut self) -> RefbgenW<'_, Pmmctl2Spec> {
        RefbgenW::new(self, 7)
    }
    #[doc = "Bit 11 - Bandgap mode. Ready only."]
    #[inline(always)]
    pub fn bgmode(&mut self) -> BgmodeW<'_, Pmmctl2Spec> {
        BgmodeW::new(self, 11)
    }
    #[doc = "Bit 12 - Variable reference voltage ready status."]
    #[inline(always)]
    pub fn refgenrdy(&mut self) -> RefgenrdyW<'_, Pmmctl2Spec> {
        RefgenrdyW::new(self, 12)
    }
    #[doc = "Bit 13 - Buffered bandgap voltage ready status."]
    #[inline(always)]
    pub fn refbgrdy(&mut self) -> RefbgrdyW<'_, Pmmctl2Spec> {
        RefbgrdyW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Power Mode Selection. The two bits are used to select the power supply in multi power supply systems. A single power supply system is not affected by the bits. Reserved for future use."]
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PwrmodeW<'_, Pmmctl2Spec> {
        PwrmodeW::new(self, 14)
    }
}
#[doc = "Power Management Module Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pmmctl2Spec;
impl crate::RegisterSpec for Pmmctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmmctl2::R`](R) reader structure"]
impl crate::Readable for Pmmctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`pmmctl2::W`](W) writer structure"]
impl crate::Writable for Pmmctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMMCTL2 to value 0"]
impl crate::Resettable for Pmmctl2Spec {}
