# RockawayX Labs Interview Task

## Current State

Solana slot update listener. Connects via WebSocket, subscribes to slot updates, logs them.

## Your Task

1. Store slot updates
   - use configurable sliding window (e.g., last 2 minutes)
   - store slot identifier and timestamp
   - updates MUST be stored **uniquely**
2. [Optional] Expose an HTTP endpoint to retrieve stored slot updates
   - `GET /slots` - returns slot updates in JSON format
     - optional parameters:
       - `since` timestamp
       - `tail` number of recent updates
   - `GET /stats` - returns aggregated statistics
     - total slots in the window
     - average slot time

> [!TIP]
> Feel free to change/delete everything in this repository. It is supposed to help with boilerplate, as we don't want you to spend hours of your free time on this, it is not required to kep any of the structure or code.

> [!CAUTION]
> Do NOT create pull requests in this repository. Send your code via established channels.

## Reference

- [SlotUpdate](https://docs.rs/solana-client/latest/solana_client/rpc_response/enum.SlotUpdate.html)
