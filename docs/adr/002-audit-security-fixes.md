# ADR-002: Security Audit and Fixes

**Date**: 2025-07-25
**Status**: Accepted
**Context**: First security audit of convergio-longrunning crate.

## Findings and Fixes

### CRITICAL — Race Conditions
| Finding | Fix |
|---------|-----|
| `checkpoint::save` DELETE+INSERT without transaction | Wrapped in `unchecked_transaction` |
| `budget::record_cost` UPDATE+SELECT+UPDATE without transaction | Wrapped in `unchecked_transaction` |
| `reaper::reap_cycle` mark+cascade+unregister without transaction | Wrapped in `unchecked_transaction` per execution |

### HIGH — Input Validation
| Finding | Fix |
|---------|-----|
| No validation on execution_id (empty, too long, path traversal chars) | Added `validate_execution_id()` — alphanumeric, `-`, `_`, `.` only, max 256 chars |
| Zero interval_secs accepted in heartbeat register | Reject `interval_secs == 0` |
| Negative/NaN cost_usd accepted in budget recording | Reject negative and non-finite values |
| Percent out of [0,100] accepted in progress update | Reject values outside 0.0–100.0 |

### HIGH — Error Information Disclosure
| Finding | Fix |
|---------|-----|
| All errors returned as 500 with full internal message | `map_err` returns 404/400/402 for known errors; 500 for internal errors logs details server-side, returns generic message to client |

### HIGH — MCP Schema Mismatch
| Finding | Fix |
|---------|-----|
| `cvg_register_heartbeat` schema used `task_id`/`agent_id` but route expects `execution_id`/`interval_secs` | Fixed schema to match route |
| `cvg_heartbeat_beat` schema used `task_id`/`status` but route expects `execution_id` | Fixed schema to match route |
| `cvg_clear_checkpoint` MCP said DELETE but route was POST | Changed route to DELETE to match MCP |

### MEDIUM — Other
| Finding | Fix |
|---------|-----|
| `u64 as i64` cast in heartbeat register could overflow | Use `i64::try_from()` with error |
| Silent stage fallback to `Running` for invalid DB values | Added `tracing::warn!` on fallback |
| Unbounded recursion in `build_tree` | Capped at `MAX_TREE_DEPTH` (64) |

## Not Applicable
- **SQL injection**: All runtime SQL uses parameterized queries ✓
- **Path traversal**: No filesystem operations ✓
- **Command injection**: No subprocess execution ✓
- **SSRF**: No outbound HTTP clients ✓
- **Secret exposure**: No credentials in code ✓
- **Unsafe blocks**: None ✓
- **Auth/AuthZ bypass**: Auth handled at gateway level, not in this crate ✓

## Test Impact
- Before: 37 tests
- After: 44 tests (+7 security validation tests)
