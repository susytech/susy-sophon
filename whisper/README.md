## Whisper

Implementation of Whisper based on the Whisper-v2 PoC.

### Usage

```
Susy Whisper-v2 CLI.
	Copyleft 2015-2019 Superstring.Community

Usage:
	whisper [options]
	whisper [-h | --help]

Options:
	--whisper-pool-size SIZE       Specify Whisper pool size [default: 10].
	-p, --port PORT                Specify which RPC port to use [default: 8545].
	-a, --address ADDRESS          Specify which address to use [default: 127.0.0.1].
	-l, --log LEVEL                Specify the logging level. Must conform to the same format as RUST_LOG [default: Error].
	-h, --help                     Display this message and exit.
```

## Susy Sophon toolchain
_This project is a part of the Susy Sophon toolchain._

- [svmbin](https://octonion.institute/susytech/susy-sophon/src/branch/master/svmbin/) - SVM implementation for Susy Sophon.
- [sofabi](https://octonion.institute/susytech/sofabi) - Susy Sophon function calls encoding.
- [sofstore](https://octonion.institute/susytech/susy-sophon/src/branch/master/accounts/sofstore) - Susy Sophon key management.
- [sofkey](https://octonion.institute/susytech/susy-sophon/src/branch/master/accounts/sofkey) - Susy Sophon keys generator.
- [whisper](https://octonion.institute/susytech/susy-sophon/src/branch/master/whisper/) - Implementation of Whisper-v2 PoC.
