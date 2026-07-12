## High-Level Overview

Your algorithm extracts elements from a 2D matrix in **clockwise spiral order** by simulating physical movement through the grid.

To achieve this without getting stuck or re-visiting numbers, your code relies on three main concepts:

1. **Directional Sub-Loops:** 4 distinct loops representing movement directions: **Left $\rightarrow$ Right**, **Top $\rightarrow$ Bottom**, **Right $\rightarrow$ Left**, and **Bottom $\rightarrow$ Top**.
2. **Visited Tracker (`is_visited`):** A boolean 2D array of matching size (`row_size` $\times$ `col_size`) that keeps track of which cells have already been processed.
3. **Termination Guard (`count >= spiral_count`):** An iteration counter checking if total added elements match total elements in the matrix ($M \times N$).

---

## Code Structure Breakdown

### 1. Edge Case Handling (1D Matrices)

Before starting the main loop, your code checks if the matrix is a single row or a single column:

* **Single Row (`row_size == 1`):** Clones and returns the first row directly since moving left-to-right completes the spiral.
* **Single Column (`col_size == 1`):** Iterates vertically down the column and returns immediately, avoiding unnecessary directional logic.

### 2. Initialization & Tracking Setup

```rust
let mut is_visited: Vec<Vec<bool>> = vec![vec![false; col_size]; row_size];
let mut r: usize = 0;
let mut c: usize = 0;
let mut count: usize = 0;
let spiral_count: usize = row_size * col_size;

```

* `is_visited`: Initialized to all `false`.
* `r` and `c`: Represent current row and column coordinates.
* `count`: Tracks visited cells against `spiral_count` to break out when complete.

---

## The Core Loop: 4-Directional Traversal

The outer `loop` continuously executes four directional passes in sequence:

```
 (r, c) --------------> Loop 1: Left to Right
   ^                         |
   |                         |
 Loop 4:                     v
 Bottom to Top          Loop 2: Top to Bottom
   ^                         |
   |                         v
 Loop 3: Right to Left <-----+

```

### 1. **Loop 1: Left $\rightarrow$ Right**

* **Goal:** Travel horizontally across the top-most unvisited row.
* **Logic:** Iterates `c` (columns) forward until hitting `col_size` or an already visited cell (`is_visited[r][c] == true`).
* **Adjustment:** Once stopped, `c -= 1` steps back onto the last valid cell, and `r += 1` shifts down one row to set up the next direction.

### 2. **Loop 2: Top $\rightarrow$ Bottom**

* **Goal:** Travel vertically down the right-most unvisited column.
* **Logic:** Increments `r` (rows) downward until reaching `row_size` or a visited cell.
* **Adjustment:** Decrements `r -= 1` to stay inside boundaries, then decrements `c -= 1` to step left for the reverse horizontal pass.

### 3. **Loop 3: Right $\rightarrow$ Left**

* **Goal:** Travel horizontally across the bottom-most unvisited row in reverse.
* **Logic:** Decrements `c` leftward. Uses explicit check `if c == 0 { break; }` to prevent Rust `usize` underflow errors when reaching the first column index.
* **Adjustment:** If hitting a visited cell, adjusts `c += 1` and breaks. Afterward, `r -= 1` steps up one row to prepare for the upward pass.

### 4. **Loop 4: Bottom $\rightarrow$ Top**

* **Goal:** Travel vertically up the left-most unvisited column.
* **Logic:** Decrements `r` upward, visiting unvisited cells until reaching an already visited cell (the top border of the current layer).
* **Adjustment:** Upon hitting a visited cell, it resets coordinates into the inner layer (`r += 1`, `c += 1`) so the next iteration of **Loop 1** starts cleanly inside the next inner ring.

---

## Loop Termination

At the end of every 4-direction cycle, the code evaluates:

```rust
if count >= spiral_count { break; }

```

Once `count` reaches `row_size * col_size`, every single cell has been visited exactly once, breaking out of the outer `loop` and returning `spiral_arr`.
