include <vars.scad>

use <rounded_square.scad>
use <mounting.scad>

module top_shell()
{
	difference()
	{
		// Base shape
		linear_extrude(top_shell_height) difference()
		{
			translate([ -case_overhang, -case_overhang ])
			square([ hand_case_width + shell_center_lip, case_height ]);

			translate([ plate_margin, plate_margin ])
			square([ hand_keyway_width, keyway_height ]);

			// Through holes for center cover spacers
			translate([plate_margin + hand_keyway_width, plate_margin])
			{
				m2_spacer([center_cover_mounting_inset, center_cover_mounting_inset]);
				m2_spacer([center_cover_mounting_inset, keyway_height - center_cover_mounting_inset]);
			}

			translate([2 * plate_margin + hand_keyway_width, plate_margin])
				square([center_cover_width - 2 * plate_margin, keyway_height]);

			translate([hand_case_width - case_overhang, 0 ])
				square([shell_center_lip + 0.01 * mm, case_height - 2 * case_overhang]);
		}

		// Cutout for center cover
		translate([plate_margin + hand_keyway_width - 0.01 * mm, 0, top_shell_height - plate_thickness])
			cube([center_cover_width, plate_height, plate_thickness + 0.02 * mm]);

		// Threaded inserts to line up with switch plate
		translate([ 0, 0, -0.01 * mm ])
		linear_extrude(thread_depth)
		{
			m2_insert([ mount_left, mount_bottom ]);
			m2_insert([ mount_hand_hcenter, mount_bottom ]);
			m2_insert([ mount_hand_right, mount_bottom ]);
			m2_insert([ mount_left, mount_vcenter ]);
			m2_insert([ mount_left, mount_top ]);
			m2_insert([ mount_hand_hcenter, mount_top ]);
			m2_insert([ mount_hand_right, mount_top ]);
		}

		// Chamfer edges
		translate([ -case_overhang, -case_overhang ])
		{
			for (i = [0:1])
			{
				translate([ 0, i * case_height, 0 ])
				rotate([ 0, 0, 45 ])
				cube([ shell_chamfer, shell_chamfer, top_shell_height * 2.1 ], center = true);

				translate([ 0, i * case_height, top_shell_height ])
				rotate([ 45, 0, 0 ])
				cube([ hand_case_width * 2.1, shell_chamfer, shell_chamfer ], center = true);
			}

			translate([ 0, 0, top_shell_height ])
			rotate([ 0, 45, 0 ])
			cube([ shell_chamfer, case_height * 2.1, shell_chamfer ], center = true);
		}
	}
}

top_shell();
