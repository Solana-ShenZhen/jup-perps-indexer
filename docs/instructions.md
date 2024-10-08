# Jupiter Perps Instructions

## User

### Position Management

1. Open position
   - Use [`CreateIncreasePositionMarketRequest`](../src/jup_perps/instructions/create_increase_position_market_request.rs#L13-L46)
   - [Example transaction: Long SOL](https://solscan.io/tx/2XjMnY2ER63LkED3URnydPTqB964hW9dqw1euHiFfRYtewn2ZPvHiZ7KXHmwVUttwKeip8MZdo3eFLmXwPisyTLK)
2. Close position
3. Modify position
   - Increase position size
   - Decrease position size / close partial position

### Collateral Management

4. Deposit collateral
5. Withdraw collateral

### Take Profit (TP) / Stop Loss (SL) Orders

6. Create TP / SL order
   [Create take profit](https://solscan.io/tx/29M9HPmHButEQungbbtZAFs2Wdg114m1iTx9attAA4CgF3d8d2qFP55UPa3pLZioEGd4xHNuEHV1dFZc4YgSvREx)
7. Edit TP / SL order
8. Close TP / SL order

## Keeper
