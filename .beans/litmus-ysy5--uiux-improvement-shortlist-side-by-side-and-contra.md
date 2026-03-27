---
# litmus-ysy5
title: 'UI/UX improvement: shortlist, side-by-side, and contrast issues'
status: in-progress
type: epic
priority: normal
created_at: 2026-03-26T17:46:47Z
updated_at: 2026-03-27T04:58:52Z
---

The side-by-side compare page is missing contrast issue indicators — one of the project's killer features. The current shortlist/compare coupling creates UX friction: shortlist is limited to 5, compare gets messy with many themes, and users may confuse the two concepts.

## Problems
1. **Missing contrast issues in side-by-side view** — the most valuable analysis feature is absent from the comparison page
2. **Compare page scales poorly** — works well for 2 themes, OK for 3, unusable for more without a very wide monitor
3. **Shortlist/compare coupling is confusing** — shortlist limited to 5 because it's tied to compare; users may not understand the relationship
4. **Count badges get too cluttered** — too many themes make the contrast issue badges unreadable

## Current Thinking
- Convert shortlist to favorites or history (decouple from compare)
- Improve side-by-side UX: limit max compare count, or enforce minimum width per theme
- Needs a larger design rethink to make these features work smoothly together

## Status
Refined — ready for implementation. See subtasks.

## Brainstorming Session (2026-03-26)

### Current State Analysis

**Compare page** (`compare.rs`):
- Shows 2-4 themes side-by-side in a grid
- Two view modes: simulated terminal output + screenshots
- Color palette section at bottom
- **No contrast issues shown** — `TermOutputView` receives empty `issue_details`
- URL: `/:provider/compare/slug1,slug2,...`

**Contrast issue system** (fully built on detail page):
- `validate_fixtures_contrast()` → `Vec<TermContrastIssue>` per theme
- `build_issue_registry()` → deduped rules (C1, C2...) by TermColor pair
- Interactive chips with click-to-cycle navigation between affected fixtures
- Span-level markers (dotted outlines), tooltips, merged footnotes
- Readability score (0-100%) per theme
- All CSS classes and components exist and are reusable

**Shortlist** (`state.rs`, `sidebar.rs`):
- `MAX_SHORTLIST = 5`, stored as `Vec<String>` of slugs
- Coupled to compare: sidebar builds compare URL from app theme + shortlist
- If < 2 themes, fills with random themes
- FIFO overflow when full

---

### Design Proposal A: "Focused Compare" — Limit to 2-3, decouple shortlist

**Core idea:** Compare works best with 2-3 themes. Optimize for that and stop pretending 4+ works.

**Changes:**
1. **Hard cap compare at 3 themes** (down from 4)
   - 2 themes: each gets ~50% width — plenty of room for contrast markers
   - 3 themes: each gets ~33% — still workable with compact chips

2. **Add contrast issues to compare page:**
   - Readability % badge in each column header (ScoreRing, already exists)
   - Compact issue chips below each theme name (reuse `detail-issue-chip`)
   - Span-level markers + tooltips on terminal output (pass `issue_details` to `TermOutputView`)
   - Footnotes at line ends
   - Per-fixture issue count badge on fixture headers

3. **Rename shortlist → "Favorites":**
   - Remove the 5-item limit (or raise to 20)
   - Favorites persist in localStorage, shown in sidebar
   - Star icon on cards and detail page
   - Not directly tied to compare URL

4. **New compare entry point:**
   - "Compare" button in sidebar picks first 2-3 favorites
   - Or: checkboxes on browse page to select specific themes to compare
   - Compare URL remains the same, just built differently

**Pros:** Simple, focused, contrast issues fit naturally. Clear separation of concerns.
**Cons:** Loses ability to compare 4 themes. Favorites redesign is extra work.

---

### Design Proposal B: "Compare with Contrast Overlay" — Enhance existing, minimal structural change

**Core idea:** Add contrast issues to the existing compare page with minimal UX disruption.

**Changes:**
1. **Keep 2-4 theme support as-is**

2. **Add contrast summary row** below column headers:
   - Readability % (ScoreRing) + total issue count per theme
   - Color-coded: green (≥85%), orange (70-84%), red (<70%)
   - Compact — one line per theme

3. **Add "Show issues" toggle** (default off to avoid clutter):
   - When ON: span markers + tooltips appear on all columns
   - Footnotes shown inline
   - Compact chip strip per theme (scrollable if many rules)

4. **Cross-theme issue highlighting:**
   - Click an issue chip on theme A → highlight same TermColor pair on all themes
   - Makes it easy to see "red-on-default is bad in theme A but fine in theme B"

5. **Shortlist tweaks (minimal):**
   - Rename to "Pinned themes" for clarity
   - Keep limit at 5 (matches max compare of 4 + app theme)
   - Add tooltip explaining purpose

**Pros:** Least disruptive. Adds contrast without changing navigation model. Cross-theme highlighting is a unique differentiator.
**Cons:** 4-theme compare + contrast markers may still be too cramped. Shortlist confusion only partially addressed.

---

### Design Proposal C: "Split Model" — Favorites + Focused Compare

**Core idea:** Completely decouple browsing (favorites) from analysis (compare). Compare becomes a deliberate, deep-analysis tool for exactly 2 themes.

**Changes:**
1. **Replace shortlist with "Favorites":**
   - Unlimited (practical limit ~50), persisted to localStorage
   - Star toggle on cards, detail page, scene-across page
   - Sidebar section: expandable list of favorited themes
   - No connection to compare

2. **Redesign compare as a 2-theme deep comparison:**
   - Always exactly 2 themes, full width split
   - Each side: full contrast analysis (chips, markers, footnotes, readability)
   - **New: "Contrast diff" view** — highlight issues unique to each theme
     - Green markers: "this theme passes where the other fails"
     - Red markers: "this theme fails where the other passes"
     - Grey: both pass or both fail
   - Entry: "Compare with..." button on detail page, or pick 2 from favorites

3. **For 3+ themes, use existing "Scene Across Themes" page:**
   - Already shows all themes for one fixture
   - Add readability badges to each theme card there
   - This is the right UX for "scanning many themes at once"

4. **New sidebar flow:**
   - ★ Favorites (expandable, unlimited)
   - Compare: shows current compare pair (if any), or "Pick 2 to compare"
   - Quick-compare: drag two favorites, or "Compare A vs B" from any theme card

**Pros:** Cleanest conceptual model. Compare is purpose-built for contrast analysis. "Scene Across" handles the "many themes" case. Favorites are intuitive.
**Cons:** Most work. Loses 3-4 theme compare (redirected to scene-across). Bigger UX change for existing users.

---

### Recommendation

**Start with Proposal A** as the pragmatic path:
- Cap at 3 themes is a small constraint with big UX payoff
- Contrast issues slot in naturally with existing infrastructure
- Favorites is a clean rename with lifted limit
- Can evolve toward Proposal C later if the 2-theme deep comparison proves valuable

**Quick win regardless of approach:** Pass `issue_details` to `TermOutputView` on the compare page + add readability badges. This alone adds huge value with minimal code change.
