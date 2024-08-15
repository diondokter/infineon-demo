#[doc = "Register `MUX_SWITCH_STATUS` reader"]
pub type R = crate::R<MuxSwitchStatusSpec>;
#[doc = "Field `MUX_FW_P0_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP0VplusR = crate::BitReader;
#[doc = "Field `MUX_FW_P1_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP1VplusR = crate::BitReader;
#[doc = "Field `MUX_FW_P2_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP2VplusR = crate::BitReader;
#[doc = "Field `MUX_FW_P3_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP3VplusR = crate::BitReader;
#[doc = "Field `MUX_FW_P4_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP4VplusR = crate::BitReader;
#[doc = "Field `MUX_FW_P5_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP5VplusR = crate::BitReader;
#[doc = "Field `MUX_FW_P6_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP6VplusR = crate::BitReader;
#[doc = "Field `MUX_FW_P7_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP7VplusR = crate::BitReader;
#[doc = "Field `MUX_FW_P0_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP0VminusR = crate::BitReader;
#[doc = "Field `MUX_FW_P1_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP1VminusR = crate::BitReader;
#[doc = "Field `MUX_FW_P2_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP2VminusR = crate::BitReader;
#[doc = "Field `MUX_FW_P3_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP3VminusR = crate::BitReader;
#[doc = "Field `MUX_FW_P4_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP4VminusR = crate::BitReader;
#[doc = "Field `MUX_FW_P5_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP5VminusR = crate::BitReader;
#[doc = "Field `MUX_FW_P6_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP6VminusR = crate::BitReader;
#[doc = "Field `MUX_FW_P7_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwP7VminusR = crate::BitReader;
#[doc = "Field `MUX_FW_VSSA_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwVssaVminusR = crate::BitReader;
#[doc = "Field `MUX_FW_TEMP_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwTempVplusR = crate::BitReader;
#[doc = "Field `MUX_FW_AMUXBUSA_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwAmuxbusaVplusR = crate::BitReader;
#[doc = "Field `MUX_FW_AMUXBUSB_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwAmuxbusbVplusR = crate::BitReader;
#[doc = "Field `MUX_FW_AMUXBUSA_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwAmuxbusaVminusR = crate::BitReader;
#[doc = "Field `MUX_FW_AMUXBUSB_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwAmuxbusbVminusR = crate::BitReader;
#[doc = "Field `MUX_FW_SARBUS0_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwSarbus0VplusR = crate::BitReader;
#[doc = "Field `MUX_FW_SARBUS1_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwSarbus1VplusR = crate::BitReader;
#[doc = "Field `MUX_FW_SARBUS0_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwSarbus0VminusR = crate::BitReader;
#[doc = "Field `MUX_FW_SARBUS1_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MuxFwSarbus1VminusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p0_vplus(&self) -> MuxFwP0VplusR {
        MuxFwP0VplusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p1_vplus(&self) -> MuxFwP1VplusR {
        MuxFwP1VplusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p2_vplus(&self) -> MuxFwP2VplusR {
        MuxFwP2VplusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p3_vplus(&self) -> MuxFwP3VplusR {
        MuxFwP3VplusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p4_vplus(&self) -> MuxFwP4VplusR {
        MuxFwP4VplusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p5_vplus(&self) -> MuxFwP5VplusR {
        MuxFwP5VplusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p6_vplus(&self) -> MuxFwP6VplusR {
        MuxFwP6VplusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p7_vplus(&self) -> MuxFwP7VplusR {
        MuxFwP7VplusR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p0_vminus(&self) -> MuxFwP0VminusR {
        MuxFwP0VminusR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p1_vminus(&self) -> MuxFwP1VminusR {
        MuxFwP1VminusR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p2_vminus(&self) -> MuxFwP2VminusR {
        MuxFwP2VminusR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p3_vminus(&self) -> MuxFwP3VminusR {
        MuxFwP3VminusR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p4_vminus(&self) -> MuxFwP4VminusR {
        MuxFwP4VminusR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p5_vminus(&self) -> MuxFwP5VminusR {
        MuxFwP5VminusR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p6_vminus(&self) -> MuxFwP6VminusR {
        MuxFwP6VminusR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p7_vminus(&self) -> MuxFwP7VminusR {
        MuxFwP7VminusR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_vssa_vminus(&self) -> MuxFwVssaVminusR {
        MuxFwVssaVminusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_temp_vplus(&self) -> MuxFwTempVplusR {
        MuxFwTempVplusR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vplus(&self) -> MuxFwAmuxbusaVplusR {
        MuxFwAmuxbusaVplusR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vplus(&self) -> MuxFwAmuxbusbVplusR {
        MuxFwAmuxbusbVplusR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vminus(&self) -> MuxFwAmuxbusaVminusR {
        MuxFwAmuxbusaVminusR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vminus(&self) -> MuxFwAmuxbusbVminusR {
        MuxFwAmuxbusbVminusR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vplus(&self) -> MuxFwSarbus0VplusR {
        MuxFwSarbus0VplusR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vplus(&self) -> MuxFwSarbus1VplusR {
        MuxFwSarbus1VplusR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vminus(&self) -> MuxFwSarbus0VminusR {
        MuxFwSarbus0VminusR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vminus(&self) -> MuxFwSarbus1VminusR {
        MuxFwSarbus1VminusR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "SARMUX switch status\n\nYou can [`read`](crate::Reg::read) this register and get [`mux_switch_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MuxSwitchStatusSpec;
impl crate::RegisterSpec for MuxSwitchStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mux_switch_status::R`](R) reader structure"]
impl crate::Readable for MuxSwitchStatusSpec {}
#[doc = "`reset()` method sets MUX_SWITCH_STATUS to value 0"]
impl crate::Resettable for MuxSwitchStatusSpec {
    const RESET_VALUE: u32 = 0;
}
