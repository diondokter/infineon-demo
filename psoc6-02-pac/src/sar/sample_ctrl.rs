#[doc = "Register `SAMPLE_CTRL` reader"]
pub type R = crate::R<SampleCtrlSpec>;
#[doc = "Register `SAMPLE_CTRL` writer"]
pub type W = crate::W<SampleCtrlSpec>;
#[doc = "Field `LEFT_ALIGN` reader - Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
pub type LeftAlignR = crate::BitReader;
#[doc = "Field `LEFT_ALIGN` writer - Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
pub type LeftAlignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Output data from a single ended conversion as a signed value If AVG_MODE = 1 (Interleaved averaging), then SINGLE_ENDED_SIGNED must be configured identically to DIFFERENTIAL_SIGNED.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SingleEndedSigned {
    #[doc = "0: Default: result data is unsigned (zero extended if needed)"]
    Unsigned = 0,
    #[doc = "1: result data is signed (sign extended if needed)"]
    Signed = 1,
}
impl From<SingleEndedSigned> for bool {
    #[inline(always)]
    fn from(variant: SingleEndedSigned) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINGLE_ENDED_SIGNED` reader - Output data from a single ended conversion as a signed value If AVG_MODE = 1 (Interleaved averaging), then SINGLE_ENDED_SIGNED must be configured identically to DIFFERENTIAL_SIGNED."]
pub type SingleEndedSignedR = crate::BitReader<SingleEndedSigned>;
impl SingleEndedSignedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SingleEndedSigned {
        match self.bits {
            false => SingleEndedSigned::Unsigned,
            true => SingleEndedSigned::Signed,
        }
    }
    #[doc = "Default: result data is unsigned (zero extended if needed)"]
    #[inline(always)]
    pub fn is_unsigned(&self) -> bool {
        *self == SingleEndedSigned::Unsigned
    }
    #[doc = "result data is signed (sign extended if needed)"]
    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        *self == SingleEndedSigned::Signed
    }
}
#[doc = "Field `SINGLE_ENDED_SIGNED` writer - Output data from a single ended conversion as a signed value If AVG_MODE = 1 (Interleaved averaging), then SINGLE_ENDED_SIGNED must be configured identically to DIFFERENTIAL_SIGNED."]
pub type SingleEndedSignedW<'a, REG> = crate::BitWriter<'a, REG, SingleEndedSigned>;
impl<'a, REG> SingleEndedSignedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default: result data is unsigned (zero extended if needed)"]
    #[inline(always)]
    pub fn unsigned(self) -> &'a mut crate::W<REG> {
        self.variant(SingleEndedSigned::Unsigned)
    }
    #[doc = "result data is signed (sign extended if needed)"]
    #[inline(always)]
    pub fn signed(self) -> &'a mut crate::W<REG> {
        self.variant(SingleEndedSigned::Signed)
    }
}
#[doc = "Output data from a differential conversion as a signed value when DIFFERENTIAL_EN or NEG_ADDR_EN is set to 1 If AVG_MODE = 1 (Interleaved averaging), then DIFFERENTIAL_SIGNED must be configured identically to SINGLE_ENDED_SIGNED.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DifferentialSigned {
    #[doc = "0: result data is unsigned (zero extended if needed)"]
    Unsigned = 0,
    #[doc = "1: Default: result data is signed (sign extended if needed)"]
    Signed = 1,
}
impl From<DifferentialSigned> for bool {
    #[inline(always)]
    fn from(variant: DifferentialSigned) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFFERENTIAL_SIGNED` reader - Output data from a differential conversion as a signed value when DIFFERENTIAL_EN or NEG_ADDR_EN is set to 1 If AVG_MODE = 1 (Interleaved averaging), then DIFFERENTIAL_SIGNED must be configured identically to SINGLE_ENDED_SIGNED."]
pub type DifferentialSignedR = crate::BitReader<DifferentialSigned>;
impl DifferentialSignedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DifferentialSigned {
        match self.bits {
            false => DifferentialSigned::Unsigned,
            true => DifferentialSigned::Signed,
        }
    }
    #[doc = "result data is unsigned (zero extended if needed)"]
    #[inline(always)]
    pub fn is_unsigned(&self) -> bool {
        *self == DifferentialSigned::Unsigned
    }
    #[doc = "Default: result data is signed (sign extended if needed)"]
    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        *self == DifferentialSigned::Signed
    }
}
#[doc = "Field `DIFFERENTIAL_SIGNED` writer - Output data from a differential conversion as a signed value when DIFFERENTIAL_EN or NEG_ADDR_EN is set to 1 If AVG_MODE = 1 (Interleaved averaging), then DIFFERENTIAL_SIGNED must be configured identically to SINGLE_ENDED_SIGNED."]
pub type DifferentialSignedW<'a, REG> = crate::BitWriter<'a, REG, DifferentialSigned>;
impl<'a, REG> DifferentialSignedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "result data is unsigned (zero extended if needed)"]
    #[inline(always)]
    pub fn unsigned(self) -> &'a mut crate::W<REG> {
        self.variant(DifferentialSigned::Unsigned)
    }
    #[doc = "Default: result data is signed (sign extended if needed)"]
    #[inline(always)]
    pub fn signed(self) -> &'a mut crate::W<REG> {
        self.variant(DifferentialSigned::Signed)
    }
}
#[doc = "Field `AVG_CNT` reader - Averaging Count for channels that have averaging enabled (AVG_EN). A channel will be sampled (1&lt;&lt;(AVG_CNT+1)) = \\[2..256\\]
times. - In ACCUNDUMP mode (1st order accumulate and dump filter) a channel will be sampled back to back, the average result is calculated and stored and then the next enabled channel is sampled. If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3). - In INTERLEAVED mode one sample is taken per triggered scan, only in the scan where the final averaging count is reached a valid average is calculated and stored in the RESULT register (by definition the same scan for all the channels that have averaging enabled). In all other scans the RESULT register for averaged channels will have an invalid result and the intermediate accumulated value is stored in the 16-bit WORK register. In this mode make sure that the averaging count is low enough to ensure that the intermediate value does not exceed 16-bits otherwise the MSBs will be lost. So for a 12-bit resolution the averaging count should be set to 16 or less (AVG_CNT=&lt;3)."]
pub type AvgCntR = crate::FieldReader;
#[doc = "Field `AVG_CNT` writer - Averaging Count for channels that have averaging enabled (AVG_EN). A channel will be sampled (1&lt;&lt;(AVG_CNT+1)) = \\[2..256\\]
times. - In ACCUNDUMP mode (1st order accumulate and dump filter) a channel will be sampled back to back, the average result is calculated and stored and then the next enabled channel is sampled. If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3). - In INTERLEAVED mode one sample is taken per triggered scan, only in the scan where the final averaging count is reached a valid average is calculated and stored in the RESULT register (by definition the same scan for all the channels that have averaging enabled). In all other scans the RESULT register for averaged channels will have an invalid result and the intermediate accumulated value is stored in the 16-bit WORK register. In this mode make sure that the averaging count is low enough to ensure that the intermediate value does not exceed 16-bits otherwise the MSBs will be lost. So for a 12-bit resolution the averaging count should be set to 16 or less (AVG_CNT=&lt;3)."]
pub type AvgCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AVG_SHIFT` reader - Averaging shifting: after averaging the result is shifted right to fit in 12 bits."]
pub type AvgShiftR = crate::BitReader;
#[doc = "Field `AVG_SHIFT` writer - Averaging shifting: after averaging the result is shifted right to fit in 12 bits."]
pub type AvgShiftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Averaging mode, in DSI mode this bit is ignored and only AccuNDump mode is available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvgMode {
    #[doc = "0: Accumulate and Dump (1st order accumulate and dump filter): a channel will be sampled back to back and averaged"]
    Accundump = 0,
    #[doc = "1: Interleaved: Each scan (trigger) one sample is taken per channel and averaged over several scans."]
    Interleaved = 1,
}
impl From<AvgMode> for bool {
    #[inline(always)]
    fn from(variant: AvgMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVG_MODE` reader - Averaging mode, in DSI mode this bit is ignored and only AccuNDump mode is available."]
