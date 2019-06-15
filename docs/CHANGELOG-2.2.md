## Susy-Sophon [v2.2.7](https://octonion.institute/susytech/susy-sophon/releases/tag/v2.2.7) (2019-01-15)

Susy-Sophon 2.2.7-stable is a consensus-relevant security release that reverts Constantinople on the Sophon network. Upgrading is mandatory for Sophon, and strongly recommended for other networks.

- **Consensus** - Sophon Network: Pull Constantinople protocol upgrade on Sophon ([#10189](https://octonion.institute/susytech/susy-sophon/pull/10189))
  - Read more: [Security Alert: Sophon Constantinople Postponement](https://blog.superstring.io/2019/01/15/security-alert-sophon-constantinople-postponement/)
- **Networking** - All networks: Ping nodes from discovery ([#10167](https://octonion.institute/susytech/susy-sophon/pull/10167))
- **Wasm** - Kovan Network: Update swasm-utils to 0.6.1 ([#10134](https://octonion.institute/susytech/susy-sophon/pull/10134))

_Note:_ This release marks Susy 2.2 as _stable_. All versions of Susy 2.1 now reached _end of life_.

The full list of included changes:

- Backports for stable 2.2.7 ([#10163](https://octonion.institute/susytech/susy-sophon/pull/10163))
  - Version: bump stable to 2.2.7
  - Version: mark 2.2 track stable
  - Version: mark update critical on all networks
  - Handle the case for contract creation on an empty but exist account with storage items ([#10065](https://octonion.institute/susytech/susy-sophon/pull/10065))
  - Fix _cannot recursively call into `Core`_ issue ([#10144](https://octonion.institute/susytech/susy-sophon/pull/10144))
  - Snap: fix path in script ([#10157](https://octonion.institute/susytech/susy-sophon/pull/10157))
  - Ping nodes from discovery ([#10167](https://octonion.institute/susytech/susy-sophon/pull/10167))
  - Version: bump fork blocks for kovan and foundation, mark releases non critical
  - Pull constantinople on sophon network ([#10189](https://octonion.institute/susytech/susy-sophon/pull/10189))

## Susy-Sophon [v2.2.6](https://octonion.institute/susytech/susy-sophon/releases/tag/v2.2.6) (2019-01-10)

Susy-Sophon 2.2.6-beta is a bugfix release that improves performance and stability.

The full list of included changes:

- Beta backports v2.2.6 ([#10113](https://octonion.institute/susytech/susy-sophon/pull/10113))
  - Version: bump beta to v2.2.6
  - Fill transaction hash on sofGetLog of light client. ([#9938](https://octonion.institute/susytech/susy-sophon/pull/9938))
  - Fix pubsub new_blocks notifications to include all blocks ([#9987](https://octonion.institute/susytech/susy-sophon/pull/9987))
  - Finality: dont require chain head to be in the chain ([#10054](https://octonion.institute/susytech/susy-sophon/pull/10054))
  - Handle the case for contract creation on an empty but exist account with storage items ([#10065](https://octonion.institute/susytech/susy-sophon/pull/10065))
  - Autogen docs for the "Configuring Susy Sophon" wiki page. ([#10067](https://octonion.institute/susytech/susy-sophon/pull/10067))
  - HF in POA Sokol (2019-01-04) ([#10077](https://octonion.institute/susytech/susy-sophon/pull/10077))
  - Add --locked when running cargo ([#10107](https://octonion.institute/susytech/susy-sophon/pull/10107))
  - Sofcore: update hardcoded headers ([#10123](https://octonion.institute/susytech/susy-sophon/pull/10123))
  - Identity fix ([#10128](https://octonion.institute/susytech/susy-sophon/pull/10128))
  - Update swasm-utils to 0.6.1 ([#10134](https://octonion.institute/susytech/susy-sophon/pull/10134))
  - Make sure parent block is not in importing queue when importing ancient blocks ([#10138](https://octonion.institute/susytech/susy-sophon/pull/10138))
  - CI: re-enable snap publishing ([#10142](https://octonion.institute/susytech/susy-sophon/pull/10142))
  - HF in POA Core (2019-01-18) - Constantinople ([#10155](https://octonion.institute/susytech/susy-sophon/pull/10155))
  - Version: mark upgrade critical on kovan

## Susy-Sophon [v2.2.5](https://octonion.institute/susytech/susy-sophon/releases/tag/v2.2.5) (2018-12-14)

Susy-Sophon 2.2.5-beta is an important release that introduces Constantinople fork at block 7080000 on Mainnet.
This release also contains a fix for chains using AuRa + EmptySteps. Read carefully if this applies to you.
If you have a chain with`empty_steps` already running, some blocks most likely contain non-strict entries (unordered or duplicated empty steps). In this release`strict_empty_steps_transition` **is enabled by default at block 0** for any chain with `empty_steps`.
If your network uses `empty_steps` you **must**:
- plan a hard fork and change `strict_empty_steps_transition` to the desire fork block
- update the clients of the whole network to 2.2.5-beta / 2.1.10-stable.
If for some reason you don't want to do this please set`strict_empty_steps_transition` to `0xfffffffff` to disable it.

The full list of included changes:
- Backports for beta 2.2.5 ([#10047](https://octonion.institute/susytech/susy-sophon/pull/10047))
	- Bump beta to 2.2.5 ([#10047](https://octonion.institute/susytech/susy-sophon/pull/10047))
	- Fix empty steps ([#9939](https://octonion.institute/susytech/susy-sophon/pull/9939))
		- Prevent sending empty step message twice
		- Prevent sending empty step and then block in the same step
		- Don't accept double empty steps
		- Do basic validation of self-sealed blocks
	- Strict empty steps validation ([#10041](https://octonion.institute/susytech/susy-sophon/pull/10041))
		- Enables strict verification of empty steps - there can be no duplicates and empty steps should be ordered inside the seal.
		- Note that authorities won't produce invalid seals after [#9939](https://octonion.institute/susytech/susy-sophon/pull/9939), this PR just adds verification to the seal to prevent forging incorrect blocks and potentially causing consensus issues.
		- This features is enabled by default so any AuRa + EmptySteps chain should set strict_empty_steps_transition fork block number in their spec and upgrade to v2.2.5-beta or v2.1.10-stable.
	- sofcore: enable constantinople on sophon ([#10031](https://octonion.institute/susytech/susy-sophon/pull/10031))
		- sofcore: change blockreward to 2e18 for foundation after constantinople
		- sofcore: delay diff bomb by 2e6 blocks for foundation after constantinople
		- sofcore: enable sip-{145,1014,1052,1283} for foundation after constantinople
	- Change test miner max memory to malloc reports. ([#10024](https://octonion.institute/susytech/susy-sophon/pull/10024))
	- Fix: test corpus_inaccessible panic ([#10019](https://octonion.institute/susytech/susy-sophon/pull/10019))

## Susy-Sophon [v2.2.2](https://octonion.institute/susytech/susy-sophon/releases/tag/v2.2.2) (2018-11-29)

Susy-Sophon 2.2.2-beta is an exciting release. Among others, it improves sync performance, peering stability, block propagation, and transaction propagation times. Also, a warp-sync no longer removes existing blocks from the database, but rather reuses locally available information to decrease sync times and reduces required bandwidth.

Before upgrading to 2.2.2, please also verify the validity of your chain specs. Susy Sophon now denies unknown fields in the specification. To do this, use the chainspec tool:

```
cargo build --release -p chainspec
./target/release/chainspec /path/to/spec.json
```

Last but not least, JSONRPC APIs which are not yet accepted as an SIP in the `sof`, `personal`, or `susyweb` namespace, are now considere experimental as their final specification might change in future. These APIs have to be manually enabled by explicitly running `--susy-jsonrpc-experimental`.

The full list of included changes:

- Backports For beta 2.2.2 ([#9976](https://octonion.institute/susytech/susy-sophon/pull/9976))
  - Version: bump beta to 2.2.2
  - Add experimental RPCs flag ([#9928](https://octonion.institute/susytech/susy-sophon/pull/9928))
  - Keep existing blocks when restoring a Snapshot ([#8643](https://octonion.institute/susytech/susy-sophon/pull/8643))
    - Rename db_restore => client
    - First step: make it compile!
    - Second step: working implementation!
    - Refactoring
    - Fix tests
    - Migrate ancient blocks interacting backward
    - Early return in block migration if snapshot is aborted
    - Remove RwLock getter (PR Grumble I)
    - Remove dependency on `Client`: only used Traits
    - Add test for recovering aborted snapshot recovery
    - Add test for migrating old blocks
    - Release RwLock earlier
    - Revert Cargo.lock
    - Update _update ancient block_ logic: set local in `commit`
    - Update typo in sofcore/src/snapshot/service.rs
  - Adjust requests costs for light client ([#9925](https://octonion.institute/susytech/susy-sophon/pull/9925))
    - Pip Table Cost relative to average peers instead of max peers
    - Add tracing in PIP new_cost_table
    - Update stat peer_count
    - Use number of leeching peers for Light serve costs
    - Fix test::light_params_load_share_depends_on_max_peers (wrong type)
    - Remove (now) useless test
    - Remove `load_share` from LightParams.Config
    - Add LEECHER_COUNT_FACTOR
    - Pr Grumble: u64 to u32 for f64 casting
    - Prevent u32 overflow for avg_peer_count
    - Add tests for LightSync::Statistics
  - Fix empty steps ([#9939](https://octonion.institute/susytech/susy-sophon/pull/9939))
    - Don't send empty step twice or empty step then block.
    - Perform basic validation of locally sealed blocks.
    - Don't include empty step twice.
  - Prevent silent errors in daemon mode, closes [#9367](https://octonion.institute/susytech/susy-sophon/issues/9367) ([#9946](https://octonion.institute/susytech/susy-sophon/pull/9946))
  - Fix a deadlock ([#9952](https://octonion.institute/susytech/susy-sophon/pull/9952))
    - Update informant:
      - Decimal in Mgas/s
      - Print every 5s (not randomly between 5s and 10s)
    - Fix dead-lock in `blockchain.rs`
    - Update locks ordering
  - Fix light client informant while syncing ([#9932](https://octonion.institute/susytech/susy-sophon/pull/9932))
    - Add `is_idle` to LightSync to check importing status
    - Use SyncStateWrapper to make sure is_idle gets updates
    - Update is_major_import to use verified queue size as well
    - Add comment for `is_idle`
    - Add Debug to `SyncStateWrapper`
    - `fn get` -> `fn into_inner`
  -  Ci: rearrange pipeline by logic ([#9970](https://octonion.institute/susytech/susy-sophon/pull/9970))
    - Ci: rearrange pipeline by logic
    - Ci: rename docs script
  - Fix docker build ([#9971](https://octonion.institute/susytech/susy-sophon/pull/9971))
  - Deny unknown fields for chainspec ([#9972](https://octonion.institute/susytech/susy-sophon/pull/9972))
    - Add deny_unknown_fields to chainspec
    - Add tests and fix existing one
    - Remove serde_ignored dependency for chainspec
    - Fix rpc test sof chain spec
    - Fix starting_nonce_test spec
  - Improve block and transaction propagation ([#9954](https://octonion.institute/susytech/susy-sophon/pull/9954))
    - Refactor sync to add priority tasks.
    - Send priority tasks notifications.
    - Propagate blocks, optimize transactions.
    - Implement transaction propagation. Use sync_channel.
    - Tone down info.
    - Prevent deadlock by not waiting forever for sync lock.
    - Fix lock order.
    - Don't use sync_channel to prevent deadlocks.
    - Fix tests.
  - Fix unstable peers and slowness in sync ([#9967](https://octonion.institute/susytech/susy-sophon/pull/9967))
    - Don't sync all peers after each response
    - Update formating
    - Fix tests: add `continue_sync` to `Sync_step`
    - Update sofcore/sync/src/chain/mod.rs
    - Fix rpc middlewares
  - Fix Cargo.lock
  - Json: resolve merge in spec
  - Rpc: fix starting_nonce_test
  - Ci: allow nightl job to fail

## Susy-Sophon [v2.2.1](https://octonion.institute/susytech/susy-sophon/releases/tag/v2.2.1) (2018-11-15)

Susy-Sophon 2.2.1-beta is the first v2.2 release, and might introduce features that break previous work flows, among others:

- Prevent zero network ID ([#9763](https://octonion.institute/susytech/susy-sophon/pull/9763)) and drop support for Olympic testnet ([#9801](https://octonion.institute/susytech/susy-sophon/pull/9801)): The Olympic test net is dead for years and never used a chain ID but network ID zero. Susy Sophon is now preventing the network ID to be zero, thus Olympic support is dropped. Make sure to chose positive non-zero network IDs in future.
- Multithreaded snapshot creation ([#9239](https://octonion.institute/susytech/susy-sophon/pull/9239)): adds a CLI argument `--snapshot-threads` which specifies the number of threads. This helps improving the performance of full nodes that wish to provide warp-snapshots for the network. The gain in performance comes with a slight drawback in increased snapshot size.
- Expose config max-round-blocks-to-import ([#9439](https://octonion.institute/susytech/susy-sophon/pull/9439)): Susy Sophon imports blocks in rounds. If at the end of any round, the queue is not empty, we consider it to be _importing_ and won't notify pubsub. On large re-orgs (10+ blocks), this is possible. The default `max_round_blocks_to_import` is increased to 12 and configurable via the `--max-round-blocks-to-import` CLI flag. With unstable network conditions, it is advised to increase the number. This shouldn't have any noticeable performance impact unless the number is set to really large.
- Increase Gas-floor-target and Gas Cap ([#9564](https://octonion.institute/susytech/susy-sophon/pull/9564)): the default values for gas floor target are `8_000_000` and gas cap `10_000_000`, similar to Graviton 1.8.15+.
- Produce portable binaries ([#9725](https://octonion.institute/susytech/susy-sophon/pull/9725)): we now produce portable binaries, but it may incur some performance degradation. For ultimate performance it's now better to compile Susy Sophon from source with `PORTABLE=OFF` environment variable.
- RPC: `susy_allTransactionHashes` ([#9745](https://octonion.institute/susytech/susy-sophon/pull/9745)): Get all pending transactions from the queue with the high performant `susy_allTransactionHashes` RPC method.
- Support `sof_chainId` RPC method ([#9783](https://octonion.institute/susytech/susy-sophon/pull/9783)): implements SIP-695 to get the chainID via RPC.
- AuRa: finalize blocks ([#9692](https://octonion.institute/susytech/susy-sophon/pull/9692)): The AuRa engine was updated to emit ancestry actions to finalize blocks. The full client stores block finality in the database, the engine builds finality from an ancestry of `ExtendedHeader`; `is_epoch_end` was updated to take a vec of recently finalized headers; `is_epoch_end_light` was added which maintains the previous interface and is used by the light client since the client itself doesn't track finality.

The full list of included changes:

- Backport to susy 2.2.1 beta ([#9905](https://octonion.institute/susytech/susy-sophon/pull/9905))
  - Bump version to 2.2.1
  - Fix: Intermittent failing CI due to addr in use ([#9885](https://octonion.institute/susytech/susy-sophon/pull/9885))
  - Fix Susy not closing on Ctrl-C ([#9886](https://octonion.institute/susytech/susy-sophon/pull/9886))
  - Fix json tracer overflow ([#9873](https://octonion.institute/susytech/susy-sophon/pull/9873))
  - Fix docker script ([#9854](https://octonion.institute/susytech/susy-sophon/pull/9854))
  - Add hardcoded headers for light client ([#9907](https://octonion.institute/susytech/susy-sophon/pull/9907))
  - Gitlab-ci: make android release build succeed ([#9743](https://octonion.institute/susytech/susy-sophon/pull/9743))
  - Allow to seal work on latest block ([#9876](https://octonion.institute/susytech/susy-sophon/pull/9876))
  - Remove rust-toolchain file ([#9906](https://octonion.institute/susytech/susy-sophon/pull/9906))
  - Light-fetch: Differentiate between out-of-gas/manual throw and use required gas from response on failure ([#9824](https://octonion.institute/susytech/susy-sophon/pull/9824))
  - Sip-712 implementation ([#9631](https://octonion.institute/susytech/susy-sophon/pull/9631))
  - Sip-191 implementation ([#9701](https://octonion.institute/susytech/susy-sophon/pull/9701))
  - Simplify cargo audit ([#9918](https://octonion.institute/susytech/susy-sophon/pull/9918))
  - Fix performance issue importing Kovan blocks ([#9914](https://octonion.institute/susytech/susy-sophon/pull/9914))
  - Ci: nuke the gitlab caches ([#9855](https://octonion.institute/susytech/susy-sophon/pull/9855))
- Backports to susy beta 2.2.0 ([#9820](https://octonion.institute/susytech/susy-sophon/pull/9820))
  - Ci: remove failing tests for android, windows, and macos ([#9788](https://octonion.institute/susytech/susy-sophon/pull/9788))
  - Implement NoProof for json tests and update tests reference ([#9814](https://octonion.institute/susytech/susy-sophon/pull/9814))
  - Move state root verification before gas used ([#9841](https://octonion.institute/susytech/susy-sophon/pull/9841))
  - Classic.json Bootnode Update ([#9828](https://octonion.institute/susytech/susy-sophon/pull/9828))
- Rpc: susy_allTransactionHashes ([#9745](https://octonion.institute/susytech/susy-sophon/pull/9745))
- Revert "prevent zero networkID ([#9763](https://octonion.institute/susytech/susy-sophon/pull/9763))" ([#9815](https://octonion.institute/susytech/susy-sophon/pull/9815))
- Allow zero chain id in SIP155 signing process ([#9792](https://octonion.institute/susytech/susy-sophon/pull/9792))
- Add readiness check for docker container ([#9804](https://octonion.institute/susytech/susy-sophon/pull/9804))
- Insert dev account before unlocking ([#9813](https://octonion.institute/susytech/susy-sophon/pull/9813))
- Removed "rustup" & added new runner tag ([#9731](https://octonion.institute/susytech/susy-sophon/pull/9731))
- Expose config max-round-blocks-to-import ([#9439](https://octonion.institute/susytech/susy-sophon/pull/9439))
- Aura: finalize blocks ([#9692](https://octonion.institute/susytech/susy-sophon/pull/9692))
- Sync: retry different peer after empty subchain heads response ([#9753](https://octonion.institute/susytech/susy-sophon/pull/9753))
- Fix(light-rpc/susy) : Remove unused client ([#9802](https://octonion.institute/susytech/susy-sophon/pull/9802))
- Drops support for olympic testnet, closes [#9800](https://octonion.institute/susytech/susy-sophon/issues/9800) ([#9801](https://octonion.institute/susytech/susy-sophon/pull/9801))
- Replace `tokio_core` with `tokio` (`ring` -> 0.13) ([#9657](https://octonion.institute/susytech/susy-sophon/pull/9657))
- Support sof_chainId RPC method ([#9783](https://octonion.institute/susytech/susy-sophon/pull/9783))
- Sofcore: bump ropsten forkblock checkpoint ([#9775](https://octonion.institute/susytech/susy-sophon/pull/9775))
- Docs: changelogs for 2.0.8 and 2.1.3 ([#9758](https://octonion.institute/susytech/susy-sophon/pull/9758))
- Prevent zero networkID ([#9763](https://octonion.institute/susytech/susy-sophon/pull/9763))
- Skip seal fields count check when --no-seal-check is used ([#9757](https://octonion.institute/susytech/susy-sophon/pull/9757))
- Aura: fix panic on extra_info with unsealed block ([#9755](https://octonion.institute/susytech/susy-sophon/pull/9755))
- Docs: update changelogs ([#9742](https://octonion.institute/susytech/susy-sophon/pull/9742))
- Removed extra assert in generation_session_is_removed_when_succeeded ([#9738](https://octonion.institute/susytech/susy-sophon/pull/9738))
- Make checkpoint_storage_at use plain loop instead of recursion ([#9734](https://octonion.institute/susytech/susy-sophon/pull/9734))
- Use signed 256-bit integer for sstore gas refund substate ([#9746](https://octonion.institute/susytech/susy-sophon/pull/9746))
- Heads ref not present for branches beta and stable ([#9741](https://octonion.institute/susytech/susy-sophon/pull/9741))
- Add Callisto support ([#9534](https://octonion.institute/susytech/susy-sophon/pull/9534))
- Add --force to cargo audit install script ([#9735](https://octonion.institute/susytech/susy-sophon/pull/9735))
- Remove unused expired value from Handshake ([#9732](https://octonion.institute/susytech/susy-sophon/pull/9732))
- Add hardcoded headers ([#9730](https://octonion.institute/susytech/susy-sophon/pull/9730))
- Produce portable binaries ([#9725](https://octonion.institute/susytech/susy-sophon/pull/9725))
- Gitlab ci: releasable_branches: change variables condition to schedule ([#9729](https://octonion.institute/susytech/susy-sophon/pull/9729))
- Update a few susy-common dependencies ([#9663](https://octonion.institute/susytech/susy-sophon/pull/9663))
- Hf in POA Core (2018-10-22) ([#9724](https://octonion.institute/susytech/susy-sophon/pull/9724))
- Schedule nightly builds ([#9717](https://octonion.institute/susytech/susy-sophon/pull/9717))
- Fix ancient blocks sync ([#9531](https://octonion.institute/susytech/susy-sophon/pull/9531))
- Ci: Skip docs job for nightly ([#9693](https://octonion.institute/susytech/susy-sophon/pull/9693))
- Fix (light/provider) : Make `read_only executions` read-only ([#9591](https://octonion.institute/susytech/susy-sophon/pull/9591))
- Sofcore: fix detection of major import ([#9552](https://octonion.institute/susytech/susy-sophon/pull/9552))
- Return 0 on error ([#9705](https://octonion.institute/susytech/susy-sophon/pull/9705))
- Sofcore: delay ropsten hardfork ([#9704](https://octonion.institute/susytech/susy-sophon/pull/9704))
- Make instantSeal engine backwards compatible, closes [#9696](https://octonion.institute/susytech/susy-sophon/issues/9696) ([#9700](https://octonion.institute/susytech/susy-sophon/pull/9700))
- Implement CREATE2 gas changes and fix some potential overflowing ([#9694](https://octonion.institute/susytech/susy-sophon/pull/9694))
- Don't hash the init_code of CREATE. ([#9688](https://octonion.institute/susytech/susy-sophon/pull/9688))
- Sofcore: minor optimization of modexp by using LR exponentiation ([#9697](https://octonion.institute/susytech/susy-sophon/pull/9697))
- Removed redundant clone before each block import ([#9683](https://octonion.institute/susytech/susy-sophon/pull/9683))
- Add Foundation Bootnodes ([#9666](https://octonion.institute/susytech/susy-sophon/pull/9666))
- Docker: run as susy user ([#9689](https://octonion.institute/susytech/susy-sophon/pull/9689))
- Sofcore: mcip3 block reward contract ([#9605](https://octonion.institute/susytech/susy-sophon/pull/9605))
- Verify block syncing responses against requests ([#9670](https://octonion.institute/susytech/susy-sophon/pull/9670))
- Add a new RPC `susy_submitWorkDetail` similar `sof_submitWork` but return block hash ([#9404](https://octonion.institute/susytech/susy-sophon/pull/9404))
- Resumable SVM and heap-allocated callstack ([#9360](https://octonion.institute/susytech/susy-sophon/pull/9360))
- Update susy-wordlist library ([#9682](https://octonion.institute/susytech/susy-sophon/pull/9682))
- Ci: Remove unnecessary pipes ([#9681](https://octonion.institute/susytech/susy-sophon/pull/9681))
- Test.sh: use cargo --target for platforms other than linux, win or mac ([#9650](https://octonion.institute/susytech/susy-sophon/pull/9650))
- Ci: fix push script ([#9679](https://octonion.institute/susytech/susy-sophon/pull/9679))
- Hardfork the testnets ([#9562](https://octonion.institute/susytech/susy-sophon/pull/9562))
- Calculate sha3 instead of sha256 for push-release. ([#9673](https://octonion.institute/susytech/susy-sophon/pull/9673))
- Sofcore-io retries failed work steal ([#9651](https://octonion.institute/susytech/susy-sophon/pull/9651))
- Fix(light_fetch): avoid race with BlockNumber::Latest ([#9665](https://octonion.institute/susytech/susy-sophon/pull/9665))
- Test fix for windows cache name... ([#9658](https://octonion.institute/susytech/susy-sophon/pull/9658))
- Refactor(fetch) : light use only one `DNS` thread ([#9647](https://octonion.institute/susytech/susy-sophon/pull/9647))
- Sophon libfuzzer integration small change ([#9547](https://octonion.institute/susytech/susy-sophon/pull/9547))
- Cli: remove reference to --no-ui in --unlock flag help ([#9616](https://octonion.institute/susytech/susy-sophon/pull/9616))
- Remove master from releasable branches ([#9655](https://octonion.institute/susytech/susy-sophon/pull/9655))
- Sofcore/VerificationQueue don't spawn up extra `worker-threads` when explictly specified not to ([#9620](https://octonion.institute/susytech/susy-sophon/pull/9620))
- Rpc: susy_getBlockReceipts ([#9527](https://octonion.institute/susytech/susy-sophon/pull/9527))
- Remove unused dependencies ([#9589](https://octonion.institute/susytech/susy-sophon/pull/9589))
- Ignore key_server_cluster randomly failing tests ([#9639](https://octonion.institute/susytech/susy-sophon/pull/9639))
- Sofcore: handle vm exception when estimating gas ([#9615](https://octonion.institute/susytech/susy-sophon/pull/9615))
- Fix bad-block reporting no reason ([#9638](https://octonion.institute/susytech/susy-sophon/pull/9638))
- Use static call and apparent value transfer for block reward contract code ([#9603](https://octonion.institute/susytech/susy-sophon/pull/9603))
- Hf in POA Sokol (2018-09-19) ([#9607](https://octonion.institute/susytech/susy-sophon/pull/9607))
- Bump smallvec to 0.6 in sofcore-light, sofstore and whisper ([#9588](https://octonion.institute/susytech/susy-sophon/pull/9588))
- Add constantinople conf to SvmTestClient. ([#9570](https://octonion.institute/susytech/susy-sophon/pull/9570))
- Fix(network): don't disconnect reserved peers ([#9608](https://octonion.institute/susytech/susy-sophon/pull/9608))
- Fix failing node-table tests on mac os, closes [#9632](https://octonion.institute/susytech/susy-sophon/issues/9632) ([#9633](https://octonion.institute/susytech/susy-sophon/pull/9633))
- Update ropsten.json ([#9602](https://octonion.institute/susytech/susy-sophon/pull/9602))
- Simplify sofcore errors by removing BlockImportError ([#9593](https://octonion.institute/susytech/susy-sophon/pull/9593))
- Fix windows compilation, replaces [#9561](https://octonion.institute/susytech/susy-sophon/issues/9561) ([#9621](https://octonion.institute/susytech/susy-sophon/pull/9621))
- Master: rpc-docs set github token ([#9610](https://octonion.institute/susytech/susy-sophon/pull/9610))
- Docs: add changelogs for 1.11.10, 1.11.11, 2.0.3, 2.0.4, 2.0.5, 2.0.6, 2.1.0, and 2.1.1 ([#9554](https://octonion.institute/susytech/susy-sophon/pull/9554))
- Docs(rpc): annotate tag with the provided message ([#9601](https://octonion.institute/susytech/susy-sophon/pull/9601))
- Ci: fix regex roll_eyes ([#9597](https://octonion.institute/susytech/susy-sophon/pull/9597))
- Remove snapcraft clean ([#9585](https://octonion.institute/susytech/susy-sophon/pull/9585))
- Add snapcraft package image (master) ([#9584](https://octonion.institute/susytech/susy-sophon/pull/9584))
- Docs(rpc): push the branch along with tags ([#9578](https://octonion.institute/susytech/susy-sophon/pull/9578))
- Fix typo for susy-jsonrpc-threads flag ([#9574](https://octonion.institute/susytech/susy-sophon/pull/9574))
- Fix informant compile ([#9571](https://octonion.institute/susytech/susy-sophon/pull/9571))
- Added ropsten bootnodes ([#9569](https://octonion.institute/susytech/susy-sophon/pull/9569))
- Increase Gas-floor-target and Gas Cap ([#9564](https://octonion.institute/susytech/susy-sophon/pull/9564))
- While working on the platform tests make them non-breaking ([#9563](https://octonion.institute/susytech/susy-sophon/pull/9563))
- Improve P2P discovery ([#9526](https://octonion.institute/susytech/susy-sophon/pull/9526))
- Move dockerfile for android build container to scripts repo ([#9560](https://octonion.institute/susytech/susy-sophon/pull/9560))
- Simultaneous platform tests WIP ([#9557](https://octonion.institute/susytech/susy-sophon/pull/9557))
- Update sofabi-derive, serde, serde_json, serde_derive, syn && quote ([#9553](https://octonion.institute/susytech/susy-sophon/pull/9553))
- Ci: fix rpc docs generation 2 ([#9550](https://octonion.institute/susytech/susy-sophon/pull/9550))
- Ci: always run build pipelines for win, mac, linux, and android ([#9537](https://octonion.institute/susytech/susy-sophon/pull/9537))
- Multithreaded snapshot creation ([#9239](https://octonion.institute/susytech/susy-sophon/pull/9239))
- New sofabi ([#9511](https://octonion.institute/susytech/susy-sophon/pull/9511))
- Remove initial token for WS. ([#9545](https://octonion.institute/susytech/susy-sophon/pull/9545))
- Net_version caches network_id to avoid redundant aquire of sync readlock ([#9544](https://octonion.institute/susytech/susy-sophon/pull/9544))
- Correct before_script for nightly build versions ([#9543](https://octonion.institute/susytech/susy-sophon/pull/9543))
- Deps: bump susykv-rocksdb to 0.1.4 ([#9539](https://octonion.institute/susytech/susy-sophon/pull/9539))
- State: test when contract creation fails, old storage values should re-appear ([#9532](https://octonion.institute/susytech/susy-sophon/pull/9532))
- Allow dropping light client RPC query with no results ([#9318](https://octonion.institute/susytech/susy-sophon/pull/9318))
- Bump master to 2.2.0 ([#9517](https://octonion.institute/susytech/susy-sophon/pull/9517))
- Enable all Constantinople hard fork changes in constantinople_test.json ([#9505](https://octonion.institute/susytech/susy-sophon/pull/9505))
- [Light] Validate `account balance` before importing transactions ([#9417](https://octonion.institute/susytech/susy-sophon/pull/9417))
- In create memory calculation is the same for create2 because the additional parameter was popped before. ([#9522](https://octonion.institute/susytech/susy-sophon/pull/9522))
- Update patricia trie to 0.2.2 ([#9525](https://octonion.institute/susytech/susy-sophon/pull/9525))
- Replace hardcoded JSON with serde json! macro ([#9489](https://octonion.institute/susytech/susy-sophon/pull/9489))
- Fix typo in version string ([#9516](https://octonion.institute/susytech/susy-sophon/pull/9516))
