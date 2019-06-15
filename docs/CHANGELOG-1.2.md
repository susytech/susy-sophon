Note: Susy 1.2 reached End-of-Life on 2016-11-07 (EOL).

## Susy [v1.2.4](https://octonion.institute/susytech/susy/releases/tag/v1.2.4) (2016-08-09)

Susy 1.2.4 Is a maintenance release that fixes a [few](https://octonion.institute/susytech/susy/pull/1888/commits) issues related to mining and peer synchronization.
This release is marked as stable.

- Backports for beta [#1888](https://octonion.institute/susytech/susy/pull/1888)
- BETA: fixed trace_transaction crash when block contained suicide [#1782](https://octonion.institute/susytech/susy/pull/1782)

## Susy [v1.2.3](https://octonion.institute/susytech/susy/releases/tag/v1.2.3) (2016-07-31)

Susy 1.2.3 is a patch release that addresses network stability issues for both Sophon HF and Sophon classic chains and brings a few changes to the transaction tracing API.

#### Tracing API changes
- Added tracing for `CALLCODE`, `DELEGATECALL` and `SUICIDE`
- `trace_call` returns traces in flat format
- Added 2 new methods: `trace_rawTransaction` and `trace_replayTransaction`

Note that to continue using tracing features in this version you need to re-sync the blockchain. This can be done by using `susy export $HOME/sophon-chain-backup.srlp` , deleting the database usually located at `~/.susy/906a34e69aec8c0d` followed by `susy import $HOME/sophon-chain-backup.srlp`.

- [beta] Updating UI [#1778](https://octonion.institute/susytech/susy/pull/1778)
- tracing backport [#1770](https://octonion.institute/susytech/susy/pull/1770)
- Backport commits to beta [#1763](https://octonion.institute/susytech/susy/pull/1763)
- Deadlock on incoming connection (#1672) [#1675](https://octonion.institute/susytech/susy/pull/1675)
- [BETA] Removed DAO soft fork traces [#1640](https://octonion.institute/susytech/susy/pull/1640)


## Susy [v1.2.2](https://octonion.institute/susytech/susy/releases/tag/v1.2.2) (2016-07-16)

#### New
- DAO hard-fork.

DAO hard-fork implementation conforms to the [specification](https://blog.slock.it/hard-fork-specification-24b889e70703) and is enabled by default.

#### Changed
- `--reseal-on-txs` defaults to `own`.
- DAO soft-fork support has been removed along with related command line options.

#### Resolved issues
- `--db-cache-size` consuming too much memory.
  `sof_getWork` RPC response additionally includes the block number.
- Skipping transactions with invalid nonces when pushing to block.
- Update sealing just once when externally importing many blocks (#1541).
- Transaction tracing skipping simple transactions (#1606).
- Other small fixes and improvements.

Full changelog

- DAO hard-fork (#1483) [#1636](https://octonion.institute/susytech/susy/pull/1636)
- Backports for beta [#1628](https://octonion.institute/susytech/susy/pull/1628)
- don't batch best block for branches (#1623) [#1626](https://octonion.institute/susytech/susy/pull/1626)
- Merge bugfixes from master to beta [#1605](https://octonion.institute/susytech/susy/pull/1605)
- (BETA) using block options cache instead of general cache for rocksdb [#1613](https://octonion.institute/susytech/susy/pull/1613)
- Backport sealing fixes to beta [#1583](https://octonion.institute/susytech/susy/pull/1583)
- v1.2.2 in beta [#1581](https://octonion.institute/susytech/susy/pull/1581)
- Skipping transactions with invalid nonces when pushing to block. (#1545) [#1547](https://octonion.institute/susytech/susy/pull/1547)


## Susy [v1.2.1](https://octonion.institute/susytech/susy/releases/tag/v1.2.1) (2016-07-01)

#### New
- Options for more precise mining tuning (see below).
- Informative notification when block mined.
- HTTP signal on new work-package.
- Optimised database insertion for self-mined blocks.
- Short-circuit for local transaction gas-price approval.
- A number of issues related to mining have been fixed.

##### Mining options
- `--author` is now required for mining.
- `--reseal-on-txs` Specify which transactions should force the node to reseal a block. By default susy updates the seal on incoming transactions to reduce transaction latency. Set this option to `none` to force updates on new blocks only.
- `--reseal-min-period` Can be used to control how often a new pending block is generated if `none` is not selected on prior option.
- `--work-queue-size` Controls how many pending blocks to keep in memory.
- `--relay-set`  Can be used to enable more strict transaction verification.
- `--remove-solved` Move solved blocks from the work package queue instead of cloning them. This gives a slightly faster import speed, but means that extra solutions submitted for the same work package will go unused.
- `--notify-work` Accepts a list of URLs that will receive a POST request when new work package is available. The body of the POST message is JSON encoded and has the same format as `sof_getWork` RPC response.

##### RPC

`sof_getWork` RPC response additionally includes the block number.

##### DAO soft-fork

DAO soft-fork control options have been replaced by the single `--fork` option which disables the soft-fork by default.

#### Changes

- v1.2.1 in beta [#1492](https://octonion.institute/susytech/susy/pull/1492)
- (BETA) add artifacts [#1420](https://octonion.institute/susytech/susy/pull/1420)

## Susy [v1.2.0: "Security"](https://octonion.institute/susytech/susy/releases/tag/v1.2.0) (2016-06-24)

[Blog post](https://blog.superstring.io/announcing-susy-1-2/)

#### New

- Transaction signing UI.
- IPC/RPC module.
- Optimised mining support.
- Windows build.
- DAO soft-fork support.

##### Transaction signing UI

This is a new framework for signing transactions. It fulfills three requirements:
- You should never have to type your passwords into a Dapp.
- No Javascript code should ever hold a secret.
- No transaction should ever be signed without the consent of the user.

The feature is enabled through the `--signer` flag. When enabled, the user must ensure at least one "Signer UI" is set-up for managing transaction confirmation. There are two such UIs available; one through a Google Chrome Extension, separately installable and the second through a special web page hosted locally. Set-up must be done once for each such UI, through copying and pasting a token from the output console of Susy into the UI. Specific instructions are given in the UI.

From this point on, no transaction may ever be signed by Susy except through one of these allowed Signer UIs, and no password should ever be entered anywhere else.

##### IPC/RPC module and Mist/Graviton compatibility

Should be started with `--graviton` to ensure Mist compatibility.

##### Optimised mining support

Numerous improvements and optimisations have been added to our mining implementation. A large "active queue" ensures that late-included transactions are included in the mined block without sacrificing older results from latent-reported `sofminer` results.

##### Windows build

We're happy to announce full Windows support with 1.2!

##### Soft-fork

This release includes support for the proposed [DAO soft-fork](https://docs.google.com/document/d/10RktunzjKNfp6Y8Cu4EhR5V9IqxEZq42LU126EYhWY4/pub). Upon upgrade, all mining nodes can vote for or against the soft fork (this is done through altering the block gas limit; a gas limit of at most 4M results in the soft-fork being triggered).

By default, nodes vote "for" the DAO soft-fork (and try to reduce the gas limit to 3.1M). To vote against the soft-fork (keeping it at 4.7M), run with `--dont-help-rescue-dao`. Not upgrading is not recommended; if the majority votes with a soft-fork, an upgrade will be necessary to mine on the correct chain.

#### Changed
- Fast pruning method is now default for a fresh sync.
- Web UI renamed to Dapps UI.
- JSONRPC and Dapps UI enabled by default.
- CLI options ending `-off` renamed to GNU-consistent prefix `--no-`.
- Dynamic gas-pricing (data feed and statistical techniques used to determine optimum gas prices).

Full changes:

- Signer enabled by default for UI [#1417](https://octonion.institute/susytech/susy/pull/1417)
- Remove experimental pruning options. [#1415](https://octonion.institute/susytech/susy/pull/1415)
- Fixing interface and port for susy ui [#1414](https://octonion.institute/susytech/susy/pull/1414)
- Configurable gas limit cap. [#1405](https://octonion.institute/susytech/susy/pull/1405)
- Bumping TopBar, Minimal SignerUI and wallet [#1413](https://octonion.institute/susytech/susy/pull/1413)
- Sync: Update highest block for progress reporting [#1411](https://octonion.institute/susytech/susy/pull/1411)
- Tweaked CLI options for the release [#1407](https://octonion.institute/susytech/susy/pull/1407)
- Further rocksdb tuning [#1409](https://octonion.institute/susytech/susy/pull/1409)
- Fixing jit compilation [#1406](https://octonion.institute/susytech/susy/pull/1406)
- Bump clippy [#1403](https://octonion.institute/susytech/susy/pull/1403)
- Shortcut SF condition when canon known [#1401](https://octonion.institute/susytech/susy/pull/1401)
- Additional assertions for internal state of queue [#1402](https://octonion.institute/susytech/susy/pull/1402)
- Replace deprecated hashdb trait names [#1394](https://octonion.institute/susytech/susy/pull/1394)
- rpc api by default for ipc [#1400](https://octonion.institute/susytech/susy/pull/1400)
- Ensure judging the SF trigger by relative branch [#1399](https://octonion.institute/susytech/susy/pull/1399)
- Signer with unlocked account working as expected. [#1398](https://octonion.institute/susytech/susy/pull/1398)
- Make --signer default. [#1392](https://octonion.institute/susytech/susy/pull/1392)
- Presale wallet [#1376](https://octonion.institute/susytech/susy/pull/1376)
- Removing signer connection limit [#1396](https://octonion.institute/susytech/susy/pull/1396)
- Optional gas price in transactions come from statistics [#1388](https://octonion.institute/susytech/susy/pull/1388)
- Update README.md with cargo install [ci-skip] [#1389](https://octonion.institute/susytech/susy/pull/1389)
- Fixing possible overflow during multiplication [#1381](https://octonion.institute/susytech/susy/pull/1381)
- Update SF to latest spec [#1386](https://octonion.institute/susytech/susy/pull/1386)
- Sync optimization [#1385](https://octonion.institute/susytech/susy/pull/1385)
- Fixing order of if statements to avoid overflows. [#1384](https://octonion.institute/susytech/susy/pull/1384)
- New topbar & signer UI [#1383](https://octonion.institute/susytech/susy/pull/1383)
- Install trigger for DAO-rescue soft-fork. [#1329](https://octonion.institute/susytech/susy/pull/1329)
- Rocksdb flush/compact limit [#1375](https://octonion.institute/susytech/susy/pull/1375)
- CentOS Dockerfile [#1377](https://octonion.institute/susytech/susy/pull/1377)
- RPC method to return number of unconfirmed transactions... [#1371](https://octonion.institute/susytech/susy/pull/1371)
- bump susy-jsonrpc-http-server [#1369](https://octonion.institute/susytech/susy/pull/1369)
- Fix lock order when updating sealing [#1364](https://octonion.institute/susytech/susy/pull/1364)
- Update sealing on new transactions [#1365](https://octonion.institute/susytech/susy/pull/1365)
- Fixed panic on aborted connection [#1370](https://octonion.institute/susytech/susy/pull/1370)
- importing presale wallet [#1368](https://octonion.institute/susytech/susy/pull/1368)
- Set default database file size large enough [#1363](https://octonion.institute/susytech/susy/pull/1363)
- Reserved peers rpc API [#1360](https://octonion.institute/susytech/susy/pull/1360)
- Fixing replacing transaction with lower gas_price result. [#1343](https://octonion.institute/susytech/susy/pull/1343)
- fixed migration of empty pruning dir [#1362](https://octonion.institute/susytech/susy/pull/1362)
- Transaction processing queue [#1335](https://octonion.institute/susytech/susy/pull/1335)
- Fixing last nonce values in case transaction is replaced [#1359](https://octonion.institute/susytech/susy/pull/1359)
- docopt is an optional dependency of sofkey and sofstore [#1358](https://octonion.institute/susytech/susy/pull/1358)
- Fixing clippy warnings [#1354](https://octonion.institute/susytech/susy/pull/1354)
- Reduce locking when syncing [#1357](https://octonion.institute/susytech/susy/pull/1357)
- removed unnecessary logs [#1356](https://octonion.institute/susytech/susy/pull/1356)
- Updating susy-dapps [#1353](https://octonion.institute/susytech/susy/pull/1353)
- moved keystore tests files from util to sofstore [#1352](https://octonion.institute/susytech/susy/pull/1352)
- removed redundant bigint deps [#1351](https://octonion.institute/susytech/susy/pull/1351)
- Reopen "reserved peers and reserved-only flag" [#1350](https://octonion.institute/susytech/susy/pull/1350)
- Configurable rocksdb cache size [#1348](https://octonion.institute/susytech/susy/pull/1348)
- Fixing future order and errors when reaching limit. [#1346](https://octonion.institute/susytech/susy/pull/1346)
- Removing priority on local transactions [#1342](https://octonion.institute/susytech/susy/pull/1342)
- Revert "Reserved peers, reserved-only flag" [#1349](https://octonion.institute/susytech/susy/pull/1349)
- Sync attack defense: Deactivate peers on invalid block bodies [#1345](https://octonion.institute/susytech/susy/pull/1345)
- Reserved peers, reserved-only flag [#1347](https://octonion.institute/susytech/susy/pull/1347)
- CI for sofkey and sofstore [#1341](https://octonion.institute/susytech/susy/pull/1341)
- Fixed empty block body composition [#1340](https://octonion.institute/susytech/susy/pull/1340)
- Provide a signer UI token by default. [#1334](https://octonion.institute/susytech/susy/pull/1334)
- docker uses rustup, fixes #1337 [#1344](https://octonion.institute/susytech/susy/pull/1344)
- Fixed network service dispose [#1339](https://octonion.institute/susytech/susy/pull/1339)
- Sync: Cache last sync round block parents [#1331](https://octonion.institute/susytech/susy/pull/1331)
- secret store separated from util [#1304](https://octonion.institute/susytech/susy/pull/1304)
- --graviton prevent getTransactionReceipt from using pending. [#1325](https://octonion.institute/susytech/susy/pull/1325)
- Fixing locks order in miner. [#1328](https://octonion.institute/susytech/susy/pull/1328)
- Update default gas limit, rename field [#1324](https://octonion.institute/susytech/susy/pull/1324)
- Use constants for DatabaseConfig [#1318](https://octonion.institute/susytech/susy/pull/1318)
- Fixing clippy warnings [#1321](https://octonion.institute/susytech/susy/pull/1321)
- Bumping topbar. Fixing ws server closing when suspending [#1312](https://octonion.institute/susytech/susy/pull/1312)
- Syncing fix [#1320](https://octonion.institute/susytech/susy/pull/1320)
- Filling-in optional fields of TransactionRequest... [#1305](https://octonion.institute/susytech/susy/pull/1305)
- Removing MakerOTC and DAO dapps  [#1319](https://octonion.institute/susytech/susy/pull/1319)
- Disabling sofcore_set* APIs by default (+ Status page update) [#1315](https://octonion.institute/susytech/susy/pull/1315)
- fixed #1180 [#1282](https://octonion.institute/susytech/susy/pull/1282)
- Network start/stop [#1313](https://octonion.institute/susytech/susy/pull/1313)
- Additional logging for own transactions in queue [#1311](https://octonion.institute/susytech/susy/pull/1311)
- DAO Rescue soft fork [#1309](https://octonion.institute/susytech/susy/pull/1309)
- Appveyor config for windows build+installer [#1302](https://octonion.institute/susytech/susy/pull/1302)
- Key load avoid warning [#1303](https://octonion.institute/susytech/susy/pull/1303)
- More meaningful errors when sending transaction [#1290](https://octonion.institute/susytech/susy/pull/1290)
- Gas price statistics. [#1291](https://octonion.institute/susytech/susy/pull/1291)
- Fix read-ahead bug. [#1298](https://octonion.institute/susytech/susy/pull/1298)
- firewall rules for windows installer [#1297](https://octonion.institute/susytech/susy/pull/1297)
- x64 program files path for installer [#1296](https://octonion.institute/susytech/susy/pull/1296)
- Fixed loosing peers on incoming connections. [#1293](https://octonion.institute/susytech/susy/pull/1293)
- fixed #1261, overflow when calculating work [#1283](https://octonion.institute/susytech/susy/pull/1283)
- snappy and minor block compression [#1286](https://octonion.institute/susytech/susy/pull/1286)
- clarify build instructions [#1287](https://octonion.institute/susytech/susy/pull/1287)
- fixed #1255 [#1280](https://octonion.institute/susytech/susy/pull/1280)
- bump rust-crypto [#1289](https://octonion.institute/susytech/susy/pull/1289)
- Security audit issues fixed [#1279](https://octonion.institute/susytech/susy/pull/1279)
- Fixing origin/host validation [#1273](https://octonion.institute/susytech/susy/pull/1273)
- windows installer + susy start ui cli option [#1284](https://octonion.institute/susytech/susy/pull/1284)
- ipc lib version bump [#1285](https://octonion.institute/susytech/susy/pull/1285)
- Syncing improvements [#1274](https://octonion.institute/susytech/susy/pull/1274)
- removed redundant if condition [#1270](https://octonion.institute/susytech/susy/pull/1270)
- Naive chunk creation, snapshotting [#1263](https://octonion.institute/susytech/susy/pull/1263)
- Fixing generating new token while another susy instance is running. [#1272](https://octonion.institute/susytech/susy/pull/1272)
- README: rustup and windows instructions [#1266](https://octonion.institute/susytech/susy/pull/1266)
- Windows build [#1253](https://octonion.institute/susytech/susy/pull/1253)
- removed try_seal from MiningBlockChainClient [#1262](https://octonion.institute/susytech/susy/pull/1262)
- simplified block opening [#1232](https://octonion.institute/susytech/susy/pull/1232)
- Clippy bump [#1259](https://octonion.institute/susytech/susy/pull/1259)
- Fixing uint ASM macros compilation [#1258](https://octonion.institute/susytech/susy/pull/1258)
- Signer port returned from RPC + Topbar showing count of unconfirmed transactions. [#1252](https://octonion.institute/susytech/susy/pull/1252)
- codegen - avoid unwraps leading to compilation crash [#1250](https://octonion.institute/susytech/susy/pull/1250)
- Dapps bump [#1257](https://octonion.institute/susytech/susy/pull/1257)
- Windows named pipes [#1254](https://octonion.institute/susytech/susy/pull/1254)
- remove unsafety from util/hash.rs and util/bigint/uint.rs [#1236](https://octonion.institute/susytech/susy/pull/1236)
- Fixing CORS settings for special values: * & null. [#1247](https://octonion.institute/susytech/susy/pull/1247)
- JSONRPC test strings avoid using \ char [#1246](https://octonion.institute/susytech/susy/pull/1246)
- Tests for JSON serialisation of statediff/vmtrace [#1241](https://octonion.institute/susytech/susy/pull/1241)
- Bumping Dapps & TopBar to newest version. [#1245](https://octonion.institute/susytech/susy/pull/1245)
- keys import [#1240](https://octonion.institute/susytech/susy/pull/1240)
- Splitting RPC Apis into more fine-grained sets [#1234](https://octonion.institute/susytech/susy/pull/1234)
- Refactor triedb constructors to error on invalid state root  [#1230](https://octonion.institute/susytech/susy/pull/1230)
- Signer RPC method to check if signer is enabled [#1238](https://octonion.institute/susytech/susy/pull/1238)
- Fixing signer behaviour when confirming transaction with wrong password. [#1237](https://octonion.institute/susytech/susy/pull/1237)
- SystemUIs authorization [#1233](https://octonion.institute/susytech/susy/pull/1233)
- IPC path for tesetnet with --graviton compatibility [#1231](https://octonion.institute/susytech/susy/pull/1231)
- Transaction tracing for sof_call [#1210](https://octonion.institute/susytech/susy/pull/1210)
- Removing compilation warnings [#1227](https://octonion.institute/susytech/susy/pull/1227)
- Allowing connections only from chrome-extension and self-hosted client [#1226](https://octonion.institute/susytech/susy/pull/1226)
- Clippy bump & fixing warnings [#1219](https://octonion.institute/susytech/susy/pull/1219)
- Bumping serde & syntex [#1216](https://octonion.institute/susytech/susy/pull/1216)
- Minimal Signer UI (System UI) exposed over websockets. [#1211](https://octonion.institute/susytech/susy/pull/1211)
- Switch RPC namespace form sofcore_ to trace_ [#1208](https://octonion.institute/susytech/susy/pull/1208)
- Verify the state root exists before creating a State [#1217](https://octonion.institute/susytech/susy/pull/1217)
- Integrate state diffing into the sofcore JSONRPC [#1206](https://octonion.institute/susytech/susy/pull/1206)
- Updating topbar to latest version [#1220](https://octonion.institute/susytech/susy/pull/1220)
- Loading local Dapps from FS. [#1214](https://octonion.institute/susytech/susy/pull/1214)
- Ipc serialization & protocol fixes [#1188](https://octonion.institute/susytech/susy/pull/1188)
- Have Ext::ret take self by value [#1187](https://octonion.institute/susytech/susy/pull/1187)
- Simple WebSockets notification about new request [#1202](https://octonion.institute/susytech/susy/pull/1202)
- Removing leftovers of sofminer [#1207](https://octonion.institute/susytech/susy/pull/1207)
- fixed #1204 [#1205](https://octonion.institute/susytech/susy/pull/1205)
- VM tracing and JSON RPC endpoint for it. [#1169](https://octonion.institute/susytech/susy/pull/1169)
- devtools helpers extended [#1186](https://octonion.institute/susytech/susy/pull/1186)
- Networking refactoring [#1172](https://octonion.institute/susytech/susy/pull/1172)
- Client & Miner refactoring [#1195](https://octonion.institute/susytech/susy/pull/1195)
- update readme [#1201](https://octonion.institute/susytech/susy/pull/1201)
- Simple signing queue, confirmation APIs exposed in signer WebSockets. [#1182](https://octonion.institute/susytech/susy/pull/1182)
- Using ordered hashmap to keep the order of dapps on home screen [#1199](https://octonion.institute/susytech/susy/pull/1199)
- Disabling `sofcore` by default, adding x-frame-options header to dapps. [#1197](https://octonion.institute/susytech/susy/pull/1197)
- transaction count verifier tests [#1196](https://octonion.institute/susytech/susy/pull/1196)
- expunge x! and xx! from the codebase [#1192](https://octonion.institute/susytech/susy/pull/1192)
- Database service upgrade (from the ipc branch) [#1185](https://octonion.institute/susytech/susy/pull/1185)
- stop sof_syncing from returning true forever [#1181](https://octonion.institute/susytech/susy/pull/1181)
- Sync fixes and tweaks [#1164](https://octonion.institute/susytech/susy/pull/1164)
- Exposing RPC over Signer WebSockets [#1167](https://octonion.institute/susytech/susy/pull/1167)
- implement missing rpc methods and tests [#1171](https://octonion.institute/susytech/susy/pull/1171)
- json ipc server version bump [#1170](https://octonion.institute/susytech/susy/pull/1170)
- Updated dependencies for windows build [#1173](https://octonion.institute/susytech/susy/pull/1173)
- Framework for improved RPC unit tests [#1141](https://octonion.institute/susytech/susy/pull/1141)
- remove all possible unsafe code in crypto [#1168](https://octonion.institute/susytech/susy/pull/1168)
- Base for Signer Websockets server [#1158](https://octonion.institute/susytech/susy/pull/1158)
- Write queue to speed-up db ipc [#1160](https://octonion.institute/susytech/susy/pull/1160)
- Fixing few clippy warnings [#1163](https://octonion.institute/susytech/susy/pull/1163)
- Change sof_signAndSendTransaction to personal_SignAndSendTransaction [#1154](https://octonion.institute/susytech/susy/pull/1154)
- Support "earliest" and specific block parameters in RPC where possible [#1149](https://octonion.institute/susytech/susy/pull/1149)
- migration fixes [#1155](https://octonion.institute/susytech/susy/pull/1155)
- Empty trusted signer crate with it's general purpose described. [#1150](https://octonion.institute/susytech/susy/pull/1150)
- More bootnodes for morden. [#1153](https://octonion.institute/susytech/susy/pull/1153)
- move existing rpc tests into mocked module [#1151](https://octonion.institute/susytech/susy/pull/1151)
- Bloomchain [#1014](https://octonion.institute/susytech/susy/pull/1014)
- Renaming dapps repos. Updating dapps [#1142](https://octonion.institute/susytech/susy/pull/1142)
- fixed pending transactions [#1147](https://octonion.institute/susytech/susy/pull/1147)
- Basic benches to provide metrics for ipc optimizations [#1145](https://octonion.institute/susytech/susy/pull/1145)
- Fixing clippy warnings [#1148](https://octonion.institute/susytech/susy/pull/1148)
- correct signature of SecTrieDB::raw_mut [#1143](https://octonion.institute/susytech/susy/pull/1143)
- Merge to master and start hypervisor for import/export [#1138](https://octonion.institute/susytech/susy/pull/1138)
- Bumping clippy. Fixing warnings [#1139](https://octonion.institute/susytech/susy/pull/1139)
- Display progress when importing [#1136](https://octonion.institute/susytech/susy/pull/1136)
- foundation of simple db migration [#1128](https://octonion.institute/susytech/susy/pull/1128)
- Fixpending [#1074](https://octonion.institute/susytech/susy/pull/1074)
- Sync: Propagate uncles and fix status reporting [#1134](https://octonion.institute/susytech/susy/pull/1134)
- Coloured, padding logging. [#1133](https://octonion.institute/susytech/susy/pull/1133)
- Importing [#1132](https://octonion.institute/susytech/susy/pull/1132)
- Have `die_with_error` use `fmt::Display` rather than Debug [#1116](https://octonion.institute/susytech/susy/pull/1116)
- Exporting [#1129](https://octonion.institute/susytech/susy/pull/1129)
- Sign and send transaction [#1124](https://octonion.institute/susytech/susy/pull/1124)
- Fixing unused imports warnings [#1125](https://octonion.institute/susytech/susy/pull/1125)
- Adding info messages on mined blocks [#1127](https://octonion.institute/susytech/susy/pull/1127)
- Fix styling - don't mix spaces with tabs!!! [#1123](https://octonion.institute/susytech/susy/pull/1123)
- Fix is_syncing so it's false as long as the update is trivial. [#1122](https://octonion.institute/susytech/susy/pull/1122)
- Relock unlocked accounts after first use [#1120](https://octonion.institute/susytech/susy/pull/1120)
- Avoid importing keys into wrong place. [#1119](https://octonion.institute/susytech/susy/pull/1119)
- Implement receipt's gasUsed field [#1118](https://octonion.institute/susytech/susy/pull/1118)
- New dapps & query parameter handling [#1113](https://octonion.institute/susytech/susy/pull/1113)
- pretty print trace error [#1098](https://octonion.institute/susytech/susy/pull/1098)
- New syncing strategy [#1095](https://octonion.institute/susytech/susy/pull/1095)
- sofcore-db crate [#1097](https://octonion.institute/susytech/susy/pull/1097)
- Fix the default for pruning. [#1107](https://octonion.institute/susytech/susy/pull/1107)
- Make Id/ID and db/Db/DB usage consistent [#1105](https://octonion.institute/susytech/susy/pull/1105)
- Miner holds it's own copy of spec/engine [#1091](https://octonion.institute/susytech/susy/pull/1091)
- Apps listing API & Home webapp. [#1101](https://octonion.institute/susytech/susy/pull/1101)
- CLI option for using JITSVM [#1103](https://octonion.institute/susytech/susy/pull/1103)
- Fix up the seal fields in RPC output [#1096](https://octonion.institute/susytech/susy/pull/1096)
- Fixing some warnings [#1102](https://octonion.institute/susytech/susy/pull/1102)
- fixed incorrect decoding of header seal_fields. added tests. #1090 [#1094](https://octonion.institute/susytech/susy/pull/1094)
- Bumping Clippy [#1093](https://octonion.institute/susytech/susy/pull/1093)
- Injectable topbar support. [#1092](https://octonion.institute/susytech/susy/pull/1092)
- New syncing part 1: Block collection [#1088](https://octonion.institute/susytech/susy/pull/1088)
- Moving all Client public API types to separate mod & binary serialization codegen for that mod [#1051](https://octonion.institute/susytech/susy/pull/1051)
- Subdomains support in content server (webapps server). [#1082](https://octonion.institute/susytech/susy/pull/1082)
- Fix uncle getter [#1087](https://octonion.institute/susytech/susy/pull/1087)
- Provide fallback for usd-per-sof option when offline. [#1085](https://octonion.institute/susytech/susy/pull/1085)
- path centralized [#1083](https://octonion.institute/susytech/susy/pull/1083)
- Limiting result of the execution to execution-specific errors [#1071](https://octonion.institute/susytech/susy/pull/1071)
- Configurable keys security [#1080](https://octonion.institute/susytech/susy/pull/1080)
- comma delimeting multiple cors headers [#1078](https://octonion.institute/susytech/susy/pull/1078)
- Update error message [#1081](https://octonion.institute/susytech/susy/pull/1081)
- Updating dapp-wallet [#1076](https://octonion.institute/susytech/susy/pull/1076)
- Fixed connecting to local nodes on startup [#1070](https://octonion.institute/susytech/susy/pull/1070)
- Validate signature in Tx queue [#1068](https://octonion.institute/susytech/susy/pull/1068)
- moving deps to sofcore/hyper and bumping susy-jsonrpc-http-server version [#1067](https://octonion.institute/susytech/susy/pull/1067)
- Updating status page. Bringing back wallet [#1064](https://octonion.institute/susytech/susy/pull/1064)
- Fix --graviton IPC for MacOS. [#1062](https://octonion.institute/susytech/susy/pull/1062)
- Fixing formatter for defaultExtraData [#1060](https://octonion.institute/susytech/susy/pull/1060)
- --graviton IPC compatibility [#1059](https://octonion.institute/susytech/susy/pull/1059)
- Moving dependencies to sofcore & uniforming syntax libs through all crates [#1050](https://octonion.institute/susytech/susy/pull/1050)
- update hyper branch mio [#1054](https://octonion.institute/susytech/susy/pull/1054)
- IPC lib update [#1047](https://octonion.institute/susytech/susy/pull/1047)
- Updating hyper-mio revision [#1048](https://octonion.institute/susytech/susy/pull/1048)
- Bump ipc-lib version [#1046](https://octonion.institute/susytech/susy/pull/1046)
- Tidy up CLI options and make JSONRPC & webapps on by default. [#1045](https://octonion.institute/susytech/susy/pull/1045)
- Fixing clippy warnings [#1044](https://octonion.institute/susytech/susy/pull/1044)
- Fixing RPC modules compatibility [#1041](https://octonion.institute/susytech/susy/pull/1041)
- Fixing hyper-mio revision [#1043](https://octonion.institute/susytech/susy/pull/1043)
- Updating locations of webapp stuff [#1040](https://octonion.institute/susytech/susy/pull/1040)
- JSON-RPC over IPC [#1039](https://octonion.institute/susytech/susy/pull/1039)
- Update nix/mio for ARM [#1036](https://octonion.institute/susytech/susy/pull/1036)
- Basic Authority [#991](https://octonion.institute/susytech/susy/pull/991)
- Prioritizing of local transaction [#1023](https://octonion.institute/susytech/susy/pull/1023)
- Version 1.2 [#1030](https://octonion.institute/susytech/susy/pull/1030)
- Bumping status page [#1033](https://octonion.institute/susytech/susy/pull/1033)
