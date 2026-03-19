# Issue: Banner Display Resolution

## Description
The banner (video/GIF) is displaying in a compressed rectangle instead of showing at proper resolution. The height is too small (100px) causing the banner to appear squished.

## Current Behavior
- Banner displays at 100px height regardless of content
- Aspect ratio is not properly maintained
- Results in a small compressed rectangle

## Expected Behavior
- Banner should display at 150px height minimum
- Aspect ratio should be calculated and maintained (16:9 for video)
- Banner should scale properly with window width

## Technical Details
- File: `injector/src/presentation/gui.rs`
- Function: `render_banner()`
- Current height: 100px
- Target height: 150px

## Solution
1. Update banner height from 100px to 150px
2. Add aspect ratio calculation for video (16:9)
3. Apply same sizing logic to both GIF and video formats
4. Ensure proper scaling with `fit_to_exact_size`

## Files Modified
- `injector/src/presentation/gui.rs` - Lines 175-210

## Status
Fixed - Commit pending

## Priority
Medium - UI/UX improvement
