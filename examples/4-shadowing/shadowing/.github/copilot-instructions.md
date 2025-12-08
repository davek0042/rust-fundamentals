# Copilot Instructions - Rust Shadowing Example

## Project Context

This is a **learning example** from a Rust fundamentals course (Module 2), specifically demonstrating variable shadowing concepts. It's not a production application but an educational code sample.

## Code Conventions

### Variable Shadowing Pattern
- This project intentionally demonstrates shadowing by redeclaring variables with `let`
- Example: `health` is shadowed from `&str` type to `bool` type in `main.rs`
- When modifying: preserve the educational intent showing type changes through shadowing

### Intentional Code Patterns
- **Unused variables with educational comments**: Lines with comments like "why is this line generating an unused variable warning?" are teaching moments
- When fixing warnings, consider whether they're intentionally demonstrating a concept
- Line 18's unused `health` variable demonstrates shadowing behavior - fix by using it or prefixing with `_`

## Development Workflow

### Building and Running
```bash
cargo build    # Compile the example
cargo run      # Run to see shadowing output
```

### Expected Output
```
Result: Tall
Health: good
```

### Compiler Warnings
- Expect `unused_variables` warnings - these may be intentional for demonstration
- Current warning: `health` on line 18 is unused before being shadowed on line 22

## Key Files
- `src/main.rs` - Single example demonstrating mutable variables, conditionals, and shadowing
- `Cargo.toml` - Minimal project with Rust 2021 edition, no external dependencies

## AI Agent Guidelines

When suggesting changes:
1. **Preserve educational value** - don't optimize away the learning examples
2. **Keep it simple** - this is for beginners; avoid advanced Rust patterns
3. **Maintain comment explanations** - in-code comments are teaching aids
4. **Fix warnings thoughtfully** - determine if they're bugs or intentional demonstrations
