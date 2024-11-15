> ## *This project still in development* 

# About

Wid is a program that tracks your daily computer activities. It records the applications you currently have open, minute by minute, and saves this information to a local file (which I will encrypt). Then, it provides an analysis of your computer usage and show the usage time of applications you select based on this data.

# Roadmap

- [x] wid-deamon (still developable)
  - [x] track the apps and write it on json file. (still developable)
  - [x] make all warnings.
- [ ] wid-cli
  - [x] make a visual screen (tui) with modern appearance.
  - [ ] visualize the datas.
  - [ ] a menu for let user choice the processes tracked.
- [ ] wid-gtk
  - [ ] create a gtk app configable
  - [ ] visualize the datas.
  - [ ] a menu for let user choice the processes tracked.
- [ ] create a wid-daemon.service.
- [ ] prepare the setup.sh.
# Installation

> This project still in development.

## Structure 
```
.
├── README.md
├── setup.sh
├── wid-cli
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── wid-daemon
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── db.json
│   ├── src
│   │   └── main.rs
│   └── track.txt
└── wid-gtk
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        └── main.rs

7 directories, 13 files
```