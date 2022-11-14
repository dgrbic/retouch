<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

![CI][build-yml-badge]

# reTouch

Set image file (jpg, tiff) create/modification datetime to datetime from EXIF.

## Description

Copying the images from your phone or camera often sets the date and time to the time of copying, so if you want to, for example, sort or find the images from particular date, you are out of luck.

This utility program helps with this by changing the creation and/or modification date and time to the time picture is actualy taken, as this information is (almost always) written in the image metadata (known as EXIF data).

Note that creation date cannot be set on Linux, as there is no support for that information on Linux.


## Installing


### Binary release

Binary releases are available in the [Github releases page](https://github.com/dgrbic/retouch/releases).



If you have [rust](https://www.rust-lang.org/), just run the following command

```sh
cargo install --force retouch
```

### Install from source

To install from source, you can clone the Git repository, build with Cargo (assuming you have [rust](https://www.rust-lang.org/)) and copy the binary into a destination directory. This will build the project from the latest commit on the main branch, which may not correspond to the latest stable release:
```
> git clone https://github.com/dgrbic/retouch.git
> cd retouch
> cargo build --release
> cp ./target/release/retouch /dest/path/
```
* Note that ```cp``` command will work on linux shells and on Windows Powershell, if you use ```CMD.EXE``` you should use ```copy``` command and change slashes to backslashes.
Also note that you should think about copying the executable to the folder that is in the ```PATH```, to be able to execute it by simly typing the executable name, without full path to the executable.


## Executing program

The ```reTouch``` is a command line utility program, and as such it should be started from your favorite shell.

For example,
```
retouch -c *.jpg
```
will change the file creation date/time (Windows only feature) of all ```jpg``` files in current folder which have embedded EXIF metadata with create date field.

## Usage

```
Usage: retouch.exe [OPTIONS] [FILES]... [-- <EXCLUDE_FILES>...]

Arguments:
  [FILES]...          One or more file specification (name or wildcard) to change date/time. Defaults to '*'
  [EXCLUDE_FILES]...  One or more file specification (filename or wildcard) to skip from <include_files> list

Options:
  -c             Set creation date
  -m             Set modification date
  -a             Set last access date
  -l, --list     Lists files, displaying EXIF embedded creation date. Other date flags are ignored, no changes are applied
  -h, --help     Print help information
  -V, --version  Print version information
```

The c, m and a could be combined, for example: ```retouch -rc *.jpg```.


Note that ```-c``` option works and is available only on Windows, as Linux does not record file creation date on every filesystem (e.g. ext2/ext3) and even where it is recorded there is no (documented, at least) way to modify information on mounted filesystem.


## Author

Dragan Grbić  
[@DraganGrbic](https://twitter.com/DraganGrbic)

## Version History

* 0.0.1
    * Initial Release

## License

This project is licensed under the MIT License - see the LICENSE.md file for details



### Built With 
[![Rust][rust-lang.org]][Rust-url]




<!-- ROADMAP 
## Roadmap

- [ ] Feature 1
- [ ] Feature 2
- [ ] Feature 3
    - [ ] Nested Feature
-->    

See the [open issues](https://github.com/dgrbic/retouch/issues) for a full list of proposed features (and known issues).



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/dgrbic/retouch.svg?style=for-the-badge
[contributors-url]: https://github.com/dgrbic/retouch/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/dgrbic/retouch.svg?style=for-the-badge
[forks-url]: https://github.com/dgrbic/retouch/network/members
[stars-shield]: https://img.shields.io/github/stars/dgrbic/retouch.svg?style=for-the-badge
[stars-url]: https://github.com/dgrbic/retouch/stargazers
[issues-shield]: https://img.shields.io/github/issues/dgrbic/retouch.svg?style=for-the-badge
[issues-url]: https://github.com/dgrbic/retouch/issues
[license-shield_old]: https://img.shields.io/license/dgrbic/retouch.svg?style=for-the-badge 
[license-shield]: https://img.shields.io/static/v1?style=for-the-badge&label=License&message=MIT&colorB=darkgray
[license-url]: https://github.com/dgrbic/retouch/blob/master/LICENSE.md
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/dragan-grbic/
[Rust-lang.org]: https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[build-yml-badge]: https://github.com/dgrbic/retouch/actions/workflows/build.yml/badge.svg
