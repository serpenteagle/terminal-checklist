# Todo
- `List` structs currently hold `Option<String>` for the `file` field. Change with to `Option<Path`.
- Write only changes instead of rewriting the whole `List` to both file and UI.

## Bugs
- Panics while parsing items with spacees
