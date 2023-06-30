<p align="center">
  <img src="https://user-images.githubusercontent.com/13013625/212359431-231687c0-4aae-4712-aae5-49c5fc6c6bbf.png" alt="Deaftione logo" title="navidrome" align="center" height="340" width="340" />

</p>

<h1 align="center">Deaftone</h1>
<div align="center"

[![license](https://img.shields.io/github/license/Deaftone/Deaftone)](https://github.com/Deaftone/Deaftone/blob/master/LICENSE)
[![CI](https://github.com/Deaftone/Deaftone/actions/workflows/ci.yml/badge.svg)](https://github.com/Deaftone/Deaftone/actions/workflows/build.yml)
[![codecov](https://codecov.io/gh/Deaftone/Deaftone/branch/master/graph/badge.svg?token=NWS6Q3W4FP)](https://app.codecov.io/github/Deaftone/Deaftone)
![GitHub repo size](https://img.shields.io/github/repo-size/Deaftone/Deaftone)
![Lines of Code](https://aschey.tech/tokei/github/Deaftone/Deaftone)
[![Version](https://img.shields.io/github/v/release/Deaftone/Deaftone)](https://github.com/Deaftone/Deaftone/releases/latest)
[![MSRV](https://img.shields.io/badge/MSRV-1.65-informational)](https://github.com/Deaftone/Deaftone/edit/master/README.md#building-from-source)
</div>

# Overview

Deaftone is a lightweight, cross-platform, open-source music server and streamer designed to make it easy to manage and listen to your music collection. Built using Rust, Axum, and SeaORM, Deaftone offers a modern and fast alternative to older, more bloated music servers like Subsonic. With its streamlined design and user-friendly API, Deaftone makes it easy to access your music library from anywhere, on any device.

Deaftone is currently in active development, with a focus on building out its core functionality and adding new features based on user feedback. Stay tuned for updates and new releases as we continue to improve and refine the platform.

# Perfomance
Currently scans 34,000 songs in ~14 mins. My testing is done on a USB Desktop HDD 3TB seagate over USB3.

# Features
* Ability to handle the largest of music collections
* Cross-platform compatibility, with support for Windows, macOS, and Linux
* Simple, user-friendly API for easy integration with third-party applications
* Efficient, low-overhead architecture designed for fast performance and low resource usage
* Support for a wide range of audio formats, including FLAC, MP3, Ogg, and more
* Advanced features like playlists, automatic metadata scrapping, and more

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

Indexing can be made as follows:
```
$ curl "http://localhost:3030/tasks?task=scan_library"
{"status":"sent"}
```
Currently, deaftone only scans flac files.

# Building from Source
When building Deaftone from source your MSRV (Minimum supported Rust version) is ``1.65 or newer``

# Project supported by JetBrains

Many thanks to Jetbrains for kindly providing a license for me to work on this and other open-source projects.

[![](https://resources.jetbrains.com/storage/products/company/brand/logos/jb_beam.svg)](https://www.jetbrains.com/?from=https://github.com/112RG)

