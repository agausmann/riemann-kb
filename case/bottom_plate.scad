include <vars.scad>

use <rounded_square.scad>
use <mounting.scad>

module bottom_plate() {
    difference() {
        rounded_square([board_plate_width, plate_height], plate_corner_radius);

        m2([mount_left, mount_bottom]);
        m2([mount_hand_hcenter, mount_bottom]);
        m2([mount_hand_right, mount_bottom]);

        m2([board_plate_width - mount_hand_right, mount_bottom]);
        m2([board_plate_width - mount_hand_hcenter, mount_bottom]);
        m2([mount_board_right, mount_bottom]);

        m2([mount_left, mount_vcenter]);
        m2([mount_board_right, mount_vcenter]);

        m2([mount_left, mount_top]);
        m2([mount_hand_hcenter, mount_top]);
        m2([mount_hand_right, mount_top]);
        
        m2([board_plate_width - mount_hand_right, mount_top]);
        m2([board_plate_width - mount_hand_hcenter, mount_top]);
        m2([mount_board_right, mount_top]);
    }
}

bottom_plate();