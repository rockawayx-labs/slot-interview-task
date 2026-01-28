# RockawayX Labs Interview Task

## Current State

Solana slot update listener. Connects via WebSocket, subscribes to slot updates, logs them.

## Your Task

1. Store slot updates
   - use configurable sliding window (e.g., last 2 minutes)
   - store slot identifier (`u64`) and its on-chain timestamp (`u64`)
   - updates MUST be stored **uniquely**, each slot has unique `u64` identifier
2. [Optional] Expose an HTTP endpoint to retrieve stored slot updates
   - `GET /slots` - returns slot updates in JSON format
     - optional parameters:
       - `since` timestamp
       - `tail` number of recent updates
   - `GET /stats` - returns aggregated statistics
     - total slots in the window
     - average slot time

> [!TIP]
> This repository is a boilerplate starting point to save you time. We don't want you spending hours of your free time on this. Feel free to refactor, replace, or remove anything as you see fit. There's no obligation to keep any of the existing structure or code.

> [!CAUTION]
> Do NOT create pull requests in this repository. Send your code via established channels.

## Reference

- [SlotUpdate](https://docs.rs/solana-client/latest/solana_client/rpc_response/enum.SlotUpdate.html)
