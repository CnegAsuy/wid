> ## *This project still in development* 

# About

Wid is a program that tracks your daily computer activities. It records the applications you currently have open, minute by minute, and saves this information to a local file (which I will encrypt). Then, it provides an analysis of your computer usage and show the usage time of applications you select based on this data.

# Roadmap

- [x] wid-deamon (still developable)
  - [x] track the apps and write it on json file. (still developable)
  - [x] make all warnings.
  - [ ] delete the data after 2 month
  - [ ] encrypt the data
  - [ ] make a systemd-service
  - [ ] make styleable
- [ ] wid-cli
  - [x] make a visual screen (tui) with modern appearance. (still developable)
  - [ ] decrypt system
  - [ ] visualize the datas.
  - [ ] a menu for let user choice the processes tracked.
- [ ] wid-gtk
  - [ ] create a gtk app configable
  - [ ] visualize the datas.
  - [ ] a menu for let user choice the processes tracked.
- [ ] prepare setup-sh
- [ ] make a AUR version
# Installation

> This project still in development.

## Structure 
```
wid
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── setup.sh
├── wid-cli
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       ├── controller.rs
│       ├── db_reader.rs
│       ├── draw_tui.rs
│       ├── main.rs
│       └── widgets.rs
├── wid-daemon
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── wid-daemon.service
└── wid-gtk
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        └── main.rs

7 directories, 22 files
```