pub type AvgModeR = crate::BitReader<AvgMode>;
impl AvgModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AvgMode {
        match self.bits {
            false => AvgMode::Accundump,
            true => AvgMode::Interleaved,
        }
    }
    #[doc = "Accumulate and Dump (1st order accumulate and dump filter): a channel will be sampled back to back and averaged"]
    #[inline(always)]
    pub fn is_accundump(&self) -> bool {
        *self == AvgMode::Accundump
    }
    #[doc = "Interleaved: Each scan (trigger) one sample is taken per channel and averaged over several scans."]
    #[inline(always)]
    pub fn is_interleaved(&self) -> bool {
        *self == AvgMode::Interleaved
    }
}
#[doc = "Field `AVG_MODE` writer - Averaging mode, in DSI mode this bit is ignored and only AccuNDump mode is available."]
pub type AvgModeW<'a, REG> = crate::BitWriter<'a, REG, AvgMode>;
impl<'a, REG> AvgModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Accumulate and Dump (1st order accumulate and dump filter): a channel will be sampled back to back and averaged"]
    #[inline(always)]
    pub fn accundump(self) -> &'a mut crate::W<REG> {
        self.variant(AvgMode::Accundump)
    }
    #[doc = "Interleaved: Each scan (trigger) one sample is taken per channel and averaged over several scans."]
    #[inline(always)]
    pub fn interleaved(self) -> &'a mut crate::W<REG> {
        self.variant(AvgMode::Interleaved)
    }
}
#[doc = "Field `CONTINUOUS` reader - - 0: Wait for next FW_TRIGGER (one shot) or hardware trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
pub type ContinuousR = crate::BitReader;
#[doc = "Field `CONTINUOUS` writer - - 0: Wait for next FW_TRIGGER (one shot) or hardware trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
pub type ContinuousW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_TRIGGER_EN` reader - - 0: firmware trigger only: disable hardware trigger tr_sar_in. - 1: enable hardware trigger tr_sar_in (e.g. from TCPWM, GPIO or UDB)."]
pub type DsiTriggerEnR = crate::BitReader;
#[doc = "Field `DSI_TRIGGER_EN` writer - - 0: firmware trigger only: disable hardware trigger tr_sar_in. - 1: enable hardware trigger tr_sar_in (e.g. from TCPWM, GPIO or UDB)."]
pub type DsiTriggerEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_TRIGGER_LEVEL` reader - - 0: trigger signal is a pulse input, a positive edge detected on the trigger signal triggers a new scan. - 1: trigger signal is a level input, as long as the trigger signal remains high the SAR will do continuous scans."]
pub type DsiTriggerLevelR = crate::BitReader;
#[doc = "Field `DSI_TRIGGER_LEVEL` writer - - 0: trigger signal is a pulse input, a positive edge detected on the trigger signal triggers a new scan. - 1: trigger signal is a level input, as long as the trigger signal remains high the SAR will do continuous scans."]
pub type DsiTriggerLevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_SYNC_TRIGGER` reader - - 0: bypass clock domain synchronization of the trigger signal. - 1: synchronize the trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
pub type DsiSyncTriggerR = crate::BitReader;
#[doc = "Field `DSI_SYNC_TRIGGER` writer - - 0: bypass clock domain synchronization of the trigger signal. - 1: synchronize the trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
pub type DsiSyncTriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Select whether UABs are scheduled or unscheduled. When no UAB is scanned this selection is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UabScanMode {
    #[doc = "0: Unscheduled UABs: one or more of the UABs scanned by the SAR is not scheduled, for each channel that scans a UAB the SAR will wait for a positive edge on the trigger output of that UAB. Caveat: in this mode the length of SAR scan can be variable."]
    Unscheduled = 0,
    #[doc = "1: Scheduled UABs: All UABs scanned by the SAR are assumed to be properly scheduled, i.e. their output is assumed to be valid when sampled by the SAR and the SAR does not wait. In this mode the length of the SAR scan is constant. This mode requires that the SAR scans strictly periodically, i.e. the SAR has to either run continuously or has to be triggered by a periodic hardware trigger (TCPWM or UDB timer). It also requires that the end of the UAB valid phase is precisely aligned with the end of the SAR sample period (using UAB.STARTUP_DELAY). Normally this scheduling is done by Creator."]
    Scheduled = 1,
}
impl From<UabScanMode> for bool {
    #[inline(always)]
    fn from(variant: UabScanMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UAB_SCAN_MODE` reader - Select whether UABs are scheduled or unscheduled. When no UAB is scanned this selection is ignored."]
