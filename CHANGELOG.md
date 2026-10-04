# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [v0.7.0]

- Regenerate with the current msp430_svd overrides, which name registers, fields and enum variants as in the other MSP430FR2xx PACs:
  - (Breaking) The eCOMP0 registers are named `cp0ctl0`, `cp0ctl1`, `cp0int`, `cp0iv`, `cp0dacctl` and `cp0dacdata`, like those of eCOMP1.
  - (Breaking) `ICCILSR0` to `ICCILSR3` are one register array, `iccilsr(n)`, with a field per interrupt source, `ilsr(n)`. Its priority enum, `Highest` to `Lowest`, is also used by `ICCSC.ICMC`.
  - (Breaking) Enum variants named after their meaning: `PMMCTL0.SVSHE` is `Disabled`/`Enabled`, `FRCTL0.NWAITS` is `Wait0` to `Wait7`, `CSCTL3.FLLREFDIV` is `_1` to `_768`, `ADCCTL1.ADCCONSEQ` is `Single`, `Sequence`, `RepeatSingle` and `RepeatSequence`, and `ADCCTL2.ADCDF` is `Unsigned`/`Signed`.
  - (Breaking) Also named after their meaning: `CSCTL7.FLLUNLOCK` is `Locked`, `TooSlow`, `TooFast` and `OutOfRange`, `CSCTL7.FLLUNLOCKHIS` is `Locked`, `TooSlow`, `TooFast` and `TooSlowAndFast`, `CSCTL7.FLLWARNEN` and `GCCTL0.FRPWR` are `Disabled`/`Enabled`, `SYSCTL.SYSPMMPE` is `Dis`/`En` and `SYSCTL.SYSBSLIND` is `Clr`/`Set`, and `SYSBSLC.SYSBSLR`, `SYSBSLOFF` and `SYSBSLPE` are `Noram`/`Ram`, `On`/`Off` and `Notprot`/`Prot`, as in the MSP430FR247x PAC.
