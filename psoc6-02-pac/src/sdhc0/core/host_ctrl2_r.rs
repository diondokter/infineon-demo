#[doc = "Register `HOST_CTRL2_R` reader"]
pub type R = crate::R<HostCtrl2RSpec>;
#[doc = "Register `HOST_CTRL2_R` writer"]
pub type W = crate::W<HostCtrl2RSpec>;
#[doc = "Field `UHS_MODE_SEL` reader - N/A"]
pub type UhsModeSelR = crate::FieldReader;
#[doc = "Field `UHS_MODE_SEL` writer - N/A"]
pub type UhsModeSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SIGNALING_EN` reader - 1.8V Signaling Enable This bit controls voltage regulator for I/O cell in SD UHS-I mode. Setting this bit from 0 to 1 starts changing the signal voltage from 3.3V to 1.8V. Host Controller clears this bit if switching to 1.8V signaling fails per protocol. The value is reflected on the io_volt_sel output which can then be used to change an external regulator to supply 1.8V instead of 3.3V on the VDDIO pin associated with the CLK/CMD/DAT signals. Note: This bit must be set for all UHS-I speed modes (SDR12/SDR25/SDR50/DDR50). Values: - 0x0 (V_3_3): 3.3V Signalling - 0x1 (V_1_8): 1.8V Signalling"]
pub type SignalingEnR = crate::BitReader;
#[doc = "Field `SIGNALING_EN` writer - 1.8V Signaling Enable This bit controls voltage regulator for I/O cell in SD UHS-I mode. Setting this bit from 0 to 1 starts changing the signal voltage from 3.3V to 1.8V. Host Controller clears this bit if switching to 1.8V signaling fails per protocol. The value is reflected on the io_volt_sel output which can then be used to change an external regulator to supply 1.8V instead of 3.3V on the VDDIO pin associated with the CLK/CMD/DAT signals. Note: This bit must be set for all UHS-I speed modes (SDR12/SDR25/SDR50/DDR50). Values: - 0x0 (V_3_3): 3.3V Signalling - 0x1 (V_1_8): 1.8V Signalling"]
pub type SignalingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRV_STRENGTH_SEL` reader - Driver Strength Select These bits are used to select the Host Controller output driver in 1.8V signaling UHS-I/eMMC speed modes. The value is reflected on the io_drive_strength\\[1:0\\]
output. - 0x0 (TYPEB): Driver TYPEB is selected - 0x1 (TYPEA): Driver TYPEA is selected - 0x2 (TYPEC): Driver TYPEC is selected - 0x3 (TYPED): Driver TYPED is selected"]
pub type DrvStrengthSelR = crate::FieldReader;
#[doc = "Field `DRV_STRENGTH_SEL` writer - Driver Strength Select These bits are used to select the Host Controller output driver in 1.8V signaling UHS-I/eMMC speed modes. The value is reflected on the io_drive_strength\\[1:0\\]
output. - 0x0 (TYPEB): Driver TYPEB is selected - 0x1 (TYPEA): Driver TYPEA is selected - 0x2 (TYPEC): Driver TYPEC is selected - 0x3 (TYPED): Driver TYPED is selected"]
pub type DrvStrengthSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXEC_TUNING` reader - N/A"]
pub type ExecTuningR = crate::BitReader;
#[doc = "Field `EXEC_TUNING` writer - N/A"]
pub type ExecTuningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLE_CLK_SEL` reader - N/A"]
pub type SampleClkSelR = crate::BitReader;
#[doc = "Field `SAMPLE_CLK_SEL` writer - N/A"]
pub type SampleClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHS2_IF_ENABLE` reader - N/A"]
pub type Uhs2IfEnableR = crate::BitReader;
#[doc = "Field `UHS2_IF_ENABLE` writer - N/A"]
pub type Uhs2IfEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA2_LEN_MODE` reader - ADMA2 Length Mode This bit selects ADMA2 Length mode to be either 16-bit or 26-bit. Values: - 0x0 (FALSE): 16-bit Data Length Mode - 0x1 (TRUE): 26-bit Data Length Mode"]
pub type Adma2LenModeR = crate::BitReader;
#[doc = "Field `ADMA2_LEN_MODE` writer - ADMA2 Length Mode This bit selects ADMA2 Length mode to be either 16-bit or 26-bit. Values: - 0x0 (FALSE): 16-bit Data Length Mode - 0x1 (TRUE): 26-bit Data Length Mode"]
pub type Adma2LenModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD23_ENABLE` reader - CMD23 Enable If the card supports CMD23, this bit is set to 1. This bit is used to select Auto CMD23 or Auto CMD12 for ADMA3 data transfer. Values: - 0x0 (FALSE): Auto CMD23 is disabled - 0x1 (TRUE): Auto CMD23 is enabled"]
pub type Cmd23EnableR = crate::BitReader;
#[doc = "Field `CMD23_ENABLE` writer - CMD23 Enable If the card supports CMD23, this bit is set to 1. This bit is used to select Auto CMD23 or Auto CMD12 for ADMA3 data transfer. Values: - 0x0 (FALSE): Auto CMD23 is disabled - 0x1 (TRUE): Auto CMD23 is enabled"]
pub type Cmd23EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_VER4_ENABLE` reader - Host Version 4 Enable This bit selects either Version 3.00 compatible mode or Version 4 mode. Functions of following fields are modified for Host Version 4 mode: - SDMA Address: SDMA uses ADMA System Address (05Fh-058h) instead of SDMA System Address register (003h-000h) - ADMA2/ADMA3 selection: ADMA3 is selected by DMA select in Host Control 1 register - 32-bit Block Count: SDMA System Address register (003h-000h) is modified to 32-bit Block Count register Note: It is recommended not to program ADMA3 Integrated Descriptor Address registers and Command Queuing registers (if applicable) while operating in Host version less than 4 mode (Host Version 4 Enable = 0). Values: - 0x0 (FALSE): Version 3.00 compatible mode - 0x1 (TRUE): Version 4 mode"]
pub type HostVer4EnableR = crate::BitReader;
#[doc = "Field `HOST_VER4_ENABLE` writer - Host Version 4 Enable This bit selects either Version 3.00 compatible mode or Version 4 mode. Functions of following fields are modified for Host Version 4 mode: - SDMA Address: SDMA uses ADMA System Address (05Fh-058h) instead of SDMA System Address register (003h-000h) - ADMA2/ADMA3 selection: ADMA3 is selected by DMA select in Host Control 1 register - 32-bit Block Count: SDMA System Address register (003h-000h) is modified to 32-bit Block Count register Note: It is recommended not to program ADMA3 Integrated Descriptor Address registers and Command Queuing registers (if applicable) while operating in Host version less than 4 mode (Host Version 4 Enable = 0). Values: - 0x0 (FALSE): Version 3.00 compatible mode - 0x1 (TRUE): Version 4 mode"]
pub type HostVer4EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSING` reader - N/A"]
pub type AddressingR = crate::BitReader;
#[doc = "Field `ADDRESSING` writer - N/A"]
pub type AddressingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNC_INT_ENABLE` reader - Asynchronous Interrupt Enable This bit can be set if a card supports asynchronous interrupts and Asynchronous Interrupt Support is set to 1 in the Capabilities register. Values: - 0x0 (FALSE): Disabled - 0x1 (TRUE): Enabled"]
pub type AsyncIntEnableR = crate::BitReader;
#[doc = "Field `ASYNC_INT_ENABLE` writer - Asynchronous Interrupt Enable This bit can be set if a card supports asynchronous interrupts and Asynchronous Interrupt Support is set to 1 in the Capabilities register. Values: - 0x0 (FALSE): Disabled - 0x1 (TRUE): Enabled"]
pub type AsyncIntEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESET_VAL_ENABLE` reader - N/A"]
pub type PresetValEnableR = crate::BitReader;
#[doc = "Field `PRESET_VAL_ENABLE` writer - N/A"]
pub type PresetValEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn uhs_mode_sel(&self) -> UhsModeSelR {
        UhsModeSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 1.8V Signaling Enable This bit controls voltage regulator for I/O cell in SD UHS-I mode. Setting this bit from 0 to 1 starts changing the signal voltage from 3.3V to 1.8V. Host Controller clears this bit if switching to 1.8V signaling fails per protocol. The value is reflected on the io_volt_sel output which can then be used to change an external regulator to supply 1.8V instead of 3.3V on the VDDIO pin associated with the CLK/CMD/DAT signals. Note: This bit must be set for all UHS-I speed modes (SDR12/SDR25/SDR50/DDR50). Values: - 0x0 (V_3_3): 3.3V Signalling - 0x1 (V_1_8): 1.8V Signalling"]
    #[inline(always)]
    pub fn signaling_en(&self) -> SignalingEnR {
        SignalingEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Driver Strength Select These bits are used to select the Host Controller output driver in 1.8V signaling UHS-I/eMMC speed modes. The value is reflected on the io_drive_strength\\[1:0\\]
output. - 0x0 (TYPEB): Driver TYPEB is selected - 0x1 (TYPEA): Driver TYPEA is selected - 0x2 (TYPEC): Driver TYPEC is selected - 0x3 (TYPED): Driver TYPED is selected"]
    #[inline(always)]
    pub fn drv_strength_sel(&self) -> DrvStrengthSelR {
        DrvStrengthSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn exec_tuning(&self) -> ExecTuningR {
        ExecTuningR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn sample_clk_sel(&self) -> SampleClkSelR {
        SampleClkSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn uhs2_if_enable(&self) -> Uhs2IfEnableR {
        Uhs2IfEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - ADMA2 Length Mode This bit selects ADMA2 Length mode to be either 16-bit or 26-bit. Values: - 0x0 (FALSE): 16-bit Data Length Mode - 0x1 (TRUE): 26-bit Data Length Mode"]
    #[inline(always)]
    pub fn adma2_len_mode(&self) -> Adma2LenModeR {
        Adma2LenModeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CMD23 Enable If the card supports CMD23, this bit is set to 1. This bit is used to select Auto CMD23 or Auto CMD12 for ADMA3 data transfer. Values: - 0x0 (FALSE): Auto CMD23 is disabled - 0x1 (TRUE): Auto CMD23 is enabled"]
    #[inline(always)]
    pub fn cmd23_enable(&self) -> Cmd23EnableR {
        Cmd23EnableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Host Version 4 Enable This bit selects either Version 3.00 compatible mode or Version 4 mode. Functions of following fields are modified for Host Version 4 mode: - SDMA Address: SDMA uses ADMA System Address (05Fh-058h) instead of SDMA System Address register (003h-000h) - ADMA2/ADMA3 selection: ADMA3 is selected by DMA select in Host Control 1 register - 32-bit Block Count: SDMA System Address register (003h-000h) is modified to 32-bit Block Count register Note: It is recommended not to program ADMA3 Integrated Descriptor Address registers and Command Queuing registers (if applicable) while operating in Host version less than 4 mode (Host Version 4 Enable = 0). Values: - 0x0 (FALSE): Version 3.00 compatible mode - 0x1 (TRUE): Version 4 mode"]
    #[inline(always)]
    pub fn host_ver4_enable(&self) -> HostVer4EnableR {
        HostVer4EnableR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn addressing(&self) -> AddressingR {
        AddressingR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Asynchronous Interrupt Enable This bit can be set if a card supports asynchronous interrupts and Asynchronous Interrupt Support is set to 1 in the Capabilities register. Values: - 0x0 (FALSE): Disabled - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn async_int_enable(&self) -> AsyncIntEnableR {
        AsyncIntEnableR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn preset_val_enable(&self) -> PresetValEnableR {
        PresetValEnableR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn uhs_mode_sel(&mut self) -> UhsModeSelW<HostCtrl2RSpec> {
        UhsModeSelW::new(self, 0)
    }
    #[doc = "Bit 3 - 1.8V Signaling Enable This bit controls voltage regulator for I/O cell in SD UHS-I mode. Setting this bit from 0 to 1 starts changing the signal voltage from 3.3V to 1.8V. Host Controller clears this bit if switching to 1.8V signaling fails per protocol. The value is reflected on the io_volt_sel output which can then be used to change an external regulator to supply 1.8V instead of 3.3V on the VDDIO pin associated with the CLK/CMD/DAT signals. Note: This bit must be set for all UHS-I speed modes (SDR12/SDR25/SDR50/DDR50). Values: - 0x0 (V_3_3): 3.3V Signalling - 0x1 (V_1_8): 1.8V Signalling"]
    #[inline(always)]
    #[must_use]
    pub fn signaling_en(&mut self) -> SignalingEnW<HostCtrl2RSpec> {
        SignalingEnW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Driver Strength Select These bits are used to select the Host Controller output driver in 1.8V signaling UHS-I/eMMC speed modes. The value is reflected on the io_drive_strength\\[1:0\\]
output. - 0x0 (TYPEB): Driver TYPEB is selected - 0x1 (TYPEA): Driver TYPEA is selected - 0x2 (TYPEC): Driver TYPEC is selected - 0x3 (TYPED): Driver TYPED is selected"]
    #[inline(always)]
    #[must_use]
    pub fn drv_strength_sel(&mut self) -> DrvStrengthSelW<HostCtrl2RSpec> {
        DrvStrengthSelW::new(self, 4)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn exec_tuning(&mut self) -> ExecTuningW<HostCtrl2RSpec> {
        ExecTuningW::new(self, 6)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sample_clk_sel(&mut self) -> SampleClkSelW<HostCtrl2RSpec> {
        SampleClkSelW::new(self, 7)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn uhs2_if_enable(&mut self) -> Uhs2IfEnableW<HostCtrl2RSpec> {
        Uhs2IfEnableW::new(self, 8)
    }
    #[doc = "Bit 10 - ADMA2 Length Mode This bit selects ADMA2 Length mode to be either 16-bit or 26-bit. Values: - 0x0 (FALSE): 16-bit Data Length Mode - 0x1 (TRUE): 26-bit Data Length Mode"]
    #[inline(always)]
    #[must_use]
    pub fn adma2_len_mode(&mut self) -> Adma2LenModeW<HostCtrl2RSpec> {
        Adma2LenModeW::new(self, 10)
    }
    #[doc = "Bit 11 - CMD23 Enable If the card supports CMD23, this bit is set to 1. This bit is used to select Auto CMD23 or Auto CMD12 for ADMA3 data transfer. Values: - 0x0 (FALSE): Auto CMD23 is disabled - 0x1 (TRUE): Auto CMD23 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd23_enable(&mut self) -> Cmd23EnableW<HostCtrl2RSpec> {
        Cmd23EnableW::new(self, 11)
    }
    #[doc = "Bit 12 - Host Version 4 Enable This bit selects either Version 3.00 compatible mode or Version 4 mode. Functions of following fields are modified for Host Version 4 mode: - SDMA Address: SDMA uses ADMA System Address (05Fh-058h) instead of SDMA System Address register (003h-000h) - ADMA2/ADMA3 selection: ADMA3 is selected by DMA select in Host Control 1 register - 32-bit Block Count: SDMA System Address register (003h-000h) is modified to 32-bit Block Count register Note: It is recommended not to program ADMA3 Integrated Descriptor Address registers and Command Queuing registers (if applicable) while operating in Host version less than 4 mode (Host Version 4 Enable = 0). Values: - 0x0 (FALSE): Version 3.00 compatible mode - 0x1 (TRUE): Version 4 mode"]
    #[inline(always)]
    #[must_use]
    pub fn host_ver4_enable(&mut self) -> HostVer4EnableW<HostCtrl2RSpec> {
        HostVer4EnableW::new(self, 12)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn addressing(&mut self) -> AddressingW<HostCtrl2RSpec> {
        AddressingW::new(self, 13)
    }
    #[doc = "Bit 14 - Asynchronous Interrupt Enable This bit can be set if a card supports asynchronous interrupts and Asynchronous Interrupt Support is set to 1 in the Capabilities register. Values: - 0x0 (FALSE): Disabled - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn async_int_enable(&mut self) -> AsyncIntEnableW<HostCtrl2RSpec> {
        AsyncIntEnableW::new(self, 14)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn preset_val_enable(&mut self) -> PresetValEnableW<HostCtrl2RSpec> {
        PresetValEnableW::new(self, 15)
    }
}
#[doc = "Host Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctrl2_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctrl2_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostCtrl2RSpec;
impl crate::RegisterSpec for HostCtrl2RSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`host_ctrl2_r::R`](R) reader structure"]
impl crate::Readable for HostCtrl2RSpec {}
#[doc = "`write(|w| ..)` method takes [`host_ctrl2_r::W`](W) writer structure"]
impl crate::Writable for HostCtrl2RSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets HOST_CTRL2_R to value 0"]
impl crate::Resettable for HostCtrl2RSpec {
    const RESET_VALUE: u16 = 0;
}
