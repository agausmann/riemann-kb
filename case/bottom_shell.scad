include <vars.scad>

use <mounting.scad>
use <rounded_square.scad>

module bottom_shell()
{
    difference()
    {
        // Base shape
        linear_extrude(bottom_shell_thickness) difference()
        {
            translate([ -case_overhang, -case_overhang ]) square([ bottom_shell_width, case_height ]);

            // Screw through holes
            m2_3dp([ mount_left, mount_bottom ]);
            m2_3dp([ mount_hand_hcenter, mount_bottom ]);
            m2_3dp([ mount_hand_right, mount_bottom ]);
            m2_3dp([ mount_left, mount_vcenter ]);
            m2_3dp([ mount_left, mount_top ]);
            m2_3dp([ mount_hand_hcenter, mount_top ]);
            m2_3dp([ mount_hand_right, mount_top ]);

            // Center dovetail joint
            translate([ bottom_shell_width - case_overhang - 0.5 * bottom_shell_joint_width, 0.3 * plate_height ])
                dovetail(bottom_shell_joint_width, 0.4 * plate_height + 0.1 * mm, bottom_shell_joint_skew);
            translate([ bottom_shell_width - case_overhang - 0.5 * bottom_shell_joint_width, 1.1 * plate_height ])
                dovetail(bottom_shell_joint_width, 0.4 * plate_height + 0.1 * mm, bottom_shell_joint_skew);
        }

        // M2 recessed screw heads
        translate([ 0, 0, bottom_shell_thickness - bottom_shell_screw_recess ])
            linear_extrude(bottom_shell_screw_recess + 0.01 * mm)
        {
            m2_head([ mount_left, mount_bottom ]);
            m2_head([ mount_hand_hcenter, mount_bottom ]);
            m2_head([ mount_hand_right, mount_bottom ]);
            m2_head([ mount_left, mount_vcenter ]);
            m2_head([ mount_left, mount_top ]);
            m2_head([ mount_hand_hcenter, mount_top ]);
            m2_head([ mount_hand_right, mount_top ]);
        }

        // Chamfer edges
        translate([ -case_overhang, -case_overhang ])
        {
            for (i = [0:1])
            {
                translate([ 0, i * case_height, 0 ]) rotate([ 0, 0, 45 ])
                    cube([ shell_chamfer, shell_chamfer, bottom_shell_thickness * 2.1 ], center = true);

                translate([ 0, i * case_height, bottom_shell_thickness ]) rotate([ 45, 0, 0 ])
                    cube([ bottom_shell_width * 2.1, shell_chamfer, shell_chamfer ], center = true);
            }

            translate([ 0, 0, bottom_shell_thickness ]) rotate([ 0, 45, 0 ])
                cube([ shell_chamfer, case_height * 2.1, shell_chamfer ], center = true);
        }

        // SKUF slots and engraved text
        translate([ 0, 0, bottom_shell_thickness - skuf_depth ]) linear_extrude(skuf_depth + 0.01 * mm) 
        {
            translate([ bottom_shell_skuf_inset + 0.5 * skuf_width, bottom_shell_skuf_inset + 0.5 * skuf_height ])
                skuf_slot();
            translate([
                bottom_shell_skuf_inset + 0.5 * skuf_width, plate_height - (bottom_shell_skuf_inset + 0.5 * skuf_height)
            ]) skuf_slot();
            translate([ hand_plate_width + 0.5 * skuf_height, 0.5 * plate_height ]) rotate([ 0, 0, 90 ]) skuf_slot();

            translate([hand_case_width, 0.5 * plate_height]) rotate([0, 0, 90])
                text("RIEMANN", halign = "center", valign = "top", size = 4 * mm);
        }

    }
}

bottom_shell();