- Add the `Tlv` peripheral with the device descriptors (SLASEC4D Table 6-70).
- Add `pmmctl0_h` and `frctl0_h`, the upper bytes of PMMCTL0 and FRCTL0, whose `lock()` locks the PMM and FRAM controller registers again, and `password()` for `WDTCTL.WDTPW`, `FRCTL0.FRCTLPW` and `SYSCFG0.FRWPPW`.
- Add `SFRRPCR.SYSFLTE`, `SYSCFG2.TB0TRGSEL` to `TB3TRGSEL`, and `PMMCTL1`.
- Add the CRC data and result fields, and `crcdi_l` and `crcdirb_l` for byte writes.
- `CSCTL0.DCO`, `CSCTL0.MOD`, `CSCTL1.DCOFTRIM`, `CSCTL2.FLLN` and the CRC data fields can be written with the safe `set()`.
- Fix `SYSCFG1.IRMSEL`, whose ASK and FSK values were swapped (SLAU445I Table 1-25).
- Fix the `PMMIFG.PMMPORIFG` and `PMMRSTIFG` enums, which were named after PMMBORIFG.
- Remove `PMMCTL0.REFLOW`, which isn't in the user's guide.
- (Breaking) Remove `PM5CTL0.LPM5SM` and `PM5CTL0.LPM5SW`: the LPM3.5 switch is only on other devices (SLAU445I Table 2-7). `GCCTL0.FRLPMPWR` is read-only, as it reads 0 on these devices (SLAU445I Table 6-3).
- `SYSCTL.SYSBSLIND` can be written, as the user's guide shows (SLAU445I Table 1-13).
- (Breaking) Names as in the user's guide, SLAU445I: the backup memory peripheral is `BAKMEM` instead of `BKMEM` (SLAU445I Table 7-1), and the `SACxIV` field is `SACIV` instead of `SACIV0` to `SACIV3` (SLAU445I Table 20-11).
- (Breaking) Remove fields that neither the user's guide nor the data sheet has: `ADCMCTL0.EXPCHEN`, `PMMCTL2.PWRMODE`, `PMMIFG.PMMSPSIFG`, `SPWRIFG` and `PPWRIFG`, `SYSCFG1.SYNCSEL` (CapTIvate, which this device doesn't have), and the 8-bit `MACS32H` field of the `MACS32H` register.
- The `P1IV` to `P4IV` fields are 16 bits wide, as in the user's guide (SLAU445I Table 8-5 to Table 8-8).
- Add the `defmt` feature, which implements `defmt::Format` for the enumerated values and `Interrupt` (svd2rust `--impl-defmt defmt`, also in `regenerate.sh`).
- (Breaking) The interrupt vector values named after their sources, as in the data sheet (SLASEC4D Table 6-12): `SYSRSTIV` is `Brownout`, `ResetPin`, `SoftwareBor`, `Lpmx5WakeUp`, `SecurityViolation`, `Svsh`, `SoftwarePor`, `WatchdogTimeout`, `WatchdogPassword`, `FramPassword`, `FramBitError`, `PeripheralAreaFetch`, `PmmPassword` and `FllUnlock`, `SYSSNIV` is `SvsLowPowerResetEntry`, `FramUncorrectableBitError`, `VacantMemoryAccess`, `JtagMailboxIn`, `JtagMailboxOut` and `FramCorrectableBitError`, and `SYSUNIV` is `NmiPin` and `OscillatorFault`. 00h, no interrupt pending, and the reserved values have no variant, so `variant()` returns `None` for them. `ADCIV` is `None`, `Overflow`, `TimeOverflow`, `AboveWindow`, `BelowWindow`, `InsideWindow` and `ResultReady` (SLAU445I Table 21-15), the `PxIV` values are `None` and `Ifg0` to `Ifg7` on every port (SLAU445I Table 8-5 to Table 8-8), and `SACxIV` is `None` and `Dacifg` (SLAU445I Table 20-11).
- (Breaking) Enum variants named after their meaning: the ADC's `ADCSHT` is `Cycles4` to `Cycles1024`, `ADCDIV` `_1` to `_8`, `ADCSSEL` `Modclk`, `Aclk` and `Smclk`, `ADCPDIV` `_1`, `_4` and `_64`, `ADCRES` `Bits8`, `Bits10` and `Bits12`, `ADCSR` `Max200ksps` and `Max50ksps`, `ADCSHS` `Software`, `Rtc`, `Timer` and `Comparator` (SLASEC4D Table 6-22) and `ADCSREF` `AvccAvss` to `VerefPlusVerefMinus` (SLAU445I Table 21-3 to Table 21-8); `CSCTL1.DCORSEL` is `Range1mhz` to `Range24mhz` (SLAU445I Table 3-5) and `PMMCTL2.REFVSEL` `V1_5`, `V2_0` and `V2_5` (SLAU445I Table 2-4). Values a field has twice, and reserved ones, have no variant.

## [v0.6.1]

- Fix docs.rs build issue

## [v0.6.0]

- (Breaking) Regenerate with svd2rust 0.37.1
  - (Breaking) Peripheral names have changed from `SCREAMING_SNAKE` to `snake_case`, and type names have changed to `PascalCase`, dropping any underscores.
  - (Breaking) All registers are now accessed through methods. Previously registers were sometimes fields and sometimes methods (based on whether the hardware address was aliased by multiple registers).
- Add missing eUSCI UCBUSY bits

## [v0.5.2] - 2022-12-24

- Bump `portable-atomic` to v0.3.16 to reverse a previous regression that disabled single-instruction atomic operations

## [v0.5.1] - 2022-10-29

- Replace `msp430-atomic` with `portable-atomic` to fix non-MSP430 builds

## [v0.5.0] - 2022-10-26

- Regenerate all file using `svd2rust` 0.26
- Add CI pipeline
