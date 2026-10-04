#[repr(C)]
#[doc = "Temperature sensor calibration with the 1.5 V, 2.0 V or 2.5 V reference (index 0, 1 or 2)"]
#[doc(alias = "ADC_TEMP_CAL")]
pub struct AdcTempCal {
    temp_30c: Temp30c,
    temp_high: TempHigh,
}
impl AdcTempCal {
    #[doc = "0x00 - Temperature sensor ADC result at 30 degrees C"]
    #[inline(always)]
    pub const fn temp_30c(&self) -> &Temp30c {
        &self.temp_30c
    }
    #[doc = "0x02 - Temperature sensor ADC result at 105 degrees C"]
    #[inline(always)]
    pub const fn temp_high(&self) -> &TempHigh {
        &self.temp_high
    }
}
#[doc = "TEMP_30C (r) register accessor: Temperature sensor ADC result at 30 degrees C\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_30c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp_30c`] module"]
#[doc(alias = "TEMP_30C")]
pub type Temp30c = crate::Reg<temp_30c::Temp30cSpec>;
#[doc = "Temperature sensor ADC result at 30 degrees C"]
pub mod temp_30c;
#[doc = "TEMP_HIGH (r) register accessor: Temperature sensor ADC result at 105 degrees C\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_high::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp_high`] module"]
#[doc(alias = "TEMP_HIGH")]
pub type TempHigh = crate::Reg<temp_high::TempHighSpec>;
#[doc = "Temperature sensor ADC result at 105 degrees C"]
pub mod temp_high;
