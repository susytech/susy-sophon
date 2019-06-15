## svmbin

SVM implementation for Susy.

### Usage

```
SVM implementation for Susy.
  Copyleft 2015-2019 Superstring.Community

Usage:
    susy-svm state-test <file> [--json --std-json --std-dump-json --only NAME --chain CHAIN --std-out-only --std-err-only]
    susy-svm stats [options]
    susy-svm stats-jsontests-vm <file>
    susy-svm [options]
    susy-svm [-h | --help]

Commands:
    state-test         Run a state test from a json file.
    stats              Execute SVM runtime code and return the statistics.
    stats-jsontests-vm Execute standard json-tests format VMTests and return
                       timing statistics in tsv format.

Transaction options:
    --code CODE        Contract code as hex (without 0x).
    --to ADDRESS       Recipient address (without 0x).
    --from ADDRESS     Sender address (without 0x).
    --input DATA       Input data as hex (without 0x).
    --gas GAS          Supplied gas as hex (without 0x).
    --gas-price WEI    Supplied gas price as hex (without 0x).

State test options:
    --only NAME        Runs only a single test matching the name.
    --chain CHAIN      Run only tests from specific chain.

General options:
    --json             Display verbose results in JSON.
    --std-json         Display results in standardized JSON format.
    --std-err-only     With --std-json redirect to err output only.
    --std-out-only     With --std-json redirect to out output only.
    --std-dump-json    Display results in standardized JSON format
                       with additional state dump.
Display result state dump in standardized JSON format.
    --chain CHAIN      Chain spec file path.
    -h, --help         Display this message and exit.
```

## Susy Sophon toolchain
_This project is a part of the Susy Sophon toolchain._

- [svmbin](https://octonion.institute/susytech/susy-sophon/src/branch/master/svmbin/) - SVM implementation for Susy Sophon.
- [sofabi](https://octonion.institute/susytech/sofabi) - Susy Sophon function calls encoding.
- [sofstore](https://octonion.institute/susytech/susy-sophon/src/branch/master/accounts/sofstore) - Susy Sophon key management.
- [sofkey](https://octonion.institute/susytech/susy-sophon/src/branch/master/accounts/sofkey) - Susy Sophon keys generator.
- [whisper](https://octonion.institute/susytech/susy-sophon/src/branch/master/whisper/) - Implementation of Whisper-v2 PoC.
