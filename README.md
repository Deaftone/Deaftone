<p align="center">
  <img src="https://user-images.githubusercontent.com/13013625/212359431-231687c0-4aae-4712-aae5-49c5fc6c6bbf.png" alt="Deaftione logo" title="navidrome" align="center" height="340" width="340" />

</p>

<h1 align="center">Deaftone</h1>
<div align="center"

[![license](https://img.shields.io/github/license/Ortygia/Deaftone)](https://github.com/Ortygia/Deaftone/blob/master/LICENSE)
[![CI](https://github.com/Ortygia/Deaftone/actions/workflows/ci.yml/badge.svg)](https://github.com/Ortygia/Deaftone/actions/workflows/build.yml)
[![codecov](https://codecov.io/gh/Ortygia/Deaftone/branch/master/graph/badge.svg?token=NWS6Q3W4FP)](https://app.codecov.io/github/Deaftone/Deaftone)
![GitHub repo size](https://img.shields.io/github/repo-size/Ortygia/Deaftone)
![Lines of Code](https://aschey.tech/tokei/github/Ortygia/Deaftone)
[![Version](https://img.shields.io/github/v/release/Ortygia/Deaftone)](https://github.com/Deaftone/Deaftone/releases/latest)
</div>

# Overview
A cross-platform open source music collection server and streamer. Leaving Subsonic in the past and starting fresh with a new design and new API
Currently in active development

# Perfomance
Currently scans 34,000 songs in ~14 mins. My testing is done on a USB Desktop HDD 3TB seagate over USB3.

# Features
* Ability to handle the largest of music collections
* Very low system resource usage
* Multi-platform. Currently building for macOS, Windows, Linux, Arm and Armhf 


# Roadmap
* Built-in metadata scrapping of sources such as MusicBrainz, LastFM and AllMusic
* SlimProto Support
* Playlist Curation
* Recommendation engine
* ReplayGain support 
* Radio mode


# Documentation
All documentation can be found in the project's website: https://deaftone.org/intro. Here are some useful direct links:
- [Overview](https://www.navidrome.org/docs/overview/)
- [Clients](https://deaftone.org/clients)
- [Installation](https://deaftone.org/setup/installation)
- [API](https://deaftone.org/api)

# Clients
* Tauri Desktop client for macOS, Windows and Linux
https://github.com/Ortygia/Orpheus currently in early stages of development but usable
* Android based application possibly native or using Tauri Mobile

# Installation
Currently to setup and install Deaftone you need to download the binarie in release for you platform or clone and build the repo.
After you have your binary in the same folder you need to place a ``settings.toml`` with the following inside it
```
log_level="deaftone=info,tower_http=info"
db_path="./deaftone.sqlite"
media_path="H:\\aa"
```
Where media_path is the location of your media
db_path is where to save the database
and logging is for change the log level of the application

# Building from Source
When building Deaftone from source your MSRV (Minimum supported Rust version) is ``1.65 or newer``

