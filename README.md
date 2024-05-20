# `graph-node` Fork with StarkNet Support

This is a [`graph-node`](https://github.com/graphprotocol/graph-node) fork with support for [StarkNet](https://starknet.io/). It's created and maintained by the [zkLend](https://zklend.com/) team.

Powered by a [GitHub Actions workflow](https://github.com/starknet-graph/graph-node/actions/workflows/sync.yml), this fork syncs the `master` branch with the upstream continuously:

- First, a commit is made on top of the upstream `master` branch to bring files from the [`home`](https://github.com/starknet-graph/graph-node/tree/home) branch to `master`. This is necessary for making changes to CI workflows and the README file you're reading right now.

- Then, actual patch commits living on the fork `master` branch gets rebased. Before pushing, the branch is compiled to make sure it still builds, and the team gets notified if it doesn't.

Whenever a version is released on the upstream project, we will make the same release except with the patch applied.
