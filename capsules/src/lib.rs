#![forbid(unsafe_code)]
#![no_std]

pub mod test;

#[macro_use]
pub mod net;

pub mod adc;
pub mod adc_microphone;
pub mod alarm;
pub mod ambient_light;
pub mod analog_comparator;
pub mod analog_sensor;
pub mod apds9960;
pub mod app_flash_driver;
pub mod ble_advertising_driver;
pub mod bmp280;
pub mod bus;
pub mod button;
pub mod buzzer;
pub mod buzzer_pwm;
pub mod console;
pub mod crc;
pub mod ctap;
pub mod dac;
pub mod debug_process_restart;
pub mod driver;
pub mod fm25cl;
pub mod ft6x06;
pub mod fxos8700cq;
pub mod gpio;
pub mod gpio_async;
pub mod hd44780;
pub mod hmac;
pub mod hts221;
pub mod humidity;
pub mod i2c_master;
pub mod i2c_master_slave_driver;
pub mod ieee802154;
pub mod isl29035;
pub mod kv_driver;
pub mod kv_store;
pub mod l3gd20;
pub mod led;
pub mod led_matrix;
pub mod log;
pub mod low_level_debug;
pub mod lps25hb;
pub mod lsm303agr;
pub mod lsm303dlhc;
pub mod lsm303xx;
pub mod lsm6dsoxtr;
pub mod ltc294x;
pub mod max17205;
pub mod mcp230xx;
pub mod mlx90614;
pub mod mx25r6435f;
pub mod ninedof;
pub mod nonvolatile_storage_driver;
pub mod nonvolatile_to_pages;
pub mod nrf51822_serialization;
pub mod panic_button;
pub mod pca9544a;
pub mod process_console;
pub mod proximity;
pub mod public_key_crypto;
pub mod read_only_state;
pub mod rf233;
pub mod rf233_const;
pub mod rng;
pub mod screen;
pub mod sdcard;
pub mod segger_rtt;
pub mod sha;
pub mod sht3x;
pub mod si7021;
pub mod sip_hash;
pub mod sound_pressure;
pub mod spi_controller;
pub mod spi_peripheral;
pub mod st77xx;
pub mod symmetric_encryption;
pub mod temperature;
pub mod temperature_rp2040;
pub mod temperature_stm;
pub mod text_screen;
pub mod tickv;
pub mod touch;
pub mod tsl2561;
pub mod usb;
pub mod virtual_adc;
pub mod virtual_aes_ccm;
pub mod virtual_alarm;
pub mod virtual_digest;
pub mod virtual_flash;
pub mod virtual_hmac;
pub mod virtual_i2c;
pub mod virtual_pwm;
pub mod virtual_rng;
pub mod virtual_sha;
pub mod virtual_spi;
pub mod virtual_timer;
pub mod virtual_uart;
