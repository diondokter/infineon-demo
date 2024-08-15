#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO &amp; UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DriveMode0 {
    #[doc = "0: Output buffer is off creating a high impedance input D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    Highz = 0,
    #[doc = "1: N/A"]
    Rsvd = 1,
    #[doc = "2: Resistive pull up For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Weak/resistive pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull up D_OUT = '1': Weak/resistive pull up"]
    Pullup = 2,
    #[doc = "3: Resistive pull down For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Weak/resistive pull down"]
    Pulldown = 3,
    #[doc = "4: Open drain, drives low For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': High Impedance When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    OdDriveslow = 4,
    #[doc = "5: Open drain, drives high For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': High Impedance D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    OdDriveshigh = 5,
    #[doc = "6: Strong D_OUTput buffer For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    Strong = 6,
    #[doc = "7: Pull up or pull down For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = '0': GPIO_DSI_OUT = '0': Weak/resistive pull down GPIO_DSI_OUT = '1': Weak/resistive pull up where 'GPIO_DSI_OUT' is a function of PORT_SEL, OUT &amp; DSI_DATA_OUT. For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Weak/resistive pull up"]
    PullupDown = 7,
}
impl From<DriveMode0> for u8 {
    #[inline(always)]
    fn from(variant: DriveMode0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DriveMode0 {
    type Ux = u8;
}
impl crate::IsEnum for DriveMode0 {}
#[doc = "Field `DRIVE_MODE0` reader - The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO &amp; UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell."]
pub type DriveMode0R = crate::FieldReader<DriveMode0>;
impl DriveMode0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DriveMode0 {
        match self.bits {
            0 => DriveMode0::Highz,
            1 => DriveMode0::Rsvd,
            2 => DriveMode0::Pullup,
            3 => DriveMode0::Pulldown,
            4 => DriveMode0::OdDriveslow,
            5 => DriveMode0::OdDriveshigh,
            6 => DriveMode0::Strong,
            7 => DriveMode0::PullupDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Output buffer is off creating a high impedance input D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    #[inline(always)]
    pub fn is_highz(&self) -> bool {
        *self == DriveMode0::Highz
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == DriveMode0::Rsvd
    }
    #[doc = "Resistive pull up For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Weak/resistive pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull up D_OUT = '1': Weak/resistive pull up"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == DriveMode0::Pullup
    }
    #[doc = "Resistive pull down For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Weak/resistive pull down"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == DriveMode0::Pulldown
    }
    #[doc = "Open drain, drives low For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': High Impedance When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    #[inline(always)]
    pub fn is_od_driveslow(&self) -> bool {
        *self == DriveMode0::OdDriveslow
    }
    #[doc = "Open drain, drives high For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': High Impedance D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    #[inline(always)]
    pub fn is_od_driveshigh(&self) -> bool {
        *self == DriveMode0::OdDriveshigh
    }
    #[doc = "Strong D_OUTput buffer For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    #[inline(always)]
    pub fn is_strong(&self) -> bool {
        *self == DriveMode0::Strong
    }
    #[doc = "Pull up or pull down For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = '0': GPIO_DSI_OUT = '0': Weak/resistive pull down GPIO_DSI_OUT = '1': Weak/resistive pull up where 'GPIO_DSI_OUT' is a function of PORT_SEL, OUT &amp; DSI_DATA_OUT. For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Weak/resistive pull up"]
    #[inline(always)]
    pub fn is_pullup_down(&self) -> bool {
        *self == DriveMode0::PullupDown
    }
}
#[doc = "Field `DRIVE_MODE0` writer - The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO &amp; UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell."]
pub type DriveMode0W<'a, REG> = crate::FieldWriter<'a, REG, 3, DriveMode0, crate::Safe>;
impl<'a, REG> DriveMode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output buffer is off creating a high impedance input D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    #[inline(always)]
    pub fn highz(self) -> &'a mut crate::W<REG> {
        self.variant(DriveMode0::Highz)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(DriveMode0::Rsvd)
    }
    #[doc = "Resistive pull up For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Weak/resistive pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull up D_OUT = '1': Weak/resistive pull up"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut crate::W<REG> {
        self.variant(DriveMode0::Pullup)
    }
    #[doc = "Resistive pull down For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Weak/resistive pull down"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut crate::W<REG> {
        self.variant(DriveMode0::Pulldown)
    }
    #[doc = "Open drain, drives low For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': High Impedance When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    #[inline(always)]
    pub fn od_driveslow(self) -> &'a mut crate::W<REG> {
        self.variant(DriveMode0::OdDriveslow)
    }
    #[doc = "Open drain, drives high For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': High Impedance D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    #[inline(always)]
    pub fn od_driveshigh(self) -> &'a mut crate::W<REG> {
        self.variant(DriveMode0::OdDriveshigh)
    }
    #[doc = "Strong D_OUTput buffer For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    #[inline(always)]
    pub fn strong(self) -> &'a mut crate::W<REG> {
        self.variant(DriveMode0::Strong)
    }
    #[doc = "Pull up or pull down For GPIO &amp; UDB/DSI peripherals: When D_OUT_EN = '0': GPIO_DSI_OUT = '0': Weak/resistive pull down GPIO_DSI_OUT = '1': Weak/resistive pull up where 'GPIO_DSI_OUT' is a function of PORT_SEL, OUT &amp; DSI_DATA_OUT. For peripherals other than GPIO &amp; UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Weak/resistive pull up"]
    #[inline(always)]
    pub fn pullup_down(self) -> &'a mut crate::W<REG> {
        self.variant(DriveMode0::PullupDown)
    }
}
#[doc = "Field `IN_EN0` reader - Enables the input buffer for IO pin 0. This bit should be cleared when analog signals are present on the pin to avoid crowbar currents. The output buffer can be used to drive analog signals high or low without issue. '0': Input buffer disabled '1': Input buffer enabled"]
pub type InEn0R = crate::BitReader;
#[doc = "Field `IN_EN0` writer - Enables the input buffer for IO pin 0. This bit should be cleared when analog signals are present on the pin to avoid crowbar currents. The output buffer can be used to drive analog signals high or low without issue. '0': Input buffer disabled '1': Input buffer enabled"]
pub type InEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVE_MODE1` reader - The GPIO drive mode for IO pin 1"]
pub type DriveMode1R = crate::FieldReader;
#[doc = "Field `DRIVE_MODE1` writer - The GPIO drive mode for IO pin 1"]
pub type DriveMode1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_EN1` reader - Enables the input buffer for IO pin 1"]
pub type InEn1R = crate::BitReader;
#[doc = "Field `IN_EN1` writer - Enables the input buffer for IO pin 1"]
pub type InEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVE_MODE2` reader - The GPIO drive mode for IO pin 2"]
pub type DriveMode2R = crate::FieldReader;
#[doc = "Field `DRIVE_MODE2` writer - The GPIO drive mode for IO pin 2"]
pub type DriveMode2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_EN2` reader - Enables the input buffer for IO pin 2"]
pub type InEn2R = crate::BitReader;
#[doc = "Field `IN_EN2` writer - Enables the input buffer for IO pin 2"]
pub type InEn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVE_MODE3` reader - The GPIO drive mode for IO pin 3"]
pub type DriveMode3R = crate::FieldReader;
#[doc = "Field `DRIVE_MODE3` writer - The GPIO drive mode for IO pin 3"]
pub type DriveMode3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_EN3` reader - Enables the input buffer for IO pin 3"]
pub type InEn3R = crate::BitReader;
#[doc = "Field `IN_EN3` writer - Enables the input buffer for IO pin 3"]
pub type InEn3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVE_MODE4` reader - The GPIO drive mode for IO pin4"]
pub type DriveMode4R = crate::FieldReader;
#[doc = "Field `DRIVE_MODE4` writer - The GPIO drive mode for IO pin4"]
pub type DriveMode4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_EN4` reader - Enables the input buffer for IO pin 4"]
pub type InEn4R = crate::BitReader;
#[doc = "Field `IN_EN4` writer - Enables the input buffer for IO pin 4"]
pub type InEn4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVE_MODE5` reader - The GPIO drive mode for IO pin 5"]
pub type DriveMode5R = crate::FieldReader;
#[doc = "Field `DRIVE_MODE5` writer - The GPIO drive mode for IO pin 5"]
pub type DriveMode5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_EN5` reader - Enables the input buffer for IO pin 5"]
pub type InEn5R = crate::BitReader;
#[doc = "Field `IN_EN5` writer - Enables the input buffer for IO pin 5"]
pub type InEn5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVE_MODE6` reader - The GPIO drive mode for IO pin 6"]
pub type DriveMode6R = crate::FieldReader;
#[doc = "Field `DRIVE_MODE6` writer - The GPIO drive mode for IO pin 6"]
pub type DriveMode6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_EN6` reader - Enables the input buffer for IO pin 6"]
pub type InEn6R = crate::BitReader;
#[doc = "Field `IN_EN6` writer - Enables the input buffer for IO pin 6"]
pub type InEn6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVE_MODE7` reader - The GPIO drive mode for IO pin 7"]
pub type DriveMode7R = crate::FieldReader;
#[doc = "Field `DRIVE_MODE7` writer - The GPIO drive mode for IO pin 7"]
pub type DriveMode7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_EN7` reader - Enables the input buffer for IO pin 7"]
pub type InEn7R = crate::BitReader;
#[doc = "Field `IN_EN7` writer - Enables the input buffer for IO pin 7"]
pub type InEn7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO &amp; UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell."]
    #[inline(always)]
    pub fn drive_mode0(&self) -> DriveMode0R {
        DriveMode0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enables the input buffer for IO pin 0. This bit should be cleared when analog signals are present on the pin to avoid crowbar currents. The output buffer can be used to drive analog signals high or low without issue. '0': Input buffer disabled '1': Input buffer enabled"]
    #[inline(always)]
    pub fn in_en0(&self) -> InEn0R {
        InEn0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - The GPIO drive mode for IO pin 1"]
    #[inline(always)]
    pub fn drive_mode1(&self) -> DriveMode1R {
        DriveMode1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Enables the input buffer for IO pin 1"]
    #[inline(always)]
    pub fn in_en1(&self) -> InEn1R {
        InEn1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - The GPIO drive mode for IO pin 2"]
    #[inline(always)]
    pub fn drive_mode2(&self) -> DriveMode2R {
        DriveMode2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Enables the input buffer for IO pin 2"]
    #[inline(always)]
    pub fn in_en2(&self) -> InEn2R {
        InEn2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - The GPIO drive mode for IO pin 3"]
    #[inline(always)]
    pub fn drive_mode3(&self) -> DriveMode3R {
        DriveMode3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Enables the input buffer for IO pin 3"]
    #[inline(always)]
    pub fn in_en3(&self) -> InEn3R {
        InEn3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - The GPIO drive mode for IO pin4"]
    #[inline(always)]
    pub fn drive_mode4(&self) -> DriveMode4R {
        DriveMode4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Enables the input buffer for IO pin 4"]
    #[inline(always)]
    pub fn in_en4(&self) -> InEn4R {
        InEn4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - The GPIO drive mode for IO pin 5"]
    #[inline(always)]
    pub fn drive_mode5(&self) -> DriveMode5R {
        DriveMode5R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Enables the input buffer for IO pin 5"]
    #[inline(always)]
    pub fn in_en5(&self) -> InEn5R {
        InEn5R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - The GPIO drive mode for IO pin 6"]
    #[inline(always)]
    pub fn drive_mode6(&self) -> DriveMode6R {
        DriveMode6R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Enables the input buffer for IO pin 6"]
    #[inline(always)]
    pub fn in_en6(&self) -> InEn6R {
        InEn6R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - The GPIO drive mode for IO pin 7"]
    #[inline(always)]
    pub fn drive_mode7(&self) -> DriveMode7R {
        DriveMode7R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Enables the input buffer for IO pin 7"]
    #[inline(always)]
    pub fn in_en7(&self) -> InEn7R {
        InEn7R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO &amp; UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell."]
    #[inline(always)]
    #[must_use]
    pub fn drive_mode0(&mut self) -> DriveMode0W<CfgSpec> {
        DriveMode0W::new(self, 0)
    }
    #[doc = "Bit 3 - Enables the input buffer for IO pin 0. This bit should be cleared when analog signals are present on the pin to avoid crowbar currents. The output buffer can be used to drive analog signals high or low without issue. '0': Input buffer disabled '1': Input buffer enabled"]
    #[inline(always)]
    #[must_use]
    pub fn in_en0(&mut self) -> InEn0W<CfgSpec> {
        InEn0W::new(self, 3)
    }
    #[doc = "Bits 4:6 - The GPIO drive mode for IO pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn drive_mode1(&mut self) -> DriveMode1W<CfgSpec> {
        DriveMode1W::new(self, 4)
    }
    #[doc = "Bit 7 - Enables the input buffer for IO pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn in_en1(&mut self) -> InEn1W<CfgSpec> {
        InEn1W::new(self, 7)
    }
    #[doc = "Bits 8:10 - The GPIO drive mode for IO pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn drive_mode2(&mut self) -> DriveMode2W<CfgSpec> {
        DriveMode2W::new(self, 8)
    }
    #[doc = "Bit 11 - Enables the input buffer for IO pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn in_en2(&mut self) -> InEn2W<CfgSpec> {
        InEn2W::new(self, 11)
    }
    #[doc = "Bits 12:14 - The GPIO drive mode for IO pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn drive_mode3(&mut self) -> DriveMode3W<CfgSpec> {
        DriveMode3W::new(self, 12)
    }
    #[doc = "Bit 15 - Enables the input buffer for IO pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn in_en3(&mut self) -> InEn3W<CfgSpec> {
        InEn3W::new(self, 15)
    }
    #[doc = "Bits 16:18 - The GPIO drive mode for IO pin4"]
    #[inline(always)]
    #[must_use]
    pub fn drive_mode4(&mut self) -> DriveMode4W<CfgSpec> {
        DriveMode4W::new(self, 16)
    }
    #[doc = "Bit 19 - Enables the input buffer for IO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn in_en4(&mut self) -> InEn4W<CfgSpec> {
        InEn4W::new(self, 19)
    }
    #[doc = "Bits 20:22 - The GPIO drive mode for IO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn drive_mode5(&mut self) -> DriveMode5W<CfgSpec> {
        DriveMode5W::new(self, 20)
    }
    #[doc = "Bit 23 - Enables the input buffer for IO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn in_en5(&mut self) -> InEn5W<CfgSpec> {
        InEn5W::new(self, 23)
    }
    #[doc = "Bits 24:26 - The GPIO drive mode for IO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn drive_mode6(&mut self) -> DriveMode6W<CfgSpec> {
        DriveMode6W::new(self, 24)
    }
    #[doc = "Bit 27 - Enables the input buffer for IO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn in_en6(&mut self) -> InEn6W<CfgSpec> {
        InEn6W::new(self, 27)
    }
    #[doc = "Bits 28:30 - The GPIO drive mode for IO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn drive_mode7(&mut self) -> DriveMode7W<CfgSpec> {
        DriveMode7W::new(self, 28)
    }
    #[doc = "Bit 31 - Enables the input buffer for IO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn in_en7(&mut self) -> InEn7W<CfgSpec> {
        InEn7W::new(self, 31)
    }
}
#[doc = "Port configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
