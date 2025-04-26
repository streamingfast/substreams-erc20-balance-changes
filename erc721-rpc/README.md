# Substreams Ethereum ERC721 RPC-heavy mints events

We have to separate this into a separate package because if we depend on prior version of `erc721.spkg` we get a `events` protobuf conflict.
