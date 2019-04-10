#![warn(clippy::all)]
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]


mod usb_fs_config;


use stm32f7_discovery::{
    gpio::{GpioPort, OutputPin},
    init,
    system_clock::{self, Hz},
    lcd,
    print,
    println
};
use stm32f7::stm32f7x6::{CorePeripherals, Peripherals};
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout as AllocLayout;
use core::panic::PanicInfo;
use core::ptr;
use cortex_m_rt::{entry, exception};

// Enables us to put stuff on the heap
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

// Called when that heap's out of memory
#[alloc_error_handler]
fn oom_handler(_: AllocLayout) -> ! {
    loop {}
}

// systick handler, increments the system clock
#[exception]
fn SysTick() {
    system_clock::tick();
}

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

struct Controller<A,B,C,D,E,F> where 
    A: stm32f7_discovery::gpio::OutputPin,
    B: stm32f7_discovery::gpio::InputPin,
    C: stm32f7_discovery::gpio::OutputPin,
    D: stm32f7_discovery::gpio::OutputPin,
    E: stm32f7_discovery::gpio::InputPin,
    F: stm32f7_discovery::gpio::InputPin,
{
    pub clock: Clock,
    pub pins: stm32f7_discovery::init::pins::Pins<A,B,C,D,E,F>,
}

struct Clock {}
impl Clock {
    fn ticks(&self) -> usize {
        system_clock::ticks()
    }

    fn wait(&self, seconds: usize, ticks: usize) { //TODO find a duration like type here
        let curr_ticks = self.ticks();
        //20 ticks = 1s
        while self.ticks() - curr_ticks < seconds * 20 + ticks {}
    }
}

#[entry]
fn init() -> ! {
    // Acquire systick
    let core_peripherals = CorePeripherals::take().unwrap();
    let mut systick = core_peripherals.SYST;

    // Initialize 216 MHz processor and enable GPIO
    let peripherals = Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC;
    let mut pwr = peripherals.PWR;
    let mut flash = peripherals.FLASH;
    let mut fmc = peripherals.FMC;
    let mut ltdc = peripherals.LTDC;
    init::init_system_clock_216mhz(&mut rcc, &mut pwr, &mut flash);
    init::enable_gpio_ports(&mut rcc);

    // Initialize GPIO
    let mut pins = init::pins(
        GpioPort::new(peripherals.GPIOA),
        GpioPort::new(peripherals.GPIOB),
        GpioPort::new(peripherals.GPIOC),
        GpioPort::new(peripherals.GPIOD),
        GpioPort::new(peripherals.GPIOE),
        GpioPort::new(peripherals.GPIOF),
        GpioPort::new(peripherals.GPIOG),
        GpioPort::new(peripherals.GPIOH),
        GpioPort::new(peripherals.GPIOI),
        GpioPort::new(peripherals.GPIOJ),
        GpioPort::new(peripherals.GPIOK)
    );

    // Initialize systick
    init::init_systick(Hz(20), &mut systick, &rcc);
    systick.enable_interrupt(); //breakpoint?

    // Enable SDRAM (prereq for LCD)
    init::init_sdram(&mut rcc, &mut fmc);
    // Initialize LCD
    let mut lcd = init::init_lcd(&mut ltdc, &mut rcc);
    // Enable display and backlight
    pins.display_enable.set(true);
    pins.backlight.set(true);

    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();
    layer_1.clear();
    layer_2.clear();
    lcd::init_stdout(layer_2);


    let mut controller = Controller {
        clock: Clock{},
        pins,
    };

    usb_fs_config::init();


    println!("End of Program");
    loop{}
}