pub type UabScanModeR = crate::BitReader<UabScanMode>;
impl UabScanModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UabScanMode {
        match self.bits {
            false => UabScanMode::Unscheduled,
            true => UabScanMode::Scheduled,
        }
    }
    #[doc = "Unscheduled UABs: one or more of the UABs scanned by the SAR is not scheduled, for each channel that scans a UAB the SAR will wait for a positive edge on the trigger output of that UAB. Caveat: in this mode the length of SAR scan can be variable."]
    #[inline(always)]
    pub fn is_unscheduled(&self) -> bool {
        *self == UabScanMode::Unscheduled
    }
    #[doc = "Scheduled UABs: All UABs scanned by the SAR are assumed to be properly scheduled, i.e. their output is assumed to be valid when sampled by the SAR and the SAR does not wait. In this mode the length of the SAR scan is constant. This mode requires that the SAR scans strictly periodically, i.e. the SAR has to either run continuously or has to be triggered by a periodic hardware trigger (TCPWM or UDB timer). It also requires that the end of the UAB valid phase is precisely aligned with the end of the SAR sample period (using UAB.STARTUP_DELAY). Normally this scheduling is done by Creator."]
    #[inline(always)]
    pub fn is_scheduled(&self) -> bool {
        *self == UabScanMode::Scheduled
    }
}
#[doc = "Field `UAB_SCAN_MODE` writer - Select whether UABs are scheduled or unscheduled. When no UAB is scanned this selection is ignored."]
pub type UabScanModeW<'a, REG> = crate::BitWriter<'a, REG, UabScanMode>;
impl<'a, REG> UabScanModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Unscheduled UABs: one or more of the UABs scanned by the SAR is not scheduled, for each channel that scans a UAB the SAR will wait for a positive edge on the trigger output of that UAB. Caveat: in this mode the length of SAR scan can be variable."]
    #[inline(always)]
    pub fn unscheduled(self) -> &'a mut crate::W<REG> {
        self.variant(UabScanMode::Unscheduled)
    }
    #[doc = "Scheduled UABs: All UABs scanned by the SAR are assumed to be properly scheduled, i.e. their output is assumed to be valid when sampled by the SAR and the SAR does not wait. In this mode the length of the SAR scan is constant. This mode requires that the SAR scans strictly periodically, i.e. the SAR has to either run continuously or has to be triggered by a periodic hardware trigger (TCPWM or UDB timer). It also requires that the end of the UAB valid phase is precisely aligned with the end of the SAR sample period (using UAB.STARTUP_DELAY). Normally this scheduling is done by Creator."]
    #[inline(always)]
    pub fn scheduled(self) -> &'a mut crate::W<REG> {
        self.variant(UabScanMode::Scheduled)
    }
}
#[doc = "Field `REPEAT_INVALID` reader - For unscheduled UAB_SCAN_MODE only, do the following if an invalid sample is received: - 0: use the last known valid sample for that channel and clear the NEWVALUE flag - 1: repeat the conversions until a valid sample is received (caveat: could be never if the UAB valid window is incorrectly schedule w.r.t. SAR sampling)"]
pub type RepeatInvalidR = crate::BitReader;
#[doc = "Field `REPEAT_INVALID` writer - For unscheduled UAB_SCAN_MODE only, do the following if an invalid sample is received: - 0: use the last known valid sample for that channel and clear the NEWVALUE flag - 1: repeat the conversions until a valid sample is received (caveat: could be never if the UAB valid window is incorrectly schedule w.r.t. SAR sampling)"]
pub type RepeatInvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALID_SEL` reader - Static UAB Valid select 0=UAB0 half 0 Valid output 1=UAB0 half 1 Valid output 2=UAB1 half 0 Valid output 3=UAB1 half 1 Valid output 4=UAB2 half 0 Valid output 5=UAB2 half 1 Valid output 6=UAB3 half 0 Valid output 7=UAB3 half 1 Valid output"]
pub type ValidSelR = crate::FieldReader;
#[doc = "Field `VALID_SEL` writer - Static UAB Valid select 0=UAB0 half 0 Valid output 1=UAB0 half 1 Valid output 2=UAB1 half 0 Valid output 3=UAB1 half 1 Valid output 4=UAB2 half 0 Valid output 5=UAB2 half 1 Valid output 6=UAB3 half 0 Valid output 7=UAB3 half 1 Valid output"]
pub type ValidSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VALID_SEL_EN` reader - Enable static UAB Valid selection (override Hardware)"]
pub type ValidSelEnR = crate::BitReader;
#[doc = "Field `VALID_SEL_EN` writer - Enable static UAB Valid selection (override Hardware)"]
pub type ValidSelEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALID_IGNORE` reader - Ignore UAB valid signal, including the dynamic/Hardware from AROUTE and the static Valid selection from the VALID_SEL fields above"]
pub type ValidIgnoreR = crate::BitReader;
#[doc = "Field `VALID_IGNORE` writer - Ignore UAB valid signal, including the dynamic/Hardware from AROUTE and the static Valid selection from the VALID_SEL fields above"]
pub type ValidIgnoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGGER_OUT_EN` reader - SAR output trigger enable (used for UAB synchronization). To ensure multiple UABs starting at the same trigger it is recommended to use this bit to temporarily disable the trigger output until all those UABs are set to run (UAB.SRAM_CTRL.RUN=1)."]
pub type TriggerOutEnR = crate::BitReader;
#[doc = "Field `TRIGGER_OUT_EN` writer - SAR output trigger enable (used for UAB synchronization). To ensure multiple UABs starting at the same trigger it is recommended to use this bit to temporarily disable the trigger output until all those UABs are set to run (UAB.SRAM_CTRL.RUN=1)."]
pub type TriggerOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS_DSI_OUT_EN` reader - Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a trigger pulse is send on the tr_sar_out signal."]
pub type EosDsiOutEnR = crate::BitReader;
#[doc = "Field `EOS_DSI_OUT_EN` writer - Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a trigger pulse is send on the tr_sar_out signal."]
pub type EosDsiOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
    #[inline(always)]
    pub fn left_align(&self) -> LeftAlignR {
        LeftAlignR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output data from a single ended conversion as a signed value If AVG_MODE = 1 (Interleaved averaging), then SINGLE_ENDED_SIGNED must be configured identically to DIFFERENTIAL_SIGNED."]
    #[inline(always)]
    pub fn single_ended_signed(&self) -> SingleEndedSignedR {
        SingleEndedSignedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output data from a differential conversion as a signed value when DIFFERENTIAL_EN or NEG_ADDR_EN is set to 1 If AVG_MODE = 1 (Interleaved averaging), then DIFFERENTIAL_SIGNED must be configured identically to SINGLE_ENDED_SIGNED."]
    #[inline(always)]
    pub fn differential_signed(&self) -> DifferentialSignedR {
        DifferentialSignedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Averaging Count for channels that have averaging enabled (AVG_EN). A channel will be sampled (1&lt;&lt;(AVG_CNT+1)) = \\[2..256\\]
times. - In ACCUNDUMP mode (1st order accumulate and dump filter) a channel will be sampled back to back, the average result is calculated and stored and then the next enabled channel is sampled. If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3). - In INTERLEAVED mode one sample is taken per triggered scan, only in the scan where the final averaging count is reached a valid average is calculated and stored in the RESULT register (by definition the same scan for all the channels that have averaging enabled). In all other scans the RESULT register for averaged channels will have an invalid result and the intermediate accumulated value is stored in the 16-bit WORK register. In this mode make sure that the averaging count is low enough to ensure that the intermediate value does not exceed 16-bits otherwise the MSBs will be lost. So for a 12-bit resolution the averaging count should be set to 16 or less (AVG_CNT=&lt;3)."]
    #[inline(always)]
    pub fn avg_cnt(&self) -> AvgCntR {
        AvgCntR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Averaging shifting: after averaging the result is shifted right to fit in 12 bits."]
    #[inline(always)]
    pub fn avg_shift(&self) -> AvgShiftR {
        AvgShiftR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Averaging mode, in DSI mode this bit is ignored and only AccuNDump mode is available."]
    #[inline(always)]
    pub fn avg_mode(&self) -> AvgModeR {
        AvgModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - - 0: Wait for next FW_TRIGGER (one shot) or hardware trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
    #[inline(always)]
    pub fn continuous(&self) -> ContinuousR {
        ContinuousR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - - 0: firmware trigger only: disable hardware trigger tr_sar_in. - 1: enable hardware trigger tr_sar_in (e.g. from TCPWM, GPIO or UDB)."]
    #[inline(always)]
    pub fn dsi_trigger_en(&self) -> DsiTriggerEnR {
        DsiTriggerEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - - 0: trigger signal is a pulse input, a positive edge detected on the trigger signal triggers a new scan. - 1: trigger signal is a level input, as long as the trigger signal remains high the SAR will do continuous scans."]
    #[inline(always)]
    pub fn dsi_trigger_level(&self) -> DsiTriggerLevelR {
        DsiTriggerLevelR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - - 0: bypass clock domain synchronization of the trigger signal. - 1: synchronize the trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
    #[inline(always)]
    pub fn dsi_sync_trigger(&self) -> DsiSyncTriggerR {
        DsiSyncTriggerR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Select whether UABs are scheduled or unscheduled. When no UAB is scanned this selection is ignored."]
    #[inline(always)]
    pub fn uab_scan_mode(&self) -> UabScanModeR {
        UabScanModeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - For unscheduled UAB_SCAN_MODE only, do the following if an invalid sample is received: - 0: use the last known valid sample for that channel and clear the NEWVALUE flag - 1: repeat the conversions until a valid sample is received (caveat: could be never if the UAB valid window is incorrectly schedule w.r.t. SAR sampling)"]
    #[inline(always)]
    pub fn repeat_invalid(&self) -> RepeatInvalidR {
        RepeatInvalidR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Static UAB Valid select 0=UAB0 half 0 Valid output 1=UAB0 half 1 Valid output 2=UAB1 half 0 Valid output 3=UAB1 half 1 Valid output 4=UAB2 half 0 Valid output 5=UAB2 half 1 Valid output 6=UAB3 half 0 Valid output 7=UAB3 half 1 Valid output"]
    #[inline(always)]
    pub fn valid_sel(&self) -> ValidSelR {
        ValidSelR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Enable static UAB Valid selection (override Hardware)"]
    #[inline(always)]
    pub fn valid_sel_en(&self) -> ValidSelEnR {
        ValidSelEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Ignore UAB valid signal, including the dynamic/Hardware from AROUTE and the static Valid selection from the VALID_SEL fields above"]
    #[inline(always)]
    pub fn valid_ignore(&self) -> ValidIgnoreR {
        ValidIgnoreR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - SAR output trigger enable (used for UAB synchronization). To ensure multiple UABs starting at the same trigger it is recommended to use this bit to temporarily disable the trigger output until all those UABs are set to run (UAB.SRAM_CTRL.RUN=1)."]
    #[inline(always)]
    pub fn trigger_out_en(&self) -> TriggerOutEnR {
        TriggerOutEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a trigger pulse is send on the tr_sar_out signal."]
    #[inline(always)]
    pub fn eos_dsi_out_en(&self) -> EosDsiOutEnR {
        EosDsiOutEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
    #[inline(always)]
    #[must_use]
    pub fn left_align(&mut self) -> LeftAlignW<SampleCtrlSpec> {
        LeftAlignW::new(self, 1)
    }
    #[doc = "Bit 2 - Output data from a single ended conversion as a signed value If AVG_MODE = 1 (Interleaved averaging), then SINGLE_ENDED_SIGNED must be configured identically to DIFFERENTIAL_SIGNED."]
    #[inline(always)]
    #[must_use]
    pub fn single_ended_signed(&mut self) -> SingleEndedSignedW<SampleCtrlSpec> {
        SingleEndedSignedW::new(self, 2)
    }
    #[doc = "Bit 3 - Output data from a differential conversion as a signed value when DIFFERENTIAL_EN or NEG_ADDR_EN is set to 1 If AVG_MODE = 1 (Interleaved averaging), then DIFFERENTIAL_SIGNED must be configured identically to SINGLE_ENDED_SIGNED."]
    #[inline(always)]
    #[must_use]
    pub fn differential_signed(&mut self) -> DifferentialSignedW<SampleCtrlSpec> {
        DifferentialSignedW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Averaging Count for channels that have averaging enabled (AVG_EN). A channel will be sampled (1&lt;&lt;(AVG_CNT+1)) = \\[2..256\\]
times. - In ACCUNDUMP mode (1st order accumulate and dump filter) a channel will be sampled back to back, the average result is calculated and stored and then the next enabled channel is sampled. If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3). - In INTERLEAVED mode one sample is taken per triggered scan, only in the scan where the final averaging count is reached a valid average is calculated and stored in the RESULT register (by definition the same scan for all the channels that have averaging enabled). In all other scans the RESULT register for averaged channels will have an invalid result and the intermediate accumulated value is stored in the 16-bit WORK register. In this mode make sure that the averaging count is low enough to ensure that the intermediate value does not exceed 16-bits otherwise the MSBs will be lost. So for a 12-bit resolution the averaging count should be set to 16 or less (AVG_CNT=&lt;3)."]
    #[inline(always)]
    #[must_use]
    pub fn avg_cnt(&mut self) -> AvgCntW<SampleCtrlSpec> {
        AvgCntW::new(self, 4)
    }
    #[doc = "Bit 7 - Averaging shifting: after averaging the result is shifted right to fit in 12 bits."]
    #[inline(always)]
    #[must_use]
    pub fn avg_shift(&mut self) -> AvgShiftW<SampleCtrlSpec> {
        AvgShiftW::new(self, 7)
    }
    #[doc = "Bit 8 - Averaging mode, in DSI mode this bit is ignored and only AccuNDump mode is available."]
    #[inline(always)]
    #[must_use]
    pub fn avg_mode(&mut self) -> AvgModeW<SampleCtrlSpec> {
        AvgModeW::new(self, 8)
    }
    #[doc = "Bit 16 - - 0: Wait for next FW_TRIGGER (one shot) or hardware trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
    #[inline(always)]
    #[must_use]
    pub fn continuous(&mut self) -> ContinuousW<SampleCtrlSpec> {
        ContinuousW::new(self, 16)
    }
    #[doc = "Bit 17 - - 0: firmware trigger only: disable hardware trigger tr_sar_in. - 1: enable hardware trigger tr_sar_in (e.g. from TCPWM, GPIO or UDB)."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_trigger_en(&mut self) -> DsiTriggerEnW<SampleCtrlSpec> {
        DsiTriggerEnW::new(self, 17)
    }
    #[doc = "Bit 18 - - 0: trigger signal is a pulse input, a positive edge detected on the trigger signal triggers a new scan. - 1: trigger signal is a level input, as long as the trigger signal remains high the SAR will do continuous scans."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_trigger_level(&mut self) -> DsiTriggerLevelW<SampleCtrlSpec> {
        DsiTriggerLevelW::new(self, 18)
    }
    #[doc = "Bit 19 - - 0: bypass clock domain synchronization of the trigger signal. - 1: synchronize the trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_sync_trigger(&mut self) -> DsiSyncTriggerW<SampleCtrlSpec> {
        DsiSyncTriggerW::new(self, 19)
    }
    #[doc = "Bit 22 - Select whether UABs are scheduled or unscheduled. When no UAB is scanned this selection is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn uab_scan_mode(&mut self) -> UabScanModeW<SampleCtrlSpec> {
        UabScanModeW::new(self, 22)
    }
    #[doc = "Bit 23 - For unscheduled UAB_SCAN_MODE only, do the following if an invalid sample is received: - 0: use the last known valid sample for that channel and clear the NEWVALUE flag - 1: repeat the conversions until a valid sample is received (caveat: could be never if the UAB valid window is incorrectly schedule w.r.t. SAR sampling)"]
    #[inline(always)]
    #[must_use]
    pub fn repeat_invalid(&mut self) -> RepeatInvalidW<SampleCtrlSpec> {
        RepeatInvalidW::new(self, 23)
    }
    #[doc = "Bits 24:26 - Static UAB Valid select 0=UAB0 half 0 Valid output 1=UAB0 half 1 Valid output 2=UAB1 half 0 Valid output 3=UAB1 half 1 Valid output 4=UAB2 half 0 Valid output 5=UAB2 half 1 Valid output 6=UAB3 half 0 Valid output 7=UAB3 half 1 Valid output"]
    #[inline(always)]
    #[must_use]
    pub fn valid_sel(&mut self) -> ValidSelW<SampleCtrlSpec> {
        ValidSelW::new(self, 24)
    }
    #[doc = "Bit 27 - Enable static UAB Valid selection (override Hardware)"]
    #[inline(always)]
    #[must_use]
    pub fn valid_sel_en(&mut self) -> ValidSelEnW<SampleCtrlSpec> {
        ValidSelEnW::new(self, 27)
    }
    #[doc = "Bit 28 - Ignore UAB valid signal, including the dynamic/Hardware from AROUTE and the static Valid selection from the VALID_SEL fields above"]
    #[inline(always)]
    #[must_use]
    pub fn valid_ignore(&mut self) -> ValidIgnoreW<SampleCtrlSpec> {
        ValidIgnoreW::new(self, 28)
    }
    #[doc = "Bit 30 - SAR output trigger enable (used for UAB synchronization). To ensure multiple UABs starting at the same trigger it is recommended to use this bit to temporarily disable the trigger output until all those UABs are set to run (UAB.SRAM_CTRL.RUN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn trigger_out_en(&mut self) -> TriggerOutEnW<SampleCtrlSpec> {
        TriggerOutEnW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a trigger pulse is send on the tr_sar_out signal."]
    #[inline(always)]
    #[must_use]
    pub fn eos_dsi_out_en(&mut self) -> EosDsiOutEnW<SampleCtrlSpec> {
        EosDsiOutEnW::new(self, 31)
    }
}
#[doc = "Sample control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sample_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SampleCtrlSpec;
impl crate::RegisterSpec for SampleCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sample_ctrl::R`](R) reader structure"]
impl crate::Readable for SampleCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sample_ctrl::W`](W) writer structure"]
impl crate::Writable for SampleCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAMPLE_CTRL to value 0x0008_0008"]
impl crate::Resettable for SampleCtrlSpec {
    const RESET_VALUE: u32 = 0x0008_0008;
}
