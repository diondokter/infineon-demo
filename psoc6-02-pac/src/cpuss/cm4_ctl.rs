#[doc = "Register `CM4_CTL` reader"]
pub type R = crate::R<Cm4CtlSpec>;
#[doc = "Register `CM4_CTL` writer"]
pub type W = crate::W<Cm4CtlSpec>;
#[doc = "Field `IOC_MASK` reader - CPU floating point unit (FPU) exception mask for the CPU's FPCSR.IOC 'invalid operation' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the ARM architecture does NOT support FPU exceptions; i.e. there is no precise FPU exception handler. Instead, FPU conditions are captured in the CPU's FPCSR register and the conditions are provided as CPU interface signals. The interface signals are 'masked' with the fields a provide by this register (CM7_0_CTL). The 'masked' signals are reduced/OR-ed into a single CPU floating point interrupt signal. The associated CPU interrupt handler allows for imprecise handling of FPU exception conditions. Note: the CPU's FPCSR exception conditions are 'sticky'. Typically, the CPU FPU interrupt handler will clear the exception condition(s) to '0'. Note: by default, the FPU exception masks are '0'. Therefore, FPU exception conditions will NOT activate the CPU's floating point interrupt."]
pub type IocMaskR = crate::BitReader;
#[doc = "Field `IOC_MASK` writer - CPU floating point unit (FPU) exception mask for the CPU's FPCSR.IOC 'invalid operation' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the ARM architecture does NOT support FPU exceptions; i.e. there is no precise FPU exception handler. Instead, FPU conditions are captured in the CPU's FPCSR register and the conditions are provided as CPU interface signals. The interface signals are 'masked' with the fields a provide by this register (CM7_0_CTL). The 'masked' signals are reduced/OR-ed into a single CPU floating point interrupt signal. The associated CPU interrupt handler allows for imprecise handling of FPU exception conditions. Note: the CPU's FPCSR exception conditions are 'sticky'. Typically, the CPU FPU interrupt handler will clear the exception condition(s) to '0'. Note: by default, the FPU exception masks are '0'. Therefore, FPU exception conditions will NOT activate the CPU's floating point interrupt."]
pub type IocMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DZC_MASK` reader - CPU FPU exception mask for the CPU's FPCSR.DZC 'divide by zero' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
pub type DzcMaskR = crate::BitReader;
#[doc = "Field `DZC_MASK` writer - CPU FPU exception mask for the CPU's FPCSR.DZC 'divide by zero' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
pub type DzcMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFC_MASK` reader - CPU FPU exception mask for the CPU's FPCSR.OFC 'overflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
pub type OfcMaskR = crate::BitReader;
#[doc = "Field `OFC_MASK` writer - CPU FPU exception mask for the CPU's FPCSR.OFC 'overflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
pub type OfcMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFC_MASK` reader - CPU FPU exception mask for the CPU's FPCSR.UFC 'underflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
pub type UfcMaskR = crate::BitReader;
#[doc = "Field `UFC_MASK` writer - CPU FPU exception mask for the CPU's FPCSR.UFC 'underflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
pub type UfcMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IXC_MASK` reader - CPU FPU exception mask for the CPU's FPCSR.IXC 'inexact' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the 'inexact' condition is set as a result of rounding. Rounding may occur frequently and is typically not an error condition. To prevent frequent CPU FPU interrupts as a result of rounding, this field is typically set to '0'."]
pub type IxcMaskR = crate::BitReader;
#[doc = "Field `IXC_MASK` writer - CPU FPU exception mask for the CPU's FPCSR.IXC 'inexact' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the 'inexact' condition is set as a result of rounding. Rounding may occur frequently and is typically not an error condition. To prevent frequent CPU FPU interrupts as a result of rounding, this field is typically set to '0'."]
pub type IxcMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDC_MASK` reader - CPU FPU exception mask for the CPU's FPCSR.IDC 'input denormalized' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: if the CPU FPCSR.FZ field is set to '1', denormalized inputs are 'flushed to zero'. Dependent on the FPU algorithm, this may or may not occur frequently. To prevent frequent CPU FPU interrupts as a result of denormalized inputs, this field may be set to '0'."]
pub type IdcMaskR = crate::BitReader;
#[doc = "Field `IDC_MASK` writer - CPU FPU exception mask for the CPU's FPCSR.IDC 'input denormalized' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: if the CPU FPCSR.FZ field is set to '1', denormalized inputs are 'flushed to zero'. Dependent on the FPU algorithm, this may or may not occur frequently. To prevent frequent CPU FPU interrupts as a result of denormalized inputs, this field may be set to '0'."]
pub type IdcMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - CPU floating point unit (FPU) exception mask for the CPU's FPCSR.IOC 'invalid operation' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the ARM architecture does NOT support FPU exceptions; i.e. there is no precise FPU exception handler. Instead, FPU conditions are captured in the CPU's FPCSR register and the conditions are provided as CPU interface signals. The interface signals are 'masked' with the fields a provide by this register (CM7_0_CTL). The 'masked' signals are reduced/OR-ed into a single CPU floating point interrupt signal. The associated CPU interrupt handler allows for imprecise handling of FPU exception conditions. Note: the CPU's FPCSR exception conditions are 'sticky'. Typically, the CPU FPU interrupt handler will clear the exception condition(s) to '0'. Note: by default, the FPU exception masks are '0'. Therefore, FPU exception conditions will NOT activate the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn ioc_mask(&self) -> IocMaskR {
        IocMaskR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU FPU exception mask for the CPU's FPCSR.DZC 'divide by zero' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn dzc_mask(&self) -> DzcMaskR {
        DzcMaskR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CPU FPU exception mask for the CPU's FPCSR.OFC 'overflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn ofc_mask(&self) -> OfcMaskR {
        OfcMaskR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CPU FPU exception mask for the CPU's FPCSR.UFC 'underflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn ufc_mask(&self) -> UfcMaskR {
        UfcMaskR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CPU FPU exception mask for the CPU's FPCSR.IXC 'inexact' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the 'inexact' condition is set as a result of rounding. Rounding may occur frequently and is typically not an error condition. To prevent frequent CPU FPU interrupts as a result of rounding, this field is typically set to '0'."]
    #[inline(always)]
    pub fn ixc_mask(&self) -> IxcMaskR {
        IxcMaskR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU FPU exception mask for the CPU's FPCSR.IDC 'input denormalized' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: if the CPU FPCSR.FZ field is set to '1', denormalized inputs are 'flushed to zero'. Dependent on the FPU algorithm, this may or may not occur frequently. To prevent frequent CPU FPU interrupts as a result of denormalized inputs, this field may be set to '0'."]
    #[inline(always)]
    pub fn idc_mask(&self) -> IdcMaskR {
        IdcMaskR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - CPU floating point unit (FPU) exception mask for the CPU's FPCSR.IOC 'invalid operation' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the ARM architecture does NOT support FPU exceptions; i.e. there is no precise FPU exception handler. Instead, FPU conditions are captured in the CPU's FPCSR register and the conditions are provided as CPU interface signals. The interface signals are 'masked' with the fields a provide by this register (CM7_0_CTL). The 'masked' signals are reduced/OR-ed into a single CPU floating point interrupt signal. The associated CPU interrupt handler allows for imprecise handling of FPU exception conditions. Note: the CPU's FPCSR exception conditions are 'sticky'. Typically, the CPU FPU interrupt handler will clear the exception condition(s) to '0'. Note: by default, the FPU exception masks are '0'. Therefore, FPU exception conditions will NOT activate the CPU's floating point interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ioc_mask(&mut self) -> IocMaskW<Cm4CtlSpec> {
        IocMaskW::new(self, 24)
    }
    #[doc = "Bit 25 - CPU FPU exception mask for the CPU's FPCSR.DZC 'divide by zero' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dzc_mask(&mut self) -> DzcMaskW<Cm4CtlSpec> {
        DzcMaskW::new(self, 25)
    }
    #[doc = "Bit 26 - CPU FPU exception mask for the CPU's FPCSR.OFC 'overflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ofc_mask(&mut self) -> OfcMaskW<Cm4CtlSpec> {
        OfcMaskW::new(self, 26)
    }
    #[doc = "Bit 27 - CPU FPU exception mask for the CPU's FPCSR.UFC 'underflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ufc_mask(&mut self) -> UfcMaskW<Cm4CtlSpec> {
        UfcMaskW::new(self, 27)
    }
    #[doc = "Bit 28 - CPU FPU exception mask for the CPU's FPCSR.IXC 'inexact' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the 'inexact' condition is set as a result of rounding. Rounding may occur frequently and is typically not an error condition. To prevent frequent CPU FPU interrupts as a result of rounding, this field is typically set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ixc_mask(&mut self) -> IxcMaskW<Cm4CtlSpec> {
        IxcMaskW::new(self, 28)
    }
    #[doc = "Bit 31 - CPU FPU exception mask for the CPU's FPCSR.IDC 'input denormalized' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: if the CPU FPCSR.FZ field is set to '1', denormalized inputs are 'flushed to zero'. Dependent on the FPU algorithm, this may or may not occur frequently. To prevent frequent CPU FPU interrupts as a result of denormalized inputs, this field may be set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn idc_mask(&mut self) -> IdcMaskW<Cm4CtlSpec> {
        IdcMaskW::new(self, 31)
    }
}
#[doc = "CM4 control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm4CtlSpec;
impl crate::RegisterSpec for Cm4CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_ctl::R`](R) reader structure"]
impl crate::Readable for Cm4CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`cm4_ctl::W`](W) writer structure"]
impl crate::Writable for Cm4CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM4_CTL to value 0"]
impl crate::Resettable for Cm4CtlSpec {
    const RESET_VALUE: u32 = 0;
}
