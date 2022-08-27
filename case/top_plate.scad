include <vars.scad>

use <rounded_square.scad>
use <mounting.scad>

module top_plate() {
    difference() {
        rounded_square([hand_plate_width, plate_height], plate_corner_radius);

        for (row = [1:rows]) {
            for (col = [1:hand_cols]) {
                translate([
                    plate_margin + (col - 0.5) * key_height,
                    plate_margin + (row - 0.5) * key_width
                ])
                    mx_1u();
            }
        }

        m2([mount_left, mount_bottom]);
        m2([mount_hand_hcenter, mount_bottom]);
        m2([mount_hand_right, mount_bottom]);
        m2([mount_left, mount_vcenter]);
        m2([mount_left, mount_top]);
        m2([mount_hand_hcenter, mount_top]);
        m2([mount_hand_right, mount_top]);
    }
}

top_plate();