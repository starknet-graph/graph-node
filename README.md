# `graph-node` Fork with StarkNet Support

This is a [`graph-node`](https://github.com/graphprotocol/graph-node) fork with support for [StarkNet](https://starknet.io/). It's created and maintained by the [zkLend](https://zklend.com/) team.

This fork syncs the `master` branch with the upstream continuously, after which it applies a single commit replacing the `README.md` file with the one you're reading right now, and another commit for making necessary CI changes. The actual code for StarkNet support lives in another branch `patch`, which always builds on the latest `master` branch.

Whenever a version is released on the upstream project, we will make the same release except with the patch applied. Our release would essentially be the patch branch rebased on the corresponding upstream tag.
