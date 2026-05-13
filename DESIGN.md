# AI Chat IDE - Design Document

## Goals

The target is an IDE-like AI Chat application mimicking the aesthetics of VSCode and Cursor.

## UI / UX Guidelines

- **Theming**: Light theme only (for now).
- **Color Palette**:
  - Primary Accent: `oklch(0.71 0.18 38.65)` (used for highlights and button hovers)
  - Button Base: `oklch(0.66 0.18 38.65)`
  - Backgrounds: Warm whites (`oklch(0.9748 0.009 70)`) and slight grays for depth.
  - Text: Dark grays (`oklch(0.25 0.01 70)`) for hierarchy.
  - Borders: Subtle dividing lines (`oklch(0.85 0.01 70)`).

## Technical Stack

- **Framework**: React + Vite
- **UI Primitives**: Radix UI for accessible base components (ScrollArea, Avatar, etc).
- **Icons**: Lucide for crisp SVG components.
