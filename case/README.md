# Riemann Case Files

## Common

- `top_plate.scad` - 2D design for top plate with MX switch cutouts.
One per hand, so a full keyboard requires two of these.

- `center_cover.scad` - Covers the area in between the hands to protect and
show off the electronics. Connects to the PCB using 4x 10mm M2 standoffs.

## Option 1: Laser-cut Bottom

This may be simpler and cheaper than the 3D-printed case. You can use
M2 standoffs to attach a bottom plate to the switch plate to cover the bottom
of the PCB. If the switch plate is 3mm acrylic, then 8mm standoffs are
recommended.

- `bottom_plate.scad` - 2D design for bottom plate. One per keyboard.
(Not compatible with shell; pick one or the other.)

- `plate_assembly.scad` - Extrusion, color and rendering of the assembly
using laser-cut plates.

## Option 2: 3D-Printed Case

If you want to have a completely 

- `top_shell.scad` - Goes on top of the switch plate. One per hand; two per keyboard.

- `middle_shell.scad` - Goes directly underneath and around the switch plate,
between the top and bottom shell pieces. One per hand; two per keyboard.

- `piccolo_io_shield.scad` - Inserted in the center between middle shell
pieces to cover the area around the rear I/O ports and buttons. Designed for
use with my USBC RPi Pico clone, the Piccolo.


- `bottom_shell.scad` - Goes underneath the middle shell; screws on through
the middle shell into the threaded inserts in the top shell. M2x16mm screws
are recommended. One per hand; two per keyboard.

- `case_assembly.scad` - Rendering of the assembly using the 3D-printed pieces.