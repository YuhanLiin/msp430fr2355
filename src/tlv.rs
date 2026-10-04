#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    info_length: InfoLength,
    crc_length: CrcLength,
    crc_value: CrcValue,
    _reserved_3_crc_data: [u8; 0xf4],
}
impl RegisterBlock {
    #[doc = "0x00 - Info length"]
    #[inline(always)]
    pub const fn info_length(&self) -> &InfoLength {
        &self.info_length
    }
    #[doc = "0x01 - CRC length"]
    #[inline(always)]
    pub const fn crc_length(&self) -> &CrcLength {
        &self.crc_length
    }
    #[doc = "0x02 - CRC value of the descriptors from 1A04h to 1AF7h (CRC-CCITT)"]
    #[inline(always)]
    pub const fn crc_value(&self) -> &CrcValue {
        &self.crc_value
    }
    #[doc = "0x04..0xf8 - Bytes 1A04h to 1AF7h, the range the CRC value covers"]
    #[inline(always)]
    pub const fn crc_data(&self, n: usize) -> &CrcData {
        #[allow(clippy::no_effect)]
        [(); 244][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0xf8 - Bytes 1A04h to 1AF7h, the range the CRC value covers"]
    #[inline(always)]
    pub fn crc_data_iter(&self) -> impl Iterator<Item = &CrcData> {
        (0..244)
            .map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).add(n).cast() })
    }
    #[doc = "0x04 - Device ID"]
    #[inline(always)]
    pub const fn device_id(&self) -> &DeviceId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x06 - Hardware revision"]
    #[inline(always)]
    pub const fn hw_revision(&self) -> &HwRevision {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x07 - Firmware revision"]
    #[inline(always)]
    pub const fn fw_revision(&self) -> &FwRevision {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    #[doc = "0x08 - Die record tag"]
    #[inline(always)]
    pub const fn die_record_tag(&self) -> &DieRecordTag {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x09 - Die record length"]
    #[inline(always)]
    pub const fn die_record_length(&self) -> &DieRecordLength {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(9).cast() }
    }
    #[doc = "0x0a - Lot/wafer ID"]
    #[inline(always)]
    pub const fn lot_wafer_id(&self) -> &LotWaferId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(10).cast() }
    }
    #[doc = "0x0e - Die X position"]
    #[inline(always)]
    pub const fn die_x_position(&self) -> &DieXPosition {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x10 - Die Y position"]
    #[inline(always)]
    pub const fn die_y_position(&self) -> &DieYPosition {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x12 - Test result"]
    #[inline(always)]
    pub const fn test_result(&self) -> &TestResult {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(18).cast() }
    }
    #[doc = "0x14 - ADC calibration tag"]
    #[inline(always)]
    pub const fn adc_cal_tag(&self) -> &AdcCalTag {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x15 - ADC calibration length"]
    #[inline(always)]
    pub const fn adc_cal_length(&self) -> &AdcCalLength {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(21).cast() }
    }
    #[doc = "0x16 - ADC gain factor"]
    #[inline(always)]
    pub const fn adc_gain_factor(&self) -> &AdcGainFactor {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(22).cast() }
    }
    #[doc = "0x18 - ADC offset"]
    #[inline(always)]
    pub const fn adc_offset(&self) -> &AdcOffset {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1a..0x26 - Temperature sensor calibration with the 1.5 V, 2.0 V or 2.5 V reference (index 0, 1 or 2)"]
    #[inline(always)]
    pub const fn adc_temp_cal(&self, n: usize) -> &AdcTempCal {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(26)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1a..0x26 - Temperature sensor calibration with the 1.5 V, 2.0 V or 2.5 V reference (index 0, 1 or 2)"]
    #[inline(always)]
    pub fn adc_temp_cal_iter(&self) -> impl Iterator<Item = &AdcTempCal> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(26)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x26 - REF calibration tag"]
    #[inline(always)]
    pub const fn ref_cal_tag(&self) -> &RefCalTag {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(38).cast() }
    }
    #[doc = "0x27 - REF calibration length"]
    #[inline(always)]
    pub const fn ref_cal_length(&self) -> &RefCalLength {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(39).cast() }
    }
    #[doc = "0x28..0x2e - Factor of the 1.5 V, 2.0 V or 2.5 V reference (index 0, 1 or 2)"]
    #[inline(always)]
    pub const fn ref_factor(&self, n: usize) -> &RefFactor {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(40)
                .add(2 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x28..0x2e - Factor of the 1.5 V, 2.0 V or 2.5 V reference (index 0, 1 or 2)"]
    #[inline(always)]
    pub fn ref_factor_iter(&self) -> impl Iterator<Item = &RefFactor> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(40)
                .add(2 * n)
                .cast()
        })
    }
    #[doc = "0x2e - DCO tap setting for 16 MHz"]
    #[inline(always)]
    pub const fn dco_tap_16mhz(&self) -> &DcoTap16mhz {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(46).cast() }
    }
    #[doc = "0x30 - DCO tap setting for 24 MHz"]
    #[inline(always)]
    pub const fn dco_tap_24mhz(&self) -> &DcoTap24mhz {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
}
#[doc = "INFO_LENGTH (r) register accessor: Info length\n\nYou can [`read`](crate::Reg::read) this register and get [`info_length::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@info_length`] module"]
#[doc(alias = "INFO_LENGTH")]
pub type InfoLength = crate::Reg<info_length::InfoLengthSpec>;
#[doc = "Info length"]
pub mod info_length;
#[doc = "CRC_LENGTH (r) register accessor: CRC length\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_length::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_length`] module"]
#[doc(alias = "CRC_LENGTH")]
pub type CrcLength = crate::Reg<crc_length::CrcLengthSpec>;
#[doc = "CRC length"]
pub mod crc_length;
#[doc = "CRC_VALUE (r) register accessor: CRC value of the descriptors from 1A04h to 1AF7h (CRC-CCITT)\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_value`] module"]
#[doc(alias = "CRC_VALUE")]
pub type CrcValue = crate::Reg<crc_value::CrcValueSpec>;
#[doc = "CRC value of the descriptors from 1A04h to 1AF7h (CRC-CCITT)"]
pub mod crc_value;
#[doc = "DEVICE_ID (r) register accessor: Device ID\n\nYou can [`read`](crate::Reg::read) this register and get [`device_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_id`] module"]
#[doc(alias = "DEVICE_ID")]
pub type DeviceId = crate::Reg<device_id::DeviceIdSpec>;
#[doc = "Device ID"]
pub mod device_id;
#[doc = "HW_REVISION (r) register accessor: Hardware revision\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_revision::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_revision`] module"]
#[doc(alias = "HW_REVISION")]
pub type HwRevision = crate::Reg<hw_revision::HwRevisionSpec>;
#[doc = "Hardware revision"]
pub mod hw_revision;
#[doc = "FW_REVISION (r) register accessor: Firmware revision\n\nYou can [`read`](crate::Reg::read) this register and get [`fw_revision::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fw_revision`] module"]
#[doc(alias = "FW_REVISION")]
pub type FwRevision = crate::Reg<fw_revision::FwRevisionSpec>;
#[doc = "Firmware revision"]
pub mod fw_revision;
#[doc = "DIE_RECORD_TAG (r) register accessor: Die record tag\n\nYou can [`read`](crate::Reg::read) this register and get [`die_record_tag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@die_record_tag`] module"]
#[doc(alias = "DIE_RECORD_TAG")]
pub type DieRecordTag = crate::Reg<die_record_tag::DieRecordTagSpec>;
#[doc = "Die record tag"]
pub mod die_record_tag;
#[doc = "DIE_RECORD_LENGTH (r) register accessor: Die record length\n\nYou can [`read`](crate::Reg::read) this register and get [`die_record_length::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@die_record_length`] module"]
#[doc(alias = "DIE_RECORD_LENGTH")]
pub type DieRecordLength = crate::Reg<die_record_length::DieRecordLengthSpec>;
#[doc = "Die record length"]
pub mod die_record_length;
#[doc = "LOT_WAFER_ID (r) register accessor: Lot/wafer ID\n\nYou can [`read`](crate::Reg::read) this register and get [`lot_wafer_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lot_wafer_id`] module"]
#[doc(alias = "LOT_WAFER_ID")]
pub type LotWaferId = crate::Reg<lot_wafer_id::LotWaferIdSpec>;
#[doc = "Lot/wafer ID"]
pub mod lot_wafer_id;
#[doc = "DIE_X_POSITION (r) register accessor: Die X position\n\nYou can [`read`](crate::Reg::read) this register and get [`die_x_position::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@die_x_position`] module"]
#[doc(alias = "DIE_X_POSITION")]
pub type DieXPosition = crate::Reg<die_x_position::DieXPositionSpec>;
#[doc = "Die X position"]
pub mod die_x_position;
#[doc = "DIE_Y_POSITION (r) register accessor: Die Y position\n\nYou can [`read`](crate::Reg::read) this register and get [`die_y_position::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@die_y_position`] module"]
#[doc(alias = "DIE_Y_POSITION")]
pub type DieYPosition = crate::Reg<die_y_position::DieYPositionSpec>;
#[doc = "Die Y position"]
pub mod die_y_position;
#[doc = "TEST_RESULT (r) register accessor: Test result\n\nYou can [`read`](crate::Reg::read) this register and get [`test_result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test_result`] module"]
#[doc(alias = "TEST_RESULT")]
pub type TestResult = crate::Reg<test_result::TestResultSpec>;
#[doc = "Test result"]
pub mod test_result;
#[doc = "ADC_CAL_TAG (r) register accessor: ADC calibration tag\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_cal_tag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_cal_tag`] module"]
#[doc(alias = "ADC_CAL_TAG")]
pub type AdcCalTag = crate::Reg<adc_cal_tag::AdcCalTagSpec>;
#[doc = "ADC calibration tag"]
pub mod adc_cal_tag;
#[doc = "ADC_CAL_LENGTH (r) register accessor: ADC calibration length\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_cal_length::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_cal_length`] module"]
#[doc(alias = "ADC_CAL_LENGTH")]
pub type AdcCalLength = crate::Reg<adc_cal_length::AdcCalLengthSpec>;
#[doc = "ADC calibration length"]
pub mod adc_cal_length;
#[doc = "ADC_GAIN_FACTOR (r) register accessor: ADC gain factor\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_gain_factor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_gain_factor`] module"]
#[doc(alias = "ADC_GAIN_FACTOR")]
pub type AdcGainFactor = crate::Reg<adc_gain_factor::AdcGainFactorSpec>;
#[doc = "ADC gain factor"]
pub mod adc_gain_factor;
#[doc = "ADC_OFFSET (r) register accessor: ADC offset\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_offset::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_offset`] module"]
#[doc(alias = "ADC_OFFSET")]
pub type AdcOffset = crate::Reg<adc_offset::AdcOffsetSpec>;
#[doc = "ADC offset"]
pub mod adc_offset;
#[doc = "Temperature sensor calibration with the 1.5 V, 2.0 V or 2.5 V reference (index 0, 1 or 2)"]
pub use self::adc_temp_cal::AdcTempCal;
#[doc = r"Cluster"]
#[doc = "Temperature sensor calibration with the 1.5 V, 2.0 V or 2.5 V reference (index 0, 1 or 2)"]
pub mod adc_temp_cal;
#[doc = "REF_CAL_TAG (r) register accessor: REF calibration tag\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_cal_tag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_cal_tag`] module"]
#[doc(alias = "REF_CAL_TAG")]
pub type RefCalTag = crate::Reg<ref_cal_tag::RefCalTagSpec>;
#[doc = "REF calibration tag"]
pub mod ref_cal_tag;
#[doc = "REF_CAL_LENGTH (r) register accessor: REF calibration length\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_cal_length::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_cal_length`] module"]
#[doc(alias = "REF_CAL_LENGTH")]
pub type RefCalLength = crate::Reg<ref_cal_length::RefCalLengthSpec>;
#[doc = "REF calibration length"]
pub mod ref_cal_length;
#[doc = "REF_FACTOR (r) register accessor: Factor of the 1.5 V, 2.0 V or 2.5 V reference (index 0, 1 or 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_factor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_factor`] module"]
#[doc(alias = "REF_FACTOR")]
pub type RefFactor = crate::Reg<ref_factor::RefFactorSpec>;
#[doc = "Factor of the 1.5 V, 2.0 V or 2.5 V reference (index 0, 1 or 2)"]
pub mod ref_factor;
#[doc = "DCO_TAP_16MHZ (r) register accessor: DCO tap setting for 16 MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`dco_tap_16mhz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dco_tap_16mhz`] module"]
#[doc(alias = "DCO_TAP_16MHZ")]
pub type DcoTap16mhz = crate::Reg<dco_tap_16mhz::DcoTap16mhzSpec>;
#[doc = "DCO tap setting for 16 MHz"]
pub mod dco_tap_16mhz;
#[doc = "DCO_TAP_24MHZ (r) register accessor: DCO tap setting for 24 MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`dco_tap_24mhz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dco_tap_24mhz`] module"]
#[doc(alias = "DCO_TAP_24MHZ")]
pub type DcoTap24mhz = crate::Reg<dco_tap_24mhz::DcoTap24mhzSpec>;
#[doc = "DCO tap setting for 24 MHz"]
pub mod dco_tap_24mhz;
#[doc = "CRC_DATA (r) register accessor: Bytes 1A04h to 1AF7h, the range the CRC value covers\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_data`] module"]
#[doc(alias = "CRC_DATA")]
pub type CrcData = crate::Reg<crc_data::CrcDataSpec>;
#[doc = "Bytes 1A04h to 1AF7h, the range the CRC value covers"]
pub mod crc_data;
