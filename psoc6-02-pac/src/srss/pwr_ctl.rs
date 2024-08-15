#[doc = "Register `PWR_CTL` reader"]
pub type R = crate::R<PwrCtlSpec>;
#[doc = "Register `PWR_CTL` writer"]
pub type W = crate::W<PwrCtlSpec>;
#[doc = "Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PowerMode {
    #[doc = "0: System is resetting."]
    Reset = 0,
    #[doc = "1: At least one CPU is running."]
    Active = 1,
    #[doc = "2: No CPUs are running. Peripherals may be running."]
    Sleep = 2,
    #[doc = "3: Main high-frequency clock is off; low speed clocks are available. Communication interface clocks may be present."]
    Deepsleep = 3,
}
impl From<PowerMode> for u8 {
    #[inline(always)]
    fn from(variant: PowerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PowerMode {
    type Ux = u8;
}
impl crate::IsEnum for PowerMode {}
#[doc = "Field `POWER_MODE` reader - Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon."]
pub type PowerModeR = crate::FieldReader<PowerMode>;
impl PowerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PowerMode {
        match self.bits {
            0 => PowerMode::Reset,
            1 => PowerMode::Active,
            2 => PowerMode::Sleep,
            3 => PowerMode::Deepsleep,
            _ => unreachable!(),
        }
    }
    #[doc = "System is resetting."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PowerMode::Reset
    }
    #[doc = "At least one CPU is running."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == PowerMode::Active
    }
    #[doc = "No CPUs are running. Peripherals may be running."]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == PowerMode::Sleep
    }
    #[doc = "Main high-frequency clock is off; low speed clocks are available. Communication interface clocks may be present."]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == PowerMode::Deepsleep
    }
}
#[doc = "Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DebugSession {
    #[doc = "0: No debug session active"]
    NoSession = 0,
    #[doc = "1: Debug session is active. Power modes behave differently to keep the debug session active."]
    SessionActive = 1,
}
impl From<DebugSession> for bool {
    #[inline(always)]
    fn from(variant: DebugSession) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUG_SESSION` reader - Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)"]
pub type DebugSessionR = crate::BitReader<DebugSession>;
impl DebugSessionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DebugSession {
        match self.bits {
            false => DebugSession::NoSession,
            true => DebugSession::SessionActive,
        }
    }
    #[doc = "No debug session active"]
    #[inline(always)]
    pub fn is_no_session(&self) -> bool {
        *self == DebugSession::NoSession
    }
    #[doc = "Debug session is active. Power modes behave differently to keep the debug session active."]
    #[inline(always)]
    pub fn is_session_active(&self) -> bool {
        *self == DebugSession::SessionActive
    }
}
#[doc = "Field `LPM_READY` reader - Indicates whether certain low power functions are ready. The low current circuits take longer to startup after XRES/POR/BOD/HIBERNATE wakeup than the normal mode circuits. HIBERNATE mode may be entered regardless of this bit. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: If a low power circuit operation is requested, it will stay in its normal operating mode until it is ready. If DEEPSLEEP is requested by all processors WFI/WFE, the device will instead enter SLEEP. When low power circuits are ready, device will automatically enter the originally requested mode. 1: Normal operation. DEEPSLEEP and low power circuits operate as requested in other registers."]
pub type LpmReadyR = crate::BitReader;
#[doc = "Field `IREF_LPMODE` reader - Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
pub type IrefLpmodeR = crate::BitReader;
#[doc = "Field `IREF_LPMODE` writer - Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
pub type IrefLpmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFBUF_OK` reader - Indicates that the voltage reference buffer is ready. Due to synchronization delays, it may take two IMO clock cycles for hardware to clear this bit after asserting VREFBUF_DIS=1."]
pub type VrefbufOkR = crate::BitReader;
#[doc = "Field `DPSLP_REG_DIS` reader - Disable the DeepSleep regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: DeepSleep Regulator is on. 1: DeepSleep Regulator is off."]
pub type DpslpRegDisR = crate::BitReader;
#[doc = "Field `DPSLP_REG_DIS` writer - Disable the DeepSleep regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: DeepSleep Regulator is on. 1: DeepSleep Regulator is off."]
pub type DpslpRegDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RET_REG_DIS` reader - Disable the Retention regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Retention Regulator is on. 1: Retention Regulator is off."]
pub type RetRegDisR = crate::BitReader;
#[doc = "Field `RET_REG_DIS` writer - Disable the Retention regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Retention Regulator is on. 1: Retention Regulator is off."]
pub type RetRegDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWELL_REG_DIS` reader - Disable the Nwell regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
pub type NwellRegDisR = crate::BitReader;
#[doc = "Field `NWELL_REG_DIS` writer - Disable the Nwell regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
pub type NwellRegDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINREG_DIS` reader - Disable the linear Core Regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear regulator is on. 1: Linear regulator is off."]
pub type LinregDisR = crate::BitReader;
#[doc = "Field `LINREG_DIS` writer - Disable the linear Core Regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear regulator is on. 1: Linear regulator is off."]
pub type LinregDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINREG_LPMODE` reader - Control the power mode of the Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
pub type LinregLpmodeR = crate::BitReader;
#[doc = "Field `LINREG_LPMODE` writer - Control the power mode of the Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
pub type LinregLpmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORBOD_LPMODE` reader - Control the power mode of the POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
pub type PorbodLpmodeR = crate::BitReader;
#[doc = "Field `PORBOD_LPMODE` writer - Control the power mode of the POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
pub type PorbodLpmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGREF_LPMODE` reader - Control the power mode of the Bandgap Voltage and Current References. This applies to voltage and current generation and is different than the reference voltage buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. When lower power mode is used, the Active Reference circuit can be disabled to reduce current. Firmware is responsible to ensure ACT_REF_OK==1 before changing back to normal mode. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Bandgap Voltage and Current Reference operates in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: Active Bandgap Voltage and Current Reference operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less. The Active Reference may be disabled using ACT_REF_DIS=0."]
pub type BgrefLpmodeR = crate::BitReader;
#[doc = "Field `BGREF_LPMODE` writer - Control the power mode of the Bandgap Voltage and Current References. This applies to voltage and current generation and is different than the reference voltage buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. When lower power mode is used, the Active Reference circuit can be disabled to reduce current. Firmware is responsible to ensure ACT_REF_OK==1 before changing back to normal mode. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Bandgap Voltage and Current Reference operates in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: Active Bandgap Voltage and Current Reference operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less. The Active Reference may be disabled using ACT_REF_DIS=0."]
pub type BgrefLpmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_LS_BYPASS` reader - Bypass level shifter inside the PLL. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
pub type PllLsBypassR = crate::BitReader;
#[doc = "Field `PLL_LS_BYPASS` writer - Bypass level shifter inside the PLL. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
pub type PllLsBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFBUF_LPMODE` reader - Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES/POR/BOD/HIBERNATE. 1: Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
pub type VrefbufLpmodeR = crate::BitReader;
#[doc = "Field `VREFBUF_LPMODE` writer - Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES/POR/BOD/HIBERNATE. 1: Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
pub type VrefbufLpmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFBUF_DIS` reader - Disable the 800mV voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type VrefbufDisR = crate::BitReader;
#[doc = "Field `VREFBUF_DIS` writer - Disable the 800mV voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type VrefbufDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACT_REF_DIS` reader - Disables the Active Reference. Firmware must ensure that LPM_READY==1 and BGREF_LPMODE==1 for at least 1us before disabling the Active Reference. When enabling the Active Reference, use ACT_REF_OK indicator to know when it is ready. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Reference is enabled 1: Active Reference is disabled"]
pub type ActRefDisR = crate::BitReader;
#[doc = "Field `ACT_REF_DIS` writer - Disables the Active Reference. Firmware must ensure that LPM_READY==1 and BGREF_LPMODE==1 for at least 1us before disabling the Active Reference. When enabling the Active Reference, use ACT_REF_OK indicator to know when it is ready. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Reference is enabled 1: Active Reference is disabled"]
pub type ActRefDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACT_REF_OK` reader - Indicates that the normal mode of the Active Reference is ready."]
pub type ActRefOkR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon."]
    #[inline(always)]
    pub fn power_mode(&self) -> PowerModeR {
        PowerModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)"]
    #[inline(always)]
    pub fn debug_session(&self) -> DebugSessionR {
        DebugSessionR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates whether certain low power functions are ready. The low current circuits take longer to startup after XRES/POR/BOD/HIBERNATE wakeup than the normal mode circuits. HIBERNATE mode may be entered regardless of this bit. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: If a low power circuit operation is requested, it will stay in its normal operating mode until it is ready. If DEEPSLEEP is requested by all processors WFI/WFE, the device will instead enter SLEEP. When low power circuits are ready, device will automatically enter the originally requested mode. 1: Normal operation. DEEPSLEEP and low power circuits operate as requested in other registers."]
    #[inline(always)]
    pub fn lpm_ready(&self) -> LpmReadyR {
        LpmReadyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 18 - Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn iref_lpmode(&self) -> IrefLpmodeR {
        IrefLpmodeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicates that the voltage reference buffer is ready. Due to synchronization delays, it may take two IMO clock cycles for hardware to clear this bit after asserting VREFBUF_DIS=1."]
    #[inline(always)]
    pub fn vrefbuf_ok(&self) -> VrefbufOkR {
        VrefbufOkR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Disable the DeepSleep regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: DeepSleep Regulator is on. 1: DeepSleep Regulator is off."]
    #[inline(always)]
    pub fn dpslp_reg_dis(&self) -> DpslpRegDisR {
        DpslpRegDisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Disable the Retention regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Retention Regulator is on. 1: Retention Regulator is off."]
    #[inline(always)]
    pub fn ret_reg_dis(&self) -> RetRegDisR {
        RetRegDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Disable the Nwell regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
    #[inline(always)]
    pub fn nwell_reg_dis(&self) -> NwellRegDisR {
        NwellRegDisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Disable the linear Core Regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear regulator is on. 1: Linear regulator is off."]
    #[inline(always)]
    pub fn linreg_dis(&self) -> LinregDisR {
        LinregDisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Control the power mode of the Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
    #[inline(always)]
    pub fn linreg_lpmode(&self) -> LinregLpmodeR {
        LinregLpmodeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Control the power mode of the POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn porbod_lpmode(&self) -> PorbodLpmodeR {
        PorbodLpmodeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Control the power mode of the Bandgap Voltage and Current References. This applies to voltage and current generation and is different than the reference voltage buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. When lower power mode is used, the Active Reference circuit can be disabled to reduce current. Firmware is responsible to ensure ACT_REF_OK==1 before changing back to normal mode. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Bandgap Voltage and Current Reference operates in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: Active Bandgap Voltage and Current Reference operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less. The Active Reference may be disabled using ACT_REF_DIS=0."]
    #[inline(always)]
    pub fn bgref_lpmode(&self) -> BgrefLpmodeR {
        BgrefLpmodeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bypass level shifter inside the PLL. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
    #[inline(always)]
    pub fn pll_ls_bypass(&self) -> PllLsBypassR {
        PllLsBypassR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES/POR/BOD/HIBERNATE. 1: Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn vrefbuf_lpmode(&self) -> VrefbufLpmodeR {
        VrefbufLpmodeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disable the 800mV voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn vrefbuf_dis(&self) -> VrefbufDisR {
        VrefbufDisR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Disables the Active Reference. Firmware must ensure that LPM_READY==1 and BGREF_LPMODE==1 for at least 1us before disabling the Active Reference. When enabling the Active Reference, use ACT_REF_OK indicator to know when it is ready. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Reference is enabled 1: Active Reference is disabled"]
    #[inline(always)]
    pub fn act_ref_dis(&self) -> ActRefDisR {
        ActRefDisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates that the normal mode of the Active Reference is ready."]
    #[inline(always)]
    pub fn act_ref_ok(&self) -> ActRefOkR {
        ActRefOkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    #[must_use]
    pub fn iref_lpmode(&mut self) -> IrefLpmodeW<PwrCtlSpec> {
        IrefLpmodeW::new(self, 18)
    }
    #[doc = "Bit 20 - Disable the DeepSleep regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: DeepSleep Regulator is on. 1: DeepSleep Regulator is off."]
    #[inline(always)]
    #[must_use]
    pub fn dpslp_reg_dis(&mut self) -> DpslpRegDisW<PwrCtlSpec> {
        DpslpRegDisW::new(self, 20)
    }
    #[doc = "Bit 21 - Disable the Retention regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Retention Regulator is on. 1: Retention Regulator is off."]
    #[inline(always)]
    #[must_use]
    pub fn ret_reg_dis(&mut self) -> RetRegDisW<PwrCtlSpec> {
        RetRegDisW::new(self, 21)
    }
    #[doc = "Bit 22 - Disable the Nwell regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
    #[inline(always)]
    #[must_use]
    pub fn nwell_reg_dis(&mut self) -> NwellRegDisW<PwrCtlSpec> {
        NwellRegDisW::new(self, 22)
    }
    #[doc = "Bit 23 - Disable the linear Core Regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear regulator is on. 1: Linear regulator is off."]
    #[inline(always)]
    #[must_use]
    pub fn linreg_dis(&mut self) -> LinregDisW<PwrCtlSpec> {
        LinregDisW::new(self, 23)
    }
    #[doc = "Bit 24 - Control the power mode of the Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
    #[inline(always)]
    #[must_use]
    pub fn linreg_lpmode(&mut self) -> LinregLpmodeW<PwrCtlSpec> {
        LinregLpmodeW::new(self, 24)
    }
    #[doc = "Bit 25 - Control the power mode of the POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    #[must_use]
    pub fn porbod_lpmode(&mut self) -> PorbodLpmodeW<PwrCtlSpec> {
        PorbodLpmodeW::new(self, 25)
    }
    #[doc = "Bit 26 - Control the power mode of the Bandgap Voltage and Current References. This applies to voltage and current generation and is different than the reference voltage buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. When lower power mode is used, the Active Reference circuit can be disabled to reduce current. Firmware is responsible to ensure ACT_REF_OK==1 before changing back to normal mode. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Bandgap Voltage and Current Reference operates in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: Active Bandgap Voltage and Current Reference operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less. The Active Reference may be disabled using ACT_REF_DIS=0."]
    #[inline(always)]
    #[must_use]
    pub fn bgref_lpmode(&mut self) -> BgrefLpmodeW<PwrCtlSpec> {
        BgrefLpmodeW::new(self, 26)
    }
    #[doc = "Bit 27 - Bypass level shifter inside the PLL. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
    #[inline(always)]
    #[must_use]
    pub fn pll_ls_bypass(&mut self) -> PllLsBypassW<PwrCtlSpec> {
        PllLsBypassW::new(self, 27)
    }
    #[doc = "Bit 28 - Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES/POR/BOD/HIBERNATE. 1: Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    #[must_use]
    pub fn vrefbuf_lpmode(&mut self) -> VrefbufLpmodeW<PwrCtlSpec> {
        VrefbufLpmodeW::new(self, 28)
    }
    #[doc = "Bit 29 - Disable the 800mV voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn vrefbuf_dis(&mut self) -> VrefbufDisW<PwrCtlSpec> {
        VrefbufDisW::new(self, 29)
    }
    #[doc = "Bit 30 - Disables the Active Reference. Firmware must ensure that LPM_READY==1 and BGREF_LPMODE==1 for at least 1us before disabling the Active Reference. When enabling the Active Reference, use ACT_REF_OK indicator to know when it is ready. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Reference is enabled 1: Active Reference is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn act_ref_dis(&mut self) -> ActRefDisW<PwrCtlSpec> {
        ActRefDisW::new(self, 30)
    }
}
#[doc = "Power Mode Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrCtlSpec;
impl crate::RegisterSpec for PwrCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_ctl::R`](R) reader structure"]
impl crate::Readable for PwrCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_ctl::W`](W) writer structure"]
impl crate::Writable for PwrCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_CTL to value 0"]
impl crate::Resettable for PwrCtlSpec {
    const RESET_VALUE: u32 = 0;
}
