#[doc = "Register `CFG_OUT` reader"]
pub type R = crate::R<CfgOutSpec>;
#[doc = "Register `CFG_OUT` writer"]
pub type W = crate::W<CfgOutSpec>;
#[doc = "Field `SLOW0` reader - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
pub type Slow0R = crate::BitReader;
#[doc = "Field `SLOW0` writer - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
pub type Slow0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW1` reader - Enables slow slew rate for IO pin 1"]
pub type Slow1R = crate::BitReader;
#[doc = "Field `SLOW1` writer - Enables slow slew rate for IO pin 1"]
pub type Slow1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW2` reader - Enables slow slew rate for IO pin 2"]
pub type Slow2R = crate::BitReader;
#[doc = "Field `SLOW2` writer - Enables slow slew rate for IO pin 2"]
pub type Slow2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW3` reader - Enables slow slew rate for IO pin 3"]
pub type Slow3R = crate::BitReader;
#[doc = "Field `SLOW3` writer - Enables slow slew rate for IO pin 3"]
pub type Slow3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW4` reader - Enables slow slew rate for IO pin 4"]
pub type Slow4R = crate::BitReader;
#[doc = "Field `SLOW4` writer - Enables slow slew rate for IO pin 4"]
pub type Slow4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW5` reader - Enables slow slew rate for IO pin 5"]
pub type Slow5R = crate::BitReader;
#[doc = "Field `SLOW5` writer - Enables slow slew rate for IO pin 5"]
pub type Slow5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW6` reader - Enables slow slew rate for IO pin 6"]
pub type Slow6R = crate::BitReader;
#[doc = "Field `SLOW6` writer - Enables slow slew rate for IO pin 6"]
pub type Slow6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW7` reader - Enables slow slew rate for IO pin 7"]
pub type Slow7R = crate::BitReader;
#[doc = "Field `SLOW7` writer - Enables slow slew rate for IO pin 7"]
pub type Slow7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sets the GPIO drive strength for IO pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DriveSel0 {
    #[doc = "0: Please refer to architecture TRM section I/O System"]
    DriveSelZero = 0,
    #[doc = "1: Please refer to architecture TRM section I/O System"]
    DriveSelOne = 1,
    #[doc = "2: Please refer to architecture TRM section I/O System"]
    DriveSelTwo = 2,
    #[doc = "3: Please refer to architecture TRM section I/O System"]
    DriveSelThree = 3,
}
impl From<DriveSel0> for u8 {
    #[inline(always)]
    fn from(variant: DriveSel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DriveSel0 {
    type Ux = u8;
}
impl crate::IsEnum for DriveSel0 {}
#[doc = "Field `DRIVE_SEL0` reader - Sets the GPIO drive strength for IO pin 0"]
pub type DriveSel0R = crate::FieldReader<DriveSel0>;
impl DriveSel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DriveSel0 {
        match self.bits {
            0 => DriveSel0::DriveSelZero,
            1 => DriveSel0::DriveSelOne,
            2 => DriveSel0::DriveSelTwo,
            3 => DriveSel0::DriveSelThree,
            _ => unreachable!(),
        }
    }
    #[doc = "Please refer to architecture TRM section I/O System"]
    #[inline(always)]
    pub fn is_drive_sel_zero(&self) -> bool {
        *self == DriveSel0::DriveSelZero
    }
    #[doc = "Please refer to architecture TRM section I/O System"]
    #[inline(always)]
    pub fn is_drive_sel_one(&self) -> bool {
        *self == DriveSel0::DriveSelOne
    }
    #[doc = "Please refer to architecture TRM section I/O System"]
    #[inline(always)]
    pub fn is_drive_sel_two(&self) -> bool {
        *self == DriveSel0::DriveSelTwo
    }
    #[doc = "Please refer to architecture TRM section I/O System"]
    #[inline(always)]
    pub fn is_drive_sel_three(&self) -> bool {
        *self == DriveSel0::DriveSelThree
    }
}
#[doc = "Field `DRIVE_SEL0` writer - Sets the GPIO drive strength for IO pin 0"]
pub type DriveSel0W<'a, REG> = crate::FieldWriter<'a, REG, 2, DriveSel0, crate::Safe>;
impl<'a, REG> DriveSel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Please refer to architecture TRM section I/O System"]
    #[inline(always)]
    pub fn drive_sel_zero(self) -> &'a mut crate::W<REG> {
        self.variant(DriveSel0::DriveSelZero)
    }
    #[doc = "Please refer to architecture TRM section I/O System"]
    #[inline(always)]
    pub fn drive_sel_one(self) -> &'a mut crate::W<REG> {
        self.variant(DriveSel0::DriveSelOne)
    }
    #[doc = "Please refer to architecture TRM section I/O System"]
    #[inline(always)]
    pub fn drive_sel_two(self) -> &'a mut crate::W<REG> {
        self.variant(DriveSel0::DriveSelTwo)
    }
    #[doc = "Please refer to architecture TRM section I/O System"]
    #[inline(always)]
    pub fn drive_sel_three(self) -> &'a mut crate::W<REG> {
        self.variant(DriveSel0::DriveSelThree)
    }
}
#[doc = "Field `DRIVE_SEL1` reader - Sets the GPIO drive strength for IO pin 1"]
pub type DriveSel1R = crate::FieldReader;
#[doc = "Field `DRIVE_SEL1` writer - Sets the GPIO drive strength for IO pin 1"]
pub type DriveSel1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRIVE_SEL2` reader - Sets the GPIO drive strength for IO pin 2"]
pub type DriveSel2R = crate::FieldReader;
#[doc = "Field `DRIVE_SEL2` writer - Sets the GPIO drive strength for IO pin 2"]
pub type DriveSel2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRIVE_SEL3` reader - Sets the GPIO drive strength for IO pin 3"]
pub type DriveSel3R = crate::FieldReader;
#[doc = "Field `DRIVE_SEL3` writer - Sets the GPIO drive strength for IO pin 3"]
pub type DriveSel3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRIVE_SEL4` reader - Sets the GPIO drive strength for IO pin 4"]
pub type DriveSel4R = crate::FieldReader;
#[doc = "Field `DRIVE_SEL4` writer - Sets the GPIO drive strength for IO pin 4"]
pub type DriveSel4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRIVE_SEL5` reader - Sets the GPIO drive strength for IO pin 5"]
pub type DriveSel5R = crate::FieldReader;
#[doc = "Field `DRIVE_SEL5` writer - Sets the GPIO drive strength for IO pin 5"]
pub type DriveSel5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRIVE_SEL6` reader - Sets the GPIO drive strength for IO pin 6"]
pub type DriveSel6R = crate::FieldReader;
#[doc = "Field `DRIVE_SEL6` writer - Sets the GPIO drive strength for IO pin 6"]
pub type DriveSel6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRIVE_SEL7` reader - Sets the GPIO drive strength for IO pin 7"]
pub type DriveSel7R = crate::FieldReader;
#[doc = "Field `DRIVE_SEL7` writer - Sets the GPIO drive strength for IO pin 7"]
pub type DriveSel7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
    #[inline(always)]
    pub fn slow0(&self) -> Slow0R {
        Slow0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables slow slew rate for IO pin 1"]
    #[inline(always)]
    pub fn slow1(&self) -> Slow1R {
        Slow1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables slow slew rate for IO pin 2"]
    #[inline(always)]
    pub fn slow2(&self) -> Slow2R {
        Slow2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables slow slew rate for IO pin 3"]
    #[inline(always)]
    pub fn slow3(&self) -> Slow3R {
        Slow3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables slow slew rate for IO pin 4"]
    #[inline(always)]
    pub fn slow4(&self) -> Slow4R {
        Slow4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables slow slew rate for IO pin 5"]
    #[inline(always)]
    pub fn slow5(&self) -> Slow5R {
        Slow5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables slow slew rate for IO pin 6"]
    #[inline(always)]
    pub fn slow6(&self) -> Slow6R {
        Slow6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables slow slew rate for IO pin 7"]
    #[inline(always)]
    pub fn slow7(&self) -> Slow7R {
        Slow7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Sets the GPIO drive strength for IO pin 0"]
    #[inline(always)]
    pub fn drive_sel0(&self) -> DriveSel0R {
        DriveSel0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Sets the GPIO drive strength for IO pin 1"]
    #[inline(always)]
    pub fn drive_sel1(&self) -> DriveSel1R {
        DriveSel1R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Sets the GPIO drive strength for IO pin 2"]
    #[inline(always)]
    pub fn drive_sel2(&self) -> DriveSel2R {
        DriveSel2R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Sets the GPIO drive strength for IO pin 3"]
    #[inline(always)]
    pub fn drive_sel3(&self) -> DriveSel3R {
        DriveSel3R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Sets the GPIO drive strength for IO pin 4"]
    #[inline(always)]
    pub fn drive_sel4(&self) -> DriveSel4R {
        DriveSel4R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Sets the GPIO drive strength for IO pin 5"]
    #[inline(always)]
    pub fn drive_sel5(&self) -> DriveSel5R {
        DriveSel5R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Sets the GPIO drive strength for IO pin 6"]
    #[inline(always)]
    pub fn drive_sel6(&self) -> DriveSel6R {
        DriveSel6R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Sets the GPIO drive strength for IO pin 7"]
    #[inline(always)]
    pub fn drive_sel7(&self) -> DriveSel7R {
        DriveSel7R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn slow0(&mut self) -> Slow0W<CfgOutSpec> {
        Slow0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enables slow slew rate for IO pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn slow1(&mut self) -> Slow1W<CfgOutSpec> {
        Slow1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables slow slew rate for IO pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn slow2(&mut self) -> Slow2W<CfgOutSpec> {
        Slow2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enables slow slew rate for IO pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn slow3(&mut self) -> Slow3W<CfgOutSpec> {
        Slow3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enables slow slew rate for IO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn slow4(&mut self) -> Slow4W<CfgOutSpec> {
        Slow4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enables slow slew rate for IO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn slow5(&mut self) -> Slow5W<CfgOutSpec> {
        Slow5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enables slow slew rate for IO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn slow6(&mut self) -> Slow6W<CfgOutSpec> {
        Slow6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enables slow slew rate for IO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn slow7(&mut self) -> Slow7W<CfgOutSpec> {
        Slow7W::new(self, 7)
    }
    #[doc = "Bits 16:17 - Sets the GPIO drive strength for IO pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel0(&mut self) -> DriveSel0W<CfgOutSpec> {
        DriveSel0W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Sets the GPIO drive strength for IO pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel1(&mut self) -> DriveSel1W<CfgOutSpec> {
        DriveSel1W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Sets the GPIO drive strength for IO pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel2(&mut self) -> DriveSel2W<CfgOutSpec> {
        DriveSel2W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Sets the GPIO drive strength for IO pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel3(&mut self) -> DriveSel3W<CfgOutSpec> {
        DriveSel3W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Sets the GPIO drive strength for IO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel4(&mut self) -> DriveSel4W<CfgOutSpec> {
        DriveSel4W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Sets the GPIO drive strength for IO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel5(&mut self) -> DriveSel5W<CfgOutSpec> {
        DriveSel5W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Sets the GPIO drive strength for IO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel6(&mut self) -> DriveSel6W<CfgOutSpec> {
        DriveSel6W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Sets the GPIO drive strength for IO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel7(&mut self) -> DriveSel7W<CfgOutSpec> {
        DriveSel7W::new(self, 30)
    }
}
#[doc = "Port output buffer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgOutSpec;
impl crate::RegisterSpec for CfgOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_out::R`](R) reader structure"]
impl crate::Readable for CfgOutSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_out::W`](W) writer structure"]
impl crate::Writable for CfgOutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_OUT to value 0"]
impl crate::Resettable for CfgOutSpec {
    const RESET_VALUE: u32 = 0;
}
