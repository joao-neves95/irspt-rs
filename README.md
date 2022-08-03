# irspt-rs

## Description

A CLI program to issue invoices by interfacing with Portugal's Tax Authority.\
It's possible to create a template to automate the process of issuing an invoice.\
This is a first version for my own use. At the moment, it does not support support features like:

- Multi-template support.
- Input selection by using reference data.
- Extensive error handling.
- And many others.

## Disclamer

THIS OPEN SOURCE SOFTWARE IS PROVIDED ON AN “AS IS” BASIS.\
IN NO EVENT UNLESS REQUIRED BY APPLICABLE LAW OR AGREED TO IN WRITING WILL ANY COPYRIGHT HOLDER,
OR ANY OTHER PARTY WHO MODIFIES AND/OR CONVEYS THE PROGRAM AS PERMITTED ABOVE, BE LIABLE TO YOU FOR DAMAGES,
INCLUDING ANY GENERAL, SPECIAL, INCIDENTAL OR CONSEQUENTIAL DAMAGES ARISING OUT OF THE USE OR INABILITY TO USE
THE PROGRAM (INCLUDING BUT NOT LIMITED TO LOSS OF DATA OR DATA BEING RENDERED INACCURATE OR LOSSES SUSTAINED BY
YOU OR THIRD PARTIES OR A FAILURE OF THE PROGRAM TO OPERATE WITH ANY OTHER PROGRAMS), EVEN IF SUCH HOLDER OR
OTHER PARTY HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.

## Prerequisites

- Have `geckodriver` running on your system. Same version as Firefox.\
  https://github.com/mozilla/geckodriver/releases

## Technologies

Made with:
- [Rust](https://github.com/rust-lang)
- [thirtyfour](https://github.com/stevepryde/thirtyfour) - WebDriver client.
- [sled](https://github.com/spacejam/sled) - Embedded database.
- [rkyv](https://github.com/rkyv/rkyv) - Zero-copy raw byte deserialization.
