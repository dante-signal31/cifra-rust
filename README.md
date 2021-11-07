% cifra(1) | cifra usage documentation

[![Crate](https://img.shields.io/crates/v/cifra.svg)](https://crates.io/crates/cifra)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/dante-signal31/cifra-rust)](https://github.com/dante-signal31/cifra-rust)
[![License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/dante-signal31/cifra-rust/test_and_deploy)
[![GitHub issues](https://img.shields.io/github/issues/dante-signal31/cifra-rust)](https://github.com/dante-signal31/cifra-rust/issues)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/y/dante-signal31/cifra-rust)](https://github.com/dante-signal31/cifra-rust/commits/master)
[![GitHub last commit](https://img.shields.io/github/last-commit/dante-signal31/cifra-rust)](https://github.com/dante-signal31/cifra-rust/commits/master)

# NAME
**cifra** — Library and console command to crypt and decrypt texts using classic methods.

# SYNOPSIS
|    `$ cifra MODE [-h | --help ]`

# DESCRIPTION
**cifra** is a console command and a python library to cipher and decipher texts
using classic methods. It also performs cryptoattacks against those methods.

I've implemented this while I read Al Sweigart's *"Cracking Codes with Python"*. While doing
it I also developed and alternative [Python implementation](https://github.com/dante-signal31/cifra)
to assess implementations differences between Python and Rust. Structure of both implementations is almost
identical, so it is interesting to compare side-to-side functions of both implementation to realize how the
same things must be expressed in Python and Rust.

Some conclusions are evident: Python is extremely expressive and can implement in just few lines what Rust
requires many more; on the other hand Rust is extremely performant and can execute the same calculations
many times quicker than Python. However, I've found out a really useful conclusion: Python is a great
prototyping language for Rust, reflexion topics apart the vast majority of everything else that you can do
in Python can be easily implemented in Rust too.

Be aware that cryptographics operations are inherently slow. Cifra does not return any visual feedback until
it has finished its work, so if you run a command and it keeps waiting for many seconds don't think it is stuck
and finish execution, chances are that command is simply doing its calculations silently. Be patient and
eventualy command will return its result.

# MODES

## Dictionary
Manage dictionaries to perform crypto attacks.

|    `$ cifra dictionary ACTION`

**Possible actions:**

* *create*: Create a dictionary of unique words.

  |        `$ cifra dictionary create NEW_DICTIONARY_NAME`

    + positional arguments:
        - NEW_DICTIONARY_NAME:    Name for the dictionary to create.
    + optional arguments:
        - -i PATH_TO FILE_WITH_WORDS | --initial_words_file PATH_TO FILE_WITH_WORDS:
          Optionally you can load in the dictionary words located in a file.
          File can be a regular text file, like a book. Redundant words are
          ignored by ingestion process.

* *delete*: Remove an existing dictionary.

  |        `$ cifra dictionary delete DICTIONARY_NAME_TO_DELETE`

    + positional arguments:
        - DICTIONARY_NAME_TO_DELETE:  Name for the dictionary to delete.

* *update*: Add words to an existing dictionary.

  |        `$ cifra dictionary update DICTIONARY_NAME_TO_UPDATE PATH_TO_FILE_WITH_WORDS`

    + positional arguments:
        - DICTIONARY_NAME_TO_UPDATE: Name for the dictionary to update with additional words.
        - PATH_TO_FILE_WITH_WORDS:  Pathname to a file with words to add to dictionary. File can be a regular text file, like
          a book. Redundant words are ignored by ingestion process.

* *list*: Show existing dictionaries.

  |        `$ cifra dictionary list`

## Cipher
Cipher a text using a key.

|    `$ cifra cipher ALGORITHM_NAME CIPHERING_KEY FILE_TO_CIPHER`

* positional arguments:
    + ALGORITHM_NAME: Algorithm to use to cipher.
    + CIPHERING_KEY: Key to use to cipher.
    + FILE_TO_CIPHER: Path to file with text to cipher.

* optional arguments:
    + -o OUTPUT_CIPHERED_FILE, --ciphered_file OUTPUT_CIPHERED_FILE:                        Path to output file to place ciphered text. If not
      used then ciphered text will be dumped to console.
    + -c CHARSET, --charset CHARSET:
      Default charset is ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefgh
      ijklmnopqrstuvwxyz1234567890 !?., but you can set here
      another.

## Decipher
Decipher a text using a key.

|    `$ cifra decipher ALGORITHM_NAME CIPHERING_KEY FILE_TO_DECIPHER`

* positional arguments:
    + ALGORITHM_NAME: Algorithm to use to cipher.
    + CIPHERING_KEY: Key to use to cipher.
    + FILE_TO_CIPHER: Path to file with text to cipher.

* optional arguments:
    + -o OUTPUT_CIPHERED_FILE, --ciphered_file OUTPUT_CIPHERED_FILE:
      Path to output file to place ciphered text. If not
      used then deciphered text will be dumped to console.
    + -c CHARSET, --charset CHARSET:
      Default charset is: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefgh
      ijklmnopqrstuvwxyz1234567890 !?., but you can set here
      another.

## Attack
Attack a ciphered text to get its plain text.

|    `$ cifra attack ALGORITHM_NAME FILE_TO_ATTACK`

* positional arguments:
    + ALGORITHM_NAME: Algorithm to attack.
    + FILE_TO_ATTACK: Path to file with text to attack.

* optional arguments:
    + -o OUTPUT_CIPHERED_FILE, --ciphered_file OUTPUT_CIPHERED_FILE:
      Path to output file to place deciphered text. If not
      used then deciphered text will be dumped to console.
    + -k, --output_recovered_key:
      Include guessed key in output. If not used only recovered text is output.
    + -c CHARSET, --charset CHARSET:
      Default charset is: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefgh
      ijklmnopqrstuvwxyz1234567890 !?., but you can set here
      another.

# ALGORITHMS
Currently these algorithms are available:

* caesar
* substitution
* transposition
* affine
* vigenere

# BUGS
Report issues at: <https://github.com/dante-signal31/cifra-rust/issues>

# INSTALLATION
To install Cifra refer to its installation instructions: <https://github.com/dante-signal31/cifra/wiki/Installation>

# AUTHOR
Dante Signal31 <dante.signal31@gmail.com>

# SEE ALSO
Website: <https://github.com/dante-signal31/cifra-rust>

# COPYRIGHT
Copyright (c) 2021 Dante-Signal31 <dante.signal31@gmail.com>. All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
following conditions are met:

    1. Redistributions of source code must retain the above copyright notice, this list of conditions and the
    following disclaimer.
    2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
    following disclaimer in the documentation and/or other materials provided with the distribution.
    3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or
    promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.