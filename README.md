# RockawayX Labs Interview Task

## Current State

Solana slot update listener. Connects via WebSocket, subscribes to slot updates, logs them.

## Your Task

Store slot updates in a sliding window with a configurable time frame.

Updates should be stored **uniquely**.

> [!CAUTION]
> Do NOT create pull requests in this repository. Send your code via established channels.

## Reference

- [SlotUpdate](https://docs.rs/solana-client/latest/solana_client/rpc_response/enum.SlotUpdate.html)
