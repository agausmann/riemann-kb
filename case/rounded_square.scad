
module rounded_square(size,radius) {
    hull() {
        translate([radius, radius])
            circle(radius);
        translate([size.x - radius, radius])
            circle(radius);
        translate([radius, size.y - radius])
            circle(radius);
        translate([size.x - radius, size.y - radius])
            circle(radius);
    }
}