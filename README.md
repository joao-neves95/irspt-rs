# irspt-rs

![CLI screenshot - date picker](/_assets/1.png)

![CLI screenshot - complete](/_assets/2.png)

---

## Description

A CLI program to issue invoices by interfacing with Portugal's Tax Authority. <br/>
It's possible to create a template to automate the process of issuing an invoice. <br/>
This is for my own use, it's hardcoded for me, and it does not have many features. <br/>
Maybe I'll make it generic in the future.

`cargo run --release`

---

## Disclamer

THIS OPEN SOURCE SOFTWARE IS PROVIDED ON AN “AS IS” BASIS.\
IN NO EVENT UNLESS REQUIRED BY APPLICABLE LAW OR AGREED TO IN WRITING WILL ANY COPYRIGHT HOLDER,
OR ANY OTHER PARTY WHO MODIFIES AND/OR CONVEYS THE PROGRAM AS PERMITTED ABOVE, BE LIABLE TO YOU FOR DAMAGES,
INCLUDING ANY GENERAL, SPECIAL, INCIDENTAL OR CONSEQUENTIAL DAMAGES ARISING OUT OF THE USE OR INABILITY TO USE
THE PROGRAM (INCLUDING BUT NOT LIMITED TO LOSS OF DATA OR DATA BEING RENDERED INACCURATE OR LOSSES SUSTAINED BY
YOU OR THIRD PARTIES OR A FAILURE OF THE PROGRAM TO OPERATE WITH ANY OTHER PROGRAMS), EVEN IF SUCH HOLDER OR
OTHER PARTY HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.

---

## Prerequisites

- Have `geckodriver` and `firefox` installed in your machine. Same version as Firefox.\
  https://github.com/mozilla/geckodriver/releases

---

## Technologies

Made with:
- [Rust](https://github.com/rust-lang)
- [Tokio](https://github.com/tokio-rs/tokio) - Async.
- [thirtyfour](https://github.com/stevepryde/thirtyfour) - WebDriver client.
- [sled](https://github.com/spacejam/sled) - Embedded database.
- [rkyv](https://github.com/rkyv/rkyv) - Zero-copy raw byte deserialization.
- [Inquire](https://github.com/mikaelmello/inquire) - Prompt.

---
