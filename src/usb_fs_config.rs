use core::ptr;
use stm32f7::stm32f7x6::OTG_FS_GLOBAL;

use stm32f7_discovery::{system_clock, print, println};


pub struct UsbHandle {
    //ToDo
}

pub fn init() -> UsbHandle {
    let global;
    unsafe {global = OTG_FS_GLOBAL::ptr().as_ref().unwrap_or_else(|| {panic!("OTG_FS_GLOBAL returned a null pointer line 12 usb_fs_config")});}

    global.otg_fs_gahbcfg.write(|bits| {bits.gint().clear_bit()}); //usb_dcd.c line 59 //Page 3 on my document

    global.otg_fs_gusbcfg.write(|bits| {bits.physel.set_bit()}); //usb_core.c line 257 //Page 4 on my document

    //soft core reset usb_core.c line 260 //Page 5 on my document
    {
        //wait until idle
        while global.otg_fs_grstctl.read().ahbidl().bit_is_set() {}

        global.otg_fs_grstctl.write(|bits| {bits.csrst().set_bit()});

        //wait until idle
        while global.otg_fs_grstctl.read().ahbidl().bit_is_set() {}
        //wait 3 phy ticks 
        //because phy ticks are tricky we just wait 3 sys ticks which is longer
        system_clock::wait_ticks(3);
    }

    global.otg_fs_gccfg.write(|bits| {bits.pwrdwn.set_bit()}); //usb_core.c line 277 //Page 4 on my document

    global.otg_fs_gusbcfg.write(|bits| {
        bits.physel.set_bit(); //we set the bit again, because otherwise we would write the reset value

        bits.fdmod.set_bit();
        bits.fhmod.set_bit();
    }); //usb_dcd.c line 66 //Page 3 on my document

    let pwrclk;
    unsafe {pwrclk = OTG_FS_PWRCLK::ptr().as_ref().unwrap_or_else(|| {panic!("OTG_FS_PWRCLK returned a null pointer line 43 usb_fs_config")});}

    pwrclk.otg_fs_pcgcctl.write(|bits| bits); //usb_core.c line 491 //Page 6 on my document

    let device;
    unsafe {device = OTG_FS_DEVICE::ptr().as_ref().unwrap_or_else(|| {panic!("OTG_FS_DIVICE returned a null pointer line 48 usb_fs_config")});}

    device.otg_fs_dcfg.write(|bits| {
        bits
        .pfivl().bits(0) //usb_core.c line 495 //Page 6 on my document
        .dspd().bits(3) //usb_core.c line 502 //Page 6 on my document
    }); 

    global.otg_fs_grxfsiz.write(|bits| {bits.rxfd().bits(128)}); //usb_core.c line 505 //Page 6 on my document

    global.otg_fs_dieptxf0_device.write(|bits| {
        bits
        .tx0fsa().bits(128)
        .tx0fd().bits(32)
    }); //usb_core.c line 510 //Page 6 on my document

    global.otg_fs_dieptxf0_device.write(|bits| {
        bits
        .tx0fsa().bits(128)
        .tx0fd().bits(32)
    }); //usb_core.c line 510 //Page 6 on my document

    global.otg_fs_dieptxf1.write(|bits| {
        bits
        .ineptxsa().bits(160)
        .ineptxfd().bits(128)
    }); //usb_core.c line 516 //Page 6 on my document

    global.otg_fs_dieptxf2.write(|bits| {
        bits
        .ineptxsa().bits(288)
        .ineptxfd().bits(32)
    }); //usb_core.c line 522 //Page 6 on my document

    global.otg_fs_dieptxf3.write(|bits| {
        bits
        .ineptxsa().bits(320)
        .ineptxfd().bits(0)
    }); //usb_core.c line 528 //Page 6 on my document

    //wait until idle
    while global.otg_fs_grstctl.read().ahbidl().bit_is_set() {}

    //flush TxFifo
    global.otg_fs_grstctl.write(|bits| {
        bits
        .txfflsh().set_bit()
        .txfnum().bits(0x10)
    }); //usb_core.c line 533 //Page 7 on my document

    //wait until idle
    while global.otg_fs_grstctl.read().ahbidl().bit_is_set() || global.otg_fs_grstctl.read().txfflsh().bit_is_set() {}

    //flush RxFifo
    global.otg_fs_grstctl.write(|bits| {
        bits
        .rxfflsh().set_bit()
    }); //usb_core.c line 534 //Page 7 on my document

    //wait until idle
    while global.otg_fs_grstctl.read().ahbidl().bit_is_set() || global.otg_fs_grstctl.read().rxfflsh().bit_is_set() {}

    //reset all Device Interupts
    device.otg_fs_diepempmsk.write(|bits| bits);
    device.otg_fs_doepmsk.write(|bits| bits);
    device.otg_fs_daint.write(|bits| bits); //in usb_core.c is this value set to 0xFFFF_FFFF but this is not the reset value
    device.otg_fs_daintmsk.write(|bits| bits);

    

}

/*pub fn init() -> UsbHandle {
    //teste screen printing:
    unsafe { ptr::write_volatile(0xC000_0000 as *mut u32, 0xFF_FF_00_00 as u32)};
    /*reg.otg_fs_gahbcfg.modify(|old_bits, new_bits| {new_bits.gint.clear_bit()})*/



    println!("Spub fn init() -> UsbHandle {chreibe 0x10 lieb in reg");
    unsafe { ptr::write_volatile(0x5000_000C as *mut u32, 0x10 as u32)};
    println!("True Value: {:X}", unsafe { ptr::read_volatile(0x5000_000C as *mut u32)});

    println!("Schreibe 0x10 hard in reg");
    unsafe { ptr::write_bytes(0x5000_000C as *mut u32, 0x10 as u8, 1)};
    println!("True Value: {:X}", unsafe { ptr::read_volatile(0x5000_000C as *mut u32)});


    println!("True true Value: {:X}", unsafe { ptr::read(0x5000_000C as *mut u32)});

    let reg;
    unsafe {
        reg = OTG_FS_GLOBAL::ptr().as_ref().unwrap_or_else(|| {panic!("OTG_FS_GLOBAL returned a null pointer line 12 usb_fs_config")});
    }

    println!("True Value: {:X}", unsafe { ptr::read_volatile(0x5000_000C as *mut u32)});

    println!("Hex: {:X}", reg.otg_fs_gahbcfg.read().bits());
    println!("Bin: {:b}", reg.otg_fs_gahbcfg.read().bits());

    /*reg.otg_fs_gahbcfg.modify(|old_bits, new_bits| {
        new_bits.gint.set_bit();
        new_bits
    })*/
    reg.otg_fs_gahbcfg.write(|bits| {
        bits.gint().set_bit();
        bits
    });

    reg.otg_fs_gahbcfg.write(|bits| {
        bits.txfelvl().set_bit();
        bits
    });

    reg.otg_fs_gahbcfg.write(|bits| {
        bits.ptxfelvl().set_bit();
        bits
    });

    reg.otg_fs_gahbcfg.write(|bits| {
        unsafe {bits.bits(1)}
    });

    system_clock::wait_ms(2_000);

    println!("Hex: {:X}", reg.otg_fs_gahbcfg.read().bits());
    println!("Bin: {:b}", reg.otg_fs_gahbcfg.read().bits());

    println!("True Value: {:X}", unsafe { ptr::read_volatile(0x5000_000C as *mut u32)});


    UsbHandle {
        //ToDo
    }
}*/









//Erster Versuch
/*
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

*/