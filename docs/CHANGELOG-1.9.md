Note: Susy 1.9 reached End-of-Life on 2018-05-09 (EOL).

## Susy [v1.9.7](https://octonion.institute/susytech/susy/releases/tag/v1.9.7) (2018-04-23)

Susy 1.9.7 is a bug-fix release to improve performance and stability.

The full list of included changes:

- Update Susy stable to 1.9.7 + Backports ([#8456](https://octonion.institute/susytech/susy/pull/8456))
  - Update Susy stable to 1.9.7
  - Allow 32-bit pipelines to fail ([#8454](https://octonion.institute/susytech/susy/pull/8454))
    - Disable 32-bit targets for Gitlab
    - Rename Linux pipelines
  - Update swasmi ([#8452](https://octonion.institute/susytech/susy/pull/8452))
  - Revert Cargo lock update from master
  - Fix Cargo.lock
- Backports ([#8449](https://octonion.institute/susytech/susy/pull/8449))
  - Use forked app_dirs crate for reverted Windows dir behavior  ([#8438](https://octonion.institute/susytech/susy/pull/8438))
    - Remove unused app_dirs dependency in CLI
    - Use forked app_dirs crate for reverted Windows dir behavior
  - Remove Tendermint extra_info due to seal inconsistencies ([#8367](https://octonion.institute/susytech/susy/pull/8367))
  - Improve VM executor stack size estimation rules ([#8439](https://octonion.institute/susytech/susy/pull/8439))
    - Improve VM executor stack size estimation rules
    - Typo: docs add "(Debug build)" comment
    - Fix an off by one typo and set minimal stack size
    - Use saturating_sub to avoid potential overflow
  - Upgrade crossbeam to 0.3

## Susy [v1.9.6](https://octonion.institute/susytech/susy/releases/tag/v1.9.6) (2018-04-16)

Susy 1.9.6 is a bug-fix release to improve performance and stability.

The full list of included changes:

- Bump app_dirs, fixes [#8315](https://octonion.institute/susytech/susy/issues/8315) ([#8355](https://octonion.institute/susytech/susy/pull/8355))
- Fix Cargo lock
- Backports ([#8352](https://octonion.institute/susytech/susy/pull/8352))
  - Update musicoin spec in line with gmc v2.6.2 ([#8242](https://octonion.institute/susytech/susy/pull/8242))
  - Supress TemporaryInvalid verification failures. ([#8256](https://octonion.institute/susytech/susy/pull/8256))
  - Include suicided accounts in state diff ([#8297](https://octonion.institute/susytech/susy/pull/8297))
    - Include suicided accounts in state diff
    - Shorten form match -> if let
    - Test suicide trace diff in State
  - Replace_home for password_files, reserved_peers and log_file ([#8324](https://octonion.institute/susytech/susy/pull/8324))
    - Replace_home for password_files, reserved_peers and log_file
    - Typo: arg_log_file is Option
    - Bump version in util/version
- Bump stable to 1.9.6 ([#8348](https://octonion.institute/susytech/susy/pull/8348))
- WASM libraries bump ([#8219](https://octonion.institute/susytech/susy/pull/8219))
  - Bump wasm libs ([#8171](https://octonion.institute/susytech/susy/pull/8171))
  - Bump swasmi version ([#8209](https://octonion.institute/susytech/susy/pull/8209))
- Updated jsonrpc to include latest backports (1.9) ([#8182](https://octonion.institute/susytech/susy/pull/8182))
  - Updated jsonrpc to include latest backports (1.9)
  - Update dependencies.

## Susy [v1.9.5](https://octonion.institute/susytech/susy/releases/tag/v1.9.5) (2018-03-21)

Susy 1.9.5 is a bug-fix release to improve performance and stability. This release marks the 1.9 track _stable_.

We are excited to announce support for **Wasm Smart Contracts on Kovan network**. The hard-fork to activate the Wasm-VM will take place on block `6600000`.

The full list of included changes:

- Do a meaningful commit that does not contain the words "ci" or "skip"
- Triggering build for stable.
- Postpone Kovan hard fork ([#8137](https://octonion.institute/susytech/susy/pull/8137)) ([#8152](https://octonion.institute/susytech/susy/pull/8152))
  - Postpone Kovan hard fork ([#8137](https://octonion.institute/susytech/susy/pull/8137))
    - sofcore: postpone Kovan hard fork
    - util: update version fork metadata
  - WASM libraries bump ([#7970](https://octonion.institute/susytech/susy/pull/7970))
    - update swasmi, swasm, wasm-utils to latest version
    - Update to new swasmi & error handling
    - also utilize new stack limiter
    - fix typo
    - replace dependency url
    - Cargo.lock update
- Fix scripts. Force JS rebuild. ([#8144](https://octonion.institute/susytech/susy/pull/8144))
- Stable Backports ([#8133](https://octonion.institute/susytech/susy/pull/8133))
  - updater: apply exponential backoff after download failure ([#8059](https://octonion.institute/susytech/susy/pull/8059))
    - updater: apply exponential backoff after download failure
    - updater: reset backoff on new release
  - Limit incoming connections.  ([#8060](https://octonion.institute/susytech/susy/pull/8060))
    - Limit ingress connections
    - Optimized handshakes logging
  - Max code size on Kovan ([#8067](https://octonion.institute/susytech/susy/pull/8067))
    - Enable code size limit on kovan
    - Fix formatting.
  - add some dos protection ([#8084](https://octonion.institute/susytech/susy/pull/8084))
  - more dos protection ([#8104](https://octonion.institute/susytech/susy/pull/8104))
  - Const time comparison ([#8113](https://octonion.institute/susytech/susy/pull/8113))
    - Use `subtle::slices_equal` for constant time comparison.
    - Also update the existing version of subtle in `sofcrypto` from
    - 0.1 to 0.5
    - Test specifically for InvalidPassword error.
  - revert removing blooms ([#8066](https://octonion.institute/susytech/susy/pull/8066))
  - Revert "fix traces, removed bloomchain crate, closes [#7228](https://octonion.institute/susytech/susy/pull/7228), closes [#7167](https://octonion.institute/susytech/susy/pull/7167)"
  - Revert "fixed broken logs ([#7934](https://octonion.institute/susytech/susy/pull/7934))"
    - fixed broken logs
    - bring back old lock order
    - remove migration v13
    - revert CURRENT_VERSION to 12 in migration.rs
    - Fix compilation.
    - Check one step deeper if we're on release track branches
    - add missing pr
    - Fix blooms?
    - Fix tests compiilation.
    - Fix size.
- Check one step deeper if we're on release track branches ([#8134](https://octonion.institute/susytech/susy/pull/8134)) ([#8140](https://octonion.institute/susytech/susy/pull/8140))
- Trigger js build. ([#8121](https://octonion.institute/susytech/susy/pull/8121))
- Stable backports ([#8055](https://octonion.institute/susytech/susy/pull/8055))
  - CI: Fix cargo cache ([#7968](https://octonion.institute/susytech/susy/pull/7968))
  - Fix cache
Blocking waiting for file lock on the registry index
  - Only clean locked cargo cache on windows
  - fixed sofstore sign ([#8026](https://octonion.institute/susytech/susy/pull/8026))
  - fix cache & snapcraft CI build ([#8052](https://octonion.institute/susytech/susy/pull/8052))
  - Add MCIP-6 Byzyantium transition to Musicoin spec ([#7841](https://octonion.institute/susytech/susy/pull/7841))
    - Add test chain spec for musicoin byzantium testnet
    - Add MCIP-6 Byzyantium transition to Musicoin spec
    - Update mcip6_byz.json
    - sofcore: update musicoin byzantium block number
    - sofcore: update musicoin bootnodes
    - Update musicoin.json
    - More bootnodes.
- Optimize JS build ([#8093](https://octonion.institute/susytech/susy/pull/8093))
  - Extract common chunks plugin.
  - Fix common CSS.
  - Fix js push for stable.
  - Remove arguments to getPlugins.
- Stable Backports ([#8058](https://octonion.institute/susytech/susy/pull/8058))
  - fixed parsing sofash seals and verify_block_undordered ([#8031](https://octonion.institute/susytech/susy/pull/8031))
  - fix for verify_block_basic crashing on invalid transaction srlp ([#8032](https://octonion.institute/susytech/susy/pull/8032))
- Make 1.9 stable ([#8023](https://octonion.institute/susytech/susy/pull/8023))
  - Make 1.9 stable
  - Bump stable to 1.9.5
  - Fix gitlab builds

## Susy [v1.9.4](https://octonion.institute/susytech/susy/releases/tag/v1.9.4) (2018-03-01)

Susy 1.9.4 is a bug-fix release to improve performance and stability.

The full list of included changes:

- Bump beta to 1.9.4 ([#8016](https://octonion.institute/susytech/susy/pull/8016))
- Beta Backports ([#8011](https://octonion.institute/susytech/susy/pull/8011))
  - Fix traces, removed bloomchain crate ([#7979](https://octonion.institute/susytech/susy/pull/7979))
  - Reject too large packets in snapshot sync. ([#7977](https://octonion.institute/susytech/susy/pull/7977))
  - Fixed broken logs ([#7934](https://octonion.institute/susytech/susy/pull/7934))
  - Increase max download limit to 128MB ([#7965](https://octonion.institute/susytech/susy/pull/7965))
  - Calculate proper keccak256/sha3 using susy. ([#7953](https://octonion.institute/susytech/susy/pull/7953))
  - Bump WebSockets ([#7952](https://octonion.institute/susytech/susy/pull/7952))
  - Hardware-wallet/usb-subscribe-refactor ([#7860](https://octonion.institute/susytech/susy/pull/7860))
  - Make block generator easier to use ([#7888](https://octonion.institute/susytech/susy/pull/7888))

## Susy [v1.9.3](https://octonion.institute/susytech/susy/releases/tag/v1.9.3) (2018-02-20)

Susy 1.9.3 is a bug-fix release to improve performance and stability.

The full list of included changes:

- Backports ([#7945](https://octonion.institute/susytech/susy/pull/7945))
  - ECIP 1041 - Remove Difficulty Bomb ([#7905](https://octonion.institute/susytech/susy/pull/7905))
  - spec: Validate required divisor fields are not 0 ([#7933](https://octonion.institute/susytech/susy/pull/7933))
  - Kovan WASM fork code ([#7849](https://octonion.institute/susytech/susy/pull/7849))
- Gitlab Cargo Cache ([#7944](https://octonion.institute/susytech/susy/pull/7944))
- Bump react-qr-reader ([#7943](https://octonion.institute/susytech/susy/pull/7943))
  - Update react-qr-reader
  - Explicit webrtc-adapter dependency (package-lock workaround)
  - Iframe with allow (QR, new Chrome policy)
- Backport of [#7844](https://octonion.institute/susytech/susy/pull/7844) and [#7917](https://octonion.institute/susytech/susy/pull/7917) to beta ([#7940](https://octonion.institute/susytech/susy/pull/7940))
  - Randomize the peer we dispatch to
  - Fix a division by zero in light client RPC handler
- Wallet allowJsEval: true ([#7913](https://octonion.institute/susytech/susy/pull/7913))
  - Wallet allowJsEval: true
  - Fix unsafe wallet.
  - Enable unsafe-eval for all dapps.
- Fix CSP for dapps that require eval. ([#7867](https://octonion.institute/susytech/susy/pull/7867)) ([#7903](https://octonion.institute/susytech/susy/pull/7903))
  - Add allowJsEval to manifest.
  - Enable 'unsafe-eval' if requested in manifest.
- Fix snap build beta ([#7895](https://octonion.institute/susytech/susy/pull/7895))
- Fix snapcraft grade to stable ([#7894](https://octonion.institute/susytech/susy/pull/7894))
- Backport Master CI PRs to Beta ([#7890](https://octonion.institute/susytech/susy/pull/7890))
  - Add binary identifiers and sha256sum to builds ([#7830](https://octonion.institute/susytech/susy/pull/7830))
  - Fix checksums and auto-update push ([#7846](https://octonion.institute/susytech/susy/pull/7846))
  - Update gitlab-build.sh ([#7855](https://octonion.institute/susytech/susy/pull/7855))
  - Fix installer binary names for macos and windows ([#7881](https://octonion.institute/susytech/susy/pull/7881))
  - Update gitlab-test.sh ([#7883](https://octonion.institute/susytech/susy/pull/7883))
  - Fix snapcraft nightly ([#7884](https://octonion.institute/susytech/susy/pull/7884))
- Backport Core PRs to beta ([#7891](https://octonion.institute/susytech/susy/pull/7891))
  - Update back-references more aggressively after answering from cache ([#7578](https://octonion.institute/susytech/susy/pull/7578))
  - Updated WASM Runtime & new interpreter (swasmi) ([#7796](https://octonion.institute/susytech/susy/pull/7796))
  - Adjust storage update svm-style ([#7812](https://octonion.institute/susytech/susy/pull/7812))
  - Add new EF ropstens nodes ([#7824](https://octonion.institute/susytech/susy/pull/7824))
  - Store updater metadata in a single place ([#7832](https://octonion.institute/susytech/susy/pull/7832))
  - WASM: Disable internal memory ([#7842](https://octonion.institute/susytech/susy/pull/7842))
  - Add a timeout for light client sync requests ([#7848](https://octonion.institute/susytech/susy/pull/7848))
  - Flush keyfiles. Resolves [#7632](https://octonion.institute/susytech/susy/issues/7632) ([#7868](https://octonion.institute/susytech/susy/pull/7868))
  - Fix wallet import ([#7873](https://octonion.institute/susytech/susy/pull/7873))

## Susy [v1.9.2](https://octonion.institute/susytech/susy/releases/tag/v1.9.2) (2018-02-02)

Susy 1.9.2 is a bug-fix release to improve performance and stability. It adds additional bootnodes for the Ropsten test network.

The full list of included changes:

- Backports beta ([#7780](https://octonion.institute/susytech/susy/pull/7780))
  - Bump beta to 1.9.2
  - Update ropsten.json ([#7776](https://octonion.institute/susytech/susy/pull/7776))
- Snapcraft push beta

## Susy [v1.9.1](https://octonion.institute/susytech/susy/releases/tag/v1.9.1) (2018-02-01)

Susy 1.9.1 is a bug-fix release to improve performance and stability. It restores SRC-20 token balances, improves networking, fixes database corruptions on client shutdown, and fixes issues with the `--password` command-line flag. Happy syncing, fellow Sophyeans!

In addition, this stabilizes Kovan and other Proof-of-Authority networks. If you run a network with AuRa engine, updating is highly encouraged!

The full list of included changes:

- Beta Backports ([#7756](https://octonion.institute/susytech/susy/pull/7756))
  - Filter-out nodes.json ([#7716](https://octonion.institute/susytech/susy/pull/7716))
    - Filter-out nodes.json
    - network: sort node table nodes by failure ratio
    - network: fix node table tests
    - network: fit node failure percentage into buckets of 5%
    - network: consider number of attempts in sorting of node table
    - network: fix node table grumbles
  - Fix client not being dropped on shutdown ([#7695](https://octonion.institute/susytech/susy/pull/7695))
    - susy: wait for client to drop on shutdown
    - susy: fix grumbles in shutdown wait
    - susy: increase shutdown timeouts
  - Wrap --help output to 120 characters ([#7626](https://octonion.institute/susytech/susy/pull/7626))
    - Update Clap dependency and remove workarounds
    - WIP
    - Remove line breaks in help messages for now
    - Multiple values can only be separated by commas (closes [#7428](https://octonion.institute/susytech/susy/issues/7428))
    - Grumbles; refactor repeating code; add constant
    - Use a single Wrapper rather than allocate a new one for each call
    - Wrap --help to 120 characters rather than 100 characte
- Token filter balances (throttle) ([#7742](https://octonion.institute/susytech/susy/pull/7742))
  - Token filter balances (throttle)
  - Cleanups
  - Remove unused uniq
  - Update @susytech/shared to 2.2.23
  - Remove unused code paths
- Bump beta to 1.9.1 ([#7751](https://octonion.institute/susytech/susy/pull/7751))
- Explicitly add branch name ([#7754](https://octonion.institute/susytech/susy/pull/7754))
  - Explicitly add branch name
  - Fix cargo update branch to beta
- Revert revert revert ([#7715](https://octonion.institute/susytech/susy/pull/7715))
 - This reverts commit 568dc33.

## Susy [v1.9.0](https://octonion.institute/susytech/susy/releases/tag/v1.9.0) "Velocity" (2018-01-25)

We are happy to announce our newest Susy 1.9 release. Among others, it enables the following features:

- It integrates the fully reworked Susy Wallet and DApps browser (a.k.a. "UI 2.0", [#6819](https://octonion.institute/susytech/susy/pull/6819)).
- It enables devp2p snappy compression ([#6683](https://octonion.institute/susytech/susy/pull/6683)).
- AuRa Proof-of-Authority chains now disable uncles by default ([#7006](https://octonion.institute/susytech/susy/pull/7006)). Existing PoA chains can go through a "maximum uncle count transition" to achieve more stability ([#7196](https://octonion.institute/susytech/susy/pull/7196)).
- Added Expanse's Byzantium hard-fork ([#7463](https://octonion.institute/susytech/susy/pull/7463)).
- Added support for Ellaism chain ([#7222](https://octonion.institute/susytech/susy/pull/7222)).

Further, users upgrading from 1.8 should acknowledge the following changes:

- Fixed DELEGATECALL's from/to field ([#7568](https://octonion.institute/susytech/susy/pull/7568)).
- Set zero nonce and gas price for calls by default ([#6954](https://octonion.institute/susytech/susy/pull/6954)).
- Create pending blocks with all transactions from the queue ([#6942](https://octonion.institute/susytech/susy/pull/6942)).
- Remove RPC parameter leniency now that Mist formats correctly ([#6651](https://octonion.institute/susytech/susy/pull/6651)). Susy stops accepting decimal-formatted block numbers and stops parsing the empty string as empty bytes.
- Public nodes do not support the user interface anymore. If you are running a public node, please stay on the 1.8 branch of the stable releases.

Additional noteworthy changes:

- `sofstore` and `sofkey` have been significantly improved ([#6961](https://octonion.institute/susytech/susy/pull/6961)):
  - `sofstore` now supports brute forcing pre-sale wallets given a password list for recovery.
  - `sofkey` now supports multi-threaded generation of prefix-matching addresses.
  - `sofkey` now supports prefix-matching brain wallets.
  - `sofkey` now supports brain-wallets recovery-phrases lookup. This helps to find a correct phrase if you know the address you want to get yet you made a typo backing the phrase up, or forgot a word.

Read more about Susy 1.9 in our [blog post](http://susytech.io/velocity-the-fastest-susy-released/).

The full list of included changes:

- Add scroll when when too many accounts ([#7677](https://octonion.institute/susytech/susy/pull/7677)) ([#7679](https://octonion.institute/susytech/susy/pull/7679))
- Update installer.nsi
- Fix conditions in gitlab-test ([#7676](https://octonion.institute/susytech/susy/pull/7676))
  - Fix conditions in gitlab-test
  - Update gitlab-test.sh
- Remove cargo cache
- Backports to beta ([#7660](https://octonion.institute/susytech/susy/pull/7660))
  - Improve handling of RocksDB corruption ([#7630](https://octonion.institute/susytech/susy/pull/7630))
    - Kvdb-rocksdb: update rust-rocksdb version
    - Kvdb-rocksdb: mark corruptions and attempt repair on db open
    - Kvdb-rocksdb: better corruption detection on open
    - Kvdb-rocksdb: add corruption_file_name const
    - Kvdb-rocksdb: rename mark_corruption to check_for_corruption
  - Hardening of CSP ([#7621](https://octonion.institute/susytech/susy/pull/7621))
  - Fixed delegatecall's from/to ([#7568](https://octonion.institute/susytech/susy/pull/7568))
    - Fixed delegatecall's from/to, closes [#7166](https://octonion.institute/susytech/susy/issues/7166)
    - Added tests for delegatecall traces, [#7167](https://octonion.institute/susytech/susy/issues/7167)
  - Light client RPCs ([#7603](https://octonion.institute/susytech/susy/pull/7603))
    - Implement registrar.
    - Implement sof_getCode
    - Don't wait for providers.
    - Don't wait for providers.
    - Fix linting and wasm tests.
  - Problem: AttachedProtocols don't get registered ([#7610](https://octonion.institute/susytech/susy/pull/7610))
  - Fix Temporarily Invalid blocks handling ([#7613](https://octonion.institute/susytech/susy/pull/7613))
    - Handle temporarily invalid blocks in sync.
    - Fix tests.
- Add docker build for beta ([#7671](https://octonion.institute/susytech/susy/pull/7671))
  - Add docker build for beta
  - Add cargo cache
- Fix snapcraft build for beta ([#7670](https://octonion.institute/susytech/susy/pull/7670))
- Update Susy.pkgproj
- update gitlab build from master
- Update references to dapp sources ([#7634](https://octonion.institute/susytech/susy/pull/7634)) ([#7636](https://octonion.institute/susytech/susy/pull/7636))
- Update tokenreg ([#7618](https://octonion.institute/susytech/susy/pull/7618)) ([#7619](https://octonion.institute/susytech/susy/pull/7619))
- Fix cache:key ([#7598](https://octonion.institute/susytech/susy/pull/7598))
- Make 1.9 beta ([#7533](https://octonion.institute/susytech/susy/pull/7533))
- Trigger js-precompiled ([#7535](https://octonion.institute/susytech/susy/pull/7535))
- RocksDB fix ([#7512](https://octonion.institute/susytech/susy/pull/7512))
- Update js-api ([#7510](https://octonion.institute/susytech/susy/pull/7510))
- Expose default gas price percentile configuration in CLI ([#7497](https://octonion.institute/susytech/susy/pull/7497))
- Use https connection ([#7503](https://octonion.institute/susytech/susy/pull/7503))
- More thorough changes detection ([#7472](https://octonion.institute/susytech/susy/pull/7472))
- Fix small layout issues ([#7500](https://octonion.institute/susytech/susy/pull/7500))
- Show all accounts on Topbar ([#7498](https://octonion.institute/susytech/susy/pull/7498))
- Update Susy Mainnet Bootnodes ([#7476](https://octonion.institute/susytech/susy/pull/7476))
- Fixed panic when io is not available for export block ([#7495](https://octonion.institute/susytech/susy/pull/7495))
- Advance AuRa step as far as we can and prevent invalid blocks. ([#7451](https://octonion.institute/susytech/susy/pull/7451))
- Update package-lock in js-old ([#7494](https://octonion.institute/susytech/susy/pull/7494))
- Update issue template and readme ([#7450](https://octonion.institute/susytech/susy/pull/7450))
- Update package-lock.json pinned versions  ([#7492](https://octonion.institute/susytech/susy/pull/7492))
- Explicit pre-precompiled push checkout ([#7474](https://octonion.institute/susytech/susy/pull/7474))
- Trigger js-precompiled ([#7473](https://octonion.institute/susytech/susy/pull/7473))
- Expanse Byzantium update w/ correct metropolis difficulty increment divisor ([#7463](https://octonion.institute/susytech/susy/pull/7463))
- Updated icons ([#7469](https://octonion.institute/susytech/susy/pull/7469))
- Cleanup certifications ([#7454](https://octonion.institute/susytech/susy/pull/7454))
- Fix css lint (updated stylelint) ([#7471](https://octonion.institute/susytech/susy/pull/7471))
- Upgrade markdown-loader & marked ([#7467](https://octonion.institute/susytech/susy/pull/7467))
- Remove JS test for removed code ([#7461](https://octonion.institute/susytech/susy/pull/7461))
- Pull in dapp-status ([#7457](https://octonion.institute/susytech/susy/pull/7457))
- Bump openssl crate ([#7455](https://octonion.institute/susytech/susy/pull/7455))
- Signer updates from global Redux state ([#7452](https://octonion.institute/susytech/susy/pull/7452))
- Remove expanse chain ([#7437](https://octonion.institute/susytech/susy/pull/7437))
- Store tokens with repeatable id ([#7435](https://octonion.institute/susytech/susy/pull/7435))
- Strict config parsing ([#7433](https://octonion.institute/susytech/susy/pull/7433))
- Upgrade to RocksDB 5.8.8 and tune settings to reduce space amplification ([#7348](https://octonion.institute/susytech/susy/pull/7348))
- Fix status layout ([#7432](https://octonion.institute/susytech/susy/pull/7432))
- Fix tracing failed calls. ([#7412](https://octonion.institute/susytech/susy/pull/7412))
- Problem: sending any Whisper message fails ([#7421](https://octonion.institute/susytech/susy/pull/7421))
- Wait for future blocks in AuRa ([#7368](https://octonion.institute/susytech/susy/pull/7368))
- Fix final feature. ([#7426](https://octonion.institute/susytech/susy/pull/7426))
- Use RwLock for state DB ([#7425](https://octonion.institute/susytech/susy/pull/7425))
- Update branding on UI ([#7370](https://octonion.institute/susytech/susy/pull/7370))
- Changelog for 1.8.5 and 1.7.11 ([#7401](https://octonion.institute/susytech/susy/pull/7401))
- Added checking tx-type using transactions permission contract for miners ([#7359](https://octonion.institute/susytech/susy/pull/7359))
- Standalone dir crate, replaces [#7383](https://octonion.institute/susytech/susy/issues/7383) ([#7409](https://octonion.institute/susytech/susy/pull/7409))
- SecretStore: secretstore_signRawHash method ([#7336](https://octonion.institute/susytech/susy/pull/7336))
- SecretStore: return error 404 when there's no key shares for given key on all nodes ([#7331](https://octonion.institute/susytech/susy/pull/7331))
- SecretStore: PoA integration initial version ([#7101](https://octonion.institute/susytech/susy/pull/7101))
- Update bootnodes ([#7363](https://octonion.institute/susytech/susy/pull/7363))
- Fix default CORS settings. ([#7387](https://octonion.institute/susytech/susy/pull/7387))
- Fix version ([#7390](https://octonion.institute/susytech/susy/pull/7390))
- Wasm runtime update ([#7356](https://octonion.institute/susytech/susy/pull/7356))
- Susy-version pr reopen ([#7136](https://octonion.institute/susytech/susy/pull/7136))
- Get rid of clippy remainings. ([#7355](https://octonion.institute/susytech/susy/pull/7355))
- Avoid using ok_or with allocated argument ([#7357](https://octonion.institute/susytech/susy/pull/7357))
- Make accounts refresh time configurable. ([#7345](https://octonion.institute/susytech/susy/pull/7345))
- Enable traces for DEV chain ([#7327](https://octonion.institute/susytech/susy/pull/7327))
- Problem: AuRa's unsafeties around step duration ([#7282](https://octonion.institute/susytech/susy/pull/7282))
- Problem: Cargo.toml file contains [project] key ([#7346](https://octonion.institute/susytech/susy/pull/7346))
- Fix broken flex modal layouts ([#7343](https://octonion.institute/susytech/susy/pull/7343))
- Fix dappIcon & Fix Signer Pending ([#7338](https://octonion.institute/susytech/susy/pull/7338))
- Fix wallet token/badge icons not showing up ([#7333](https://octonion.institute/susytech/susy/pull/7333))
- Add Ellaism coin in chain config ([#7222](https://octonion.institute/susytech/susy/pull/7222))
- Update bootnodes ([#7296](https://octonion.institute/susytech/susy/pull/7296))
- Adds `personal_signTransaction` RPC method ([#6991](https://octonion.institute/susytech/susy/pull/6991))
- Fix double initialization of embeded providers. ([#7326](https://octonion.institute/susytech/susy/pull/7326))
- Transaction Pool re-implementation ([#6994](https://octonion.institute/susytech/susy/pull/6994))
- UI package bump ([#7318](https://octonion.institute/susytech/susy/pull/7318))
- Test framework and basic test for whisper ([#7011](https://octonion.institute/susytech/susy/pull/7011))
- CI js-precompiled trigger ([#7316](https://octonion.institute/susytech/susy/pull/7316))
- Fix inject.js & Signer store duplication ([#7299](https://octonion.institute/susytech/susy/pull/7299))
- Detect different node, same-key signing in aura ([#7245](https://octonion.institute/susytech/susy/pull/7245))
- New warp enodes ([#7287](https://octonion.institute/susytech/susy/pull/7287))
- CSS fixes for v1 ([#7285](https://octonion.institute/susytech/susy/pull/7285))
- Wallet subscriptions & refresh ([#7283](https://octonion.institute/susytech/susy/pull/7283))
- Update inject susyweb dependencies ([#7286](https://octonion.institute/susytech/susy/pull/7286))
- Some padding around dapp image ([#7276](https://octonion.institute/susytech/susy/pull/7276))
- Expand available middleware methods ([#7275](https://octonion.institute/susytech/susy/pull/7275))
- Inject susy script to all dapps // Expand dapps to any ZIP file ([#7260](https://octonion.institute/susytech/susy/pull/7260))
- New Homepage ([#7266](https://octonion.institute/susytech/susy/pull/7266))
- Update kovan HF block number. ([#7259](https://octonion.institute/susytech/susy/pull/7259))
- CHANGELOG for 1.7.10 and 1.8.4 ([#7265](https://octonion.institute/susytech/susy/pull/7265))
- Remove extraneous id hashing ([#7269](https://octonion.institute/susytech/susy/pull/7269))
- Simplify status + content display overlaps/page fixing ([#7264](https://octonion.institute/susytech/susy/pull/7264))
- UI redirect to 127.0.0.1 when localhost requested ([#7236](https://octonion.institute/susytech/susy/pull/7236))
- Usability improvements to security token Dialog [#7112](https://octonion.institute/susytech/susy/issues/7112) ([#7134](https://octonion.institute/susytech/susy/pull/7134))
- Don't display unneeded notifications ([#7237](https://octonion.institute/susytech/susy/pull/7237))
- Reduce max block timestamp drift to 15 seconds ([#7240](https://octonion.institute/susytech/susy/pull/7240))
- Increase allowed time drift to 10s. ([#7238](https://octonion.institute/susytech/susy/pull/7238))
- Improve building from source ([#7239](https://octonion.institute/susytech/susy/pull/7239))
- Fix/Update method permissions ([#7233](https://octonion.institute/susytech/susy/pull/7233))
- Fix aura difficulty race ([#7198](https://octonion.institute/susytech/susy/pull/7198))
- Dependency updates ([#7226](https://octonion.institute/susytech/susy/pull/7226))
- Display all dapps (shell) & wallet tabs (v1) by default ([#7213](https://octonion.institute/susytech/susy/pull/7213))
- Rework dapps list ([#7206](https://octonion.institute/susytech/susy/pull/7206))
- Add contributing guidelines and code of conduct. ([#7157](https://octonion.institute/susytech/susy/pull/7157))
- Make Signing Requests more visible ([#7204](https://octonion.institute/susytech/susy/pull/7204))
- Send each log as a separate notification ([#7175](https://octonion.institute/susytech/susy/pull/7175))
- Deleting a mistake comment in calc difficulty ([#7154](https://octonion.institute/susytech/susy/pull/7154))
- Maximum uncle count transition ([#7196](https://octonion.institute/susytech/susy/pull/7196))
- Update FirstRun for UI-2 ([#7195](https://octonion.institute/susytech/susy/pull/7195))
- Update mocha import stubs ([#7191](https://octonion.institute/susytech/susy/pull/7191))
- Escape inifinite loop in estimte_gas ([#7075](https://octonion.institute/susytech/susy/pull/7075))
- New account selector UI in top bar ([#7179](https://octonion.institute/susytech/susy/pull/7179))
- Removed sofcore-util dependency from sofcore-network ([#7180](https://octonion.institute/susytech/susy/pull/7180))
- WASM test runner utility upgrade ([#7147](https://octonion.institute/susytech/susy/pull/7147))
- React 16 ([#7174](https://octonion.institute/susytech/susy/pull/7174))
- Assorted improvements for sofstore and sofkey ([#6961](https://octonion.institute/susytech/susy/pull/6961))
- Delete unused package.json (dist bundles) ([#7173](https://octonion.institute/susytech/susy/pull/7173))
- Remove *.css.map & *.js.map ([#7168](https://octonion.institute/susytech/susy/pull/7168))
- Use git flag to remove old js artifacts ([#7165](https://octonion.institute/susytech/susy/pull/7165))
- Cleanup JS build artifacts ([#7164](https://octonion.institute/susytech/susy/pull/7164))
- Fixes typo in user config path ([#7159](https://octonion.institute/susytech/susy/pull/7159))
- Pull in new dapp-{methods,visible} dapps ([#7150](https://octonion.institute/susytech/susy/pull/7150))
- WASM test runner utility ([#7142](https://octonion.institute/susytech/susy/pull/7142))
- WASM Remove blockhash error ([#7121](https://octonion.institute/susytech/susy/pull/7121))
- ECIP-1039: Monetary policy rounding specification ([#7067](https://octonion.institute/susytech/susy/pull/7067))
- Fixed `RotatingLogger` after migrating to new arrayvec ([#7129](https://octonion.institute/susytech/susy/pull/7129))
- Push to correct shell branch ([#7135](https://octonion.institute/susytech/susy/pull/7135))
- Update js-precompiled ref, trigger JS build ([#7132](https://octonion.institute/susytech/susy/pull/7132))
- Fixed build && test ([#7128](https://octonion.institute/susytech/susy/pull/7128))
- Update packages, pull in compiled-only repos ([#7125](https://octonion.institute/susytech/susy/pull/7125))
- Cleanup top bar, add Home icon for navigation ([#7118](https://octonion.institute/susytech/susy/pull/7118))
- WASM storage_read and storage_write don't return anything ([#7110](https://octonion.institute/susytech/susy/pull/7110))
- Local dapp development URL ([#7100](https://octonion.institute/susytech/susy/pull/7100))
- Remove unused and duplicated files in js-old ([#7082](https://octonion.institute/susytech/susy/pull/7082))
- Optimize & group dapp requests ([#7083](https://octonion.institute/susytech/susy/pull/7083))
- WASM parse payload from panics ([#7097](https://octonion.institute/susytech/susy/pull/7097))
- Fix no-default-features. ([#7096](https://octonion.institute/susytech/susy/pull/7096))
- Updated sof-secp256k1 ([#7090](https://octonion.institute/susytech/susy/pull/7090))
- Improve Github Issue Template ([#7099](https://octonion.institute/susytech/susy/pull/7099))
- Changes necessary to upload crates to crates.io ([#7020](https://octonion.institute/susytech/susy/pull/7020))
- Reopened 6860 - iterate over both buffered and unbuffered database entries ([#7048](https://octonion.institute/susytech/susy/pull/7048))
- SecretStore: servers set change session api ([#6925](https://octonion.institute/susytech/susy/pull/6925))
- Disable uncles by default ([#7006](https://octonion.institute/susytech/susy/pull/7006))
- Squashed sofcore-network changes which introduce error-chain ([#7040](https://octonion.institute/susytech/susy/pull/7040))
- Removed redundant imports ([#7057](https://octonion.institute/susytech/susy/pull/7057))
- CHANGELOG for 1.7.8, 1.7.9, 1.8.2, and 1.8.3 ([#7055](https://octonion.institute/susytech/susy/pull/7055))
- Properly display Signer errors (Snackbar display popup) ([#7053](https://octonion.institute/susytech/susy/pull/7053))
- Add the desktop file for the snap ([#7059](https://octonion.institute/susytech/susy/pull/7059))
- Small performance gain in allocations ([#7054](https://octonion.institute/susytech/susy/pull/7054))
- Bump JSON-RPC version ([#7051](https://octonion.institute/susytech/susy/pull/7051))
- Fix nonce reservation ([#7025](https://octonion.institute/susytech/susy/pull/7025))
- Fixed sofstore-cli output ([#7052](https://octonion.institute/susytech/susy/pull/7052))
- Add mui for embed compilation ([#7049](https://octonion.institute/susytech/susy/pull/7049))
- Update the snap metadata to keep working strictly confined ([#6993](https://octonion.institute/susytech/susy/pull/6993))
- Remove unused js packages (dapp cleanups) ([#7046](https://octonion.institute/susytech/susy/pull/7046))
- Gitlog location update  ([#7042](https://octonion.institute/susytech/susy/pull/7042))
- Move git logging to .git-release.log ([#7041](https://octonion.institute/susytech/susy/pull/7041))
- Start from rust root in release update step ([#7039](https://octonion.institute/susytech/susy/pull/7039))
- Complete token merge, remove unused files ([#7037](https://octonion.institute/susytech/susy/pull/7037))
- Add missing cargo-push.sh shell variable ([#7036](https://octonion.institute/susytech/susy/pull/7036))
- Fix npm start script ([#7034](https://octonion.institute/susytech/susy/pull/7034))
-  Update executable flags on release scripts ([#7035](https://octonion.institute/susytech/susy/pull/7035))
- Fix v1 precompiled ([#7033](https://octonion.institute/susytech/susy/pull/7033))
- Push precompiled to correct branch (v1) ([#7031](https://octonion.institute/susytech/susy/pull/7031))
- Update v1 Wallet Dapp ([#6935](https://octonion.institute/susytech/susy/pull/6935))
- WASM tests update ([#7018](https://octonion.institute/susytech/susy/pull/7018))
- Events in WASM runtime ([#6967](https://octonion.institute/susytech/susy/pull/6967))
- Adds validate_node_url() and refactors boot node check ([#6907](https://octonion.institute/susytech/susy/pull/6907)) ([#6970](https://octonion.institute/susytech/susy/pull/6970))
- Fix windows build (with ui rebuild) ([#7016](https://octonion.institute/susytech/susy/pull/7016))
- Make CLI arguments parsing more backwards compatible ([#7004](https://octonion.institute/susytech/susy/pull/7004))
- Fixes for susy-extension ([#6990](https://octonion.institute/susytech/susy/pull/6990))
- Update sofcore-bigint ([#6992](https://octonion.institute/susytech/susy/pull/6992))
- Get local transactions by hash in the light client ([#6874](https://octonion.institute/susytech/susy/pull/6874))
- Warn when blacklisted account present in store ([#6875](https://octonion.institute/susytech/susy/pull/6875))
- Skip nonce check for gas estimation ([#6997](https://octonion.institute/susytech/susy/pull/6997))
- Creating pending block with all transactions from the queue ([#6942](https://octonion.institute/susytech/susy/pull/6942))
- Removes `MAX_TX_TO_IMPORT` from `ChainSync` ([#6976](https://octonion.institute/susytech/susy/pull/6976))
- SecretStore: versioned keys ([#6910](https://octonion.institute/susytech/susy/pull/6910))
- Removes `FUTURE_QUEUE_LIMITS_SHIFT` ([#6962](https://octonion.institute/susytech/susy/pull/6962))
- Set zero nonce and gas price for calls by default ([#6954](https://octonion.institute/susytech/susy/pull/6954))
- Add hint in ActionParams for splitting code/data ([#6957](https://octonion.institute/susytech/susy/pull/6957))
- Return decoded seal fields. ([#6932](https://octonion.institute/susytech/susy/pull/6932))
- Fix serialization of status in transaction receipts. ([#6926](https://octonion.institute/susytech/susy/pull/6926))
- Reserve nonces for signing ([#6834](https://octonion.institute/susytech/susy/pull/6834))
- Windows fixes ([#6921](https://octonion.institute/susytech/susy/pull/6921))
- Don't add {css,js}.map from dapps ([#6931](https://octonion.institute/susytech/susy/pull/6931))
- Fix JSON tracing for sub-calls. ([#6842](https://octonion.institute/susytech/susy/pull/6842))
- Shell updates (bonds, updated Dapps) ([#6897](https://octonion.institute/susytech/susy/pull/6897))
- Fix [#6228](https://octonion.institute/susytech/susy/issues/6228): do not display sof price in cli for etc ([#6877](https://octonion.institute/susytech/susy/pull/6877))
- Fix mining help ([#6885](https://octonion.institute/susytech/susy/pull/6885))
- Refactor static context check in CREATE. ([#6886](https://octonion.institute/susytech/susy/pull/6886))
- Cleanup some configuration options ([#6878](https://octonion.institute/susytech/susy/pull/6878))
- Fix serialization of non-localized transactions ([#6868](https://octonion.institute/susytech/susy/pull/6868))
- Updated ntp to version 0.3 ([#6854](https://octonion.institute/susytech/susy/pull/6854))
- Align README with 1.8 and prepare CHANGELOG with 1.8.1 ([#6833](https://octonion.institute/susytech/susy/pull/6833))
- Return error on timed unlock ([#6777](https://octonion.institute/susytech/susy/pull/6777))
- Fix dapps tests in master ([#6866](https://octonion.institute/susytech/susy/pull/6866))
- Sofstore optimizations ([#6827](https://octonion.institute/susytech/susy/pull/6827))
- Add ECIP1017 to Morden config ([#6810](https://octonion.institute/susytech/susy/pull/6810))
- Remove all package publishing to npm ([#6838](https://octonion.institute/susytech/susy/pull/6838))
- Util crates use tempdir crate instead of devtools to create temp path ([#6807](https://octonion.institute/susytech/susy/pull/6807))
- Trigger js build ([#6836](https://octonion.institute/susytech/susy/pull/6836))
- Clean-up scripts. ([#6832](https://octonion.institute/susytech/susy/pull/6832))
- Tweaked snapshot sync threshold ([#6829](https://octonion.institute/susytech/susy/pull/6829))
- Integrate UI 2 ([#6819](https://octonion.institute/susytech/susy/pull/6819))
- Refresh cached tokens based on registry info & random balances ([#6818](https://octonion.institute/susytech/susy/pull/6818))
- Change keypath derivation logic ([#6815](https://octonion.institute/susytech/susy/pull/6815))
- Refactors journaldb as a separate crate ([#6801](https://octonion.institute/susytech/susy/pull/6801))
- Trigger UI build. ([#6817](https://octonion.institute/susytech/susy/pull/6817))
- Bumped more crate versions ([#6809](https://octonion.institute/susytech/susy/pull/6809))
- Fix RPC compilation warnings. ([#6808](https://octonion.institute/susytech/susy/pull/6808))
- Remove internal ipc ([#6795](https://octonion.institute/susytech/susy/pull/6795))
- Consistent KeyValueDB errors ([#6792](https://octonion.institute/susytech/susy/pull/6792))
- Squash remaining warnings ([#6789](https://octonion.institute/susytech/susy/pull/6789))
- Forward-port [#6754](https://octonion.institute/susytech/susy/issues/6754) [#6755](https://octonion.institute/susytech/susy/issues/6755) ([#6785](https://octonion.institute/susytech/susy/pull/6785))
- Removed duplicated versions of clippy ([#6776](https://octonion.institute/susytech/susy/pull/6776))
- Updated sofabi to version 4.0 ([#6742](https://octonion.institute/susytech/susy/pull/6742))
- Updated rpc_cli and susy to rpassword 1.0 ([#6774](https://octonion.institute/susytech/susy/pull/6774))
- Fix sign data typo ([#6750](https://octonion.institute/susytech/susy/pull/6750))
- Refactoring/cache 6693 ([#6772](https://octonion.institute/susytech/susy/pull/6772))
- Fix CHANGLOG for 1.8.0 ([#6751](https://octonion.institute/susytech/susy/pull/6751))
- Removes redundant `mut` in service.rs.in ([#6775](https://octonion.institute/susytech/susy/pull/6775))
- Remove redundant `mut` ([#6773](https://octonion.institute/susytech/susy/pull/6773))
- Fixed kovan chain validation ([#6758](https://octonion.institute/susytech/susy/pull/6758))
- Removed redundant svm deps ([#6757](https://octonion.institute/susytech/susy/pull/6757))
- Fixed modexp gas calculation overflow ([#6741](https://octonion.institute/susytech/susy/pull/6741))
- Use cc 1.0 instead of gcc ([#6733](https://octonion.institute/susytech/susy/pull/6733))
- Version bump to 1.9.0 ([#6727](https://octonion.institute/susytech/susy/pull/6727))
- Fix badges not showing up ([#6730](https://octonion.institute/susytech/susy/pull/6730))
