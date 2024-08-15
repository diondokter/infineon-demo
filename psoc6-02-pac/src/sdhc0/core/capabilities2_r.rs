#[doc = "Register `CAPABILITIES2_R` reader"]
pub type R = crate::R<Capabilities2RSpec>;
#[doc = "Field `SDR50_SUPPORT` reader - SDR50 Support (UHS-I only) Thsi bit indicates that SDR50 is supported. The bit 13 (USE_TUNING_SDR50) indicates whether SDR50 requires tuning or not. Values: - 0x0 (FALSE): SDR50 is not supported - 0x1 (TRUE): SDR50 is supported"]
pub type Sdr50SupportR = crate::BitReader;
#[doc = "Field `SDR104_SUPPORT` reader - SDR104 Support (UHS-I only) This bit mentions that SDR104 requires tuning. Values: - 0x0 (FALSE): SDR104 is not supported - 0x1 (TRUE): SDR104 is supported (NOT ACTUALLY SUPPORTED)"]
pub type Sdr104SupportR = crate::BitReader;
#[doc = "Field `DDR50_SUPPORT` reader - DDR50 Support (UHS-I only) Values: - 0x0 (FALSE): DDR50 is not supported - 0x1 (TRUE): DDR50 is supported"]
pub type Ddr50SupportR = crate::BitReader;
#[doc = "Field `UHS2_SUPPORT` reader - UHS-II Support (UHS-II only) This bit indicates whether Host Controller supports UHS-II. Values: - 0x0 (FALSE): UHS-II is not supported - 0x1 (TRUE): UHS-II is supported"]
pub type Uhs2SupportR = crate::BitReader;
#[doc = "Field `DRV_TYPEA` reader - Driver Type A Support (UHS-I only) This bit indicates support of Driver Type A for 1.8 Signaling. Values: - 0x0 (FALSE): Driver Type A is not supported - 0x1 (TRUE): Driver Type A is supported"]
pub type DrvTypeaR = crate::BitReader;
#[doc = "Field `DRV_TYPEC` reader - Driver Type C Support (UHS-I only) This bit indicates support of Driver Type C for 1.8 Signaling. Values: - 0x0 (FALSE): Driver Type C is not supported - 0x1 (TRUE): Driver Type C is supported"]
pub type DrvTypecR = crate::BitReader;
#[doc = "Field `DRV_TYPED` reader - Driver Type D Support (UHS-I only) This bit indicates support of Driver Type D for 1.8 Signaling. Values: - 0x0 (FALSE): Driver Type D is not supported - 0x1 (TRUE): Driver Type D is supported"]
pub type DrvTypedR = crate::BitReader;
#[doc = "Field `RETUNE_CNT` reader - N/A"]
pub type RetuneCntR = crate::FieldReader;
#[doc = "Field `USE_TUNING_SDR50` reader - Use Tuning for SDR50 (UHS-I only) Values: - 0x0 (ZERO): SDR50 does not require tuning - 0x1 (ONE): SDR50 requires tuning"]
pub type UseTuningSdr50R = crate::BitReader;
#[doc = "Field `RE_TUNING_MODES` reader - N/A"]
pub type ReTuningModesR = crate::FieldReader;
#[doc = "Field `CLK_MUL` reader - Clock Multiplier These bits indicate the clock multiplier of the programmable clock generator. Setting these bits to 0 means that the Host Controller does not support a programmable clock generator. - 0x0: Clock Multiplier is not Supported - 0x1: Clock Multiplier M = 2 - 0x2: Clock Multiplier M = 3 - ......... - 0xFF: Clock Multiplier M = 256"]
pub type ClkMulR = crate::FieldReader;
#[doc = "Field `ADMA3_SUPPORT` reader - ADMA3 Support This bit indicates whether the Host Controller is capable of using ADMA3. Values: - 0x0 (FALSE): ADMA3 not Supported - 0x1 (TRUE): ADMA3 Supported"]
pub type Adma3SupportR = crate::BitReader;
#[doc = "Field `VDD2_18V_SUPPORT` reader - 1.8V VDD2 Support This bit indicates support of VDD2 for the Host System. Values: - 0x0 (FALSE): 1.8V VDD2 is not Supported - 0x1 (TRUE): 1.8V VDD2 is Supported"]
pub type Vdd2_18vSupportR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SDR50 Support (UHS-I only) Thsi bit indicates that SDR50 is supported. The bit 13 (USE_TUNING_SDR50) indicates whether SDR50 requires tuning or not. Values: - 0x0 (FALSE): SDR50 is not supported - 0x1 (TRUE): SDR50 is supported"]
    #[inline(always)]
    pub fn sdr50_support(&self) -> Sdr50SupportR {
        Sdr50SupportR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDR104 Support (UHS-I only) This bit mentions that SDR104 requires tuning. Values: - 0x0 (FALSE): SDR104 is not supported - 0x1 (TRUE): SDR104 is supported (NOT ACTUALLY SUPPORTED)"]
    #[inline(always)]
    pub fn sdr104_support(&self) -> Sdr104SupportR {
        Sdr104SupportR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDR50 Support (UHS-I only) Values: - 0x0 (FALSE): DDR50 is not supported - 0x1 (TRUE): DDR50 is supported"]
    #[inline(always)]
    pub fn ddr50_support(&self) -> Ddr50SupportR {
        Ddr50SupportR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UHS-II Support (UHS-II only) This bit indicates whether Host Controller supports UHS-II. Values: - 0x0 (FALSE): UHS-II is not supported - 0x1 (TRUE): UHS-II is supported"]
    #[inline(always)]
    pub fn uhs2_support(&self) -> Uhs2SupportR {
        Uhs2SupportR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Driver Type A Support (UHS-I only) This bit indicates support of Driver Type A for 1.8 Signaling. Values: - 0x0 (FALSE): Driver Type A is not supported - 0x1 (TRUE): Driver Type A is supported"]
    #[inline(always)]
    pub fn drv_typea(&self) -> DrvTypeaR {
        DrvTypeaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Driver Type C Support (UHS-I only) This bit indicates support of Driver Type C for 1.8 Signaling. Values: - 0x0 (FALSE): Driver Type C is not supported - 0x1 (TRUE): Driver Type C is supported"]
    #[inline(always)]
    pub fn drv_typec(&self) -> DrvTypecR {
        DrvTypecR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Driver Type D Support (UHS-I only) This bit indicates support of Driver Type D for 1.8 Signaling. Values: - 0x0 (FALSE): Driver Type D is not supported - 0x1 (TRUE): Driver Type D is supported"]
    #[inline(always)]
    pub fn drv_typed(&self) -> DrvTypedR {
        DrvTypedR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - N/A"]
    #[inline(always)]
    pub fn retune_cnt(&self) -> RetuneCntR {
        RetuneCntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50 (UHS-I only) Values: - 0x0 (ZERO): SDR50 does not require tuning - 0x1 (ONE): SDR50 requires tuning"]
    #[inline(always)]
    pub fn use_tuning_sdr50(&self) -> UseTuningSdr50R {
        UseTuningSdr50R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - N/A"]
    #[inline(always)]
    pub fn re_tuning_modes(&self) -> ReTuningModesR {
        ReTuningModesR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Clock Multiplier These bits indicate the clock multiplier of the programmable clock generator. Setting these bits to 0 means that the Host Controller does not support a programmable clock generator. - 0x0: Clock Multiplier is not Supported - 0x1: Clock Multiplier M = 2 - 0x2: Clock Multiplier M = 3 - ......... - 0xFF: Clock Multiplier M = 256"]
    #[inline(always)]
    pub fn clk_mul(&self) -> ClkMulR {
        ClkMulR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 27 - ADMA3 Support This bit indicates whether the Host Controller is capable of using ADMA3. Values: - 0x0 (FALSE): ADMA3 not Supported - 0x1 (TRUE): ADMA3 Supported"]
    #[inline(always)]
    pub fn adma3_support(&self) -> Adma3SupportR {
        Adma3SupportR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1.8V VDD2 Support This bit indicates support of VDD2 for the Host System. Values: - 0x0 (FALSE): 1.8V VDD2 is not Supported - 0x1 (TRUE): 1.8V VDD2 is Supported"]
    #[inline(always)]
    pub fn vdd2_18v_support(&self) -> Vdd2_18vSupportR {
        Vdd2_18vSupportR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Capabilities Register - 32 to 63\n\nYou can [`read`](crate::Reg::read) this register and get [`capabilities2_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Capabilities2RSpec;
impl crate::RegisterSpec for Capabilities2RSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capabilities2_r::R`](R) reader structure"]
impl crate::Readable for Capabilities2RSpec {}
#[doc = "`reset()` method sets CAPABILITIES2_R to value 0x0800_0077"]
impl crate::Resettable for Capabilities2RSpec {
    const RESET_VALUE: u32 = 0x0800_0077;
}
