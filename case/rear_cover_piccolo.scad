include <vars.scad>

switch_cutout_position = [ -0.01, -0.01, 5 ] * mm;
switch_cutout_size = [ 8.01 * mm, front_cover_depth + 0.02 * mm, 6 * mm ];

usb_cutout_size = [ 10 * mm, front_cover_depth + 0.02 * mm, 3.5 * mm ];
usb_cutout_position = [ (front_cover_width - usb_cutout_size[0]) / 2, -0.01 * mm, 7 * mm ];

module rear_cover_piccolo()
{
    difference()
    {
        cube([ front_cover_width, front_cover_depth, front_cover_height ]);

        translate([ shell_center_lip, -0.01 * mm, -0.01 * mm ]) cube([
            front_cover_width - 2 * shell_center_lip, front_cover_depth - rear_cover_thickness + 0.01 * mm,
            front_cover_height + 0.02 *
            mm
        ]);

        translate(switch_cutout_position) cube(switch_cutout_size);
        translate(usb_cutout_position) cube(usb_cutout_size);
    }
}

rear_cover_piccolo();
