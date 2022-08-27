include <vars.scad>

module m2(position) {
    translate(position)
        circle(1.1 * mm);
}

module mx_1u() {
    square(14 * mm, center=true);
}