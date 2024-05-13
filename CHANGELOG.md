# Changelog

## 1.6.0 (2024-05-13)

- Uniform handling of control characters
- Remove poor any_ascii transliterations
- Don't spell out skin tone modifiers

## 1.4.4 (2024-04-13)

- Revert "Change mapping of Ä"

## 1.4.3 (2024-02-16)

- Slightly improved compression
- New emoji

## 1.4.2 (2023-12-10)

- Change mapping of Ä

## 1.4.0 (2023-09-15)

- Make the iter implement Display

## 1.3.3 (2022-12-15)

- Add fallback from unicode decomposition

## 1.3.2 (2022-08-21)

- More emoji
- Support no_std

## 1.3.0 (2021-05-07)

- Fall back to any_ascii

## 1.2.0 (2021-03-24)

- Fast path for ASCII-only strings
- Update gemoji

## 1.1.1 (2020-04-23)

- Fixed shifted/cropped replacements

## 1.1.0 (2020-02-24)

- Update emoji database
- Edition 2018

## 1.0.0 (2018-12-22)

- Added more emoji

## 0.4.0 (2017-05-05)

- More efficient lookup table which gives smaller memory footprint
- Emoji! (partial support since there's so many of them)
- Fixed trailing spaces

## 0.3.0 (2016-12-25)

- Updated mappings from Text::Unidecode version 1.30.

## 0.2.0 (2015-07-01)

- Switched from phf map to lookup table for speed increase

## 0.1.10 (2015-07-01)

- Added `unidecode_char()` function

## 0.1.9 (2015-04-14)

- Fixed incorrect visibility modifier

## 0.1.8 (2015-04-12)

- Replaced `phf_macros` usage with `phf_codegen`, which works on stable Rust

## 0.1.7 (2015-03-26)

- Updated dependencies

## 0.1.6 (2015-03-22)

- Updated dependencies

## 0.1.5 (2015-03-20)

- Updated dependencies

## 0.1.4 (2015-03-18)

- Fixed typos

## 0.1.3 (2015-03-17)

- Changed badges in README to use shields.io

## 0.1.2 (2015-03-17)

- Added version badge to README

## 0.1.1 (2015-03-17)

- Added link to documentation in the README

## 0.1.0 (2015-03-17)

- Initial release
- Entire `Text::Unidecode` data set exported into a Rust code file
