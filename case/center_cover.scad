include <vars.scad>

use <rounded_square.scad>
use <mounting.scad>

module center_cover() {
    difference() {
        rounded_square([center_cover_width, plate_height], plate_corner_radius);

        m2([
            center_cover_mounting_inset,
            center_cover_mounting_inset + plate_margin
        ]);
        m2([
            center_cover_width - center_cover_mounting_inset,
            center_cover_mounting_inset + plate_margin
        ]);
        m2([
            center_cover_mounting_inset,
            plate_height - center_cover_mounting_inset - plate_margin
        ]);
        m2([
            center_cover_width - center_cover_mounting_inset,
            plate_height - center_cover_mounting_inset - plate_margin
        ]);
    }
}

center_cover();