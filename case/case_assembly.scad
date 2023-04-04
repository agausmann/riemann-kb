include <vars.scad>

use <center_cover.scad>
use <middle_shell.scad>
use <top_plate.scad>
use <top_shell.scad>
use <bottom_shell.scad>

module pcb()
{
    color([ 0, 0.75, 0 ]) translate([ -39.5, 144.5, -3.5 ]) scale(25.4) import("pcb-export.stl");
}

module plates()
{
    color(plate_color)
    {
        linear_extrude(plate_thickness)
        {
            top_plate();

            translate([ board_plate_width, plate_height, 0 ]) rotate([0, 0, 180]) top_plate();
        }

        translate([ 0.5 * (board_plate_width - center_cover_width), 0, plate_thickness + 5 * mm ])
            linear_extrude(plate_thickness)
        {
            center_cover();
        }
    }
}

module case()
{
    for (i = [0:1])
    {
        // Offset halves to inspect joints
        // translate([ 0, 0, i * 5 * mm ])
        translate([ i * board_plate_width, i * plate_height, 0 ]) rotate([0, 0, i * 180])
        {
            translate([ 0, 0, plate_thickness ]) top_shell();

            translate([ 0, 0, -middle_shell_standoff_height ]) middle_shell();

            translate([0, plate_height, -middle_shell_standoff_height ]) rotate([180, 0, 0])
            bottom_shell();
        }
    }

}

module full_assembly()
{
    // pcb();
    // plates();
    case();
}

full_assembly();
