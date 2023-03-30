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

			translate([ hand_plate_width, 0 ])
			square([ shell_center_lip, plate_height ]);
		}

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
				cube([ hand_plate_width * 2.1, shell_chamfer, shell_chamfer ], center = true);
			}

			translate([ 0, 0, top_shell_height ])
			rotate([ 0, 45, 0 ])
			cube([ shell_chamfer, case_height * 2.1, shell_chamfer ], center = true);
		}
	}
}

top_shell();
