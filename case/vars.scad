$fs = 0.1;

mm = 1.0;
inch = 25.4;

rows = 5;
board_cols = 15;
hand_cols = 6;
center_cols = board_cols - 2 * hand_cols;

key_width = 0.75 * inch;
key_height = 0.75 * inch;

keyway_height = key_height * rows;
board_keyway_width = key_width * board_cols;
hand_keyway_width = key_width * hand_cols;
center_cover_width = key_width * center_cols;

plate_margin = 8 * mm;

plate_height = keyway_height + 2 * plate_margin;
board_plate_width = board_keyway_width + 2 * plate_margin;
hand_plate_width = hand_keyway_width + plate_margin; // margin only on outer edge

plate_corner_radius = 2 * mm;

mount_bottom = 0.5 * plate_margin;
mount_vcenter = 0.5 * plate_height;
mount_top = plate_height - 0.5 * plate_margin;

mount_left = 0.5 * plate_margin;
mount_hand_hcenter = 0.5 * hand_plate_width;
mount_hand_right = hand_plate_width - 0.5 * plate_margin;
mount_board_hcenter = 0.5 * board_plate_width;
mount_board_right = board_plate_width - 0.5 * plate_margin;

plate_thickness = 3 * mm;
case_depth = 10 * mm;

plate_color = [ 0, 0, 0, 0.2 ]; // rgba

center_cover_mounting_inset = 0.15 * inch;

case_overhang = 2 * mm;
hand_case_width = hand_plate_width + case_overhang + plate_margin;
middle_shell_bridge_height = 6 * mm;
case_height = plate_height + 2 * case_overhang;
top_shell_height = 8 * mm;
thread_depth = 6 * mm;
shell_chamfer = 2 * mm;
middle_shell_standoff_height = 8 * mm;
middle_shell_total_height = middle_shell_standoff_height + plate_thickness;
shell_center_lip = 2 * mm;
bottom_shell_thickness = 3 * mm;
bottom_shell_screw_recess = 2 * mm;
bottom_shell_skuf_inset = 8 * mm;
bottom_shell_joint_width = 2 * mm;
bottom_shell_joint_skew = 1 * mm;
bottom_shell_width = hand_plate_width + case_overhang + 0.5 * center_cover_width + 0.5 * bottom_shell_joint_width;

front_cover_width = center_cover_width - 2 * plate_margin;
front_cover_height = middle_shell_total_height + top_shell_height - plate_thickness;
front_cover_depth = plate_margin;

rear_cover_thickness = 1 * mm;

skuf_width = 42.0 * mm;
skuf_height = 6.0 * mm;
skuf_depth = 0.5 * mm;
