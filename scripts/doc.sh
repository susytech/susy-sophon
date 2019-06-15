#!/usr/bin/env sh
# generate documentation only for partiy and sofcore libraries

cargo doc --no-deps --verbose --all --exclude susy-ipfs-api &&
	echo '<meta http-equiv=refresh content=0;url=sofcore/index.html>' > target/doc/index.html
