include <vars.scad>

use <rounded_square.scad>
use <mounting.scad>

module middle_shell()
{
	difference()
	{
		// Base shape
		linear_extrude(middle_shell_total_height) difference()
		{
			translate([ -case_overhang, -case_overhang ])
			square([ hand_case_width + shell_center_lip, case_height ]);

			translate([ plate_margin, plate_margin ])
			square([ hand_keyway_width, keyway_height ]);

			translate([ hand_plate_width, 0 ])
			square([ shell_center_lip, plate_height ]);

			// M2 screw holes to line up with switch plate
			m2([ mount_left, mount_bottom ]);
			m2([ mount_hand_hcenter, mount_bottom ]);
			m2([ mount_hand_right, mount_bottom ]);
			m2([ mount_left, mount_vcenter ]);
			m2([ mount_left, mount_top ]);
			m2([ mount_hand_hcenter, mount_top ]);
			m2([ mount_hand_right, mount_top ]);
		}

		// Plate recess
		translate([ 0, 0, middle_shell_standoff_height ])
		cube([ hand_plate_width, plate_height, plate_thickness + 0.01 * mm ]);

		// Chamfer edges
		translate([ -case_overhang, -case_overhang ])
		{
			for (i = [0:1])
			{
				translate([ 0, i * case_height, 0 ])
				rotate([ 0, 0, 45 ])
				cube([ shell_chamfer, shell_chamfer, middle_shell_total_height * 2.1 ], center = true);
			}
		}
	}
}

middle_shell();
