#[doc = "Register `CLK_TRIM_PILO_CTL3` reader"]
pub type R = crate::R<ClkTrimPiloCtl3Spec>;
#[doc = "Register `CLK_TRIM_PILO_CTL3` writer"]
pub type W = crate::W<ClkTrimPiloCtl3Spec>;
#[doc = "Field `PILO_ENGOPT` reader - Engineering options for PILO circuits 0: Short vdda to vpwr 1: Beta:mult current change 2: Iref generation Ptat current addition 3: Disable current path in secondary Beta:mult startup circuit 4: Double oscillator current 5: Switch between deep:sub:threshold and sub:threshold stacks in Vref generation block 6: Spare 7: Ptat component increase in Iref 8: vpwr_rc and vpwr_dig_rc shorting testmode 9: Switch b/w psub connection for cascode nfet for vref generation 10: Switch between sub:threshold and deep:sub:threshold stacks in comparator. 15-11: Frequency fine trim. See AKK-444 for an overview of the trim strategy."]
pub type PiloEngoptR = crate::FieldReader<u16>;
#[doc = "Field `PILO_ENGOPT` writer - Engineering options for PILO circuits 0: Short vdda to vpwr 1: Beta:mult current change 2: Iref generation Ptat current addition 3: Disable current path in secondary Beta:mult startup circuit 4: Double oscillator current 5: Switch between deep:sub:threshold and sub:threshold stacks in Vref generation block 6: Spare 7: Ptat component increase in Iref 8: vpwr_rc and vpwr_dig_rc shorting testmode 9: Switch b/w psub connection for cascode nfet for vref generation 10: Switch between sub:threshold and deep:sub:threshold stacks in comparator. 15-11: Frequency fine trim. See AKK-444 for an overview of the trim strategy."]
pub type PiloEngoptW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Engineering options for PILO circuits 0: Short vdda to vpwr 1: Beta:mult current change 2: Iref generation Ptat current addition 3: Disable current path in secondary Beta:mult startup circuit 4: Double oscillator current 5: Switch between deep:sub:threshold and sub:threshold stacks in Vref generation block 6: Spare 7: Ptat component increase in Iref 8: vpwr_rc and vpwr_dig_rc shorting testmode 9: Switch b/w psub connection for cascode nfet for vref generation 10: Switch between sub:threshold and deep:sub:threshold stacks in comparator. 15-11: Frequency fine trim. See AKK-444 for an overview of the trim strategy."]
    #[inline(always)]
    pub fn pilo_engopt(&self) -> PiloEngoptR {
        PiloEngoptR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Engineering options for PILO circuits 0: Short vdda to vpwr 1: Beta:mult current change 2: Iref generation Ptat current addition 3: Disable current path in secondary Beta:mult startup circuit 4: Double oscillator current 5: Switch between deep:sub:threshold and sub:threshold stacks in Vref generation block 6: Spare 7: Ptat component increase in Iref 8: vpwr_rc and vpwr_dig_rc shorting testmode 9: Switch b/w psub connection for cascode nfet for vref generation 10: Switch between sub:threshold and deep:sub:threshold stacks in comparator. 15-11: Frequency fine trim. See AKK-444 for an overview of the trim strategy."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_engopt(&mut self) -> PiloEngoptW<ClkTrimPiloCtl3Spec> {
        PiloEngoptW::new(self, 0)
    }
}
#[doc = "PILO Trim Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_trim_pilo_ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_trim_pilo_ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTrimPiloCtl3Spec;
impl crate::RegisterSpec for ClkTrimPiloCtl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_trim_pilo_ctl3::R`](R) reader structure"]
impl crate::Readable for ClkTrimPiloCtl3Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_trim_pilo_ctl3::W`](W) writer structure"]
impl crate::Writable for ClkTrimPiloCtl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_TRIM_PILO_CTL3 to value 0x4800"]
impl crate::Resettable for ClkTrimPiloCtl3Spec {
    const RESET_VALUE: u32 = 0x4800;
}
