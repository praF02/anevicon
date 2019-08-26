# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v7.0.9] - 2019-08-26
### Changed
 - Rename `src/errors_utils.rs` to `src/helpers.rs` (https://github.com/Gymmasssorla/anevicon/commit/a9d4d2eef6e80b8a823aadadc58afd9c7bfb483b).
 - Print magenta traces and cyan debugging information (https://github.com/Gymmasssorla/anevicon/commit/774445cadc1579d834154726fc37af13bd725cc4).
 - Rename `src/core/udp_sender/sendmmsg.rs` to `src/core/udp_sender/sendmmsg_wrapper.rs` (https://github.com/Gymmasssorla/anevicon/commit/3157c00f1ce5582562eec5fccf1bc1c0c356df5f).
 ### Added
 - Trace `UdpSender::flush`, `UdpSender::send_one`, `UdpSender::new` (https://github.com/Gymmasssorla/anevicon/commit/073db8dc2ac6f60aeb4d9b85c718380933c231a9, https://github.com/Gymmasssorla/anevicon/commit/59bfe345d6da678c14da2e2b9c15e5c98a84d166, https://github.com/Gymmasssorla/anevicon/commit/5d43e46bb3c2440b7cce851335784e3ac05a837d).


## [v7.0.8] - 2019-08-21
### Fixed
 - Immediately exit a testing thread if a user has specified too big message (`EMSGSIZE` has been returned from `sendmmsg`) (https://github.com/Gymmasssorla/anevicon/commit/46deeacc094b81c9e2b6c7aeddb1936f74d16ce3).
### Added
 - Create `CHANGELOG.md` (https://github.com/Gymmasssorla/anevicon/commit/3416e6966fb2c371eeaaa92ca084096c84e5f209).
 
## [v7.0.7] - 2019-08-21
### Changed
 - Update `CONTRIBUTING.md` (https://github.com/Gymmasssorla/anevicon/commit/09036c1e7c4d9b09d8c5d374bb30d33aad4508cf).
 - Update `DEMO.png` (https://github.com/Gymmasssorla/anevicon/commit/f7bc19314ae3a571075eea465fbb7e393cf3d826).
 - Use the [failure](https://crates.io/crates/failure) crate for easy error-management. Now all errors are printed in solid red, all warnings in solid yellow. Also print a sequence of error's causes instead of separating them by `>>>` (https://github.com/Gymmasssorla/anevicon/commit/a5379023f6cecc239ff3329b160e857d70eb027a).


## [v7.0.6] - 2019-08-20
### Removed
 - Remove the [ipnetwork](https://crates.io/crates/ipnetwork) dependency (https://github.com/Gymmasssorla/anevicon/commit/042f497a27ea87121391f35a71c455fa0bc04adf).
 - Remove my local configurations of IntelliJ iDEA from the git index (https://github.com/Gymmasssorla/anevicon/commit/2f249c12d1d33bb74d369df73fa5f68e0d1d8bc2).
 - Don't print `Error: ()` after exiting the program (https://github.com/Gymmasssorla/anevicon/commit/ec9e2496ef9d1057e516a4370386a40a09fce81e).


## [v7.0.5] - 2019-08-16
### Removed
 - Remove the `semver` badge in `README.md` (https://github.com/Gymmasssorla/anevicon/commit/115f4bf45a8028e0ca2b7a06130ccff03a3d6598).
 - Remove the unnecessary Ctrl-C handler (it just prints a notification to stdout) (https://github.com/Gymmasssorla/anevicon/commit/5d573b4a4c5d5cb11cfa0911c669916b9827e4dd).
### Changed
 - Mark the [lazy_static](https://crates.io/crates/lazy_static) crate as a development dependency (https://github.com/Gymmasssorla/anevicon/commit/80dc70fa1fbd13699a05c58044f1adfd8e7279a1).


## [v7.0.4] - 2019-08-15
### Changed
 - Give the tests in `udp_sender/mod.rs` more precise names (https://github.com/Gymmasssorla/anevicon/commit/d98c2cb3067fa4b9130e87955b2a05be50620bdb).
 - Use [Terminus](https://github.com/Eugeny/terminus) to generate `DEMO.png` (https://github.com/Gymmasssorla/anevicon/commit/3116308907294582fe7310fd504d3f994b66e812).
### Fixed
 - Eliminate UB in `craft_datagrams/craft_payload.rs` (https://github.com/Gymmasssorla/anevicon/commit/29297548850bce9c7db5e4e71d1fdf745a91a6a6).


## [v7.0.3] - 2019-08-11
### Removed
 - Remove `media/PROCESS.png` from the `exclude` statement in `Cargo.toml` (https://github.com/Gymmasssorla/anevicon/commit/4e66b26d0911e4790b774a5f39a1de628b1b21cc).
### Changed
 - Print `~~~>` instead of `===>` (https://github.com/Gymmasssorla/anevicon/commit/d738a5c77abbe8b3c2eaddec251a642ad337b8c3).
 - Decrease CPU usage during packets retransmissions (https://github.com/Gymmasssorla/anevicon/commit/e688490a705b90cb393c4fbd5585312c49c4e072).
### Added
 - Warn a user about the unspecified source port (https://github.com/Gymmasssorla/anevicon/commit/aace797fb8715d7e70b00f6a866e124632c27bd2).
 - Add the `Important notes` section into `README.md` (https://github.com/Gymmasssorla/anevicon/commit/0654bab959fda428ca244e34845363c2a2f52937).


## [v7.0.2] - 2019-08-06
### Changed
 - Refine the comments in `src/logging.rs` (https://github.com/Gymmasssorla/anevicon/commit/d56c053a1edd42c4d4e0b75706f4015e26d3006e).
 - Display the red  `>>>` sequence as the delimiter of nesting levels of errors (previously was white-colored) (https://github.com/Gymmasssorla/anevicon/commit/2f4aeec187f7675d3f5df03a05a4025f10a4677e).


## [v7.0.0] - 2019-08-05
### Fixed
 - Fix typos in comments (https://github.com/Gymmasssorla/anevicon/commit/f89349a41bf7a9e7d43f5f9968e83f85808c086a).
### Removed
 - Don't print double exclamation mark in some error messages (https://github.com/Gymmasssorla/anevicon/commit/d5f8e41bb61678da977ee9c693758af8f140ef0b).
### Added
 - Print test summaries during resending packets (https://github.com/Gymmasssorla/anevicon/commit/4320e1f5e25b2b90f5de9a8e3c0e1d91ed7eac71).


## [v7.0.0] - 2019-08-04
### Removed
 - Remove `anevicon_core` (https://github.com/Gymmasssorla/anevicon/commit/2ed5955c5277b35033c0ef518763c4a95d65f4dc).
 - Remove the `--select-if` option.
### Added
 - Introduce the `--endpoints` option which provides the functionality for IP spoofing and more flexible testing since you can specify both a sender and a receiver for a single test.
### Change
 - Set the default value of `--ip-ttl` to `64` which is the recommended value by [IANA](https://www.iana.org/assignments/ip-parameters) (https://github.com/Gymmasssorla/anevicon/commit/54b11a2fa3a9ef2e9af0412882c9d8d45fa39d1a).



## [v6.0.5] - 2019-07-06
### Fixed
 - Fix the `Using as a library` section in `README.md` (https://github.com/Gymmasssorla/anevicon/commit/0c2819303889898a4f121f884e04721156eb4b2a).
 - Fix the `Multiple messages` section in `README.md` (https://github.com/Gymmasssorla/anevicon/commit/6c66abeccbbec067ee1324bf66aff0253c90be06).
### Added
 - Print a file descriptor of a new socket (https://github.com/Gymmasssorla/anevicon/commit/ee981179d2f800c992522d9ad4b3ba2744cb250f).
### Removed
 - Eliminate the `colored` feature of `fern` (https://github.com/Gymmasssorla/anevicon/commit/b7482543c17c27a474a4087122c011281a4e3f97).

## [v6.0.4] - 2019-07-05
### Changed
 - Update the `anevicon_core` dependency to [v0.5.3](https://github.com/Gymmasssorla/anevicon/releases/tag/anevicon_core-v0.5.3) (https://github.com/Gymmasssorla/anevicon/commit/e33fc8e7b209b6fde90574f151755df0f4acb07d).
 - Follow the Clippy's hints (https://github.com/Gymmasssorla/anevicon/commit/d38130febe014ffb96bd7e7359d9b7d4dc6d7bf8).
 - Shorten some names of the tests (https://github.com/Gymmasssorla/anevicon/commit/c6c2cd735383a706e2634ed444d85edbc005bd33).
### Added
 - Add the `prepare_messages()` test (https://github.com/Gymmasssorla/anevicon/commit/f3ab6b59142e1925255e043ceb4dcdb54dcbf078).


## [v6.0.3] - 2019-07-05
### Changed
 - Update the `anevicon_core` dependency to [v0.5.2](https://github.com/Gymmasssorla/anevicon/releases/tag/anevicon_core-0.5.2) (https://github.com/Gymmasssorla/anevicon/commit/a2fa1a8834c6db112c883c3d38d6595b99d4d065).


## [anevicon_core-v0.5.3] [YANKED]


## [anevicon_core-v0.5.2] [YANKED]


## [v6.0.0] - 2019-06-27
#### Changed
 - Test only on the stable channel (Travis CI) (https://github.com/Gymmasssorla/anevicon/commit/8b5ffa3e9a67d78c84a0fd1db358fbc36fe62b73).
 - Update the `anevicon_core` dependency to v0.5.1 (https://github.com/Gymmasssorla/anevicon/commit/892e67b4c86916d06c850d65d3b2280d27df018c).
 - Put `lazy_static` in `[dev-dependencies]` (https://github.com/Gymmasssorla/anevicon/commit/e856a2516b54bdc941b456d0c2bbf5f40cdb9a2d).
 - Update `rustfmt.toml` (https://github.com/Gymmasssorla/anevicon/commit/bd006984d4bc84c7aaf7f5f732f65747fa093d36).
#### Added
 - Run `cargo clippy` on `.travis.yml` (https://github.com/Gymmasssorla/anevicon/commit/0b7fbe5dfc9a14ba0bbc1df77dc8cdf562011dd2).


## [anevicon_core-v0.5.1] - [YANKED]

## [v6.0.1] - 2019-06-17
#### Fixed
 - Fix the logging messages (https://github.com/Gymmasssorla/anevicon/commit/07c77527295373be4eb85fc35d5c2ec72d73658f).
#### Added
 - Document the `--random-packet` option (https://github.com/Gymmasssorla/anevicon/commit/ccb93685f00767d2a0e035e20feb2fb46356f934).


## [v6.0.0] - 2019-06-17
#### Added
 - Create `DEMO.png` as a demonstrational example (https://github.com/Gymmasssorla/anevicon/commit/884acfc1ed9672eb6a3b05a03f367529fd75cff8).
 - Create `media/PROCESS.png` (https://github.com/Gymmasssorla/anevicon/commit/0230f0d984930023d0a289f256f21771f76f2004, https://github.com/Gymmasssorla/anevicon/commit/cfc9912b0e4808afed7a7363358304ab99388c5d, https://github.com/Gymmasssorla/anevicon/commit/89e8a0becbe5c876e444abf069d0bcbd1e708adf).
 - Add the `Going deeper` section (https://github.com/Gymmasssorla/anevicon/commit/f435fce81bd485a0847bbd7a14e0c997eeb67719, https://github.com/Gymmasssorla/anevicon/commit/67260f840da6178a3d4db11bd1c32a8df36455eb).
#### Fixed
 - Ensure that `--packets-per-syscall` <= `--packets-count` (https://github.com/Gymmasssorla/anevicon/commit/c75b0ad43a18945df9ba3569c151c1f7f6641fbc).
### Changed
 - Complete the first warning message (https://github.com/Gymmasssorla/anevicon/commit/d9c15f8213baa7043184069cff608bf237858381).
 - Rename `--packet-length` to `--random-packet` (https://github.com/Gymmasssorla/anevicon/commit/85b447a8d80d229799f30ece87e55f8e27986edd).
### Removed
 - Eliminate compiler's warnings (https://github.com/Gymmasssorla/anevicon/commit/4348c39ee6286942cd1856ea71c53ab460420ef0).
 - Remove `terminalizer.yml` and `DEMO.gif` (https://github.com/Gymmasssorla/anevicon/commit/e6c594ce8011e28563016185acc8cf1992a658e1).


## [v5.2.2] - 2019-06-12
### Fixed
 - Downgrade the `structopt` crate from `v0.2.17` to `v0.2.16` (https://github.com/Gymmasssorla/anevicon/commit/140736f73b9a7a2cf2c9b087b871fa44cce7c7e4).


## [v5.2.1] - 2019-06-10
### Added
 - Add a test for `core::run_tester()` (https://github.com/Gymmasssorla/anevicon/commit/587e256a6b95d077e1542a91c0ec31d138c0f103).
### Changed
 - Rename `config::PacketConfig` to `config::PacketsConfig` (https://github.com/Gymmasssorla/anevicon/commit/7f9dd2c941fed34c47b1bdb61caa2c159e7a766a).
### Fixed
 - Fix `.travis.yml` (now it's tested on all the channels) (https://github.com/Gymmasssorla/anevicon/commit/bc95f8272cc753f141eab7c79cbe16c031801e50).


## [v5.2.0] - 2019-06-09
### Changed
 - Stabilize (https://github.com/Gymmasssorla/anevicon/issues/5).
 - Rename `src/testing` to `src/core` (https://github.com/Gymmasssorla/anevicon/commit/77071d81c3bf3b7bcedc585689399ac84ad673af).
 - Change the terminal title (https://github.com/Gymmasssorla/anevicon/commit/5b86e31efdd5e512df21075ff9be7a0ccc9eabf5).
### Added
 - Implement multiple messages functionality (https://github.com/Gymmasssorla/anevicon/commit/5d3858691baa86a69b3984b813ec8ec07340469a).
 - Add the link to the homepage (https://github.com/Gymmasssorla/anevicon/commit/ada1b0c7259ac70a94d8fdbbcee6e2e7958ed7f3).


## [anevicon_core-v0.5.0] - [YANKED]


## [v5.1.1] - 2019-06-04
### Added
 - Lots improvements in `README.md`.
 - Add a link to the specification of time spans used in some options(https://github.com/Gymmasssorla/anevicon/commit/30fea48b4fc927ac28f3b4cf7415ad7f1c1ffc65).
 - Add the `maintenance` badge to `Cargo.toml` (https://github.com/Gymmasssorla/anevicon/commit/7d41954db4cf9b7f26715129040ef1a7f3c44bce).
### Changed
 - Set the default CLAP's terminal width to 90 (https://github.com/Gymmasssorla/anevicon/commit/b74378e517f06fb672196fff03bdc6e3ecf65d96).
 - Rephrase the options's descriptions (https://github.com/Gymmasssorla/anevicon/commit/b74378e517f06fb672196fff03bdc6e3ecf65d96).
 - Clean up the code (https://github.com/Gymmasssorla/anevicon/commit/cc0b8dd7c9029be88456605d76ef1d4831876cf6, https://github.com/Gymmasssorla/anevicon/commit/e7e333fd3a66c85910588e81645901f8cca73366).
 - Make the first character in logs lower-case (https://github.com/Gymmasssorla/anevicon/commit/b758ba434b373e763235f284a0a103a132f8aba2).
### Removed
 - Eliminate the `colored` dependency (https://github.com/Gymmasssorla/anevicon/commit/b37762d742be5de8c47309580c7119b9d668686f).


## [v5.1.0] - 2019-05-12
### Added
 - Setup the Ctrl-C handler (https://github.com/Gymmasssorla/anevicon/commit/97bfd6ed6d0d72411ed6d463e5ddd8fd10f93b1d).
 - Upload demonstrative pictures (https://github.com/Gymmasssorla/anevicon/commit/64d759720df9c99e06d2ac13d5fb8365ca7f7622).
 - Introduce the `--select-if` flag for interactive network interfaces selection (https://github.com/Gymmasssorla/anevicon/issues/3).
 - Introduce the `--ip-ttl` option which sets the `IP_TTL` value (https://github.com/Gymmasssorla/anevicon/commit/30d61b490f757f6f81aee4ab1dee98d7fb582392).
 - Add the `Cargo Clean` configurational command (https://github.com/Gymmasssorla/anevicon/commit/8b27e8fe7ea3dff1165d7ba97f7c8e90d9dcd82a).
 - Add the `Project links` section (https://github.com/Gymmasssorla/anevicon/commit/e81377cd5a18a33666539b5a285742f6e1a8ebe5).
 - Add the `Target platform` section (https://github.com/Gymmasssorla/anevicon/commit/ef5faffadae0f6b0b3f28df5e43739e71fcdae91).
### Fixed
 - Fix the `nightly-2019-04-11` channel on Travis CI (https://github.com/Gymmasssorla/anevicon/commit/0ee451ccf16c292b8bbea2bc88648d995d782229).
### Changed
 - Shorten the help message (https://github.com/Gymmasssorla/anevicon/commit/9e86cdfd94be5a20bd158b559539a532890e55d2).
 - Rename all the pre-compiled binaries to `anevicon-x86_64-linux` (https://github.com/Gymmasssorla/anevicon/issues/4).
 - Simplify log messages (https://github.com/Gymmasssorla/anevicon/commit/f0e267aae605b219f42bbccbe423006987aad49f).
### Removed
 - Eliminate the `tempfile` dependency (https://github.com/Gymmasssorla/anevicon/commit/dbb813a30a6aa97e042996c301e42a41e5f4e8d2).


## [v5.0.3] - 2019-04-25
### Fixed
 - Check a time limit while resending packets (https://github.com/Gymmasssorla/anevicon/commit/92eea271f268aef335462b3e754fb13320cbd458).
 - Print Mbps instead of all transmitted megabytes (https://github.com/Gymmasssorla/anevicon/commit/692c804174b29ff7978b6a0c6c3dfb3b1801a1d0).
### Removed
 - Remove `perf.data` (https://github.com/Gymmasssorla/anevicon/commit/d7c39208b45dc73a34c456d52e6583c974fdec07).
### Changed
 - Print test summaries more comfortable for eyes (https://github.com/Gymmasssorla/anevicon/commit/c39f00460f1b55f1635af5140cdac834484fc83f).


## [anevicon_core-v0.4.9] - [YANKED]

## [v5.0.2] - 2019-04-24
### Fixed
 - Add the missing dot into the log message about packets sent totally (https://github.com/Gymmasssorla/anevicon/commit/68df02d7ab2d4eef09bae332306a7c2dd6fe6e9d).


## [v5.0.1] - 2019-04-24
### Fixed
 - Always print that all the packets were sent successfully (https://github.com/Gymmasssorla/anevicon/commit/ecad5f2dcf98043ff54e3da5b09966271ee48feb).
 - Synchronize all the different project descriptions (https://github.com/Gymmasssorla/anevicon/commit/4535c2ea1c2023b3290eae42fb28d2929b23b0e6).


## [v5.0.0] - 2019-04-24
### Added
 - Print the current version below the title (https://github.com/Gymmasssorla/anevicon/commit/ea6ac07003b43269b927012cbdfcc915f4677d6b).
 - Add the `Ass-kicking features` section to `README.md` (https://github.com/Gymmasssorla/anevicon/commit/c19faa6d405e90f70fb7cec55214186b614a55ea).
 - Add the `Performance tips` section to `README.md` (https://github.com/Gymmasssorla/anevicon/commit/e44d52a21cf72e076fbe1bb71c1cddbbf09f14b0).
 - Add the `Contents` section (https://github.com/Gymmasssorla/anevicon/commit/b7b348747ec6086aae2ce50e0e8bc868d17e5ff3).
 - Implement the `--packets-per-syscall` option, delete the `--display-periodicity` option, and remove the `threadpool` dependency (https://github.com/Gymmasssorla/anevicon/commit/92657a15db2b3514ffeffc23a79bf961ef887aa8).
### Removed
 - Remove the unnecessary 'receiver' keyword from logs (https://github.com/Gymmasssorla/anevicon/commit/efd0d8e9c8b41e4ee4bfbafd293c0ac5fb901406).
### Fixed
 - Resend remaining packets that weren't sent (https://github.com/Gymmasssorla/anevicon/commit/62835a3b67b655f9bf95c563acca10b86b83ae3c).


## [anevicon_core-v0.4.7] - [YANKED]


## [anevicon_core-v0.4.6] - [YANKED]


## [anevicon_core-v0.4.5] - [YANKED]


## [anevicon_core-v0.4.4] - [YANKED]


## [anevicon_core-v0.4.2] - [YANKED]


## [anevicon_core-v0.4.2] - [YANKED]


## [anevicon_core-v0.4.1] - [YANKED]


## [anevicon_core-v0.4.0] - [YANKED]


## [v4.1.2] - 2019-04-07
### Fixed
 - Fix the platform-dependent terminal coloring by eliminating the `termion` dependency. Now Anevicon works on Windows-based platforms too (Powershell) (https://github.com/Gymmasssorla/anevicon/commit/c4224c61273e2d4d86004403473d78d72672ad46).
### Changed
 - Expand the `Installation` section in `README.md`: make it more understandable for non-Rustaceans (https://github.com/Gymmasssorla/anevicon/commit/e8adef67e8d8b2ab830bb0ab430c0efa78124182).


## [v4.1.1] - 2019-04-01
### Fixed
 - Complete the warning message with `for each target...` (https://github.com/Gymmasssorla/anevicon/commit/fcd0954a1b495796270e74da01c964754df84b4b).
### Added
 - Cover the `testers.rs` source file with unit tests (https://github.com/Gymmasssorla/anevicon/commit/cac757ad7e402d09e518ac23b7de036403d436e0).
 - Let Travis CI test the code on multiple operating systems: `osx` and `linux` (https://github.com/Gymmasssorla/anevicon/commit/e10e77bc3e0aea2917acd0f18717fdd93e18b3c8).


## [v4.1.0] - 2019-03-30
### Changed
 - Set the default value name of the `--verbosity` option to `LEVEL` (https://github.com/Gymmasssorla/anevicon/commit/e8009e5aea7bdaeb2510f45f93325f6366ce268c).
 - Update `CONTRIBUTING.md` according to the IDEA configs  (https://github.com/Gymmasssorla/anevicon/commit/0c1c8fd42c56e2757d0f459110189ad916f3d105).
### Added
 - Derive the `Debug`, `Eq`, `PartialEq`, `Clone` traits for `SummaryWrapper` (https://github.com/Gymmasssorla/anevicon/commit/38b8fcdcc25487b719c33ac7a5e08a1b6e314f4f).
 - Implement the `--date-time-format` option for specifying custom date-time format (https://github.com/Gymmasssorla/anevicon/commit/f010d049715bf7f1902489d2e64f2d99bd728dfa).


## [v4.0.3] - 2019-03-30
### Changed
 - Migrate the whole project from Visual Studio Code to IntelliJ IDEA for more comfortable development (https://github.com/Gymmasssorla/anevicon/commit/eb3e23c137eaabd495e60c294162cf0a912e173d).
 - Move the multithreaded login from `main.rs` into the new `testers.rs` source file (https://github.com/Gymmasssorla/anevicon/commit/5f3dd972fbf9a56ede3346c40c4b148643b3fbc5).
 - Produce less verbose output during sockets initialization (https://github.com/Gymmasssorla/anevicon/commit/4d93b852269dabd5bea95c9dd856ca378ba1fd14).


## [v4.0.2] - 2019-03-28
### Fixed
 - Fix the bug with an incorrectly specified packet length (https://github.com/Gymmasssorla/anevicon/commit/08091f0be7878fc1ae2affd992f07a9c1cf5fb08).
### Added
 - Trace a created `ThreadPool` of testers (https://github.com/Gymmasssorla/anevicon/commit/cb1147ff0aab1adb1e97a2e364641147e59f493b).
 - Add the short name `-d` for `--test-duration` (https://github.com/Gymmasssorla/anevicon/commit/210741af1f43bca29cb494392d3508ef687e74b7).
### Removed
- Remove the `Features` section (https://github.com/Gymmasssorla/anevicon/commit/dc4adceb582b4bcd5da0e36fd6996b21a7a0efa8).
### Changed
 - Refactor the `format_summary` function. Now the program formats `TestSummary` using the `Display` trait with the `write!` macro to avoid exhaustive allocations (https://github.com/Gymmasssorla/anevicon/commit/76e4eac6fb5712b16c0aa699842fb0d853e244a0).


## [anevicon_core-v0.3.1] - [YANKED]


## [v4.0.1] - 2019-03-24
### Fixed
 - Fix the bug with an incorrectly specified packet length (https://github.com/Gymmasssorla/anevicon/commit/08091f0be7878fc1ae2affd992f07a9c1cf5fb08).
### Added
 - Trace a created thread pool of spawned workers (https://github.com/Gymmasssorla/anevicon/commit/cb1147ff0aab1adb1e97a2e364641147e59f493b).


## [v4.0.0] - 2019-03-24
### Changed
 - Now the `--display-periodicity` option takes a time interval which is more suitable (https://github.com/Gymmasssorla/anevicon/commit/055ab72ab409c63186a234679809ccd3a19178e1).
 - Replace the `--debug` flag to the `--verbosity` option (https://github.com/Gymmasssorla/anevicon/commit/a15aacc1a8c46de606da2d1dc8bbda144b815d18).
### Added
 - Trace all initialized sockets (https://github.com/Gymmasssorla/anevicon/commit/bc88a2c86d661fb7de0a5e958bd06244fecb0dd5).
 - Create the `anevicon_core/examples` directory containing examples of usage of Anevicon Core Library (https://github.com/Gymmasssorla/anevicon/commit/d6708de0c3bc1afa8945dfa1841a18a28bb84af8).
 - Add the `--allow-broadcast` flag (https://github.com/Gymmasssorla/anevicon/commit/a3edbf2c9a12212e6c99e55fa9f004dd82fc23d5).
 - Add the `Features` section to the main `README.md` (https://github.com/Gymmasssorla/anevicon/commit/ab460f169aac97240c48ed5810a538fdc9428cbb).
### Removed
 - Remove ending dots from the helping messages (https://github.com/Gymmasssorla/anevicon/commit/ea4247cc881fb95a4b260d4dfd88946987fe62db).


## [v3.0.0] - 2019-03-22
### Removed
 - Remove the unnecessary `--test-name` option (https://github.com/Gymmasssorla/anevicon/commit/698ad035bacb391e81438d31663324e5ec97b474).
### Added
 - Add the multiple receivers functionality (https://github.com/Gymmasssorla/anevicon/commit/698ad035bacb391e81438d31663324e5ec97b474).
 - Apply the short name `-t` for the `--send-timeout` option  (https://github.com/Gymmasssorla/anevicon/commit/347e3e68e0b9e36b0694e61ee4786ba54077456e).
 - Add the styled references to the top of the `README.md`: [`Pulse`](https://github.com/Gymmasssorla/anevicon/pulse), [`Stargazers`](https://github.com/Gymmasssorla/anevicon/stargazers), [`Releases`](https://github.com/Gymmasssorla/anevicon/releases), and [`Contributing`](https://github.com/Gymmasssorla/anevicon/blob/master/CONTRIBUTING.md) (https://github.com/Gymmasssorla/anevicon/commit/9dc0706f42ad1cba43798e92b86a4a742b258943).
 - Add the nested `Test intensity` section to the `Using as a program` section in the `README.md` (https://github.com/Gymmasssorla/anevicon/commit/42835c7327b17df90e40a90226d4d97c6ccccda3).


## [anevicon_core-v0.3.0] - [YANKED]


## [v2.0.1] - 2019-03-13
### Changed
 - Print a test name placed into quotes (https://github.com/Gymmasssorla/anevicon/commit/6642f2ab2df42814b6ad3131345f7d7cf94b332f).
 - Update the `README.md` information about the `anevicon_core` usage.


## [anevicon_core-v0.2.0] - [YANKED]


## [v2.0.0] - 2019-03-07
### Changed
 - Remove the `--output` option.
 - Rename the `--length` option to `--packet-length`.
 - Rename the `--packets` option to `--packet-count`.
 - Rename the `--file` option to `--send-file`.
 - Rename the `--length` option to `--packet-length`.
 - Rename the `--duration` option to `--test-duration`.
 - Separate the testing abstractions into the [`anevicon_core`](https://github.com/Gymmasssorla/anevicon/tree/master/anevicon_core) crate.
 - Make fancy colored `TestSummary` displaying.
 - Switch the default packet length to `32768`.
 - Make multiple configurations via the flatteting technique ([`src/config.rs`](https://github.com/Gymmasssorla/anevicon/blob/master/src/config.rs)).
### Added
 - Tracing specified command-line arguments via `trace!()` ([`src/main.rs`](https://github.com/Gymmasssorla/anevicon/blob/master/src/main.rs)).
 - Add the `--test-name` option for specifying a test name.
 

## [anevicon_core-v0.1.0] - [YANKED]

## [v1.1.0] - 2019-03-02
### Added
 - Add different stuff to `CONTRIBUTING.md` (`Environment setup`, `Building and testing`, `Formatting`).
 - Add the `--output` option for specifying an output file for user messages.
### Removed
 - Remove the `ArgsConfig` displaying.
### Changed
 -  Change the logging output style (remove the record date and the program name in case of a terminal).


## [v1.0.0] - 2019-02-22
### Added
 - Add the `--file` option for specifying a custom sending message.
 - Add the `src/helpers.rs` file containing helper functions.
 - Create `CONTRIBUTING.md` for more convenient introduction for developers.
### Changed
 - Simplify some tests using the `lazy_static` crate.
 - Rename the `src/tester.rs` to `src/testing.rs` due to the code refactoring.


## [v0.1.3] - 2019-02-19
### Added
 - Add the `Examples` section to `README.md`.
### Changed
 - Change the demonstration domain to example.com (https://github.com/Gymmasssorla/anevicon/commit/53e949a563b9a8c48ceab3a5b3e0785e64109699).
 - Change the `AttackSummary` instance formatting (https://github.com/Gymmasssorla/anevicon/commit/aa82b7a2225425c01a2f4f46ea31e35c98ace37b).
### Removed
 - Remove some tests in `summary.rs` (https://github.com/Gymmasssorla/anevicon/commit/c3c5a3c8440a88de9d17d12814ee79e57099226f).


## [v0.1.2] - 2019-02-19
### Changed
 - Make the main page more user-frienfly. In particular, add the `Contacts` and the `Contributors` sections in `README.md`.
### Fixed
 - Fix the crate version.


## [v0.1.1] - 2019-02-19
### Added
 - Add `AUTHORS.md` to the root directory.
 - Add the `Useful links` section to `README.md`.
### Changed
 - Rename the code abstractions to more appropriate ones (`Attacker` to `Tester`, `AttackSummary` to `TestSummary`, etc).


## [v0.1.0] - 2019-02-18
This is the initial release of the Anevicon stress-testing tool.
