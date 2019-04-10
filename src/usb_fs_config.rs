use core::ptr;

static BASE_ADDR: usize = 0x5000_0000;
static OTG_DCFG_OFFSET: usize = 0x800;
static OTG_GINTMSK_OFFSET: usize = 0x018;

/// following the steps on page 1411 37.16.3 Device initialization of the reference manual
pub fn init() {
    //Program the OTG_DCFG register: Page 1370
    {
        //Set Device speed to Full speed(only option on the FS USB Port)
        let part_0: u32 = 0x3 << 0; //bit index=0 and 1
        //Set Non-zero-length status OUT handshake to 0 (not sure what this dose maybe 1 would be better)
        let part_1: u32 = 0x0 << 2; //0x4; //bit index=2 

        let config: u32 = part_0 | part_1;

        //write to the right register
        unsafe { ptr::write_volatile((BASE_ADDR + OTG_DCFG_OFFSET) as *mut u32, config) };
    }

    //Program the OTG_GINTMSK register: Page 1341
    {
        //Set the Device to unmask USB reset
        let part_0: u32 = 0x1 << 12; //bit index=12
        //Set the Device to unmask Enumeration done
        let part_1: u32 = 0x1 << 13; //bit index=13
        //Set the Device to unmask Early suspend
        let part_2: u32 = 0x1 << 10; //bit index=10
        //Set the Device to unmask USB suspend
        let part_3: u32 = 0x1 << 11; //bit index=11
        //Set the Device to unmask Start of frame mask
        let part_4: u32 = 0x1 << 3; //bit index=3

        let config = part_0 | part_1 | part_2 | part_3 | part_4;

        //write to the right register
        unsafe { ptr::write_volatile((BASE_ADDR + OTG_GINTMSK_OFFSET) as *mut u32, config) };
    }
}

