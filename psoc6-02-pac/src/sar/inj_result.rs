#[doc = "Register `INJ_RESULT` reader"]
pub type R = crate::R<InjResultSpec>;
#[doc = "Field `INJ_RESULT` reader - SAR conversion result of the channel."]
pub type InjResultR = crate::FieldReader<u16>;
#[doc = "Field `INJ_NEWVALUE` reader - The data in this register received a new value (only relevant for UAB, this bit shows the value of the UAB valid bit)"]
pub type InjNewvalueR = crate::BitReader;
#[doc = "Field `INJ_COLLISION_INTR_MIR` reader - mirror bit of corresponding bit in SAR_INTR register"]
pub type InjCollisionIntrMirR = crate::BitReader;
#[doc = "Field `INJ_SATURATE_INTR_MIR` reader - mirror bit of corresponding bit in SAR_INTR register"]
pub type InjSaturateIntrMirR = crate::BitReader;
#[doc = "Field `INJ_RANGE_INTR_MIR` reader - mirror bit of corresponding bit in SAR_INTR register"]
pub type InjRangeIntrMirR = crate::BitReader;
#[doc = "Field `INJ_EOC_INTR_MIR` reader - mirror bit of corresponding bit in SAR_INTR register"]
pub type InjEocIntrMirR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - SAR conversion result of the channel."]
    #[inline(always)]
    pub fn inj_result(&self) -> InjResultR {
        InjResultR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 27 - The data in this register received a new value (only relevant for UAB, this bit shows the value of the UAB valid bit)"]
    #[inline(always)]
    pub fn inj_newvalue(&self) -> InjNewvalueR {
        InjNewvalueR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_collision_intr_mir(&self) -> InjCollisionIntrMirR {
        InjCollisionIntrMirR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_saturate_intr_mir(&self) -> InjSaturateIntrMirR {
        InjSaturateIntrMirR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_range_intr_mir(&self) -> InjRangeIntrMirR {
        InjRangeIntrMirR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_eoc_intr_mir(&self) -> InjEocIntrMirR {
        InjEocIntrMirR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Injection channel result register\n\nYou can [`read`](crate::Reg::read) this register and get [`inj_result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InjResultSpec;
impl crate::RegisterSpec for InjResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inj_result::R`](R) reader structure"]
impl crate::Readable for InjResultSpec {}
#[doc = "`reset()` method sets INJ_RESULT to value 0"]
impl crate::Resettable for InjResultSpec {
    const RESET_VALUE: u32 = 0;
}
