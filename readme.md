# Broken Motorwerks Telemetry

## Introduction

This project encompasses a real-time telemetry system for Broken MotorWerk's [LDRL](https://www.racelucky.com/) race car. The project consists of a rust application running on an STM32 that reads CAN data from the vehicle, and broadcasts the data over an XBEE RF link.

![pacific](./assets/pacific.jpg)

This project is a playground for a Rust based telemetry system.

## Project Goals

- [ ] Custom PCB utilizing stm32
- [ ] Telemetry application
- [ ] Client application for real-time monitoring
  - [ ] GUI or dashboard based application
  - [ ] Alarms and position monitoring
- [ ] Cloud integration

## Components

### PCB

- STM32F303VC
- [Digi XBee-PRO 900HP RF Module](https://www.digi.com/products/embedded-systems/digi-xbee/rf-modules/sub-1-ghz-rf-modules/xbee-pro-900hp)
- [SN65HVD230](https://www.ti.com/product/SN65HVD230) CAN transceiver

## Developing

- Install probe.rs

```
cargo run
```
