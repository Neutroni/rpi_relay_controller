# rpi_relay_controller
Relay control program for Raspberry Pi computers written in Rust that only 
activates relay if weather condition allows operation.

To get weather info the program uses separate program to serve weather info and 
when the relay control program is started current weather info is loaded from a 
path defined in a configuration file.

## Requirements
- [rpi_bme280_server](https://github.com/Neutroni/rpi-bme280-server) for getting weather info from
- [RPPAL](https://docs.golemparts.com/rppal/) compatible operating system and hardware

## Configuration
Configuration is loaded from a [TOML](https://toml.io/) file that contains parameters
required for the operation. [Sample of the configuration](samples/config.toml)

Path to the rpi_bme280_server, pin the relay is connected to, operation duration, 
too low temperature limit and too high humidity limit can be customized to requirements
without having to recompile the code and to allow one program to control multiple relays.

## Startup using systemd
[Samples](samples/) folder contains following files related to systemd startup:

- [sample.service](samples/sample.service) That must be modified to match compiled programs location
  - `ExecStart` Defines the program that is to be run. Sample assumes that binary 
    is located in `/home/Pi/Programs/rpi_relay_controller/` directory and named 
    `rpi_relay_controller` and that `config.toml` file is present in the working directory.
  - `WorkingDirectory` Sets the working directory for the program, if `ExecStart`
    line does not contain the configuration file location `config.toml` file must
    exist in the working directory.
- [sample.timer](samples/sample.timer) That must be modified to match required activation interval
  -  `OnCalendar` Defines when the relay should activate, default is daily at 21:00
      for more info for the format see [systemd.time(7)](https://man.archlinux.org/man/systemd.time.7.en#CALENDAR_EVENTS)

Both files can be copied to target systems `/etc/systemd/system/` folder with 
appropriate names for the device the relay controls. After both files have been 
copied the relay can be started by using `$sudo systemctl start sample.timer` command.
