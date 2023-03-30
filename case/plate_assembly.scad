include <vars.scad>

use <bottom_plate.scad>
use <top_plate.scad>
use <center_cover.scad>

module full_assembly() {
    color([0, 0.75, 0])
        translate([-39.5, 144.5, 5])
            scale(25.4)
                import("pcb-export.stl");

    color(plate_color) {
        linear_extrude(plate_thickness)
            bottom_plate();


        translate([0, 0, case_depth + plate_thickness])
            linear_extrude(plate_thickness) {
                top_plate();

                translate([board_plate_width, 0, 0])
                    scale([-1, 1, 1])
                        top_plate();
            }

        translate([
            0.5 * (board_plate_width - center_cover_width),
            0,
            case_depth + plate_thickness + 6 * mm
        ])
            linear_extrude(plate_thickness) {
                center_cover();
            }
    }
}

full_assembly();