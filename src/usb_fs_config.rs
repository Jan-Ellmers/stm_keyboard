use core::ptr;
use stm32f7::stm32f7x6::{OTG_FS_GLOBAL, RCC, OTG_FS_PWRCLK, OTG_FS_DEVICE};

use stm32f7_discovery::{system_clock, print, println};

pub fn init_usb_mem_clock() {
    let rcc;
    unsafe {rcc = RCC::ptr().as_ref().unwrap_or_else(|| {panic!("RCC returned a null pointer line {} usb_fs_config", line!())});}

    rcc.ahb2enr.modify(|_, bits| bits.otgfsen().set_bit()); 

    for _ in 0..10 {
        if rcc.ahb2enr.read().otgfsen().bit_is_set() {
            return;
        }
        system_clock::wait_ms(500);
    }
    panic!("Could not start USB FS clock");
}


pub struct UsbHandle {
    //ToDo
}

pub fn init() -> UsbHandle {
    let global;
    unsafe {global = OTG_FS_GLOBAL::ptr().as_ref().unwrap_or_else(|| {panic!("OTG_FS_GLOBAL returned a null pointer line {} usb_fs_config", line!())});}

    global.otg_fs_gahbcfg.write(|bits| {bits.gint().clear_bit()}); //usb_dcd.c line 59 //Page 3 on my document

    global.otg_fs_gusbcfg.write(|bits| {bits.physel().set_bit()}); //usb_core.c line 257 //Page 4 on my document

    //soft core reset usb_core.c line 260 //Page 5 on my document
    println!("Soft core reset");
    {
        //wait until idle
        println!("Wait for ide {}", line!());
        while global.otg_fs_grstctl.read().ahbidl().bit_is_clear() {}

        global.otg_fs_grstctl.write(|bits| {bits.csrst().set_bit()});

        //wait until idle
        println!("Wait for ide {}", line!());
        while global.otg_fs_grstctl.read().ahbidl().bit_is_clear() {}
        //wait 3 phy ticks 
        //because phy ticks are tricky we just wait 3 sys ticks which is longer
        system_clock::wait_ticks(3);
    }
    println!("Soft core reset finished");

    global.otg_fs_gccfg.write(|bits| {bits.pwrdwn().set_bit()}); //usb_core.c line 277 //Page 4 on my document

    global.otg_fs_gusbcfg.write(|bits| { bits
        .physel().set_bit() //we set the bit again, because otherwise we would write the reset value

        .fdmod().set_bit()
        .fhmod().set_bit()
    }); //usb_dcd.c line 66 //Page 3 on my document

    let pwrclk;
    unsafe {pwrclk = OTG_FS_PWRCLK::ptr().as_ref().unwrap_or_else(|| {panic!("OTG_FS_PWRCLK returned a null pointer line {} usb_fs_config", line!())});}

    pwrclk.otg_fs_pcgcctl.write(|bits| bits); //usb_core.c line 491 //Page 6 on my document

    let device;
    unsafe {device = OTG_FS_DEVICE::ptr().as_ref().unwrap_or_else(|| {panic!("OTG_FS_DIVICE returned a null pointer line {} usb_fs_config", line!())});}

    device.otg_fs_dcfg.write(|bits| {unsafe {
        bits
        .pfivl().bits(0) //usb_core.c line 495 //Page 6 on my document
        .dspd().bits(3) //usb_core.c line 502 //Page 6 on my document
    }}); 

    global.otg_fs_grxfsiz.write(|bits| {unsafe {bits.rxfd().bits(128)}}); //usb_core.c line 505 //Page 6 on my document

    global.otg_fs_dieptxf0_device.write(|bits| {unsafe {
        bits
        .tx0fsa().bits(128)
        .tx0fd().bits(32)
    }}); //usb_core.c line 510 //Page 6 on my document

    global.otg_fs_dieptxf0_device.write(|bits| {unsafe {
        bits
        .tx0fsa().bits(128)
        .tx0fd().bits(32)
    }}); //usb_core.c line 510 //Page 6 on my document

    global.otg_fs_dieptxf1.write(|bits| {unsafe {
        bits
        .ineptxsa().bits(160)
        .ineptxfd().bits(128)
    }}); //usb_core.c line 516 //Page 6 on my document

    global.otg_fs_dieptxf2.write(|bits| {unsafe {
        bits
        .ineptxsa().bits(288)
        .ineptxfd().bits(32)
    }}); //usb_core.c line 522 //Page 6 on my document

    global.otg_fs_dieptxf3.write(|bits| {unsafe {
        bits
        .ineptxsa().bits(320)
        .ineptxfd().bits(0)
    }}); //usb_core.c line 528 //Page 6 on my document

    //wait until idle
    println!("Wait for ide {}", line!());
    while global.otg_fs_grstctl.read().ahbidl().bit_is_clear() {}

    //flush TxFifo
    global.otg_fs_grstctl.write(|bits| {unsafe {
        bits
        .txfflsh().set_bit()
        .txfnum().bits(0x10)
    }}); //usb_core.c line 533 //Page 7 on my document

    //wait until idle
    println!("Wait for ide {}", line!());
    while global.otg_fs_grstctl.read().ahbidl().bit_is_clear() || global.otg_fs_grstctl.read().txfflsh().bit_is_set() {}

    //flush RxFifo
    global.otg_fs_grstctl.write(|bits| {
        bits
        .rxfflsh().set_bit()
    }); //usb_core.c line 534 //Page 7 on my document

    //wait until idle
    println!("Wait for ide {}", line!());
    while global.otg_fs_grstctl.read().ahbidl().bit_is_clear() || global.otg_fs_grstctl.read().rxfflsh().bit_is_set() {}

    //reset all Device Interupts
    device.otg_fs_diepempmsk.write(|bits| bits);
    device.otg_fs_doepmsk.write(|bits| bits);
    //device.otg_fs_daint.write(|bits| bits); //in usb_core.c is this value set to 0xFFFF_FFFF but this is not the reset value
    device.otg_fs_daintmsk.write(|bits| bits);

    device.otg_fs_diepctl0.write(|bits| bits);
    device.otg_fs_dieptsiz0.write(|bits| bits);

    device.otg_fs_diepctl1.write(|bits| bits);
    device.otg_fs_dieptsiz1.write(|bits| bits);

    device.otg_fs_diepctl2.write(|bits| bits);
    device.otg_fs_dieptsiz2.write(|bits| bits);

    device.otg_fs_diepctl3.write(|bits| bits);
    device.otg_fs_dieptsiz3.write(|bits| bits);

    //usb_core.c line 556 writes to reserved bits i ignore it

    
    device.otg_fs_doepctl0.write(|bits| bits);
    device.otg_fs_doeptsiz0.write(|bits| bits);

    device.otg_fs_doepctl1.write(|bits| bits);
    device.otg_fs_doeptsiz1.write(|bits| bits);

    device.otg_fs_doepctl2.write(|bits| bits);
    device.otg_fs_doeptsiz2.write(|bits| bits);

    device.otg_fs_doepctl3.write(|bits| bits);
    device.otg_fs_doeptsiz3.write(|bits| bits);

    //usb_core.c line 574 writes to reserved bits i ignore it

    global.otg_fs_gintmsk.write(|bits| bits); //usb_core.c line 604
    let old_value: u32 = global.otg_fs_gintsts.read().bits();
    global.otg_fs_gintsts.modify(|_, bits| { 
        unsafe{
            bits.bits(
                old_value | (1 << 27)
            ); //lpmint is missing in otg_fs_gintsts
        }
        bits
        .mmis().set_bit()
        .sof().set_bit()
        .esusp().set_bit()
        .usbsusp().set_bit()
        .usbrst().set_bit()
        .enumdne().set_bit()
        .isoodrp().set_bit()
        .eopf().set_bit()
        .iisoixfr().set_bit()
        .ipxfr_incompisoout().set_bit()
        .rstdet().set_bit()
        .cidschg().set_bit()
        .discint().set_bit()
        .srqint().set_bit()
        .wkupint().set_bit()
    }); //usb_core.c line 606
    global.otg_fs_gotgint.write(|bits| bits); //usb_core.c line 608

    global.otg_fs_gintmsk.write(|bits| {bits
        .usbsuspm().set_bit()
        .wuim().set_bit()

        .usbrst().set_bit()
        .enumdnem().set_bit()
        .iepint().set_bit()
        .oepint().set_bit()
        .sofm().set_bit()
        .iisoixfrm().set_bit()
        .ipxfrm_iisooxfrm().set_bit()
        .srqim().set_bit()
        .otgint().set_bit()
    });

    device.otg_fs_daintmsk.write(|bits| {bits
        .iepm().set_bit()
        .oepint().set_bit()
    });

    

    
    UsbHandle {
        //ToDo
    }
}










/*
static ADDR: usize = 0x5000_0000;
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
        unsafe { ptr::write_volatile((ADDR + OTG_DCFG_OFFSET) as *mut u32, config) };
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
        unsafe { ptr::write_volatile((ADDR + OTG_GINTMSK_OFFSET) as *mut u32, config) };
    }
}*/

